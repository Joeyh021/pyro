mod api;
mod config;
mod runner;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    init_tracing();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(api::app().into_make_service())
        .await
        .expect("Could not start server");

    opentelemetry::global::shutdown_tracer_provider();
}

fn init_tracing() {
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint("localhost:6831")
        .with_service_name("pyromaniac")
        .with_auto_split_batch(true)
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Could not init otel tracer");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
}
