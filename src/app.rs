
#[allow(unused_imports)]
use crate::db::{TransactionStatus, PaymentMethod};
#[allow(unused_imports)]
use leptos_router::hooks::{use_location, use_params_map};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    ParamSegment, StaticSegment,
};
use crate::auth::User;
use crate::db::{DbStreamer, DbTransaction};
use serde::{Deserialize, Serialize};


use crate::pages::landing::LandingPage;
use crate::pages::explore::ExplorePage;
use crate::pages::leaderboard::LeaderboardPage;
use crate::pages::streamer::StreamerPage;



#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StreamerAnalytics {
    pub total_revenue: f64,
    pub donation_count: i64,
    pub avg_donation: f64,
    pub top_single_donation: f64,
    pub top_donors: Vec<(String, f64)>,
    pub revenue_over_time: Vec<(String, f64)>,
    pub payment_method_breakdown: Vec<(String, i64)>,
    pub amount_distribution: Vec<(String, i64)>,
    pub cumulative_revenue: Vec<(String, f64)>,
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html class="dark" lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link
                    href="https://fonts.googleapis.com/css2?family=Montserrat:wght@700;800&family=Inter:wght@400;500;600&display=swap"
                    rel="stylesheet"
                />
                <link
                    href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:wght,FILL@100..700,0..1&display=swap"
                    rel="stylesheet"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="bg-background text-on-background selection:bg-primary selection:text-on-primary text-left min-h-screen flex flex-col">
                <App />
            </body>
        </html>
    }
}

