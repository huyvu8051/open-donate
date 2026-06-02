use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn Header() -> impl IntoView {
    let user_resource = use_context::<Resource<Result<Option<crate::auth::User>, ServerFnError>>>()
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
                    <a
                        href="/"
                        class="text-headline-md font-headline-md font-extrabold text-primary tracking-tighter"
                    >
                        "Glint"
                    </a>
                    <nav class="hidden md:flex items-center gap-md ml-lg">
                        <a class=move || nav_class("/") href="/">
                            "Explore"
                        </a>
                        <a class=move || nav_class("/explore") href="/explore">
                            "Creators"
                        </a>
                        <a class=move || nav_class("/leaderboard") href="/leaderboard">
                            "Leaderboard"
                        </a>
                    </nav>
                </div>
                <div class="flex items-center gap-md">
                    <Suspense fallback=move || {
                        view! { <span class="text-on-surface-variant">"Loading..."</span> }
                    }>
                        {move || {
                            match user_resource.get() {
                                Some(Ok(Some(user))) => {
                                    view! {
                                        <div class="flex items-center gap-sm">
                                            <a
                                                href="/dashboard"
                                                class="hidden sm:inline text-on-surface text-label-md font-semibold hover:text-primary transition-colors"
                                            >
                                                "Hello, "
                                                {user.name.clone()}
                                            </a>
                                            <ActionForm action=logout_action>
                                                <button
                                                    data-testid="logout-button"
                                                    type="submit"
                                                    class=move || {
                                                        format!(
                                                            "px-sm py-xs bg-surface-container-highest border border-white/10 text-on-surface rounded-lg text-label-sm font-label-sm transition-all flex items-center gap-xs {}",
                                                            if logout_action.pending().get() {
                                                                "opacity-50 cursor-wait"
                                                            } else {
                                                                "hover:bg-surface-container-highest/80"
                                                            },
                                                        )
                                                    }
                                                    disabled=move || logout_action.pending().get()
                                                >
                                                    {move || {
                                                        if logout_action.pending().get() {
                                                            view! {
                                                                <span class="material-symbols-outlined text-[14px] animate-spin">
                                                                    "progress_activity"
                                                                </span>
                                                            }
                                                                .into_any()
                                                        } else {
                                                            view! {}.into_any()
                                                        }
                                                    }}
                                                    {leptos_fluent::move_tr!("header-logout")}
                                                </button>
                                            </ActionForm>
                                            <LanguageSwitcher />
                                        </div>
                                    }
                                        .into_any()
                                }
                                _ => {
                                    view! {
                                        <div class="flex items-center gap-sm">
                                            <LanguageSwitcher />
                                        </div>
                                    }
                                        .into_any()
                                }
                            }
                        }}
                    </Suspense>
                    <button
                        class="md:hidden text-on-surface flex items-center justify-center"
                        on:click=move |_| set_is_open.update(|o| *o = !*o)
                    >
                        <span class="material-symbols-outlined">
                            {move || if is_open.get() { "close" } else { "menu" }}
                        </span>
                    </button>
                </div>
            </div>
            {move || {
                if is_open.get() {
                    view! {
                        <nav class="md:hidden flex flex-col gap-sm pb-md border-t border-white/10 pt-md">
                            <a class=move || nav_class("/") href="/">
                                "Explore"
                            </a>
                            <a class=move || nav_class("/explore") href="/explore">
                                "Creators"
                            </a>
                            <a class=move || nav_class("/leaderboard") href="/leaderboard">
                                "Leaderboard"
                            </a>
                            <Suspense fallback=move || {
                                view! {}
                            }>
                                {move || match user_resource.get() {
                                    Some(Ok(Some(_))) => {
                                        view! {
                                            <a class=move || nav_class("/dashboard") href="/dashboard">
                                                "Dashboard"
                                            </a>
                                        }
                                            .into_any()
                                    }
                                    _ => view! {}.into_any(),
                                }}
                            </Suspense>
                        </nav>
                    }
                        .into_any()
                } else {
                    view! {}.into_any()
                }
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
                class=move || {
                    format!(
                        "px-2 py-0.5 rounded text-[10px] font-bold transition-all {}",
                        if i18n.language.get().id.to_string() == "en" {
                            "bg-primary text-on-primary shadow-sm"
                        } else {
                            "text-on-surface-variant hover:text-on-surface"
                        },
                    )
                }
                on:click=move |_| {
                    if let Some(lang) = i18n.languages.iter().find(|l| l.id.to_string() == "en") {
                        i18n.language.set(lang);
                    }
                }
            >
                "EN"
            </button>
            <button
                class=move || {
                    format!(
                        "px-2 py-0.5 rounded text-[10px] font-bold transition-all {}",
                        if i18n.language.get().id.to_string() == "vi" {
                            "bg-primary text-on-primary shadow-sm"
                        } else {
                            "text-on-surface-variant hover:text-on-surface"
                        },
                    )
                }
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
                    <a
                        href="/"
                        class="text-headline-md font-headline-md font-bold text-primary hover:text-primary/80 transition-colors"
                    >
                        "Glint"
                    </a>
                    <p class="text-on-surface-variant text-body-md font-body-md">
                        {leptos_fluent::move_tr!("footer-copyright")}
                    </p>
                </div>
                <nav class="flex flex-wrap justify-center gap-md">
                    <a
                        class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors"
                        href="/about"
                    >
                        {leptos_fluent::move_tr!("footer-about")}
                    </a>
                    <a
                        class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors"
                        href="/faq"
                    >
                        {leptos_fluent::move_tr!("footer-faq")}
                    </a>
                    <a
                        class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors"
                        href="/privacy"
                    >
                        {leptos_fluent::move_tr!("footer-privacy")}
                    </a>
                    <a
                        class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors"
                        href="/terms"
                    >
                        {leptos_fluent::move_tr!("footer-terms")}
                    </a>
                    <a
                        class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors"
                        href="#"
                    >
                        {leptos_fluent::move_tr!("footer-discord")}
                    </a>
                </nav>
            </div>
        </footer>
    }
}
