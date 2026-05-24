//! Sitemap and robots.txt handlers for SEO.

use axum::{
    extract::Extension,
    http::{header, StatusCode},
    response::IntoResponse,
};
use sqlx::PgPool;

/// Handler for GET /sitemap.xml
/// Dynamically generates a sitemap including all public pages and streamer pages.
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

    // Static public pages
    let static_pages = [
        ("/", "1.0", "daily"),
        ("/explore", "0.9", "hourly"),
        ("/leaderboard", "0.8", "hourly"),
        ("/about", "0.5", "monthly"),
        ("/faq", "0.5", "monthly"),
        ("/privacy", "0.3", "monthly"),
        ("/terms", "0.3", "monthly"),
    ];

    for (path, priority, changefreq) in &static_pages {
        xml.push_str(&format!(
            r#"
  <url>
    <loc>{base_url}{path}</loc>
    <lastmod>{today}</lastmod>
    <changefreq>{changefreq}</changefreq>
    <priority>{priority}</priority>
  </url>"#
        ));
    }

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
        "User-agent: *\nAllow: /\nDisallow: /dashboard/\nDisallow: /overlay/\nDisallow: /api/\n\nSitemap: {base_url}/sitemap.xml\n"
    );

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        body,
    )
}
