
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

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

    let app = Router::new()
        .route("/api/login", axum::routing::get(open_donate::auth::handlers::login))
        .route("/api/auth/callback", axum::routing::get(open_donate::auth::handlers::callback))
        .route("/api/logout", axum::routing::get(open_donate::auth::handlers::logout))
        .route("/sitemap.xml", axum::routing::get(open_donate::sitemap::sitemap_xml))
        .route("/robots.txt", axum::routing::get(open_donate::sitemap::robots_txt))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(axum::Extension(pool))
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
