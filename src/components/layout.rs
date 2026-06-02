use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn Header() -> impl IntoView {
    let user_resource = use_context::<Resource<Result<Option<crate::auth::User>, ServerFnError>>>()
        .expect("User resource must be provided");

    let location = use_location();
    let i18n = expect_context::<leptos_fluent::I18n>();
    let current_lang = move || i18n.language.get().id.to_string();

    let make_href = move |base_path: &'static str| {
        if base_path == "/" {
            format!("/{}", current_lang())
        } else {
            format!("/{}/{}", current_lang(), base_path.trim_start_matches('/'))
        }
    };

    let nav_class = move |base_path: &'static str| {
        let path = location.pathname.get();
        let target = make_href(base_path);
        
        let active = if base_path == "/" {
            path == target || path == format!("{}/", target)
        } else {
            path.starts_with(&target)
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
                        href=move || make_href("/")
                        class="text-headline-md font-headline-md font-extrabold text-primary tracking-tighter"
                    >
                        "Glint"
                    </a>
                    <nav class="hidden md:flex items-center gap-md ml-lg">
                        <a class=move || nav_class("/") href=move || make_href("/")>
                            {leptos_fluent::move_tr!("header-explore")}
                        </a>
                        <a class=move || nav_class("/explore") href=move || make_href("/explore")>
                            {leptos_fluent::move_tr!("header-creators")}
                        </a>
                        <a class=move || nav_class("/leaderboard") href=move || make_href("/leaderboard")>
                            {leptos_fluent::move_tr!("header-leaderboard")}
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
                                                href=move || make_href("/dashboard")
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
                            <a class=move || nav_class("/") href=move || make_href("/")>
                                {leptos_fluent::move_tr!("header-explore")}
                            </a>
                            <a class=move || nav_class("/explore") href=move || make_href("/explore")>
                                {leptos_fluent::move_tr!("header-creators")}
                            </a>
                            <a class=move || nav_class("/leaderboard") href=move || make_href("/leaderboard")>
                                {leptos_fluent::move_tr!("header-leaderboard")}
                            </a>
                            <Suspense fallback=move || {
                                view! {}
                            }>
                                {move || match user_resource.get() {
                                    Some(Ok(Some(_))) => {
                                        view! {
                                            <a class=move || nav_class("/dashboard") href=move || make_href("/dashboard")>
                                                {leptos_fluent::move_tr!("header-dashboard")}
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
                    if let Some(_lang) = i18n.languages.iter().find(|l| l.id.to_string() == "en") {
                        let path = leptos_router::hooks::use_location().pathname.get();
                        let current_lang_prefix = format!("/{}", i18n.language.get().id.to_string());
                        let new_path = if path.starts_with(&current_lang_prefix) {
                            path.replacen(&current_lang_prefix, "/en", 1)
                        } else {
                            format!("/en{}", path)
                        };
                        let _ = leptos::prelude::window().location().set_href(&new_path);
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
                    if let Some(_lang) = i18n.languages.iter().find(|l| l.id.to_string() == "vi") {
                        let path = leptos_router::hooks::use_location().pathname.get();
                        let current_lang_prefix = format!("/{}", i18n.language.get().id.to_string());
                        let new_path = if path.starts_with(&current_lang_prefix) {
                            path.replacen(&current_lang_prefix, "/vi", 1)
                        } else {
                            format!("/vi{}", path)
                        };
                        let _ = leptos::prelude::window().location().set_href(&new_path);
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
    let i18n = expect_context::<leptos_fluent::I18n>();
    let current_lang = move || i18n.language.get().id.to_string();

    let make_href = move |base_path: &'static str| {
        if base_path == "/" {
            format!("/{}", current_lang())
        } else {
            format!("/{}/{}", current_lang(), base_path.trim_start_matches('/'))
        }
    };

    view! {
        <footer class="bg-surface-dim border-t border-surface-container-highest w-full py-lg mt-auto">
            <div class="max-w-7xl mx-auto px-margin-desktop flex flex-col md:flex-row justify-between items-center gap-md">
                <div class="flex flex-col items-center md:items-start gap-xs">
                    <a
                        href=move || make_href("/")
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
                        href=move || make_href("/about")
                    >
                        {leptos_fluent::move_tr!("footer-about")}
                    </a>
                    <a
                        class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors"
                        href=move || make_href("/faq")
                    >
                        {leptos_fluent::move_tr!("footer-faq")}
                    </a>
                    <a
                        class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors"
                        href=move || make_href("/privacy")
                    >
                        {leptos_fluent::move_tr!("footer-privacy")}
                    </a>
                    <a
                        class="text-on-surface-variant text-label-sm font-label-sm hover:text-secondary transition-colors"
                        href=move || make_href("/terms")
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
