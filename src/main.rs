use std::error::Error;

use opentelemetry::sdk::trace::Tracer;
use opentelemetry::trace::TraceError;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;


mod yak_shave;

fn init_tracer() -> Result<Tracer, TraceError> {
    opentelemetry_jaeger::new_pipeline()
        .with_service_name("jaeger_example")
        .install_simple()
}

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let tracer = init_tracer().expect("Failed to initialize tracer");
    tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new("TRACE"))
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .try_init()
            .expect("Failed to register tracer with registry");

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = yak_shave::shave_all(number_of_yaks);
    info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );
    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}