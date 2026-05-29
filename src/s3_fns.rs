
#[server(CheckS3Status, "/api")]
pub async fn check_s3_status() -> Result<bool, ServerFnError> {
    use crate::s3::S3_STATUS;
    let mut rx = S3_STATUS.1.clone();
    
    // If it hasn't polled yet, wait for the first poll
    if rx.borrow().is_none() {
        let _ = rx.changed().await;
    }
    
    Ok(rx.borrow().unwrap_or(false))
}

#[server(GetStreamerMedia, "/api")]
pub async fn get_streamer_media() -> Result<Vec<crate::db::DbStreamerMedia>, ServerFnError> {
    let user = match get_me().await? {
        Some(u) => u,
        None => return Err(ServerFnError::new("Not authenticated")),
    };

    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let streamer_id: i32 = sqlx::query_scalar("SELECT id FROM streamers WHERE user_id = $1")
        .bind(&user.id)
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Streamer not found: {}", e)))?;

    let rows = sqlx::query_as!(
        crate::db::DbStreamerMedia,
        "SELECT id, streamer_id, file_name, file_url, size_bytes, TO_CHAR(created_at, 'YYYY-MM-DD HH24:MI:SS') as \"created_at!\" FROM streamer_media WHERE streamer_id = $1 ORDER BY created_at DESC",
        streamer_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

    Ok(rows)
}

#[server(GetDefaultMedias, "/api")]
pub async fn get_default_medias() -> Result<Vec<String>, ServerFnError> {
    // Return list of available system default audio files
    Ok(vec![
        "/default_donate.mp3".to_string(),
        "/audio/funny_1.mp3".to_string(),
        "/audio/cheer_1.mp3".to_string(),
    ])
}

#[server(SaveMediaSettings, "/api")]
pub async fn save_media_settings(
    selected_media_id: Option<uuid::Uuid>,
    fallback_media_file: String,
) -> Result<(), ServerFnError> {
    let user = match get_me().await? {
        Some(u) => u,
        None => return Err(ServerFnError::new("Not authenticated")),
    };

    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    sqlx::query(
        "UPDATE streamers SET selected_media_id = $1, fallback_media_file = $2 WHERE user_id = $3"
    )
    .bind(selected_media_id)
    .bind(fallback_media_file)
    .bind(&user.id)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database update failed: {}", e)))?;

    Ok(())
}

