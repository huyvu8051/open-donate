use crate::db::{TransactionStatus, PaymentMethod};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    hooks::{use_location, use_params_map},
    ParamSegment, StaticSegment,
};
use crate::auth::User;
use crate::db::{DbStreamer, DbTransaction};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MockPaymentInit {
    pub tx_id: i32,
    pub status: TransactionStatus,
    pub display_qr: Option<String>,
    pub display_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MockPaymentStatus { pub tx_id: i32, pub status: TransactionStatus, }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LeaderboardEntry {
    pub streamer_id: i32,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub total_amount: f64,
    pub donation_count: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StreamerAnalytics {
    pub total_revenue: f64,
    pub donation_count: i64,
    pub top_donors: Vec<(String, f64)>,
    pub revenue_over_time: Vec<(String, f64)>,
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
    let user_resource = Resource::new(|| (), |_| get_me());
    provide_context(user_resource);

    view! {
        <I18nProvider>
            <Stylesheet id="leptos" href="/pkg/open-donate.css"/>
            <Title text="Glint | Empower Your Content"/>

            <Router>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=LandingPage/>
                    <Route path=StaticSegment("explore") view=ExplorePage/>
                    <Route path=StaticSegment("leaderboard") view=LeaderboardPage/>
                    <Route path=StaticSegment("about") view=crate::pages::AboutPage/>
                    <Route path=StaticSegment("faq") view=crate::pages::FaqPage/>
                    <Route path=StaticSegment("privacy") view=crate::pages::PrivacyPage/>
                    <Route path=StaticSegment("terms") view=crate::pages::TermsPage/>
                    <ParentRoute path=StaticSegment("dashboard") view=crate::dashboard::DashboardLayout>
                        <Route path=StaticSegment("") view=crate::dashboard::DashboardHome/>
                        <Route path=StaticSegment("settings") view=crate::dashboard::SettingsPage/>
                        <Route path=StaticSegment("payments") view=crate::dashboard::PaymentsPage/>
                        <Route path=StaticSegment("analytics") view=crate::dashboard::AnalyticsPage/>
                    </ParentRoute>
                    <Route path=StaticSegment("login") view=crate::auth::LoginPage/>
                    <Route path=StaticSegment("register") view=crate::auth::RegisterPage/>
                    <Route path=(StaticSegment("streamer"), ParamSegment("username")) view=StreamerPage/>
                    <Route path=(StaticSegment("overlay"), ParamSegment("token")) view=crate::overlay::OverlayPage/>
                </Routes>
            </Router>
        </I18nProvider>
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
    let (is_open, set_is_open) = signal(false);
    let logout_action = ServerAction::<crate::auth::Logout>::new();

    Effect::new(move |_| {
        if let Some(Ok(_)) = logout_action.value().get() {
            let _ = leptos::prelude::window().location().set_href("/");
        }
    });

    view! {
        <header class="fixed top-0 w-full z-50 bg-surface/60 backdrop-blur-xl border-b border-white/20 shadow-sm flex flex-col px-margin-mobile md:px-margin-desktop">
            <div class="h-20 flex justify-between items-center w-full">
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
                                        <a href="/dashboard" class="hidden sm:inline text-on-surface text-label-md font-semibold hover:text-primary transition-colors">"Hello, " {user.name.clone()}</a>
                                        <ActionForm action=logout_action>
                                            <button data-testid="logout-button" type="submit" 
                                                class=move || format!("px-sm py-xs bg-surface-container-highest border border-white/10 text-on-surface rounded-lg text-label-sm font-label-sm transition-all flex items-center gap-xs {}", if logout_action.pending().get() { "opacity-50 cursor-wait" } else { "hover:bg-surface-container-highest/80" })
                                                disabled=move || logout_action.pending().get()>
                                                {move || if logout_action.pending().get() {
                                                    view! { <span class="material-symbols-outlined text-[14px] animate-spin">"progress_activity"</span> }.into_any()
                                                } else {
                                                    view! { }.into_any()
                                                }}
                                                {leptos_fluent::move_tr!("header-logout")}
                                            </button>
                                        </ActionForm>
                                        <LanguageSwitcher />
                                    </div>
                                }.into_any(),
                                _ => view! {
                                    <div class="flex items-center gap-sm">
                                        <LanguageSwitcher />
                                    </div>
                                }.into_any()
                            }
                        }}
                    </Suspense>
                    <button class="md:hidden text-on-surface flex items-center justify-center" on:click=move |_| set_is_open.update(|o| *o = !*o)>
                        <span class="material-symbols-outlined">{move || if is_open.get() { "close" } else { "menu" }}</span>
                    </button>
                </div>
            </div>
            {move || if is_open.get() {
                view! {
                    <nav class="md:hidden flex flex-col gap-sm pb-md border-t border-white/10 pt-md">
                        <a class=move || nav_class("/") href="/">"Explore"</a>
                        <a class=move || nav_class("/explore") href="/explore">"Creators"</a>
                        <a class=move || nav_class("/leaderboard") href="/leaderboard">"Leaderboard"</a>
                        <Suspense fallback=move || view! {}>
                            {move || match user_resource.get() {
                                Some(Ok(Some(_))) => view! {
                                    <a class=move || nav_class("/dashboard") href="/dashboard">"Dashboard"</a>
                                }.into_any(),
                                _ => view! {}.into_any()
                            }}
                        </Suspense>
                    </nav>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </header>
    }
}

#[component]
pub fn LanguageSwitcher() -> impl IntoView {
    let i18n = expect_context::<leptos_fluent::I18n>();

    view! {
        <div class="flex items-center gap-1 bg-surface-container border border-white/10 rounded-lg p-1 shadow-inner">
            <button
                class=move || format!("px-2 py-0.5 rounded text-[10px] font-bold transition-all {}", if i18n.language.get().id.to_string() == "en" { "bg-primary text-on-primary shadow-sm" } else { "text-on-surface-variant hover:text-on-surface" })
                on:click=move |_| {
                    if let Some(lang) = i18n.languages.iter().find(|l| l.id.to_string() == "en") {
                        i18n.language.set(lang);
                    }
                }
            >
                "EN"
            </button>
            <button
                class=move || format!("px-2 py-0.5 rounded text-[10px] font-bold transition-all {}", if i18n.language.get().id.to_string() == "vi" { "bg-primary text-on-primary shadow-sm" } else { "text-on-surface-variant hover:text-on-surface" })
                on:click=move |_| {
                    if let Some(lang) = i18n.languages.iter().find(|l| l.id.to_string() == "vi") {
                        i18n.language.set(lang);
                    }
                }
            >
                "VI"
            </button>
        </div>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-surface-dim border-t border-surface-container-highest w-full py-lg mt-auto">
            <div class="max-w-7xl mx-auto px-margin-desktop flex flex-col md:flex-row justify-between items-center gap-md">
                <div class="flex flex-col items-center md:items-start gap-xs">
                    <a href="/" class="text-headline-md font-headline-md font-bold text-primary hover:text-primary/80 transition-colors">"Glint"</a>
                    <p class="text-on-surface-variant text-body-md font-body-md">{leptos_fluent::move_tr!("footer-copyright")}</p>
                </div>
                <nav class="flex flex-wrap justify-center gap-md">
                    <a class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors" href="/about">{leptos_fluent::move_tr!("footer-about")}</a>
                    <a class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors" href="/faq">{leptos_fluent::move_tr!("footer-faq")}</a>
                    <a class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors" href="/privacy">{leptos_fluent::move_tr!("footer-privacy")}</a>
                    <a class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors" href="/terms">{leptos_fluent::move_tr!("footer-terms")}</a>
                    <a class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors" href="#">{leptos_fluent::move_tr!("footer-discord")}</a>
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
                    {leptos_fluent::move_tr!("landing-future-of-support")}
                </span>
                <h1 class="text-headline-xl-mobile md:text-headline-xl font-headline-xl text-on-surface">
                    {leptos_fluent::move_tr!("landing-title-start")} <span class="text-primary italic">{leptos_fluent::move_tr!("landing-title-glint")}</span> {leptos_fluent::move_tr!("landing-title-end")}
                </h1>
                <p class="text-body-lg font-body-lg text-on-surface-variant/80 max-w-lg mx-auto italic">
                    {leptos_fluent::move_tr!("landing-trusted-by")}
                </p>
                <div class="flex flex-col md:flex-row items-center justify-center gap-md pt-base">
                    <button class="px-lg py-md bg-secondary text-on-secondary-container rounded-xl font-headline-md text-headline-md neon-glow-secondary hover:scale-105 transition-transform active:scale-95 duration-150">
                        {leptos_fluent::move_tr!("landing-donate-now")}
                    </button>
                    <a href="/register" class="px-lg py-md bg-surface-container-highest/40 backdrop-blur-md border border-white/20 text-on-surface rounded-xl font-headline-md text-headline-md hover:bg-surface-container-highest/60 transition-all inline-block text-center">
                        {leptos_fluent::move_tr!("landing-start-creating")}
                    </a>
                </div>
            </div>
            <div class="mt-xl grid grid-cols-1 md:grid-cols-3 gap-md w-full max-w-5xl">
                <div class="glass-card flex-1 min-w-[200px] p-lg md:p-xl rounded-2xl flex flex-col items-center justify-center gap-xs">
                    <span class="text-display-sm md:text-display-md font-display-md font-bold text-primary">"10K+"</span>
                    <span class="text-label-md md:text-label-lg font-label-lg text-on-surface-variant font-medium tracking-wide uppercase">{leptos_fluent::move_tr!("landing-active-creators")}</span>
                </div>
                <div class="glass-card flex-1 min-w-[200px] p-lg md:p-xl rounded-2xl flex flex-col items-center justify-center gap-xs">
                    <span class="text-display-sm md:text-display-md font-display-md font-bold text-secondary">"2M+"</span>
                    <span class="text-label-md md:text-label-lg font-label-lg text-on-surface-variant font-medium tracking-wide uppercase">{leptos_fluent::move_tr!("landing-live-viewers")}</span>
                </div>
                <div class="glass-card flex-1 min-w-[200px] p-lg md:p-xl rounded-2xl flex flex-col items-center justify-center gap-xs">
                    <span class="text-display-sm md:text-display-md font-display-md font-bold text-tertiary">"50M+"</span>
                    <span class="text-label-md md:text-label-lg font-label-lg text-on-surface-variant font-medium tracking-wide uppercase">{leptos_fluent::move_tr!("landing-total-glints")}</span>
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
                        {leptos_fluent::move_tr!("explore-title")}
                    </h1>
                    <p class="text-headline-sm font-headline-sm text-on-surface-variant max-w-2xl mx-auto">
                        {leptos_fluent::move_tr!("explore-subtitle")}
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
                                                                            {leptos_fluent::move_tr!("explore-live")}
                                                                        </div>
                                                                    }.into_any()
                                                                } else {
                                                                    view! {
                                                                        <div class="absolute -bottom-1 -right-1 bg-surface-variant text-on-surface-variant text-[10px] font-bold px-2 py-0.5 rounded-full border-2 border-surface">
                                                                            {leptos_fluent::move_tr!("explore-offline")}
                                                                        </div>
                                                                    }.into_any()
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
                                                            <span class="text-primary text-label-md font-label-md font-bold group-hover:underline">{leptos_fluent::move_tr!("explore-view-profile")} " →"</span>
                                                        </div>
                                                    </a>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }
                            Err(_) => view! {
                                <div class="text-center text-error mt-xl">{leptos_fluent::move_tr!("explore-error")}</div>
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
                <h2 class="text-headline-lg font-headline-lg text-on-surface mb-xl">{leptos_fluent::move_tr!("landing-how-it-works")}</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-lg relative">
                    <div class="hidden md:block absolute top-1/2 left-0 w-full h-px bg-gradient-to-r from-transparent via-white/20 to-transparent -translate-y-1/2 z-0"></div>
                    <div class="relative z-10 group">
                        <div class="w-16 h-16 bg-surface-container-highest rounded-full flex items-center justify-center mx-auto border-2 border-primary neon-glow-primary mb-md group-hover:scale-110 transition-transform">
                            <span class="text-headline-md font-headline-md text-primary">"1"</span>
                        </div>
                        <h4 class="text-headline-md font-headline-md text-on-surface">{leptos_fluent::move_tr!("how-step1-title")}</h4>
                        <p class="text-on-surface-variant mt-sm">{leptos_fluent::move_tr!("how-step1-desc")}</p>
                    </div>
                    <div class="relative z-10 group">
                        <div class="w-16 h-16 bg-surface-container-highest rounded-full flex items-center justify-center mx-auto border-2 border-secondary neon-glow-secondary mb-md group-hover:scale-110 transition-transform">
                            <span class="text-headline-md font-headline-md text-secondary">"2"</span>
                        </div>
                        <h4 class="text-headline-md font-headline-md text-on-surface">{leptos_fluent::move_tr!("how-step2-title")}</h4>
                        <p class="text-on-surface-variant mt-sm">{leptos_fluent::move_tr!("how-step2-desc")}</p>
                    </div>
                    <div class="relative z-10 group">
                        <div class="w-16 h-16 bg-surface-container-highest rounded-full flex items-center justify-center mx-auto border-2 border-tertiary mb-md group-hover:scale-110 transition-transform">
                            <span class="text-headline-md font-headline-md text-tertiary">"3"</span>
                        </div>
                        <h4 class="text-headline-md font-headline-md text-on-surface">{leptos_fluent::move_tr!("how-step3-title")}</h4>
                        <p class="text-on-surface-variant mt-sm">{leptos_fluent::move_tr!("how-step3-desc")}</p>
                    </div>
                </div>
            </section>
            
            // CTA Section
            <section class="py-xl px-margin-mobile md:px-margin-desktop text-center">
                <div class="max-w-4xl mx-auto bg-gradient-to-br from-primary/10 to-secondary/10 backdrop-blur-xl p-xl rounded-[2rem] border border-white/10">
                    <h2 class="text-headline-lg md:text-headline-xl font-headline-xl text-on-surface mb-md">{leptos_fluent::move_tr!("cta-title")}</h2>
                    <p class="text-body-lg text-on-surface-variant mb-lg">{leptos_fluent::move_tr!("cta-subtitle")}</p>
                    <div class="flex flex-col sm:flex-row items-center justify-center gap-md">
                        <button class="w-full sm:w-auto px-lg py-md bg-secondary text-on-secondary-container rounded-xl font-headline-md text-headline-md neon-glow-secondary hover:scale-105 transition-transform pulse-accent">
                            {leptos_fluent::move_tr!("cta-donate-now")}
                        </button>
                        <button class="w-full sm:w-auto px-lg py-md bg-surface text-on-surface border border-white/20 rounded-xl font-headline-md text-headline-md hover:bg-surface-container-highest transition-colors">
                            {leptos_fluent::move_tr!("cta-view-leaders")}
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
    let payment_method = RwSignal::new(PaymentMethod::CreditCard);
    let transactions_trigger = RwSignal::new(0);
    let form_error = RwSignal::new(None::<String>);

    let show_payment_window = RwSignal::new(false);
    let otp = RwSignal::new("".to_string());
    let mock_tx_id = RwSignal::new(None::<i32>);
    let mock_tx_status = RwSignal::new(TransactionStatus::Initialize);
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

    let accept_action = Action::new(move |tx_id: &i32| {
        let tx = *tx_id;
        async move { accept_mock_payment(tx).await }
    });

    let reject_action = Action::new(move |tx_id: &i32| {
        let tx = *tx_id;
        async move { reject_mock_payment(tx).await }
    });

    // React to payment init
    Effect::new(move || {
        if let Some(result) = payment_action.value().get() {
            match result {
                Ok(init) => {
                    show_payment_window.set(true);
                    mock_tx_id.set(Some(init.tx_id));
                    mock_tx_status.set(init.status.clone());
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
                                        if new_status == TransactionStatus::ReadyForDisplay {
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
        // enum is never empty
        if false {
            form_error.set(Some("Please select a payment method.".to_string()));
            return;
        }
        form_error.set(None);
        show_payment_window.set(false);
        otp.set("".to_string());
        mock_tx_id.set(None);
        mock_tx_status.set(TransactionStatus::Initialize);
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
        <main class="pt-24 pb-xl px-margin-mobile md:px-margin-desktop max-w-5xl mx-auto flex-1 w-full flex flex-col gap-md animate-fade-in relative">
            <div aria-hidden="true" class="pointer-events-none absolute -top-24 -left-24 h-72 w-72 rounded-full bg-gradient-to-br from-primary/30 via-secondary/10 to-transparent blur-3xl"></div>
            <div aria-hidden="true" class="pointer-events-none absolute -top-16 -right-20 h-72 w-72 rounded-full bg-gradient-to-br from-secondary/25 via-primary/10 to-transparent blur-3xl"></div>
            // Streamer Profile Header
            <Suspense fallback=move || view! {
                <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl p-md mb-md relative overflow-hidden flex flex-col md:flex-row gap-md items-center md:items-center min-h-[140px] animate-pulse shadow-[0_20px_60px_rgba(0,0,0,0.25)] ring-1 ring-white/5">
                    <div class="w-24 h-24 md:w-28 md:h-28 rounded-2xl bg-white/5 border border-white/10"></div>
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
                                    <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl p-md mb-md relative overflow-hidden flex flex-col md:flex-row gap-md items-center md:items-center shadow-[0_20px_60px_rgba(0,0,0,0.25)] ring-1 ring-white/5">
                                        <div class="absolute inset-0 opacity-70 bg-gradient-to-br from-white/12 via-transparent to-transparent"></div>
                                        <div class="relative w-24 h-24 md:w-28 md:h-28 rounded-2xl overflow-hidden border border-white/15 shadow-[0_0_0_6px_rgba(255,255,255,0.03)]">
                                            <img alt="Streamer Avatar" class="w-full h-full object-cover" src=avatar/>
                                        </div>
                                        <div class="flex-1 flex flex-col gap-xs text-center md:text-left relative z-10">
                                            <div class="flex items-center justify-center md:justify-start gap-sm">
                                                <h1 class="text-headline-md md:text-headline-lg font-headline-lg text-on-surface">{name}</h1>
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
                                            <p class="text-on-surface-variant text-body-sm md:text-body-md font-body-md leading-snug max-w-prose mx-auto md:mx-0">{bio}</p>
                                            <div class="flex items-center justify-center md:justify-start gap-sm text-label-sm text-on-surface-variant/90">
                                                <span class="inline-flex items-center gap-1 rounded-full border border-white/10 bg-white/5 px-sm py-xs">
                                                    <span class="material-symbols-outlined text-primary text-[18px]">"bolt"</span>
                                                    "Fast"
                                                </span>
                                                <span class="inline-flex items-center gap-1 rounded-full border border-white/10 bg-white/5 px-sm py-xs">
                                                    <span class="material-symbols-outlined text-primary text-[18px]">"lock"</span>
                                                    "Secure"
                                                </span>
                                                <span class="inline-flex items-center gap-1 rounded-full border border-white/10 bg-white/5 px-sm py-xs">
                                                    <span class="material-symbols-outlined text-primary text-[18px]">"favorite"</span>
                                                    "Support"
                                                </span>
                                            </div>
                                        </div>
                                    </section>
                                  }.into_any()
                              }
                              _ => view! {
                                  <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl p-md mb-md text-center text-on-surface-variant">
                                      "Streamer profile not found in database."
                                  </section>
                              }.into_any()
                          }
                      })
                  }}
              </Suspense>

              <div class="grid grid-cols-1 md:grid-cols-5 gap-md items-start relative z-10">
                  // Donation Form
                  <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl p-md md:p-lg flex flex-col gap-md md:col-span-3 shadow-[0_20px_60px_rgba(0,0,0,0.25)] ring-1 ring-white/5">
                      <div class="flex items-center gap-sm">
                          <span class="material-symbols-outlined text-secondary" data-icon="volunteer_activism">"volunteer_activism"</span>
                          <h2 class="text-headline-sm md:text-headline-md font-headline-md text-on-surface">{leptos_fluent::move_tr!("donate-send-a-glint")}</h2>
                      </div>

                      // Your Name Input
                      <div class="flex flex-col gap-xs">
                          <label class="text-label-md font-label-md text-on-surface-variant">{leptos_fluent::move_tr!("donate-your-name")}</label>
                          <input
                              data-testid="donor-name-input"
                              class="w-full bg-surface-container-low/40 border border-white/10 rounded-xl px-md py-sm text-body-md font-body-md focus:outline-none focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all text-on-surface"
                              placeholder=move || leptos_fluent::tr!("donate-your-name-placeholder")
                              type="text"
                              prop:value=move || donor_name.get()
                              on:input=move |ev| donor_name.set(event_target_value(&ev))
                          />
                      </div>

                      // Amount Selection
                      <div class="flex flex-col gap-xs">
                          <label class="text-label-md font-label-md text-on-surface-variant">{leptos_fluent::move_tr!("donate-amount")}</label>
                          <div class="grid grid-cols-2 md:grid-cols-4 gap-sm">
                              {
                                  let amounts = vec!["5", "10", "25", "50"];
                                  amounts.into_iter().map(|amt| {
                                      let amt_clone = amt.to_string();
                                      view! {
                                          <button
                                              class=move || {
                                                  if amount.get() == amt_clone {
                                                      "bg-white/5 backdrop-blur-md border-2 px-md py-sm rounded-xl text-body-md font-semibold text-on-surface transition-all border-secondary shadow-[inset_0_0_15px_rgba(77,224,130,0.12)] hover:-translate-y-[1px]".to_string()
                                                  } else {
                                                      "bg-white/5 backdrop-blur-md border border-white/10 px-md py-sm rounded-xl text-body-md font-semibold text-on-surface hover:border-secondary transition-all hover:-translate-y-[1px]".to_string()
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
                          <div class="relative mt-sm">
                              <span class="absolute left-md top-1/2 -translate-y-1/2 text-on-surface-variant">"$"</span>
                              <input
                                  class="w-full bg-surface-container-low/40 border border-white/10 rounded-xl pl-xl pr-md py-sm text-body-md font-body-md focus:outline-none focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all text-on-surface"
                                  placeholder="Custom Amount" 
                                  type="number"
                                  prop:value=move || amount.get()
                                  on:input=move |ev| amount.set(event_target_value(&ev))
                              />
                          </div>
                      </div>

                      // Your Message
                      <div class="flex flex-col gap-xs">
                          <label class="text-label-md font-label-md text-on-surface-variant">{leptos_fluent::move_tr!("donate-message")}</label>
                          <textarea
                              data-testid="donation-message-input"
                              class="w-full bg-surface-container-low/40 border border-white/10 rounded-xl px-md py-sm text-body-md font-body-md focus:outline-none focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all min-h-[96px] resize-none text-on-surface"
                              placeholder=move || leptos_fluent::tr!("donate-message-placeholder")
                              prop:value=move || message.get()
                              on:input=move |ev| message.set(event_target_value(&ev))
                          ></textarea>
                      </div>

                      // Payment Method
                      <div class="flex flex-col gap-xs">
                          <label class="text-label-md font-label-md text-on-surface-variant">{leptos_fluent::move_tr!("donate-payment-method")}</label>
                          <div class="grid grid-cols-2 sm:grid-cols-3 gap-sm">
                              <Suspense fallback=move || view! { <span class="text-on-surface-variant">"Loading methods..."</span> }>
                                  {move || {
                                      match streamer_resource.get() {
                                          Some(Ok(Some(s))) => {
                                              let methods = s.payment_methods.clone();
                                              if methods.is_empty() {
                                                  view! { <span class="text-on-surface-variant col-span-3">"No payment methods available."</span> }.into_any()
                                              } else {
                                                  methods.into_iter().map(|pm| {
                                                      let icon = if pm == PaymentMethod::MockAuto { "autorenew" } else { "pan_tool" };
                                                      let pm_clone = pm.clone();
                                                      let pm_clone2 = pm.clone();
                                                      view! {
                                                          <label class="cursor-pointer" on:click=move |_| payment_method.set(pm_clone.clone())>
                                                              <input checked=move || payment_method.get() == pm_clone2 class="hidden peer" name="payment" type="radio"/>
                                                              <div class="flex items-center justify-center gap-xs bg-white/5 backdrop-blur-md px-sm py-sm rounded-xl text-center border border-white/10 peer-checked:border-primary peer-checked:bg-primary/5 transition-all hover:-translate-y-[1px] hover:border-white/20">
                                                                  <span class="material-symbols-outlined text-primary text-[18px]">{icon}</span>
                                                                  <span class="text-label-sm font-label-sm">{pm.to_string()}</span>
                                                              </div>
                                                          </label>
                                                      }
                                                  }).collect_view().into_any()
                                              }
                                          },
                                          _ => view! {}.into_any()
                                      }
                                  }}
                              </Suspense>
                          </div>
                      </div>

                      // Donate Now Button
                      <button
                          data-testid="donate-submit-btn"
                          class="bg-gradient-to-r from-secondary to-secondary-container text-on-secondary-container py-sm rounded-xl text-body-lg font-bold shadow-[0_18px_50px_rgba(0,181,93,0.20)] hover:shadow-[0_18px_60px_rgba(0,181,93,0.30)] transition-all mt-sm active:scale-[0.98] hover:-translate-y-[1px]"
                          on:click=handle_submit
                      >
                          {leptos_fluent::move_tr!("landing-donate-now")}
                      </button>
                      {move || {
                          form_error.get().map(|err| view! {
                              <div class="mt-sm bg-error-container/20 border border-error/30 text-error rounded-xl px-md py-sm text-body-sm">
                                  {err}
                              </div>
                          })
                      }}

                      {move || if show_payment_window.get() {
                          let status = mock_tx_status.get();
                          view! {
                              <div class="mt-sm bg-white/5 backdrop-blur-md border border-white/10 rounded-xl p-md flex flex-col gap-base">
                                  <div class="flex items-center justify-between">
                                      <div class="text-on-surface font-semibold">{leptos_fluent::move_tr!("streamer-mock-payment-required")}</div>
                                      <div class="text-label-sm text-on-surface-variant">
                                          {leptos_fluent::move_tr!("streamer-status-label")} " " <span class="text-on-surface">{status.to_string()}</span>
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

                                  {move || if status == TransactionStatus::ReadyForDisplay {
                                      view! {
                                          <div data-testid="payment-success-msg" class="bg-secondary/10 border border-secondary/20 text-secondary rounded-xl px-md py-sm">
                                              {leptos_fluent::move_tr!("streamer-payment-success")}
                                          </div>
                                      }.into_any()
                                  } else if status == TransactionStatus::Rejected {
                                      view! {
                                          <div class="bg-error/10 border border-error/20 text-error rounded-xl px-md py-sm">
                                              {leptos_fluent::move_tr!("streamer-payment-failed")}
                                          </div>
                                      }.into_any()
                                  } else {
                                      if payment_method.get() == PaymentMethod::MockManual {
                                          view! {
                                              <div class="flex gap-sm mt-sm">
                                                  <button
                                                      class="flex-1 bg-secondary text-on-secondary-container py-sm rounded-lg font-bold hover:brightness-110"
                                                      on:click=move |_| {
                                                          if let Some(id) = mock_tx_id.get() {
                                                              accept_action.dispatch(id);
                                                              mock_tx_status.set(TransactionStatus::ReadyForDisplay);
                                                              transactions_trigger.update(|n| *n += 1);
                                                          }
                                                      }
                                                  >{leptos_fluent::move_tr!("streamer-btn-accept")}</button>
                                                  <button
                                                      class="flex-1 bg-error text-on-error py-sm rounded-lg font-bold hover:brightness-110"
                                                      on:click=move |_| {
                                                          if let Some(id) = mock_tx_id.get() {
                                                              reject_action.dispatch(id);
                                                              mock_tx_status.set(TransactionStatus::Rejected);
                                                          }
                                                      }
                                                  >{leptos_fluent::move_tr!("streamer-btn-reject")}</button>
                                              </div>
                                          }.into_any()
                                      } else {
                                          view! {
                                              <div class="text-on-surface-variant text-body-sm">
                                                  {leptos_fluent::move_tr!("streamer-waiting-payment")}
                                              </div>
                                          }.into_any()
                                      }
                                  }}
                              </div>
                          }.into_any()
                      } else {
                          view! {}.into_any()
                      }}
                      <p class="text-center text-label-sm font-label-sm text-on-surface-variant">{leptos_fluent::move_tr!("streamer-glint-matches")}</p>
                  </section>

                  // Recent Tributes Section
                  <section data-testid="recent-tributes-section" class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl p-md md:p-lg flex flex-col gap-md md:col-span-2 md:sticky md:top-28 shadow-[0_20px_60px_rgba(0,0,0,0.25)] ring-1 ring-white/5">
                  <div class="flex items-center gap-sm">
                      <span class="material-symbols-outlined text-primary" data-icon="history">"history"</span>
                      <h2 class="text-headline-sm md:text-headline-md font-headline-md text-on-surface">{leptos_fluent::move_tr!("streamer-recent-tributes")}</h2>
                  </div>
                  
                  <Suspense fallback=move || view! { <div class="text-on-surface-variant">{leptos_fluent::move_tr!("streamer-loading-tributes")}</div> }>
                      {move || {
                          transactions_resource.get().map(|res| {
                              match res {
                                  Ok(txs) => {
                                      if txs.is_empty() {
                                          view! {
                                              <p class="text-on-surface-variant text-center py-md">{leptos_fluent::move_tr!("streamer-no-tributes")}</p>
                                          }.into_any()
                                      } else {
                                          view! {
                                              <div class="flex flex-col gap-sm">
                                                  {txs.into_iter().map(|tx| {
                                                      let msg = tx.message.clone();
                                                      view! {
                                                          <div class="bg-white/5 border border-white/10 rounded-xl p-sm flex flex-col gap-xs transition-all hover:bg-white/10 hover:border-white/20">
                                                              <div class="flex items-start gap-sm">
                                                                  <div class="shrink-0 w-9 h-9 rounded-full bg-gradient-to-br from-primary/25 to-secondary/15 border border-white/10 flex items-center justify-center">
                                                                      <span class="material-symbols-outlined text-primary text-[18px]">"person"</span>
                                                                  </div>
                                                                  <div class="flex-1 min-w-0 flex flex-col gap-[2px]">
                                                                      <div class="flex items-center justify-between gap-sm">
                                                                          <span class="text-on-surface font-semibold text-body-md truncate">{tx.donor_name.clone()}</span>
                                                                          <span class="text-secondary font-bold text-body-lg shrink-0">"$" {format!("{:.2}", tx.amount)}</span>
                                                                      </div>
                                                                      <div class="flex items-center justify-between gap-sm text-label-sm text-on-surface-variant/80">
                                                                          <span class="truncate">{leptos_fluent::move_tr!("streamer-via")} " " {tx.payment_method.to_string()}</span>
                                                                          <span class="shrink-0">{tx.created_at.clone()}</span>
                                                                      </div>
                                                                  </div>
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
                                                          </div>
                                                      }
                                                  }).collect_view()}
                                              </div>
                                          }.into_any()
                                      }
                                  }
                                  Err(_) => view! { <div class="text-on-surface-variant text-center">{leptos_fluent::move_tr!("streamer-tributes-failed")}</div> }.into_any()
                              }
                          })
                      }}
                  </Suspense>
              </section>
              </div>
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
        "SELECT id, username, display_name, avatar_url, bio, is_live, user_id, overlay_token, active_overlay_session, payment_methods, overlay_paused, overlay_sound_enabled FROM streamers WHERE username = $1"
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
                payment_methods: r.try_get("payment_methods").unwrap_or_else(|_| vec![PaymentMethod::MockAuto, PaymentMethod::MockManual]),
                overlay_paused: r.try_get("overlay_paused").unwrap_or(false),
                overlay_sound_enabled: r.try_get("overlay_sound_enabled").unwrap_or(true),
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
        "SELECT id, username, display_name, avatar_url, bio, is_live, user_id, overlay_token, active_overlay_session, payment_methods, overlay_paused, overlay_sound_enabled FROM streamers WHERE user_id = $1"
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
         VALUES ($1, $2, $3, $4, $5, $6, gen_random_uuid(), $7)
         RETURNING id, username, display_name, avatar_url, bio, is_live, user_id, overlay_token, active_overlay_session, payment_methods"
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

    let stats = sqlx::query(
        "SELECT COALESCE(SUM(amount), 0) as total_revenue, COUNT(*) as donation_count 
         FROM transactions 
         WHERE streamer_id = $1"
    )
    .bind(streamer_id)
    .fetch_one(&pool)
    .await;
    
    use sqlx::Row;
    let (total_revenue, donation_count) = match stats {
        Ok(r) => (r.try_get::<f64, _>("total_revenue").unwrap_or(0.0), r.try_get::<i64, _>("donation_count").unwrap_or(0)),
        Err(_) => (0.0, 0),
    };

    let top_donors = sqlx::query(
        "SELECT donor_name, SUM(amount) as total_donated 
         FROM transactions 
         WHERE streamer_id = $1
         GROUP BY donor_name 
         ORDER BY total_donated DESC 
         LIMIT 5"
    )
    .bind(streamer_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let donors = top_donors.into_iter().map(|r| {
        (r.get::<String, _>("donor_name"), r.get::<f64, _>("total_donated"))
    }).collect();

    let revenue_time = if time_range == "day" {
        sqlx::query(
            "SELECT TO_CHAR(created_at, 'HH24:00') as date, SUM(amount) as daily_revenue
             FROM transactions 
             WHERE streamer_id = $1 AND created_at >= NOW() - INTERVAL '24 hours'
             GROUP BY TO_CHAR(created_at, 'HH24:00')
             ORDER BY date ASC"
        )
        .bind(streamer_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default()
    } else if time_range == "month" {
        sqlx::query(
            "SELECT TO_CHAR(created_at, 'MM-DD') as date, SUM(amount) as daily_revenue
             FROM transactions 
             WHERE streamer_id = $1 AND created_at >= NOW() - INTERVAL '30 days'
             GROUP BY TO_CHAR(created_at, 'MM-DD')
             ORDER BY date ASC"
        )
        .bind(streamer_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default()
    } else { // "week"
        sqlx::query(
            "SELECT TO_CHAR(created_at, 'MM-DD') as date, SUM(amount) as daily_revenue
             FROM transactions 
             WHERE streamer_id = $1 AND created_at >= NOW() - INTERVAL '7 days'
             GROUP BY TO_CHAR(created_at, 'MM-DD')
             ORDER BY date ASC"
        )
        .bind(streamer_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default()
    };

    let revenue_over_time: Vec<(String, f64)> = revenue_time.into_iter().map(|r| {
        (r.get::<String, _>("date"), r.get::<f64, _>("daily_revenue"))
    }).collect();

    Ok(StreamerAnalytics {
        total_revenue,
        donation_count,
        top_donors: donors,
        revenue_over_time,
    })
}

#[server(GetAllStreamers, "/api")]
pub async fn get_all_streamers() -> Result<Vec<DbStreamer>, ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let rows = sqlx::query(
        "SELECT id, username, display_name, avatar_url, bio, is_live, user_id, overlay_token, active_overlay_session, payment_methods, overlay_paused, overlay_sound_enabled FROM streamers ORDER BY is_live DESC, id DESC"
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
    .bind(TransactionStatus::Initialize)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database insert failed: {}", e)))?;

    use sqlx::Row;
    Ok(row.get::<i32, _>("id"))
}

#[cfg(feature = "ssr")]
mod mock_payments {
    use crate::db::TransactionStatus;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use tokio::sync::RwLock;

    #[derive(Clone, Debug)]
    pub struct MockTxState { pub status: TransactionStatus, }

    pub static MOCK_TXS: Lazy<RwLock<HashMap<i32, MockTxState>>> =
        Lazy::new(|| RwLock::new(HashMap::new()));
}

#[server(CreateMockPayment, "/api")]
pub async fn create_mock_payment(
    streamer_id: i32,
    donor_name: String,
    amount: f64,
    message: String,
    payment_method: PaymentMethod,
) -> Result<MockPaymentInit, ServerFnError> {
    let tx_id = create_donation(
        streamer_id,
        donor_name,
        amount,
        message,
        payment_method.to_string(),
    )
    .await?;

    let (display_qr, display_url) = match payment_method {
        PaymentMethod::MockAuto => (None, Some(format!("https://example.com/mock-auto?tx_id={tx_id}"))),
        PaymentMethod::MockManual => (None, Some(format!("https://example.com/mock-manual?tx_id={tx_id}"))),
        PaymentMethod::Crypto => (Some(format!("MOCK-QR:tx_id={tx_id};amount={amount:.2}")), None),
        PaymentMethod::PayPal => (None, Some(format!("https://example.com/mock-pay?provider=paypal&tx_id={tx_id}"))),
        _ => (None, Some(format!("https://example.com/mock-pay?provider=card&tx_id={tx_id}"))),
    };

    #[cfg(feature = "ssr")]
    {
        use tokio::time::{sleep, Duration};

        mock_payments::MOCK_TXS
            .write()
            .await
            .insert(tx_id, mock_payments::MockTxState { status: TransactionStatus::Initialize });

        let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
            .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

        let pm_clone = payment_method.clone();
        let delay_ms = 2000u64 + (chrono::Utc::now().timestamp_subsec_millis() as u64 % 3001u64);
        tokio::spawn(async move {
            if pm_clone != PaymentMethod::MockManual {
                sleep(Duration::from_millis(delay_ms)).await;
                if let Some(state) = mock_payments::MOCK_TXS.write().await.get_mut(&tx_id) {
                    state.status = TransactionStatus::ReadyForDisplay;
                }
                let _ = sqlx::query("UPDATE transactions SET status = 'READY_FOR_DISPLAY' WHERE id = $1")
                    .bind(tx_id)
                    .execute(&pool)
                    .await;
            }
        });
    }

    Ok(MockPaymentInit {
        tx_id,
        status: TransactionStatus::Initialize,
        display_qr,
        display_url,
    })
}

#[server(AcceptMockPayment, "/api")]
pub async fn accept_mock_payment(tx_id: i32) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        if let Some(state) = mock_payments::MOCK_TXS.write().await.get_mut(&tx_id) {
            state.status = TransactionStatus::ReadyForDisplay;
        }

        let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
            .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;
            
        let _ = sqlx::query("UPDATE transactions SET status = 'READY_FOR_DISPLAY' WHERE id = $1")
            .bind(tx_id)
            .execute(&pool)
            .await;
    }
    Ok(())
}

#[server(RejectMockPayment, "/api")]
pub async fn reject_mock_payment(tx_id: i32) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        if let Some(state) = mock_payments::MOCK_TXS.write().await.get_mut(&tx_id) {
            state.status = TransactionStatus::Rejected;
        }

        let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
            .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;
            
        let _ = sqlx::query("UPDATE transactions SET status = 'REJECTED' WHERE id = $1")
            .bind(tx_id)
            .execute(&pool)
            .await;
    }
    Ok(())
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
            .unwrap_or_else(|| TransactionStatus::Initialize);

        return Ok(MockPaymentStatus { tx_id, status });
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = tx_id;
        Ok(MockPaymentStatus {
            tx_id,
            status: TransactionStatus::Initialize,
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

#[server(PrefetchUpcomingTransactions, "/api")]
pub async fn prefetch_upcoming_transactions(token: String, session_id: String) -> Result<(Vec<DbTransaction>, bool, bool), ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let streamer = sqlx::query(
        "SELECT id, active_overlay_session, overlay_paused, overlay_sound_enabled FROM streamers WHERE overlay_token = $1"
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

    let _ = sqlx::query("UPDATE streamers SET last_overlay_ping = CURRENT_TIMESTAMP WHERE id = $1")
        .bind(streamer_id)
        .execute(&pool)
        .await;

    if overlay_paused {
        return Ok((vec![], overlay_paused, overlay_sound_enabled));
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

    Ok((txs, overlay_paused ,overlay_sound_enabled))
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

    sqlx::query(
        "INSERT INTO transactions (streamer_id, donor_name, amount, message, payment_method, status)
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(streamer_id)
    .bind("System Test")
    .bind(5.0)
    .bind("This is a test donation for your overlay!")
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
