use std::collections::HashMap;

use eyre::Result;
use opentelemetry::{KeyValue, trace::TracerProvider};
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::{Resource, trace};
use opentelemetry_semantic_conventions::{
    SCHEMA_URL,
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
};
use secrecy::ExposeSecret;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_tree::HierarchicalLayer;

mod config;

pub use config::Config;

// Create a Resource that captures information about the entity for which telemetry is recorded.
fn resource(environment: &str) -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
            KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, environment.to_string()),
        ],
        SCHEMA_URL,
    )
}

pub fn setup(config: &Config) -> Result<()> {
    let resource = resource(config.environment());

    let map = HashMap::from([
        (
            "x-environment".to_string(),
            config.environment().to_string(),
        ),
        (
            "x-honeycomb-team".to_string(),
            config.honeycomb_key().expose_secret().to_string(),
        ),
    ]);

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_headers(map)
        .with_endpoint(config.honeycomb_endpoint().clone())
        .build()?;

    let span_limits = trace::SpanLimits {
        max_events_per_span: 1024,
        ..Default::default()
    };

    let tracer = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .with_span_limits(span_limits)
        .with_resource(resource)
        .build()
        .tracer("sql-grimoire");

    let telemetry = tracing_opentelemetry::OpenTelemetryLayer::new(tracer);

    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(
            HierarchicalLayer::new(*config.tree_indent_count())
                .with_targets(true)
                .with_indent_lines(true)
                .with_bracketed_fields(true)
                .with_thread_names(true)
                .with_thread_ids(true),
        )
        .with(telemetry)
        .init();

    Ok(())
}

pub fn teardown() {
    opentelemetry::global::shutdown_tracer_provider();
}
