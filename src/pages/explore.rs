use leptos::prelude::*;
use crate::components::layout::{Header, Footer};
#[allow(unused_imports)]
use crate::db::{DbStreamer, PaymentMethod};

#[component]
pub fn ExplorePage() -> impl IntoView {
    let streamers_resource = Resource::new_blocking(|| (), |_| get_all_streamers());

    view! {
        <Header />
        <main class="pt-24 pb-xl px-margin-mobile md:px-margin-desktop min-h-screen">
            <div class="max-w-7xl mx-auto flex flex-col gap-xl">
                <div class="flex flex-col gap-xs text-center">
                    <h1 class="text-display-sm md:text-display-md font-display-md font-extrabold text-on-surface tracking-tight">
                        {leptos_fluent::move_tr!("explore-title")}
                    </h1>
                    <p class="text-headline-sm font-headline-sm text-on-surface-variant max-w-2xl mx-auto">
                        {leptos_fluent::move_tr!("explore-subtitle")}
                    </p>
                </div>

                <Suspense fallback=move || {
                    view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-lg mt-xl">
                            {(0..6).map(|_| {
                                view! {
                                    <div class="flex flex-col gap-md bg-surface-container-low/40 backdrop-blur-md border border-white/10 rounded-2xl p-lg animate-pulse">
                                        <div class="flex items-center gap-md">
                                            <div class="w-20 h-20 rounded-full bg-white/10 shrink-0 border border-white/5"></div>
                                            <div class="flex flex-col gap-sm w-full">
                                                <div class="h-6 bg-white/10 rounded-md w-3/4"></div>
                                                <div class="h-4 bg-white/10 rounded-md w-1/2"></div>
                                            </div>
                                        </div>
                                        <div class="flex flex-col gap-2 mt-xs">
                                            <div class="h-4 bg-white/10 rounded-md w-full"></div>
                                            <div class="h-4 bg-white/10 rounded-md w-5/6"></div>
                                        </div>
                                        <div class="mt-auto pt-sm flex justify-end">
                                            <div class="h-4 bg-white/10 rounded-md w-24"></div>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }
                }>
                    {move || {
                        streamers_resource
                            .get()
                            .map(|res| match res {
                                Ok(streamers) => {
                                    if streamers.is_empty() {
                                        view! {
                                            <div class="text-center text-on-surface-variant mt-xl">
                                                "No streamers found."
                                            </div>
                                        }
                                            .into_any()
                                    } else {
                                        view! {
                                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-lg">
                                                {streamers
                                                    .into_iter()
                                                    .map(|s| {
                                                        let profile_url = format!("/streamer/{}", s.username);
                                                        let avatar = if s.avatar_url.is_empty() {
                                                            "https://api.dicebear.com/9.x/avataaars/svg".to_string()
                                                        } else {
                                                            s.avatar_url.clone()
                                                        };
                                                        let display_name = if s.display_name.is_empty() {
                                                            s.username.clone()
                                                        } else {
                                                            s.display_name.clone()
                                                        };

                                                        view! {
                                                            <a
                                                                data-testid="streamer-card"
                                                                href=profile_url
                                                                class="group flex flex-col gap-md bg-surface-container-low/40 backdrop-blur-md border border-white/10 rounded-2xl p-lg hover:border-primary/50 transition-all hover:transform hover:-translate-y-1"
                                                            >
                                                                <div class="flex items-center gap-md">
                                                                    <div class="relative w-20 h-20 shrink-0">
                                                                        <img
                                                                            src=avatar
                                                                            class="w-full h-full rounded-full object-cover bg-surface-container-highest border border-white/10"
                                                                        />
                                                                        {if s.is_live {
                                                                            view! {
                                                                                <div class="absolute -bottom-1 -right-1 bg-error text-on-error text-[10px] font-bold px-2 py-0.5 rounded-full border-2 border-surface animate-pulse">
                                                                                    {leptos_fluent::move_tr!("explore-live")}
                                                                                </div>
                                                                            }
                                                                                .into_any()
                                                                        } else {
                                                                            view! {
                                                                                <div class="absolute -bottom-1 -right-1 bg-surface-variant text-on-surface-variant text-[10px] font-bold px-2 py-0.5 rounded-full border-2 border-surface">
                                                                                    {leptos_fluent::move_tr!("explore-offline")}
                                                                                </div>
                                                                            }
                                                                                .into_any()
                                                                        }}
                                                                    </div>
                                                                    <div class="flex flex-col overflow-hidden">
                                                                        <h3
                                                                            data-testid="streamer-display-name"
                                                                            class="text-headline-sm font-headline-sm text-on-surface font-bold truncate"
                                                                        >
                                                                            {display_name}
                                                                        </h3>
                                                                        <p
                                                                            data-testid="streamer-username"
                                                                            class="text-label-md font-label-md text-on-surface-variant truncate"
                                                                        >
                                                                            "@"
                                                                            {s.username.clone()}
                                                                        </p>
                                                                    </div>
                                                                </div>
                                                                <p class="text-body-md font-body-md text-on-surface-variant line-clamp-2 mt-xs min-h-[3rem]">
                                                                    {s.bio.clone()}
                                                                </p>
                                                                <div class="mt-auto pt-sm flex justify-end">
                                                                    <span class="text-primary text-label-md font-label-md font-bold group-hover:underline">
                                                                        {leptos_fluent::move_tr!("explore-view-profile")} " →"
                                                                    </span>
                                                                </div>
                                                            </a>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        }
                                            .into_any()
                                    }
                                }
                                Err(_) => {
                                    view! {
                                        <div class="text-center text-error mt-xl">
                                            {leptos_fluent::move_tr!("explore-error")}
                                        </div>
                                    }
                                        .into_any()
                                }
                            })
                    }}
                </Suspense>
            </div>
        </main>
        <Footer />
    }
}

#[server(GetAllStreamers, "/api")]
pub async fn get_all_streamers() -> Result<Vec<DbStreamer>, ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let rows = sqlx::query(
        "SELECT id, username, display_name, avatar_url, bio, is_live, user_id, overlay_token, active_overlay_session, payment_methods, overlay_paused, overlay_sound_enabled, selected_media_id, fallback_media_file FROM streamers ORDER BY is_live DESC, id DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

    use sqlx::Row;
    let streamers = rows.into_iter().map(|r| {
        DbStreamer {
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
        }
    }).collect();

    Ok(streamers)
}
