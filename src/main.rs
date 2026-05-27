
#[cfg(feature = "ssr")]
fn init_tracing() {
    use opentelemetry::global;
    use opentelemetry::KeyValue;
    use opentelemetry_otlp::{SpanExporter, LogExporter};
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::trace::SdkTracerProvider;
    use opentelemetry_sdk::logs::SdkLoggerProvider;
    use opentelemetry_sdk::Resource;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;

    let otlp_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "https://otel.unghotui.vn/v1/traces".to_string());

    let exporter = SpanExporter::builder()
        .with_http()
        .with_endpoint(otlp_endpoint)
        .build()
        .expect("Failed to create OTLP exporter");

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(
            Resource::builder_empty()
                .with_attributes(vec![KeyValue::new("service.name", "open-donate")])
                .build()
        )
        .build();

    global::set_tracer_provider(provider.clone());
    let tracer = global::tracer("open-donate");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Setup Log Exporter
    let otlp_logs_endpoint = std::env::var("OTEL_EXPORTER_OTLP_LOGS_ENDPOINT")
        .unwrap_or_else(|_| "https://otel.unghotui.vn/v1/logs".to_string());

    let log_exporter = LogExporter::builder()
        .with_http()
        .with_endpoint(otlp_logs_endpoint)
        .build()
        .expect("Failed to create OTLP log exporter");

    let logger_provider = SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .with_resource(
            Resource::builder_empty()
                .with_attributes(vec![KeyValue::new("service.name", "open-donate")])
                .build()
        )
        .build();

    let log_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "open_donate=debug,info,axum_tracing_opentelemetry=error".into());

    let fmt_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .with(telemetry)
        .with(log_layer)
        .init();
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    init_tracing();

    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use open_donate::app::*;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Connect to app database
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url).await.expect("Failed to connect to database");

    // Run SQLx migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // Seed sample data
    open_donate::db::db_ops::seed_data(&pool).await.expect("Failed to seed sample data");

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    use tower_sessions::{SessionManagerLayer, Expiry};
    use tower_sessions_sqlx_store::PostgresStore;

    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await.unwrap();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(tower_sessions::cookie::time::Duration::days(14)));

    let app = Router::new()
        .route("/sitemap.xml", axum::routing::get(open_donate::sitemap::sitemap_xml))
        .route("/robots.txt", axum::routing::get(open_donate::sitemap::robots_txt))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(axum::Extension(pool))
        .layer(session_layer)
        .layer(axum_tracing_opentelemetry::middleware::OtelAxumLayer::default())
        .with_state(leptos_options);

    // run our app with hyper
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