#[component]
fn I18nProvider(children: Children) -> impl IntoView {
    leptos_fluent::leptos_fluent! {
        children: children(),
        locales: "./locales",
        default_language: "en",
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Load current user
    let user_resource = Resource::new(|| (), |_| async move { crate::utils::with_min_delay(get_me()).await });
    provide_context(user_resource);

    view! {
        <I18nProvider>
            <Stylesheet id="leptos" href="/pkg/open-donate.css" />
            <Title text="Glint | Empower Your Content" />

            <Router>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=LandingPage />
                    <Route path=StaticSegment("explore") view=ExplorePage />
                    <Route path=StaticSegment("leaderboard") view=LeaderboardPage />
                    <Route path=StaticSegment("about") view=crate::pages::AboutPage />
                    <Route path=StaticSegment("faq") view=crate::pages::FaqPage />
                    <Route path=StaticSegment("privacy") view=crate::pages::PrivacyPage />
                    <Route path=StaticSegment("terms") view=crate::pages::TermsPage />
                    <ParentRoute
                        path=StaticSegment("dashboard")
                        view=crate::dashboard::DashboardLayout
                    >
                        <Route path=StaticSegment("") view=crate::dashboard::DashboardHome />
                        <Route path=StaticSegment("settings") view=crate::dashboard::SettingsPage />
                        <Route path=StaticSegment("payments") view=crate::dashboard::PaymentsPage />
                        <Route
                            path=StaticSegment("analytics")
                            view=crate::dashboard::AnalyticsPage
                        />
                    </ParentRoute>
                    <Route path=StaticSegment("login") view=crate::pages::login::LoginPage />
                    <Route
                        path=StaticSegment("register")
                        view=crate::pages::register::RegisterPage
                    />
                    <Route
                        path=(StaticSegment("streamer"), ParamSegment("username"))
                        view=StreamerPage
                    />
                    <Route
                        path=(StaticSegment("overlay"), ParamSegment("token"))
                        view=crate::overlay::OverlayPage
                    />
                </Routes>
            </Router>
        </I18nProvider>
    }
}


#[server(GetMe, "/api")]
pub async fn get_me() -> Result<Option<User>, ServerFnError> {
    let session = match leptos_axum::extract::<tower_sessions::Session>().await {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };

    let user: Option<User> = session.get("user").await.unwrap_or(None);
    Ok(user)
}


#[server(GetOrCreateStreamer, "/api")]
pub async fn get_or_create_streamer() -> Result<Option<DbStreamer>, ServerFnError> {
    let user = match get_me().await? {
        Some(u) => u,
        None => return Ok(None),
    };

    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let existing = sqlx::query(
        "SELECT id, username, display_name, avatar_url, bio, is_live, user_id, overlay_token, active_overlay_session, payment_methods, overlay_paused, overlay_sound_enabled, selected_media_id, fallback_media_file FROM streamers WHERE user_id = $1"
    )
    .bind(&user.id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

    use sqlx::Row;

    if let Some(r) = existing {
        return Ok(Some(DbStreamer {
            id: r.get("id"),
            username: r.get("username"),
            display_name: r.get("display_name"),
            avatar_url: r.get("avatar_url"),
            bio: r.get("bio"),
            is_live: r.get("is_live"),
            user_id: r.try_get("user_id").unwrap_or(None),
            overlay_token: r.get("overlay_token"),
            active_overlay_session: r.try_get("active_overlay_session").unwrap_or(None),
            payment_methods: r.try_get("payment_methods").unwrap_or_else(|_| vec![PaymentMethod::MockAuto, PaymentMethod::MockManual]),
            overlay_paused: r.try_get("overlay_paused").unwrap_or(false),
            overlay_sound_enabled: r.try_get("overlay_sound_enabled").unwrap_or(true),
            selected_media_id: r.try_get("selected_media_id").unwrap_or(None),
            fallback_media_file: r.try_get("fallback_media_file").unwrap_or_else(|_| "/default_donate.mp3".to_string()),
        }));
    }

    let prefix = user.email.split('@').next().unwrap_or("user").to_string();
    let mut username = prefix.clone();

    for _ in 0..10 {
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM streamers WHERE username = $1)")
            .bind(&username)
            .fetch_one(&pool)
            .await
            .unwrap_or(true);

        if !exists {
            break;
        }

        let random_num = rand::random::<u16>();
        username = format!("{}_{}", prefix, random_num);
    }

    let display_name = if user.name.is_empty() { prefix.clone() } else { user.name.clone() };
    let bio = "New to Glint!".to_string();
    let avatar_url = format!("https://api.dicebear.com/9.x/avataaars/svg?seed={}", urlencoding::encode(&username));

    let row = sqlx::query(
        "INSERT INTO streamers (username, display_name, avatar_url, bio, is_live, user_id, overlay_token, payment_methods, overlay_paused, overlay_sound_enabled)
         VALUES ($1, $2, $3, $4, $5, $6, gen_random_uuid(), $7, false, true)
         RETURNING id, username, display_name, avatar_url, bio, is_live, user_id, overlay_token, active_overlay_session, payment_methods, overlay_paused, overlay_sound_enabled, selected_media_id, fallback_media_file"
    )
    .bind(&username)
    .bind(&display_name)
    .bind(&avatar_url)
    .bind(&bio)
    .bind(false)
    .bind(&user.id)
    .bind(vec![PaymentMethod::MockAuto, PaymentMethod::MockManual])
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database insert failed: {}", e)))?;

    Ok(Some(DbStreamer {
        id: row.get("id"),
        username: row.get("username"),
        display_name: row.get("display_name"),
        avatar_url: row.get("avatar_url"),
        bio: row.get("bio"),
        is_live: row.get("is_live"),
        user_id: row.try_get("user_id").unwrap_or(None),
        overlay_token: row.get("overlay_token"),
        active_overlay_session: row.try_get("active_overlay_session").unwrap_or(None),
        payment_methods: row.try_get("payment_methods").unwrap_or_else(|_| vec![PaymentMethod::MockAuto, PaymentMethod::MockManual]),
        overlay_paused: row.try_get("overlay_paused").unwrap_or(false),
        overlay_sound_enabled: row.try_get("overlay_sound_enabled").unwrap_or(true),
        selected_media_id: row.try_get("selected_media_id").unwrap_or(None),
        fallback_media_file: row.try_get("fallback_media_file").unwrap_or_else(|_| "/default_donate.mp3".to_string()),
    }))
}

#[server(UpdateStreamerProfile, "/api")]
pub async fn update_streamer_profile(
    new_display_name: String,
    new_bio: String,
    new_username: String,
    payment_methods: Option<Vec<String>>,
) -> Result<(), ServerFnError> {
    let user = match get_me().await? {
        Some(u) => u,
        None => return Err(ServerFnError::new("Unauthorized")),
    };

    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    // Check if the new username is already taken by a different user
    let existing_user_with_username: Option<String> = sqlx::query_scalar(
        "SELECT user_id FROM streamers WHERE username = $1"
    )
    .bind(&new_username)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

    if let Some(existing_user_id) = existing_user_with_username {
        if existing_user_id != user.id {
            return Err(ServerFnError::new("Username is already taken by another user."));
        }
    }

    sqlx::query(
        "UPDATE streamers SET display_name = $1, bio = $2, username = $3, payment_methods = $4 WHERE user_id = $5"
    )
    .bind(&new_display_name)
    .bind(&new_bio)
    .bind(&new_username)
    .bind(&payment_methods.unwrap_or_default())
    .bind(&user.id)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database update failed: {}", e)))?;

    Ok(())
}

#[server(GetStreamerAnalytics, "/api")]
pub async fn get_streamer_analytics(streamer_id: i32, time_range: String) -> Result<StreamerAnalytics, ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    use sqlx::Row;

    // === Basic stats ===
    let stats = sqlx::query(
        "SELECT COALESCE(SUM(amount), 0) as total_revenue, COUNT(*) as donation_count,
                COALESCE(AVG(amount), 0) as avg_donation,
                COALESCE(MAX(amount), 0) as top_single_donation
         FROM transactions 
         WHERE streamer_id = $1"
    )
    .bind(streamer_id)
    .fetch_one(&pool)
    .await;

    let (total_revenue, donation_count, avg_donation, top_single_donation) = match stats {
        Ok(r) => (
            r.try_get::<f64, _>("total_revenue").unwrap_or(0.0),
            r.try_get::<i64, _>("donation_count").unwrap_or(0),
            r.try_get::<f64, _>("avg_donation").unwrap_or(0.0),
            r.try_get::<f64, _>("top_single_donation").unwrap_or(0.0),
        ),
        Err(_) => (0.0, 0, 0.0, 0.0),
    };

    // === Top donors ===
    let top_donors_rows = sqlx::query(
        "SELECT donor_name, SUM(amount) as total_donated 
         FROM transactions 
         WHERE streamer_id = $1
         GROUP BY donor_name 
         ORDER BY total_donated DESC 
         LIMIT 10"
    )
    .bind(streamer_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let top_donors = top_donors_rows.into_iter().map(|r| {
        (r.get::<String, _>("donor_name"), r.get::<f64, _>("total_donated"))
    }).collect();

    // === Revenue over time ===
    let interval = match time_range.as_str() { "day" => "24 hours", "month" => "30 days", _ => "7 days" };
    let date_format = match time_range.as_str() { "day" => "HH24:00", _ => "MM-DD" };
    let revenue_time_rows = sqlx::query(
        &format!(
            "SELECT TO_CHAR(created_at, '{}') as date, SUM(amount) as daily_revenue
             FROM transactions 
             WHERE streamer_id = $1 AND created_at >= NOW() - INTERVAL '{}'
             GROUP BY TO_CHAR(created_at, '{}')
             ORDER BY date ASC",
            date_format, interval, date_format
        )
    )
    .bind(streamer_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let revenue_over_time: Vec<(String, f64)> = revenue_time_rows.into_iter().map(|r| {
        (r.get::<String, _>("date"), r.get::<f64, _>("daily_revenue"))
    }).collect();

    // === Cumulative revenue ===
    let cumulative_revenue: Vec<(String, f64)> = {
        let mut running = 0.0_f64;
        revenue_over_time.iter().map(|(date, rev)| {
            running += rev;
            (date.clone(), running)
        }).collect()
    };

    // === Payment method breakdown ===
    let payment_rows = sqlx::query(
        "SELECT payment_method::text as method, COUNT(*) as cnt
         FROM transactions
         WHERE streamer_id = $1
         GROUP BY payment_method
         ORDER BY cnt DESC"
    )
    .bind(streamer_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let payment_method_breakdown: Vec<(String, i64)> = payment_rows.into_iter().map(|r| {
        (r.get::<String, _>("method"), r.get::<i64, _>("cnt"))
    }).collect();

    // === Amount distribution (buckets) ===
    let bucket_rows = sqlx::query(
        "SELECT
            CASE
                WHEN amount < 1    THEN '< $1'
                WHEN amount < 5    THEN '$1 - $5'
                WHEN amount < 10   THEN '$5 - $10'
                WHEN amount < 50   THEN '$10 - $50'
                WHEN amount < 100  THEN '$50 - $100'
                ELSE '$100+'
            END as bucket,
            COUNT(*) as cnt
         FROM transactions
         WHERE streamer_id = $1
         GROUP BY bucket
         ORDER BY MIN(amount) ASC"
    )
    .bind(streamer_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let amount_distribution: Vec<(String, i64)> = bucket_rows.into_iter().map(|r| {
        (r.get::<String, _>("bucket"), r.get::<i64, _>("cnt"))
    }).collect();

    Ok(StreamerAnalytics {
        total_revenue,
        donation_count,
        avg_donation,
        top_single_donation,
        top_donors,
        revenue_over_time,
        payment_method_breakdown,
        amount_distribution,
        cumulative_revenue,
    })
}








#[server(GetDashboardTransactions, "/api")]
pub async fn get_dashboard_transactions(streamer_id: i32, page: i64, page_size: i64, _trigger: i32) -> Result<(Vec<DbTransaction>, i64), ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let offset = (page - 1) * page_size;

    let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM transactions WHERE streamer_id = $1")
        .bind(streamer_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database count failed: {}", e)))?;

    let rows = sqlx::query(
        "SELECT t.id, t.streamer_id, t.donor_name, t.amount, t.message, t.payment_method, t.status, TO_CHAR(t.created_at, 'YYYY-MM-DD HH:MI AM') as formatted_date 
         FROM transactions t
         WHERE t.streamer_id = $1 
         ORDER BY t.id DESC 
         LIMIT $2 OFFSET $3"
    )
    .bind(streamer_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

    use sqlx::Row;
    let txs = rows.into_iter().map(|r| {
        DbTransaction {
            id: r.get("id"),
            streamer_id: r.get("streamer_id"),
            donor_name: r.get("donor_name"),
            amount: r.get("amount"),
            message: r.get("message"),
            payment_method: r.get("payment_method"),
            status: r.get("status"),
            created_at: r.get("formatted_date"),
        }
    }).collect();

    Ok((txs, total_count))
}


#[server(GetReadyForDisplayTransactions, "/api")]
pub async fn get_ready_for_display_transactions(
    username: String,
) -> Result<Vec<DbTransaction>, ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let rows = sqlx::query(
        "SELECT t.id, t.streamer_id, t.donor_name, t.amount, t.message, t.payment_method, t.status, TO_CHAR(t.created_at, 'YYYY-MM-DD HH:MI AM') as formatted_date
         FROM transactions t
         JOIN streamers s ON t.streamer_id = s.id
         WHERE s.username = $1 AND t.status = 'READY_FOR_DISPLAY'
         ORDER BY t.id ASC
         LIMIT 10"
    )
    .bind(username)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

    use sqlx::Row;
    Ok(rows
        .into_iter()
        .map(|r| DbTransaction {
            id: r.get("id"),
            streamer_id: r.get("streamer_id"),
            donor_name: r.get("donor_name"),
            amount: r.get("amount"),
            message: r.get("message"),
            payment_method: r.get("payment_method"),
            status: r.get("status"),
            created_at: r.get("formatted_date"),
        })
        .collect())
}

#[server(MarkTransactionDisplayed, "/api")]
pub async fn mark_transaction_displayed(tx_id: i32) -> Result<(), ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    sqlx::query(
        "UPDATE transactions SET status = 'DISPLAYED' WHERE id = $1 AND status = 'READY_FOR_DISPLAY'"
    )
    .bind(tx_id)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database update failed: {}", e)))?;

    Ok(())
}

#[server(InitOverlaySession, "/api")]
pub async fn init_overlay_session(token: String, session_id: String) -> Result<(), ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let rows_affected = sqlx::query(
        "UPDATE streamers SET active_overlay_session = $1 WHERE overlay_token = $2"
    )
    .bind(&session_id)
    .bind(&token)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database update failed: {}", e)))?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ServerFnError::new("Invalid overlay token"));
    }

    Ok(())
}

#[server(PrefetchUpcomingTransactions, "/api")]
pub async fn prefetch_upcoming_transactions(token: String, session_id: String) -> Result<(Vec<DbTransaction>, bool, bool, Option<String>, String), ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let streamer = sqlx::query(
        "SELECT s.id, s.active_overlay_session, s.overlay_paused, s.overlay_sound_enabled, sm.file_url, s.fallback_media_file 
         FROM streamers s 
         LEFT JOIN streamer_media sm ON s.selected_media_id = sm.id 
         WHERE s.overlay_token = $1"
    )
    .bind(&token)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

    use sqlx::Row;
    let streamer = match streamer {
        Some(s) => s,
        None => return Err(ServerFnError::new("Invalid overlay token")),
    };

    let active_session: Option<String> = streamer.try_get("active_overlay_session").unwrap_or(None);
    if active_session.as_deref() != Some(&session_id) {
        return Err(ServerFnError::ServerError("SessionRevoked".to_string()));
    }

    let streamer_id: i32 = streamer.get("id");
    let overlay_paused: bool = streamer.try_get("overlay_paused").unwrap_or(false);
    let overlay_sound_enabled: bool = streamer.try_get("overlay_sound_enabled").unwrap_or(true);
    let primary_media_url: Option<String> = streamer.try_get("file_url").unwrap_or(None);
    let fallback_media_file: String = streamer.try_get("fallback_media_file").unwrap_or_else(|_| "/default_donate.mp3".to_string());

    let _ = sqlx::query("UPDATE streamers SET last_overlay_ping = CURRENT_TIMESTAMP WHERE id = $1")
        .bind(streamer_id)
        .execute(&pool)
        .await;

    if overlay_paused {
        return Ok((vec![], overlay_paused, overlay_sound_enabled, primary_media_url, fallback_media_file));
    }

    let rows = sqlx::query(
        "SELECT t.id, t.streamer_id, t.donor_name, t.amount, t.message, t.payment_method, t.status, TO_CHAR(t.created_at, 'YYYY-MM-DD HH:MI AM') as formatted_date 
         FROM transactions t
         WHERE t.streamer_id = $1 AND t.status = 'READY_FOR_DISPLAY'
         ORDER BY t.id ASC
         LIMIT 5"
    )
    .bind(streamer_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

    let txs: Vec<DbTransaction> = rows.into_iter().map(|r| DbTransaction {
        id: r.get("id"),
        streamer_id: r.get("streamer_id"),
        donor_name: r.get("donor_name"),
        amount: r.get("amount"),
        message: r.get("message"),
        payment_method: r.get("payment_method"),
        status: r.get("status"),
        created_at: r.get("formatted_date"),
    }).collect();

    Ok((txs, overlay_paused ,overlay_sound_enabled, primary_media_url, fallback_media_file))
}

#[server(LockTransaction, "/api")]
pub async fn lock_transaction(token: String, session_id: String, tx_id: i32) -> Result<bool, ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let streamer = sqlx::query(
        "SELECT id, active_overlay_session FROM streamers WHERE overlay_token = $1"
    )
    .bind(&token)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

    use sqlx::Row;
    let streamer = match streamer {
        Some(s) => s,
        None => return Err(ServerFnError::new("Invalid overlay token")),
    };

    let active_session: Option<String> = streamer.try_get("active_overlay_session").unwrap_or(None);
    if active_session.as_deref() != Some(&session_id) {
        return Err(ServerFnError::ServerError("SessionRevoked".to_string()));
    }

    let streamer_id: i32 = streamer.get("id");

    // Try to update the transaction status from READY_FOR_DISPLAY to DISPLAYED
    let res = sqlx::query(
        "UPDATE transactions SET status = 'DISPLAYED' WHERE id = $1 AND streamer_id = $2 AND status = 'READY_FOR_DISPLAY'"
    )
    .bind(tx_id)
    .bind(streamer_id)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database update failed: {}", e)))?;

    // If rows_affected > 0, we successfully locked it
    Ok(res.rows_affected() > 0)
}


