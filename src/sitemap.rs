//! Sitemap and robots.txt handlers for SEO.

use axum::{
    extract::Extension,
    http::{header, StatusCode},
    response::IntoResponse,
};
use sqlx::PgPool;

/// Handler for GET /sitemap.xml
/// Dynamically generates a sitemap including all streamer pages.
pub async fn sitemap_xml(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let base_url = std::env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let rows = sqlx::query_scalar::<_, String>(
        "SELECT username FROM streamers ORDER BY id"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);

    // Homepage
    xml.push_str(&format!(
        r#"
  <url>
    <loc>{base_url}/</loc>
    <lastmod>{today}</lastmod>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>"#
    ));

    // Streamer pages
    for username in &rows {
        xml.push_str(&format!(
            r#"
  <url>
    <loc>{base_url}/streamer/{username}</loc>
    <lastmod>{today}</lastmod>
    <changefreq>daily</changefreq>
    <priority>0.8</priority>
  </url>"#
        ));
    }

    xml.push_str("\n</urlset>\n");

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        xml,
    )
}

/// Handler for GET /robots.txt
pub async fn robots_txt() -> impl IntoResponse {
    let base_url = std::env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let body = format!(
        "User-agent: *\nAllow: /\n\nSitemap: {base_url}/sitemap.xml\n"
    );

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        body,
    )
}
