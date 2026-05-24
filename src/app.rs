use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::{use_location, use_params_map},
    ParamSegment, StaticSegment,
};
use crate::auth::User;
use crate::db::{DbStreamer, DbTransaction};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MockPaymentInit {
    pub tx_id: i32,
    pub status: String,
    pub display_qr: Option<String>,
    pub display_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MockPaymentStatus {
    pub tx_id: i32,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LeaderboardEntry {
    pub streamer_id: i32,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub total_amount: f64,
    pub donation_count: i64,
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html class="dark" lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link href="https://fonts.googleapis.com/css2?family=Montserrat:wght@700;800&family=Inter:wght@400;500;600&display=swap" rel="stylesheet"/>
                <link href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:wght,FILL@100..700,0..1&display=swap" rel="stylesheet"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-background text-on-background selection:bg-primary selection:text-on-primary text-left min-h-screen flex flex-col">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Load current user
    let user_resource = Resource::new(|| (), |_| get_me());
    provide_context(user_resource);

    view! {
        <Stylesheet id="leptos" href="/pkg/open-donate.css"/>
        <Title text="Glint | Empower Your Content"/>

        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=StaticSegment("") view=LandingPage/>
                <Route path=StaticSegment("explore") view=ExplorePage/>
                <Route path=StaticSegment("leaderboard") view=LeaderboardPage/>
                <Route path=StaticSegment("dashboard") view=crate::dashboard::DashboardPage/>
                <Route path=(StaticSegment("streamer"), ParamSegment("username")) view=StreamerPage/>
                <Route path=(StaticSegment("overlay"), ParamSegment("token")) view=crate::overlay::OverlayPage/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    let user_resource = use_context::<Resource<Result<Option<User>, ServerFnError>>>()
        .expect("User resource must be provided");

    let location = use_location();
    let nav_class = move |href: &'static str| {
        let path = location.pathname.get();
        let active = if href == "/" {
            path == "/"
        } else {
            path.starts_with(href)
        };

        if active {
            "text-primary font-bold border-b-2 border-primary pb-1 text-label-md font-label-md hover:text-primary transition-colors"
        } else {
            "text-on-surface-variant font-medium text-label-md font-label-md hover:text-primary transition-colors"
        }
    };

    view! {
        <header class="fixed top-0 w-full z-50 bg-surface/60 backdrop-blur-xl border-b border-white/20 shadow-sm h-20 flex justify-between items-center px-margin-desktop">
            <div class="flex items-center gap-md">
                <a href="/" class="text-headline-md font-headline-md font-extrabold text-primary tracking-tighter">"Glint"</a>
                <nav class="hidden md:flex items-center gap-md ml-lg">
                    <a class=move || nav_class("/") href="/">"Explore"</a>
                    <a class=move || nav_class("/explore") href="/explore">"Creators"</a>
                    <a class=move || nav_class("/leaderboard") href="/leaderboard">"Leaderboard"</a>
                </nav>
            </div>
            <div class="flex items-center gap-md">
                <Suspense fallback=move || view! { <span class="text-on-surface-variant">"Loading..."</span> }>
                    {move || {
                        match user_resource.get() {
                            Some(Ok(Some(user))) => view! {
                                <div class="flex items-center gap-sm">
                                    <span class="text-on-surface text-label-md font-semibold">"Hello, " {user.name}</span>
                                    <a href="/api/logout" rel="external" class="px-sm py-xs bg-surface-container-highest border border-white/10 hover:bg-surface-container-highest/80 text-on-surface rounded-lg text-label-sm font-label-sm transition-all">
                                        "Logout"
                                    </a>
                                </div>
                            }.into_any(),
                            _ => view! {
                                <a href="/api/login" rel="external" class="px-md py-xs bg-primary text-on-primary rounded-lg text-label-md font-label-md font-semibold hover:bg-primary/95 transition-all shadow-sm">
                                    "Login with Zitadel"
                                </a>
                            }.into_any()
                        }
                    }}
                </Suspense>
            </div>
        </header>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-surface-dim border-t border-surface-container-highest w-full py-lg mt-auto">
            <div class="max-w-7xl mx-auto px-margin-desktop flex flex-col md:flex-row justify-between items-center gap-md">
                <div class="flex flex-col items-center md:items-start gap-xs">
                    <span class="text-headline-md font-headline-md font-bold text-primary">"Glint"</span>
                    <p class="text-on-surface-variant text-body-md font-body-md">"© 2024 Glint Technologies. Optimism in every transaction."</p>
                </div>
                <nav class="flex flex-wrap justify-center gap-md">
                    <a class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors" href="#">"Support"</a>
                    <a class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors" href="#">"Privacy Policy"</a>
                    <a class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors" href="#">"Terms of Service"</a>
                    <a class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors" href="#">"Discord"</a>
                    <a class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors" href="#">"Instagram"</a>
                </nav>
            </div>
        </footer>
    }
}

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="relative min-h-[921px] flex flex-col items-center justify-center text-center px-margin-mobile md:px-margin-desktop overflow-hidden">
            <div class="js-glow absolute top-1/4 -left-20 w-96 h-96 bg-primary/20 rounded-full blur-[120px]"></div>
            <div class="js-glow absolute bottom-1/4 -right-20 w-96 h-96 bg-secondary/10 rounded-full blur-[120px]"></div>
            <div class="relative z-10 max-w-4xl mx-auto space-y-md">
                <span class="inline-block px-sm py-xs bg-surface-container-highest/40 backdrop-blur-md rounded-full border border-white/10 text-secondary text-label-md font-label-md">
                    "Future of Digital Support"
                </span>
                <h1 class="text-headline-xl-mobile md:text-headline-xl font-headline-xl text-on-surface">
                    "Empower Your Content, One " <span class="text-primary italic">"Glint"</span> " at a Time"
                </h1>
                <p class="text-body-lg font-body-lg text-on-surface-variant max-w-2xl mx-auto">
                    "The ultra-fast, transparent platform connecting creators and fans through real-time rewards and deep interactive experiences."
                </p>
                <div class="flex flex-col md:flex-row items-center justify-center gap-md pt-base">
                    <button class="px-lg py-md bg-secondary text-on-secondary-container rounded-xl font-headline-md text-headline-md neon-glow-secondary hover:scale-105 transition-transform active:scale-95 duration-150">
                        "Donate Now"
                    </button>
                    <button class="px-lg py-md bg-surface-container-highest/40 backdrop-blur-md border border-white/20 text-on-surface rounded-xl font-headline-md text-headline-md hover:bg-surface-container-highest/60 transition-all">
                        "Start Creating"
                    </button>
                </div>
            </div>
            <div class="mt-xl grid grid-cols-2 md:grid-cols-4 gap-md w-full max-w-5xl">
                <div class="p-md rounded-xl bg-surface-container-low/40 backdrop-blur-md border border-white/5">
                    <p class="text-primary text-headline-md font-headline-md">"$2.4M"</p>
                    <p class="text-on-surface-variant text-label-sm font-label-sm">"Total Distributed"</p>
                </div>
                <div class="p-md rounded-xl bg-surface-container-low/40 backdrop-blur-md border border-white/5">
                    <p class="text-secondary text-headline-md font-headline-md">"150K+"</p>
                    <p class="text-on-surface-variant text-label-sm font-label-sm">"Active Creators"</p>
                </div>
                <div class="p-md rounded-xl bg-surface-container-low/40 backdrop-blur-md border border-white/5">
                    <p class="text-tertiary text-headline-md font-headline-md">"0.2s"</p>
                    <p class="text-on-surface-variant text-label-sm font-label-sm">"Avg. Payout Time"</p>
                </div>
                <div class="p-md rounded-xl bg-surface-container-low/40 backdrop-blur-md border border-white/5">
                    <p class="text-on-surface text-headline-md font-headline-md">"99.9%"</p>
                    <p class="text-on-surface-variant text-label-sm font-label-sm">"Transparency Score"</p>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn ExplorePage() -> impl IntoView {
    let streamers_resource = Resource::new(|| (), |_| get_all_streamers());

    view! {
        <Header/>
        <main class="pt-24 pb-xl px-margin-mobile md:px-margin-desktop min-h-screen">
            <div class="max-w-7xl mx-auto flex flex-col gap-xl">
                <div class="flex flex-col gap-xs text-center">
                    <h1 class="text-display-sm md:text-display-md font-display-md font-extrabold text-on-surface tracking-tight">
                        "Explore Creators"
                    </h1>
                    <p class="text-headline-sm font-headline-sm text-on-surface-variant max-w-2xl mx-auto">
                        "Discover and support your favorite live streamers."
                    </p>
                </div>

                <Suspense fallback=move || view! { <div class="text-center text-on-surface-variant mt-xl">"Loading streamers..."</div> }>
                    {move || {
                        streamers_resource.get().map(|res| match res {
                            Ok(streamers) => {
                                if streamers.is_empty() {
                                    view! {
                                        <div class="text-center text-on-surface-variant mt-xl">"No streamers found."</div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-lg">
                                            {streamers.into_iter().map(|s| {
                                                let profile_url = format!("/streamer/{}", s.username);
                                                let avatar = if s.avatar_url.is_empty() { "https://api.dicebear.com/9.x/avataaars/svg".to_string() } else { s.avatar_url.clone() };
                                                let display_name = if s.display_name.is_empty() { s.username.clone() } else { s.display_name.clone() };
                                                
                                                view! {
                                                    <a data-testid="streamer-card" href=profile_url class="group flex flex-col gap-md bg-surface-container-low/40 backdrop-blur-md border border-white/10 rounded-2xl p-lg hover:border-primary/50 transition-all hover:transform hover:-translate-y-1">
                                                        <div class="flex items-center gap-md">
                                                            <div class="relative w-20 h-20 shrink-0">
                                                                <img src=avatar class="w-full h-full rounded-full object-cover bg-surface-container-highest border border-white/10"/>
                                                                {if s.is_live {
                                                                    view! {
                                                                        <div class="absolute -bottom-1 -right-1 bg-error text-on-error text-[10px] font-bold px-2 py-0.5 rounded-full border-2 border-surface animate-pulse">
                                                                            "LIVE"
                                                                        </div>
                                                                    }.into_any()
                                                                } else {
                                                                    view! {}.into_any()
                                                                }}
                                                            </div>
                                                            <div class="flex flex-col overflow-hidden">
                                                                <h3 data-testid="streamer-display-name" class="text-headline-sm font-headline-sm text-on-surface font-bold truncate">
                                                                    {display_name}
                                                                </h3>
                                                                <p data-testid="streamer-username" class="text-label-md font-label-md text-on-surface-variant truncate">
                                                                    "@" {s.username.clone()}
                                                                </p>
                                                            </div>
                                                        </div>
                                                        <p class="text-body-md font-body-md text-on-surface-variant line-clamp-2 mt-xs min-h-[3rem]">
                                                            {s.bio.clone()}
                                                        </p>
                                                        <div class="mt-auto pt-sm flex justify-end">
                                                            <span class="text-primary text-label-md font-label-md font-bold group-hover:underline">"View Profile →"</span>
                                                        </div>
                                                    </a>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }
                            Err(_) => view! {
                                <div class="text-center text-error mt-xl">"Failed to load streamers."</div>
                            }.into_any()
                        })
                    }}
                </Suspense>
            </div>
        </main>
        <Footer/>
    }
}

#[component]
pub fn LeaderboardPage() -> impl IntoView {
    let leaderboard_resource = Resource::new(|| (), |_| get_streamer_leaderboard());

    view! {
        <Header/>
        <main class="pt-24 pb-xl px-margin-mobile md:px-margin-desktop min-h-screen">
            <div class="max-w-6xl mx-auto flex flex-col gap-xl">
                <div class="flex flex-col gap-xs text-center">
                    <h1 class="text-display-sm md:text-display-md font-display-md font-extrabold text-on-surface tracking-tight">
                        "Leaderboard"
                    </h1>
                    <p class="text-headline-sm font-headline-sm text-on-surface-variant max-w-2xl mx-auto">
                        "Top creators by total donations."
                    </p>
                </div>

                <Suspense fallback=move || view! { <div class="text-center text-on-surface-variant mt-xl">"Loading leaderboard..."</div> }>
                    {move || {
                        leaderboard_resource.get().map(|res| match res {
                            Ok(entries) => {
                                if entries.is_empty() {
                                    view! {
                                        <div class="text-center text-on-surface-variant mt-xl">"No donations yet."</div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl overflow-hidden">
                                            <div class="grid grid-cols-12 gap-2 px-5 py-4 text-on-surface-variant text-label-sm font-label-sm border-b border-white/10">
                                                <div class="col-span-1">"#"</div>
                                                <div class="col-span-6">"Creator"</div>
                                                <div class="col-span-2 text-right">"Donations"</div>
                                                <div class="col-span-3 text-right">"Total"</div>
                                            </div>

                                            <div class="divide-y divide-white/10">
                                                {entries.into_iter().enumerate().map(|(idx, e)| {
                                                    let avatar = if e.avatar_url.is_empty() { "https://api.dicebear.com/9.x/avataaars/svg".to_string() } else { e.avatar_url.clone() };
                                                    let name = if e.display_name.is_empty() { e.username.clone() } else { e.display_name.clone() };
                                                    let profile_url = format!("/streamer/{}", e.username);
                                                    let rank = (idx + 1).to_string();
                                                    view! {
                                                        <a href=profile_url class="grid grid-cols-12 gap-2 px-5 py-4 hover:bg-white/5 transition-colors items-center">
                                                            <div class="col-span-1 text-on-surface font-semibold">{rank}</div>
                                                            <div class="col-span-6 flex items-center gap-3">
                                                                <img src=avatar class="w-10 h-10 rounded-full object-cover bg-surface-container-highest border border-white/10"/>
                                                                <div class="flex flex-col">
                                                                    <div class="text-on-surface font-semibold">{name}</div>
                                                                    <div class="text-on-surface-variant text-label-sm">@{e.username}</div>
                                                                </div>
                                                            </div>
                                                            <div class="col-span-2 text-right text-on-surface">{e.donation_count}</div>
                                                            <div class="col-span-3 text-right text-secondary font-extrabold">
                                                                "$" {format!("{:.2}", e.total_amount)}
                                                            </div>
                                                        </a>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                            }
                            Err(e) => view! {
                                <div class="text-center text-error mt-xl">{format!("Failed to load leaderboard: {e}")}</div>
                            }.into_any(),
                        })
                    }}
                </Suspense>
            </div>
        </main>
    }
}

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <Header/>
        <main class="pt-20 text-left flex-1">
            <Hero/>
            // For Streamers Section
            <section class="py-xl px-margin-mobile md:px-margin-desktop max-w-7xl mx-auto">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-lg items-center">
                    <div class="space-y-md">
                        <h2 class="text-headline-lg font-headline-lg text-secondary">"For Streamers"</h2>
                        <h3 class="text-headline-md font-headline-md text-on-surface">"Accelerate your growth with tools built for speed."</h3>
                        <ul class="space-y-base">
                            <li class="flex items-start gap-sm p-md rounded-xl bg-surface-container-low/40 backdrop-blur-md border border-white/5 hover:border-secondary/30 transition-colors">
                                <span class="material-symbols-outlined text-secondary" data-icon="bolt">"bolt"</span>
                                <div>
                                    <p class="text-on-surface font-bold">"Real-time alerts"</p>
                                    <p class="text-on-surface-variant text-label-md">"Low-latency notifications that keep your community engaged instantly."</p>
                                </div>
                            </li>
                            <li class="flex items-start gap-sm p-md rounded-xl bg-surface-container-low/40 backdrop-blur-md border border-white/5 hover:border-secondary/30 transition-colors">
                                <span class="material-symbols-outlined text-secondary" data-icon="account_balance_wallet">"account_balance_wallet"</span>
                                <div>
                                    <p class="text-on-surface font-bold">"Instant payouts"</p>
                                    <p class="text-on-surface-variant text-label-md">"No more waiting weeks. Your earnings are yours, immediately."</p>
                                </div>
                            </li>
                            <li class="flex items-start gap-sm p-md rounded-xl bg-surface-container-low/40 backdrop-blur-md border border-white/5 hover:border-secondary/30 transition-colors">
                                <span class="material-symbols-outlined text-secondary" data-icon="insights">"insights"</span>
                                <div>
                                    <p class="text-on-surface font-bold">"Detailed analytics"</p>
                                    <p class="text-on-surface-variant text-label-md">"Deep dive into viewer behavior and contribution trends."</p>
                                </div>
                            </li>
                        </ul>
                    </div>
                    <div class="relative group">
                        <div class="absolute inset-0 bg-secondary/10 blur-3xl group-hover:bg-secondary/20 transition-all"></div>
                        <div class="relative bg-surface-container-highest/30 backdrop-blur-xl rounded-2xl border border-white/10 p-base overflow-hidden">
                            <img alt="Streamer Dashboard" class="rounded-xl w-full" src="https://lh3.googleusercontent.com/aida-public/AB6AXuD0f6PmQqoeoKjRMaM9Rt_FTi6mYBDOuxzSzE6xLTkQ8pP8qR8z2hmpOdLQG0UMV7U1lH7UiZk35FgVwKKEv6pK7wYFwnpRE9VXdwzxAGitfXl8Q75e6DZhE6L1E_SUol1j8c8-AaKPPoVJiDFt_LwDu_q6SQrMvUXSUum3GsaMDmpyefq61E_KA0VG2WSA9mKS2kUg4bc6Y3FFeQO-Xd4HC_vUSTi_SxylwTLBZbAWTmayjnnNxkyr_bJ5JJdTidK42GHrlQREd-E"/>
                            <div class="absolute bottom-md left-md right-md p-md bg-background/80 backdrop-blur-md rounded-xl border border-secondary/30 flex justify-between items-center">
                                <div class="flex items-center gap-sm">
                                    <div class="w-10 h-10 rounded-full bg-secondary flex items-center justify-center">
                                        <span class="material-symbols-outlined text-on-secondary" data-icon="trending_up">"trending_up"</span>
                                    </div>
                                    <span class="text-on-surface font-bold">"+24% revenue this week"</span>
                                </div>
                                <span class="text-secondary text-label-sm font-label-sm">"LIVE NOW"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            // For Fans Section
            <section class="py-xl px-margin-mobile md:px-margin-desktop max-w-7xl mx-auto bg-surface-container-lowest/50 rounded-3xl">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-lg items-center">
                    <div class="order-2 lg:order-1 relative group">
                        <div class="absolute inset-0 bg-primary/10 blur-3xl group-hover:bg-primary/20 transition-all"></div>
                        <div class="relative grid grid-cols-2 gap-md p-base">
                            <div class="space-y-md">
                                <div class="bg-surface-container-highest/50 backdrop-blur-lg p-md rounded-2xl border border-white/10 neon-glow-primary">
                                    <span class="material-symbols-outlined text-primary text-headline-lg" data-icon="military_tech">"military_tech"</span>
                                    <p class="text-on-surface font-bold mt-sm">"Loyalty Badges"</p>
                                    <p class="text-on-surface-variant text-label-sm">"Unlock exclusive status based on your support."</p>
                                </div>
                                <div class="bg-surface-container-highest/50 backdrop-blur-lg p-md rounded-2xl border border-white/10">
                                    <span class="material-symbols-outlined text-tertiary text-headline-lg" data-icon="forum">"forum"</span>
                                    <p class="text-on-surface font-bold mt-sm">"Chat Highlights"</p>
                                    <p class="text-on-surface-variant text-label-sm">"Stand out with neon-bordered messages."</p>
                                </div>
                            </div>
                            <div class="space-y-md pt-lg">
                                <div class="bg-surface-container-highest/50 backdrop-blur-lg p-md rounded-2xl border border-white/10">
                                    <span class="material-symbols-outlined text-secondary text-headline-lg" data-icon="card_giftcard">"card_giftcard"</span>
                                    <p class="text-on-surface font-bold mt-sm">"Personal Tributes"</p>
                                    <p class="text-on-surface-variant text-label-sm">"Send personalized gifts to your idols."</p>
                                </div>
                                <div class="bg-surface-container-highest/50 backdrop-blur-lg p-md rounded-2xl border border-white/10">
                                    <span class="material-symbols-outlined text-primary text-headline-lg" data-icon="verified_user">"verified_user"</span>
                                    <p class="text-on-surface font-bold mt-sm">"Secure Vault"</p>
                                    <p class="text-on-surface-variant text-label-sm">"Your transactions are encrypted and private."</p>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="order-1 lg:order-2 space-y-md">
                        <h2 class="text-headline-lg font-headline-lg text-primary">"For Fans"</h2>
                        <h3 class="text-headline-md font-headline-md text-on-surface">"Experience deeper connections with the creators you love."</h3>
                        <p class="text-body-md text-on-surface-variant">"Glint turns every transaction into a moment of interaction. Support isn't just a number; it's a personalized message, a badge of honor, and a direct impact on the content you enjoy."</p>
                        <button class="px-lg py-md bg-surface-container-highest/40 backdrop-blur-md border border-primary/30 text-primary rounded-xl font-headline-md text-headline-md hover:bg-primary/10 transition-all flex items-center gap-sm">
                            "Find a Creator " <span class="material-symbols-outlined" data-icon="chevron_right">"chevron_right"</span>
                        </button>
                    </div>
                </div>
            </section>
            
            // How it works
            <section class="py-xl px-margin-mobile md:px-margin-desktop max-w-7xl mx-auto text-center">
                <h2 class="text-headline-lg font-headline-lg text-on-surface mb-xl">"How it Works"</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-lg relative">
                    <div class="hidden md:block absolute top-1/2 left-0 w-full h-px bg-gradient-to-r from-transparent via-white/20 to-transparent -translate-y-1/2 z-0"></div>
                    <div class="relative z-10 group">
                        <div class="w-16 h-16 bg-surface-container-highest rounded-full flex items-center justify-center mx-auto border-2 border-primary neon-glow-primary mb-md group-hover:scale-110 transition-transform">
                            <span class="text-headline-md font-headline-md text-primary">"1"</span>
                        </div>
                        <h4 class="text-headline-md font-headline-md text-on-surface">"Connect Account"</h4>
                        <p class="text-on-surface-variant mt-sm">"Link your existing streaming or social profile in seconds with our secure API."</p>
                    </div>
                    <div class="relative z-10 group">
                        <div class="w-16 h-16 bg-surface-container-highest rounded-full flex items-center justify-center mx-auto border-2 border-secondary neon-glow-secondary mb-md group-hover:scale-110 transition-transform">
                            <span class="text-headline-md font-headline-md text-secondary">"2"</span>
                        </div>
                        <h4 class="text-headline-md font-headline-md text-on-surface">"Customize Experience"</h4>
                        <p class="text-on-surface-variant mt-sm">"Set up your Glint alerts, donation tiers, and loyalty milestones to match your brand."</p>
                    </div>
                    <div class="relative z-10 group">
                        <div class="w-16 h-16 bg-surface-container-highest rounded-full flex items-center justify-center mx-auto border-2 border-tertiary mb-md group-hover:scale-110 transition-transform">
                            <span class="text-headline-md font-headline-md text-tertiary">"3"</span>
                        </div>
                        <h4 class="text-headline-md font-headline-md text-on-surface">"Receive & Payout"</h4>
                        <p class="text-on-surface-variant mt-sm">"Watch the Glints roll in and withdraw your funds to your preferred method instantly."</p>
                    </div>
                </div>
            </section>
            
            // CTA Section
            <section class="py-xl px-margin-mobile md:px-margin-desktop text-center">
                <div class="max-w-4xl mx-auto bg-gradient-to-br from-primary/10 to-secondary/10 backdrop-blur-xl p-xl rounded-[2rem] border border-white/10">
                    <h2 class="text-headline-lg md:text-headline-xl font-headline-xl text-on-surface mb-md">"Ready to join the revolution?"</h2>
                    <p class="text-body-lg text-on-surface-variant mb-lg">"Join 150,000+ creators who are already using Glint to power their communities."</p>
                    <div class="flex flex-col sm:flex-row items-center justify-center gap-md">
                        <button class="w-full sm:w-auto px-lg py-md bg-secondary text-on-secondary-container rounded-xl font-headline-md text-headline-md neon-glow-secondary hover:scale-105 transition-transform pulse-accent">
                            "Donate Now"
                        </button>
                        <button class="w-full sm:w-auto px-lg py-md bg-surface text-on-surface border border-white/20 rounded-xl font-headline-md text-headline-md hover:bg-surface-container-highest transition-colors">
                            "View Leaders"
                        </button>
                    </div>
                </div>
            </section>
        </main>
        <Footer/>

        <script>
            "document.addEventListener('mousemove', (e) => {
                const x = e.clientX / window.innerWidth;
                const y = e.clientY / window.innerHeight;
                
                const glows = document.querySelectorAll('.js-glow');
                glows.forEach((glow, index) => {
                    const speed = (index + 1) * 20;
                    glow.style.transform = `translate(${x * speed}px, ${y * speed}px)`;
                });
            });"
        </script>
    }
}

#[component]
pub fn StreamerPage() -> impl IntoView {
    let params = use_params_map();
    let username = move || {
        params.with_untracked(|p| p.get("username").unwrap_or_default())
    };

    let amount = RwSignal::new("25".to_string());
    let donor_name = RwSignal::new("".to_string());
    let message = RwSignal::new("".to_string());
    let payment_method = RwSignal::new("Credit Card".to_string());
    let transactions_trigger = RwSignal::new(0);
    let form_error = RwSignal::new(None::<String>);

    let show_payment_window = RwSignal::new(false);
    let otp = RwSignal::new("".to_string());
    let mock_tx_id = RwSignal::new(None::<i32>);
    let mock_tx_status = RwSignal::new("".to_string());
    let mock_display_qr = RwSignal::new(None::<String>);
    let mock_display_url = RwSignal::new(None::<String>);

    #[cfg(feature = "hydrate")]
    let poll_interval: StoredValue<
        Option<gloo_timers::callback::Interval>,
        leptos::prelude::LocalStorage,
    > = StoredValue::new_local(None);

    // Load profile info from database using URL param (SSR-friendly)
    let streamer_resource = Resource::new(
        move || username(),
        |uname| async move {
            if uname.is_empty() {
                return Ok(None);
            }
            get_streamer(uname).await
        },
    );

    // Load recent transactions (SSR-friendly)
    let transactions_resource = Resource::new(
        move || (username(), transactions_trigger.get()),
        |(uname, _)| async move {
            if uname.is_empty() {
                return Ok(vec![]);
            }
            get_recent_transactions(uname).await
        },
    );

    let payment_action = Action::new(move |_: &()| {
        let amt_val = amount.get_untracked().parse::<f64>().unwrap_or(0.0);
        let donor = donor_name.get_untracked();
        let msg = message.get_untracked();
        let pm = payment_method.get_untracked();
        let streamer_id = match streamer_resource.get() {
            Some(Ok(Some(s))) => s.id,
            _ => 1,
        };
        async move {
            create_mock_payment(streamer_id, donor, amt_val, msg, pm).await
        }
    });

    // React to payment init
    Effect::new(move || {
        if let Some(result) = payment_action.value().get() {
            match result {
                Ok(init) => {
                    show_payment_window.set(true);
                    mock_tx_id.set(Some(init.tx_id));
                    mock_tx_status.set(init.status);
                    mock_display_qr.set(init.display_qr);
                    mock_display_url.set(init.display_url);

                    #[cfg(feature = "hydrate")]
                    {
                        use gloo_timers::callback::Interval;
                        use wasm_bindgen_futures::spawn_local;

                        poll_interval.update_value(|existing| {
                            existing.take();
                        });
                        let interval = Interval::new(500, move || {
                            if let Some(tx_id) = mock_tx_id.get_untracked() {
                                spawn_local(async move {
                                    if let Ok(status) = get_mock_payment_status(tx_id).await {
                                        let new_status = status.status;
                                        mock_tx_status.set(new_status.clone());
                                        if new_status == "READY_FOR_DISPLAY" {
                                            poll_interval.update_value(|existing| {
                                                existing.take();
                                            });
                                            transactions_trigger.update(|n| *n += 1);
                                        }
                                    }
                                });
                            }
                        });
                        poll_interval.set_value(Some(interval));
                    }
                }
                Err(e) => {
                    leptos::logging::error!("Payment init failed: {:?}", e);
                }
            }
        }
    });

    let handle_submit = move |_| {
        let amt_val = amount.get().trim().parse::<f64>().unwrap_or(-1.0);
        if amt_val <= 0.0 {
            form_error.set(Some("Please enter a valid amount.".to_string()));
            return;
        }
        if payment_method.get().trim().is_empty() {
            form_error.set(Some("Please select a payment method.".to_string()));
            return;
        }
        form_error.set(None);
        show_payment_window.set(false);
        otp.set("".to_string());
        mock_tx_id.set(None);
        mock_tx_status.set("INITIALIZE".to_string());
        mock_display_qr.set(None);
        mock_display_url.set(None);

        #[cfg(feature = "hydrate")]
        poll_interval.update_value(|existing| {
            existing.take();
        });

        payment_action.dispatch(());
    };

    view! {
        <Suspense fallback=move || view! { <Title text="Glint | Donate"/> }>
            {move || {
                match streamer_resource.get() {
                    Some(Ok(Some(s))) => {
                        let display = if s.display_name.is_empty() { s.username } else { s.display_name };
                        let title = format!("Donate to {display} | Glint");
                        view! { <Title text=title/> }.into_any()
                    }
                    _ => view! { <Title text="Glint | Donate"/> }.into_any(),
                }
            }}
        </Suspense>
        <Header/>
        <main class="pt-24 pb-lg px-margin-mobile md:px-margin-desktop max-w-4xl mx-auto flex-1 w-full flex flex-col gap-lg animate-fade-in">
            // Streamer Profile Header
            <Suspense fallback=move || view! {
                <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-xl p-md mb-lg relative overflow-hidden flex flex-col md:flex-row gap-md items-center md:items-end min-h-[180px] animate-pulse">
                    <div class="w-32 h-32 md:w-40 md:h-40 rounded-2xl bg-white/5 border border-white/10"></div>
                    <div class="flex-1 flex flex-col gap-xs text-center md:text-left">
                        <div class="h-8 w-48 bg-white/5 rounded mx-auto md:mx-0"></div>
                        <div class="h-4 w-full bg-white/5 rounded mt-sm"></div>
                    </div>
                </section>
            }>
                {move || {
                    streamer_resource.get().map(|res| {
                        match res {
                            Ok(Some(streamer)) => {
                                let avatar = streamer.avatar_url.clone();
                                let name = streamer.display_name.clone();
                                let bio = streamer.bio.clone();
                                let is_live = streamer.is_live;
                                view! {
                                    <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-xl p-md mb-lg relative overflow-hidden flex flex-col md:flex-row gap-md items-center md:items-end">
                                        <div class="absolute inset-0 opacity-20 bg-[url('https://www.transparenttextures.com/patterns/carbon-fibre.png')]"></div>
                                        <div class="relative w-32 h-32 md:w-40 md:h-40 rounded-2xl overflow-hidden border-2 border-secondary shadow-[0_0_20px_rgba(77,224,130,0.3)]">
                                            <img alt="Streamer Avatar" class="w-full h-full object-cover" src=avatar/>
                                        </div>
                                        <div class="flex-1 flex flex-col gap-xs text-center md:text-left relative z-10">
                                            <div class="flex items-center justify-center md:justify-start gap-sm">
                                                <h1 class="text-headline-lg font-headline-lg text-on-surface">{name}</h1>
                                                {if is_live {
                                                    view! {
                                                        <span class="bg-secondary/10 text-secondary border border-secondary/20 px-sm py-xs rounded-full text-label-sm font-label-sm flex items-center gap-xs">
                                                            <span class="w-2 h-2 rounded-full bg-secondary animate-pulse"></span>
                                                            "LIVE"
                                                        </span>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <span class="bg-white/10 text-on-surface-variant border border-white/15 px-sm py-xs rounded-full text-label-sm font-label-sm flex items-center gap-xs">
                                                            "OFFLINE"
                                                        </span>
                                                    }.into_any()
                                                }}
                                            </div>
                                            <p class="text-on-surface-variant text-body-md font-body-md max-w-[600px] mx-auto md:mx-0">{bio}</p>
                                        </div>
                                    </section>
                                  }.into_any()
                              }
                              _ => view! {
                                  <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-xl p-md mb-lg text-center text-on-surface-variant">
                                      "Streamer profile not found in database."
                                  </section>
                              }.into_any()
                          }
                      })
                  }}
              </Suspense>

              // Donation Form
              <div class="flex flex-col gap-gutter">
                  <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-xl p-md md:p-xl flex flex-col gap-md">
                      <div class="flex items-center gap-sm">
                          <span class="material-symbols-outlined text-secondary" data-icon="volunteer_activism">"volunteer_activism"</span>
                          <h2 class="text-headline-md font-headline-md text-on-surface">"Send a Glint"</h2>
                      </div>

                      // Your Name Input
                      <div class="flex flex-col gap-base">
                          <label class="text-label-md font-label-md text-on-surface-variant">"Your Name"</label>
                          <input 
                              data-testid="donor-name-input"
                              class="w-full bg-surface-container-low/40 border border-white/10 rounded-xl px-md py-md text-body-md font-body-md focus:outline-none focus:border-primary transition-all text-on-surface" 
                              placeholder="Enter your name (e.g. Anonymous)" 
                              type="text"
                              prop:value=move || donor_name.get()
                              on:input=move |ev| donor_name.set(event_target_value(&ev))
                          />
                      </div>

                      // Amount Selection
                      <div class="flex flex-col gap-base">
                          <label class="text-label-md font-label-md text-on-surface-variant">"Amount"</label>
                          <div class="grid grid-cols-2 md:grid-cols-4 gap-base">
                              {
                                  let amounts = vec!["5", "10", "25", "50"];
                                  amounts.into_iter().map(|amt| {
                                      let amt_clone = amt.to_string();
                                      view! {
                                          <button 
                                              class=move || {
                                                  if amount.get() == amt_clone {
                                                      "bg-white/5 backdrop-blur-md border-2 p-md rounded-xl text-headline-md font-headline-md text-on-surface transition-all border-secondary shadow-[inset_0_0_15px_rgba(77,224,130,0.1)]".to_string()
                                                  } else {
                                                      "bg-white/5 backdrop-blur-md border border-white/10 p-md rounded-xl text-headline-md font-headline-md text-on-surface hover:border-secondary transition-all".to_string()
                                                  }
                                              }
                                              on:click=move |_| amount.set(amt.to_string())
                                          >
                                              "$" {amt}
                                          </button>
                                      }
                                  }).collect_view()
                              }
                          </div>
                          <div class="relative mt-base">
                              <span class="absolute left-md top-1/2 -translate-y-1/2 text-on-surface-variant">"$"</span>
                              <input 
                                  class="w-full bg-surface-container-low/40 border border-white/10 rounded-xl px-xl py-md text-body-lg font-body-lg focus:outline-none focus:border-primary transition-all text-on-surface" 
                                  placeholder="Custom Amount" 
                                  type="number"
                                  prop:value=move || amount.get()
                                  on:input=move |ev| amount.set(event_target_value(&ev))
                              />
                          </div>
                      </div>

                      // Your Message
                      <div class="flex flex-col gap-base">
                          <label class="text-label-md font-label-md text-on-surface-variant">"Your Message"</label>
                          <textarea 
                              data-testid="donation-message-input"
                              class="w-full bg-surface-container-low/40 border border-white/10 rounded-xl px-md py-md text-body-md font-body-md focus:outline-none focus:border-primary transition-all min-h-[120px] resize-none text-on-surface" 
                              placeholder="Enter a message to be read on stream..."
                              prop:value=move || message.get()
                              on:input=move |ev| message.set(event_target_value(&ev))
                          ></textarea>
                      </div>

                      // Payment Method
                      <div class="flex flex-col gap-base">
                          <label class="text-label-md font-label-md text-on-surface-variant">"Payment Method"</label>
                          <div class="grid grid-cols-3 gap-base">
                              <label class="cursor-pointer" on:click=move |_| payment_method.set("Credit Card".to_string())>
                                  <input checked=move || payment_method.get() == "Credit Card" class="hidden peer" name="payment" type="radio"/>
                                  <div class="flex flex-col items-center justify-center bg-white/5 backdrop-blur-md p-sm rounded-xl text-center border border-white/10 peer-checked:border-primary peer-checked:bg-primary/5 transition-all">
                                      <span class="material-symbols-outlined text-primary mb-xs">"payments"</span>
                                      <span class="text-label-sm font-label-sm">"Credit Card"</span>
                                  </div>
                              </label>
                              <label class="cursor-pointer" on:click=move |_| payment_method.set("PayPal".to_string())>
                                  <input checked=move || payment_method.get() == "PayPal" class="hidden peer" name="payment" type="radio"/>
                                  <div class="flex flex-col items-center justify-center bg-white/5 backdrop-blur-md p-sm rounded-xl text-center border border-white/10 peer-checked:border-primary peer-checked:bg-primary/5 transition-all">
                                      <span class="material-symbols-outlined text-primary mb-xs">"account_balance_wallet"</span>
                                      <span class="text-label-sm font-label-sm">"PayPal"</span>
                                  </div>
                              </label>
                              <label class="cursor-pointer" on:click=move |_| payment_method.set("Crypto".to_string())>
                                  <input checked=move || payment_method.get() == "Crypto" class="hidden peer" name="payment" type="radio"/>
                                  <div class="flex flex-col items-center justify-center bg-white/5 backdrop-blur-md p-sm rounded-xl text-center border border-white/10 peer-checked:border-primary peer-checked:bg-primary/5 transition-all">
                                      <span class="material-symbols-outlined text-primary mb-xs">"currency_bitcoin"</span>
                                      <span class="text-label-sm font-label-sm">"Crypto"</span>
                                  </div>
                              </label>
                          </div>
                      </div>

                      // Donate Now Button
                      <button 
                          data-testid="donate-submit-btn"
                          class="bg-secondary-container text-on-secondary-container py-md rounded-xl text-headline-md font-headline-md hover:shadow-[0_0_30px_rgba(0,181,93,0.3)] transition-all mt-base active:scale-[0.98]"
                          on:click=handle_submit
                      >
                          "Donate Now"
                      </button>
                      {move || {
                          form_error.get().map(|err| view! {
                              <div class="mt-base bg-error-container/20 border border-error/30 text-error rounded-xl px-md py-sm text-body-sm">
                                  {err}
                              </div>
                          })
                      }}

                      {move || if show_payment_window.get() {
                          let status = mock_tx_status.get();
                          view! {
                              <div class="mt-base bg-white/5 backdrop-blur-md border border-white/10 rounded-xl p-md flex flex-col gap-base">
                                  <div class="flex items-center justify-between">
                                      <div class="text-on-surface font-semibold">"Mock payment (test)"</div>
                                      <div class="text-label-sm text-on-surface-variant">
                                          "Status: " <span class="text-on-surface">{status.clone()}</span>
                                      </div>
                                  </div>

                                  {move || {
                                      if let Some(qr) = mock_display_qr.get() {
                                          view! {
                                              <div class="flex flex-col gap-xs">
                                                  <div class="text-label-sm text-on-surface-variant">"QR (mock)"</div>
                                                  <pre class="bg-surface-container-low/40 border border-white/10 rounded-xl p-sm text-body-sm overflow-x-auto text-on-surface">{qr}</pre>
                                              </div>
                                          }.into_any()
                                      } else if let Some(url) = mock_display_url.get() {
                                          let href = url.clone();
                                          view! {
                                              <div class="flex flex-col gap-xs">
                                                  <div class="text-label-sm text-on-surface-variant">"Payment link (mock)"</div>
                                                  <a class="text-primary underline break-all" href=href target="_blank" rel="noopener noreferrer">{url}</a>
                                              </div>
                                          }.into_any()
                                      } else {
                                          view! {}.into_any()
                                      }
                                  }}

                                  <div class="flex flex-col gap-xs">
                                      <label class="text-label-sm text-on-surface-variant">"OTP (mock)"</label>
                                      <input
                                          data-testid="mock-otp-input"
                                          class="w-full bg-surface-container-low/40 border border-white/10 rounded-xl px-md py-sm text-body-md focus:outline-none focus:border-primary transition-all text-on-surface"
                                          placeholder="Enter OTP (any value)"
                                          type="text"
                                          prop:value=move || otp.get()
                                          on:input=move |ev| otp.set(event_target_value(&ev))
                                      />
                                  </div>

                                  {move || if status == "READY_FOR_DISPLAY" {
                                      view! {
                                          <div data-testid="payment-success-msg" class="bg-secondary/10 border border-secondary/20 text-secondary rounded-xl px-md py-sm">
                                              "Payment success (mock)"
                                          </div>
                                      }.into_any()
                                  } else {
                                      view! {
                                          <div class="text-on-surface-variant text-body-sm">
                                              "Waiting for payment to be ready..."
                                          </div>
                                      }.into_any()
                                  }}
                              </div>
                          }.into_any()
                      } else {
                          view! {}.into_any()
                      }}
                      <p class="text-center text-label-sm font-label-sm text-on-surface-variant">"Glint matches 5% of all stream donations today."</p>
                  </section>
              </div>

              // Recent Tributes Section
              <section data-testid="recent-tributes-section" class="bg-white/5 backdrop-blur-md border border-white/10 rounded-xl p-md md:p-xl flex flex-col gap-md">
                  <div class="flex items-center gap-sm">
                      <span class="material-symbols-outlined text-primary" data-icon="history">"history"</span>
                      <h2 class="text-headline-md font-headline-md text-on-surface">"Recent Tributes"</h2>
                  </div>
                  
                  <Suspense fallback=move || view! { <div class="text-on-surface-variant">"Loading recent tributes..."</div> }>
                      {move || {
                          transactions_resource.get().map(|res| {
                              match res {
                                  Ok(txs) => {
                                      if txs.is_empty() {
                                          view! {
                                              <p class="text-on-surface-variant text-center py-md">"No tributes yet. Be the first to send a Glint!"</p>
                                          }.into_any()
                                      } else {
                                          view! {
                                              <div class="flex flex-col gap-base">
                                                  {txs.into_iter().map(|tx| {
                                                      let msg = tx.message.clone();
                                                      view! {
                                                          <div class="bg-white/5 border border-white/10 rounded-xl p-base flex flex-col gap-xs transition-all hover:bg-white/10">
                                                              <div class="flex justify-between items-center">
                                                                  <span class="text-on-surface font-semibold text-body-md">{tx.donor_name.clone()}</span>
                                                                  <span class="text-secondary font-bold text-headline-sm">"$" {format!("{:.2}", tx.amount)}</span>
                                                                </div>
                                                                {if let Some(msg_str) = msg {
                                                                    view! {
                                                                        <p class="text-on-surface-variant text-body-sm italic bg-surface-container-low/40 border border-white/5 rounded-lg p-sm mt-xs">
                                                                            "\"" {msg_str} "\""
                                                                        </p>
                                                                    }.into_any()
                                                                } else {
                                                                    view! {}.into_any()
                                                                }}
                                                                <div class="flex justify-between items-center text-label-sm text-on-surface-variant/80 mt-xs">
                                                                    <span>"via " {tx.payment_method.clone()}</span>
                                                                    <span>{tx.created_at.clone()}</span>
                                                                </div>
                                                            </div>
                                                      }
                                                  }).collect_view()}
                                              </div>
                                          }.into_any()
                                      }
                                  }
                                  Err(_) => view! { <div class="text-on-surface-variant text-center">"Failed to load recent tributes."</div> }.into_any()
                              }
                          })
                      }}
                  </Suspense>
              </section>
          </main>
          <Footer/>
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

#[server(GetStreamer, "/api")]
pub async fn get_streamer(username: String) -> Result<Option<DbStreamer>, ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let row = sqlx::query(
        "SELECT id, username, display_name, avatar_url, bio, is_live, user_id, overlay_token, active_overlay_session FROM streamers WHERE username = $1"
    )
    .bind(username)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

    match row {
        Some(r) => {
            use sqlx::Row;
            Ok(Some(DbStreamer {
                id: r.get("id"),
                username: r.get("username"),
                display_name: r.get("display_name"),
                avatar_url: r.get("avatar_url"),
                bio: r.get("bio"),
                is_live: r.get("is_live"),
                user_id: r.try_get("user_id").unwrap_or(None),
                overlay_token: r.get("overlay_token"),
                active_overlay_session: r.try_get("active_overlay_session").unwrap_or(None),
            }))
        }
        None => Ok(None),
    }
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
        "SELECT id, username, display_name, avatar_url, bio, is_live, user_id, overlay_token, active_overlay_session FROM streamers WHERE user_id = $1"
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
    let avatar_url = format!("https://ui-avatars.com/api/?name={}", urlencoding::encode(&display_name));

    let row = sqlx::query(
        "INSERT INTO streamers (username, display_name, avatar_url, bio, is_live, user_id, overlay_token)
         VALUES ($1, $2, $3, $4, $5, $6, gen_random_uuid())
         RETURNING id, username, display_name, avatar_url, bio, is_live, user_id, overlay_token, active_overlay_session"
    )
    .bind(&username)
    .bind(&display_name)
    .bind(&avatar_url)
    .bind(&bio)
    .bind(false)
    .bind(&user.id)
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
    }))
}

#[server(GetAllStreamers, "/api")]
pub async fn get_all_streamers() -> Result<Vec<DbStreamer>, ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let rows = sqlx::query(
        "SELECT id, username, display_name, avatar_url, bio, is_live, user_id, overlay_token, active_overlay_session FROM streamers ORDER BY is_live DESC, id DESC"
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
        }
    }).collect();

    Ok(streamers)
}

#[server(CreateDonation, "/api")]
pub async fn create_donation(
    streamer_id: i32,
    donor_name: String,
    amount: f64,
    message: String,
    payment_method: String,
) -> Result<i32, ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let donor = if donor_name.trim().is_empty() {
        "Anonymous".to_string()
    } else {
        donor_name
    };

    let message_opt = if message.trim().is_empty() {
        None
    } else {
        Some(message)
    };

    let row = sqlx::query(
        "INSERT INTO transactions (streamer_id, donor_name, amount, message, payment_method, status)\n         VALUES ($1, $2, $3, $4, $5, $6)\n         RETURNING id"
    )
    .bind(streamer_id)
    .bind(donor)
    .bind(amount)
    .bind(message_opt)
    .bind(payment_method)
    .bind("INITIALIZE")
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database insert failed: {}", e)))?;

    use sqlx::Row;
    Ok(row.get::<i32, _>("id"))
}

#[cfg(feature = "ssr")]
mod mock_payments {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use tokio::sync::RwLock;

    #[derive(Clone, Debug)]
    pub struct MockTxState {
        pub status: String,
    }

    pub static MOCK_TXS: Lazy<RwLock<HashMap<i32, MockTxState>>> =
        Lazy::new(|| RwLock::new(HashMap::new()));
}

#[server(CreateMockPayment, "/api")]
pub async fn create_mock_payment(
    streamer_id: i32,
    donor_name: String,
    amount: f64,
    message: String,
    payment_method: String,
) -> Result<MockPaymentInit, ServerFnError> {
    let tx_id = create_donation(
        streamer_id,
        donor_name,
        amount,
        message,
        payment_method.clone(),
    )
    .await?;

    let (display_qr, display_url) = match payment_method.as_str() {
        "Crypto" => (Some(format!("MOCK-QR:tx_id={tx_id};amount={amount:.2}")), None),
        "PayPal" => (None, Some(format!("https://example.com/mock-pay?provider=paypal&tx_id={tx_id}"))),
        _ => (None, Some(format!("https://example.com/mock-pay?provider=card&tx_id={tx_id}"))),
    };

    #[cfg(feature = "ssr")]
    {
        use tokio::time::{sleep, Duration};

        mock_payments::MOCK_TXS
            .write()
            .await
            .insert(tx_id, mock_payments::MockTxState { status: "INITIALIZE".to_string() });

        let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
            .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

        let delay_ms = 2000u64 + (chrono::Utc::now().timestamp_subsec_millis() as u64 % 3001u64);
        tokio::spawn(async move {
            sleep(Duration::from_millis(delay_ms)).await;
            if let Some(state) = mock_payments::MOCK_TXS.write().await.get_mut(&tx_id) {
                state.status = "READY_FOR_DISPLAY".to_string();
            }
            let _ = sqlx::query("UPDATE transactions SET status = 'READY_FOR_DISPLAY' WHERE id = $1")
                .bind(tx_id)
                .execute(&pool)
                .await;
        });
    }

    Ok(MockPaymentInit {
        tx_id,
        status: "INITIALIZE".to_string(),
        display_qr,
        display_url,
    })
}

#[server(GetMockPaymentStatus, "/api")]
pub async fn get_mock_payment_status(tx_id: i32) -> Result<MockPaymentStatus, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let status = mock_payments::MOCK_TXS
            .read()
            .await
            .get(&tx_id)
            .map(|s| s.status.clone())
            .unwrap_or_else(|| "INITIALIZE".to_string());

        return Ok(MockPaymentStatus { tx_id, status });
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = tx_id;
        Ok(MockPaymentStatus {
            tx_id,
            status: "INITIALIZE".to_string(),
        })
    }
}

#[server(GetRecentTransactions, "/api")]
pub async fn get_recent_transactions(username: String) -> Result<Vec<DbTransaction>, ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let rows = sqlx::query(
        "SELECT t.id, t.streamer_id, t.donor_name, t.amount, t.message, t.payment_method, t.status, TO_CHAR(t.created_at, 'YYYY-MM-DD HH:MI AM') as formatted_date 
         FROM transactions t
         JOIN streamers s ON t.streamer_id = s.id
         WHERE s.username = $1 
         ORDER BY t.id DESC 
         LIMIT 10"
    )
    .bind(username)
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

    Ok(txs)
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

#[server(PollOverlayTransactions, "/api")]
pub async fn poll_overlay_transactions(token: String, session_id: String) -> Result<Vec<DbTransaction>, ServerFnError> {
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