#[server(GetOverlayStatus, "/api")]
pub async fn get_overlay_status() -> Result<bool, ServerFnError> {
    let user = match get_me().await {
        Ok(Some(u)) => u,
        _ => return Err(ServerFnError::new("Unauthorized")),
    };

    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let row = sqlx::query(
        "SELECT last_overlay_ping FROM streamers WHERE user_id = $1"
    )
    .bind(user.id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    use sqlx::Row;
    if let Some(r) = row {
        if let Ok(ping) = r.try_get::<chrono::DateTime<chrono::Utc>, _>("last_overlay_ping") {
            let now = chrono::Utc::now();
            if now.signed_duration_since(ping).num_seconds() <= 10 {
                return Ok(true);
            }
        }
    }
    
    Ok(false)
}

#[server(TestOverlayDonation, "/api")]
pub async fn test_overlay_donation() -> Result<(), ServerFnError> {
    let user = match get_me().await {
        Ok(Some(u)) => u,
        _ => return Err(ServerFnError::new("Unauthorized")),
    };

    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let row = sqlx::query("SELECT id FROM streamers WHERE user_id = $1")
        .bind(user.id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    use sqlx::Row;
    let streamer_id: i32 = match row {
        Some(r) => r.get("id"),
        None => return Err(ServerFnError::new("Streamer not found")),
    };

    let (donor_name, amount, message) = {
        #[cfg(feature = "ssr")]
        {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let donor_names = [
                "DragonSlayer99", "MoonWatcher", "PixelKnight", "StardustGamer",
                "NightOwl_42", "CrypticFox", "SilverArrow", "ThunderBolt",
                "CosmicRaider", "NeonPhoenix", "IronWolf", "QuantumByte"
            ];
            let donor_name = donor_names[rng.gen_range(0..donor_names.len())];
            
            let amounts = [1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0];
            let amount = amounts[rng.gen_range(0..amounts.len())];
            
            let message_bodies = [
                "Keep up the great content!",
                "Love watching your streams!",
                "You're the best streamer out there!",
                "GG ez, but seriously great game!",
                "Pog! That was insane!",
                "First time donating, won't be the last!",
                "Your community is amazing ❤️",
                "This stream made my day!"
            ];
            let body = message_bodies[rng.gen_range(0..message_bodies.len())];
            let message = format!("{}", body);
            
            (donor_name.to_string(), amount, message)
        }
        #[cfg(not(feature = "ssr"))]
        {
            ("System Test".to_string(), 5.0, "This is a test donation for your overlay!".to_string())
        }
    };

    sqlx::query(
        "INSERT INTO transactions (streamer_id, donor_name, amount, message, payment_method, status)
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(streamer_id)
    .bind(donor_name)
    .bind(amount)
    .bind(message)
    .bind("test_mock")
    .bind(TransactionStatus::ReadyForDisplay)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to insert test donation: {}", e)))?;
    Ok(())
}


#[server(ToggleOverlayPause, "/api")]
pub async fn toggle_overlay_pause(paused: bool) -> Result<(), ServerFnError> {
    let user = get_me().await?.ok_or_else(|| ServerFnError::new("Unauthorized"))?;
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;
    sqlx::query("UPDATE streamers SET overlay_paused = $1 WHERE user_id = $2")
        .bind(paused).bind(&user.id).execute(&pool).await
        .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;
    Ok(())
}

#[server(ToggleOverlaySound, "/api")]
pub async fn toggle_overlay_sound(enabled: bool) -> Result<(), ServerFnError> {
    let user = get_me().await?.ok_or_else(|| ServerFnError::new("Unauthorized"))?;
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;
    sqlx::query("UPDATE streamers SET overlay_sound_enabled = $1 WHERE user_id = $2")
        .bind(enabled).bind(&user.id).execute(&pool).await
        .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;
    Ok(())
}

#[server(MarkTransactionViewed, "/api")]
pub async fn mark_transaction_viewed(tx_id: i32) -> Result<(), ServerFnError> {
    let user = get_me().await?.ok_or_else(|| ServerFnError::new("Unauthorized"))?;
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;
    
    let streamer_id: i32 = sqlx::query_scalar("SELECT id FROM streamers WHERE user_id = $1")
        .bind(&user.id).fetch_one(&pool).await
        .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;
        
    sqlx::query("UPDATE transactions SET status = 'DISPLAYED' WHERE id = $1 AND streamer_id = $2")
        .bind(tx_id).bind(streamer_id).execute(&pool).await
        .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;
    Ok(())
}


#[server(CheckS3Status, "/api")]
pub async fn check_s3_status() -> Result<bool, ServerFnError> {
    use crate::s3::S3_STATUS;
    let mut rx = S3_STATUS.1.clone();
    
    // If it hasn't polled yet, wait for the first poll
    if rx.borrow().is_none() {
        let _ = rx.changed().await;
    }
    
    let status = *rx.borrow();
    Ok(status.unwrap_or(false))
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

    let rows = sqlx::query_as::<_, crate::db::DbStreamerMedia>(
        "SELECT id, streamer_id, file_name, file_url, size_bytes, TO_CHAR(created_at, 'YYYY-MM-DD HH24:MI:SS') as created_at FROM streamer_media WHERE streamer_id = $1 ORDER BY created_at DESC"
    )
    .bind(streamer_id)
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

#[server(GetPresignedUrl, "/api")]
pub async fn get_presigned_url(file_name: String, content_type: String) -> Result<(String, String), ServerFnError> {
    let _user = match get_me().await? {
        Some(u) => u,
        None => {
            tracing::error!("get_presigned_url: Not authenticated");
            return Err(ServerFnError::new("Not authenticated"));
        }
    };

    let (public_url, upload_url) = crate::s3::generate_presigned_url(&file_name, &content_type)
        .await
        .map_err(|e| {
            tracing::error!("get_presigned_url error: {}", e);
            ServerFnError::new(e)
        })?;
        
    Ok((public_url, upload_url))
}

#[server(UpdateAvatarUrl, "/api")]
pub async fn update_avatar_url(new_avatar_url: String) -> Result<(), ServerFnError> {
    let user = match get_me().await? {
        Some(u) => u,
        None => return Err(ServerFnError::new("Not authenticated")),
    };

    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    sqlx::query(
        "UPDATE streamers SET avatar_url = $1 WHERE user_id = $2"
    )
    .bind(new_avatar_url)
    .bind(&user.id)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(())
}

#[server(SaveMediaRecord, "/api")]
pub async fn save_media_record(file_name: String, file_url: String, size_bytes: i32) -> Result<(), ServerFnError> {
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

    sqlx::query(
        "INSERT INTO streamer_media (streamer_id, file_name, file_url, size_bytes) VALUES ($1, $2, $3, $4)"
    )
    .bind(streamer_id)
    .bind(file_name)
    .bind(file_url)
    .bind(size_bytes)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(())
}

