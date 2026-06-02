use leptos::prelude::*;
use crate::components::layout::{Header};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LeaderboardEntry {
    pub streamer_id: i32,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub total_amount: f64,
    pub donation_count: i64,
}

#[component]
pub fn LeaderboardPage() -> impl IntoView {
    let leaderboard_resource = Resource::new_blocking(|| (), |_| async move {
        crate::utils::with_min_delay(get_streamer_leaderboard()).await
    });

    view! {
        <Header />
        <main class="pt-24 pb-xl px-margin-mobile md:px-margin-desktop min-h-screen">
            <div class="max-w-6xl mx-auto flex flex-col gap-xl">
                <div class="flex flex-col gap-xs text-center">
                    <h1
                        data-testid="page-header"
                        class="text-display-sm md:text-display-md font-display-md font-extrabold text-on-surface tracking-tight"
                    >
                        "Leaderboard"
                    </h1>
                    <p class="text-headline-sm font-headline-sm text-on-surface-variant max-w-2xl mx-auto">
                        "Top creators by total donations."
                    </p>
                </div>

                <Suspense fallback=move || {
                    view! {
                        <div class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl overflow-hidden animate-pulse">
                            <div class="grid grid-cols-12 gap-2 px-5 py-4 text-on-surface-variant text-label-sm font-label-sm border-b border-white/10">
                                <div class="col-span-1">"#"</div>
                                <div class="col-span-6">"Creator"</div>
                                <div class="col-span-2 text-right">"Donations"</div>
                                <div class="col-span-3 text-right">"Total"</div>
                            </div>
                            <div class="divide-y divide-white/10">
                                {(0..10).map(|_| {
                                    view! {
                                        <div class="grid grid-cols-12 gap-2 px-5 py-4 items-center">
                                            <div class="col-span-1">
                                                <div class="w-4 h-4 bg-white/10 rounded"></div>
                                            </div>
                                            <div class="col-span-6 flex items-center gap-3">
                                                <div class="w-10 h-10 rounded-full bg-white/10 shrink-0"></div>
                                                <div class="flex flex-col gap-2 w-full">
                                                    <div class="h-4 bg-white/10 rounded-md w-1/3"></div>
                                                    <div class="h-3 bg-white/10 rounded-md w-1/4"></div>
                                                </div>
                                            </div>
                                            <div class="col-span-2 flex justify-end">
                                                <div class="h-4 bg-white/10 rounded-md w-8"></div>
                                            </div>
                                            <div class="col-span-3 flex justify-end">
                                                <div class="h-4 bg-white/10 rounded-md w-16"></div>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }
                }>
                    {move || {
                        leaderboard_resource
                            .get()
                            .map(|res| match res {
                                Ok(entries) => {
                                    if entries.is_empty() {
                                        view! {
                                            <div
                                                data-testid="empty-state"
                                                class="text-center text-on-surface-variant mt-xl"
                                            >
                                                "No donations yet."
                                            </div>
                                        }
                                            .into_any()
                                    } else {
                                        view! {
                                            <div class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl overflow-hidden">
                                                <div
                                                    data-testid="leaderboard-header"
                                                    class="grid grid-cols-12 gap-2 px-5 py-4 text-on-surface-variant text-label-sm font-label-sm border-b border-white/10"
                                                >
                                                    <div class="col-span-1">"#"</div>
                                                    <div class="col-span-6">"Creator"</div>
                                                    <div class="col-span-2 text-right">"Donations"</div>
                                                    <div class="col-span-3 text-right">"Total"</div>
                                                </div>

                                                <div class="divide-y divide-white/10">
                                                    {entries
                                                        .into_iter()
                                                        .enumerate()
                                                        .map(|(idx, e)| {
                                                            let avatar = if e.avatar_url.is_empty() {
                                                                "https://api.dicebear.com/9.x/avataaars/svg".to_string()
                                                            } else {
                                                                e.avatar_url.clone()
                                                            };
                                                            let name = if e.display_name.is_empty() {
                                                                e.username.clone()
                                                            } else {
                                                                e.display_name.clone()
                                                            };
                                                            let profile_url = format!("/streamer/{}", e.username);
                                                            let rank = (idx + 1).to_string();
                                                            view! {
                                                                <a
                                                                    href=profile_url
                                                                    class="grid grid-cols-12 gap-2 px-5 py-4 hover:bg-white/5 transition-colors items-center"
                                                                >
                                                                    <div class="col-span-1 text-on-surface font-semibold">
                                                                        {rank}
                                                                    </div>
                                                                    <div class="col-span-6 flex items-center gap-3">
                                                                        <img
                                                                            src=avatar
                                                                            class="w-10 h-10 rounded-full object-cover bg-surface-container-highest border border-white/10"
                                                                        />
                                                                        <div class="flex flex-col">
                                                                            <div class="text-on-surface font-semibold">{name}</div>
                                                                            <div class="text-on-surface-variant text-label-sm">
                                                                                @{e.username}
                                                                            </div>
                                                                        </div>
                                                                    </div>
                                                                    <div class="col-span-2 text-right text-on-surface">
                                                                        {e.donation_count}
                                                                    </div>
                                                                    <div class="col-span-3 text-right text-secondary font-extrabold">
                                                                        "$" {format!("{:.2}", e.total_amount)}
                                                                    </div>
                                                                </a>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </div>
                                            </div>
                                        }
                                            .into_any()
                                    }
                                }
                                Err(e) => {
                                    view! {
                                        <div class="text-center text-error mt-xl">
                                            {format!("Failed to load leaderboard: {e}")}
                                        </div>
                                    }
                                        .into_any()
                                }
                            })
                    }}
                </Suspense>
            </div>
        </main>
    }
}

#[server(GetStreamerLeaderboard, "/api")]
pub async fn get_streamer_leaderboard() -> Result<Vec<LeaderboardEntry>, ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let rows = sqlx::query(
        "SELECT
            s.id as streamer_id,
            s.username,
            s.display_name,
            s.avatar_url,
            COALESCE(SUM(t.amount), 0) as total_amount,
            COUNT(t.id) as donation_count
         FROM streamers s
         JOIN transactions t ON t.streamer_id = s.id
         WHERE t.status = 'DISPLAYED'
         GROUP BY s.id, s.username, s.display_name, s.avatar_url
         ORDER BY total_amount DESC, donation_count DESC, s.id ASC
         LIMIT 50"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

    use sqlx::Row;
    Ok(rows
        .into_iter()
        .map(|r| LeaderboardEntry {
            streamer_id: r.get("streamer_id"),
            username: r.get("username"),
            display_name: r.get("display_name"),
            avatar_url: r.get("avatar_url"),
            total_amount: r.get("total_amount"),
            donation_count: r.get("donation_count"),
        })
        .collect())
}
