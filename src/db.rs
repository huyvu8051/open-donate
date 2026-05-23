use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DbStreamer {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub bio: String,
    pub is_live: bool,
    pub user_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DbTransaction {
    pub id: i32,
    pub streamer_id: i32,
    pub donor_name: String,
    pub amount: f64,
    pub message: Option<String>,
    pub payment_method: String,
    pub created_at: String,
}

#[cfg(feature = "ssr")]
pub mod db_ops {
    use sqlx::PgPool;

    /// Seed sample data (NeonViper streamer) if the table is empty.
    /// Table creation is handled by SQLx migrations.
    pub async fn seed_data(pool: &PgPool) -> Result<(), sqlx::Error> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM streamers WHERE username = 'neonviper')"
        )
        .fetch_one(pool)
        .await?;

        if !exists {
            sqlx::query(
                "INSERT INTO streamers (username, display_name, avatar_url, bio, is_live, user_id)
                 VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind("neonviper")
            .bind("NeonViper")
            .bind("https://lh3.googleusercontent.com/aida-public/AB6AXuBLjEWKtvutw2bXJ6cXjX35VKSvndfcZFjksgkktDcFmWKH5w3JqiRsBENEnrWm0JHREPHPBQRwTGM2krlAjj-4IyFB_LtaFrMOvwlpVF-S4Wn-Qpc0Of9KKyyIayT9k7z69aL3NoVoXBzHPX-ZTbmlTm1ZFFq2kN49w8irdbwsj0edERW-AXu_cuLLa2XaiDOHQM4f5mbEU5MqTwigjzU5okvpS1kdr5WuV-yhcWwXphzBaqQ11rEVUtD0TpCxcHePnYEOrnYJOdc")
            .bind("Pushing the boundaries of competitive play. Today we're smashing the charity goals for the Digital Oceans Fund!")
            .bind(true)
            .bind(Some("seed_user_neonviper"))
            .execute(pool)
            .await?;
        }

        Ok(())
    }
}
