use crate::config::Config;
use opentelemetry::sdk::trace::Tracer;
use opentelemetry::{
    sdk::{propagation::TraceContextPropagator, trace::Sampler, Resource},
    trace::TraceError,
};
use tracing::{subscriber::SetGlobalDefaultError, Subscriber};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(thiserror::Error, Debug)]
pub enum TracingError {
    #[error("Failed to connect with opentelemetry '{0}'")]
    ConnectionFailed(#[from] BoxError),
    #[error("Failed to set global output'{0}'")]
    SetGlobalDefaultError(#[from] SetGlobalDefaultError),
}

fn build_logger_text<S>() -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    Box::new(
        tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .with_timer(tracing_subscriber::fmt::time::uptime()),
    )
}

pub(crate) fn build_loglevel_filter_layer(
    config: &Config,
) -> tracing_subscriber::filter::EnvFilter {
    // filter what is output on log (fmt)
    // std::env::set_var("RUST_LOG", "warn,axum_tracing_opentelemetry=info,otel=debug");
    let log_level = config.log_level.to_string();
    std::env::set_var(
        "RUST_LOG",
        format!(
            // `axum_tracing_opentelemetry` should be a level info to emit opentelemetry trace & span
            // `otel::setup` set to debug to log detected resources, configuration read and infered
            "{},axum_tracing_opentelemetry=info,otel=debug",
            log_level
        ),
    );
    EnvFilter::from_default_env()
}

fn infer_protocol_and_endpoint(
    (maybe_protocol, maybe_endpoint): (Option<String>, Option<String>),
) -> (String, String) {
    let protocol = maybe_protocol.unwrap_or_else(|| {
        match maybe_endpoint
            .as_ref()
            .map(|e| e.contains(":4317"))
            .unwrap_or(false)
        {
            true => "grpc",
            false => "http/protobuf",
        }
        .to_string()
    });

    let endpoint = match protocol.as_str() {
        "http/protobuf" => maybe_endpoint.unwrap_or_else(|| "http://localhost:4318".to_string()), //Devskim: ignore DS137138
        _ => maybe_endpoint.unwrap_or_else(|| "http://localhost:4317".to_string()), //Devskim: ignore DS137138
    };

    (protocol, endpoint)
}

pub(crate) fn init_tracer<F>(
    config: &Config,
    resource: Resource,
    transform: F,
) -> Result<Tracer, TraceError>
where
    F: FnOnce(opentelemetry_otlp::OtlpTracePipeline) -> opentelemetry_otlp::OtlpTracePipeline,
{
    use opentelemetry_otlp::SpanExporterBuilder;
    use opentelemetry_otlp::WithExportConfig;

    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
    let (protocol, endpoint) =
        infer_protocol_and_endpoint((None, Some(config.otlp_url.clone().unwrap())));
    tracing::debug!(target: "otel::setup", OTEL_EXPORTER_OTLP_ENDPOINT = endpoint);
    tracing::debug!(target: "otel::setup", OTEL_EXPORTER_OTLP_PROTOCOL = protocol);
    let exporter: SpanExporterBuilder = match protocol.as_str() {
        "http/protobuf" => opentelemetry_otlp::new_exporter()
            .http()
            .with_endpoint(endpoint)
            .into(),
        _ => opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(endpoint)
            .into(),
    };
    println!("exporter: {:?}", exporter);

    let mut pipeline = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            opentelemetry::sdk::trace::config()
                .with_resource(resource)
                .with_sampler(Sampler::AlwaysOn),
        );
    pipeline = transform(pipeline);
    pipeline.install_batch(opentelemetry::runtime::Tokio)
}

pub(crate) fn build_otel_layer<S>(
    config: &Config,
) -> Result<OpenTelemetryLayer<S, Tracer>, BoxError>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    use axum_tracing_opentelemetry::{init_propagator, otlp, resource::DetectResource};
    let otel_rsrc = DetectResource::default().build();
    let otel_tracer = init_tracer(config, otel_rsrc, otlp::identity)?;
    init_propagator()?;
    Ok(tracing_opentelemetry::layer().with_tracer(otel_tracer))
}

pub(crate) fn setup_tracing(config: &mut Config) -> Result<(), TracingError> {
    if config.otlp_url.is_some() {
        let subscriber = tracing_subscriber::registry()
            .with(build_loglevel_filter_layer(config))
            .with(build_logger_text());
        let _guard = tracing::subscriber::set_default(subscriber);
        tracing::info!("init logging & tracing");

        let subscriber = tracing_subscriber::registry()
            .with(build_otel_layer(config)?)
            .with(build_loglevel_filter_layer(config))
            .with(build_logger_text());
        tracing::subscriber::set_global_default(subscriber)?;
    } else {
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Into::<tracing::Level>::into(config.log_level.clone()))
            .init();
    }
    Ok(())
}
