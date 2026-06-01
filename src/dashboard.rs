use crate::db::{TransactionStatus, PaymentMethod};
use leptos::prelude::*;
use crate::app::{get_or_create_streamer, get_dashboard_transactions, Header, Footer, UpdateStreamerProfile};
use leptos_router::components::{Outlet};
use leptos::form::ActionForm;

#[component]
pub fn DashboardLayout() -> impl IntoView {
    let onboard_resource = LocalResource::new(move || {
        async move {
            get_or_create_streamer().await
        }
    });

    let location = leptos_router::hooks::use_location();
    let path = move || location.pathname.get();
    
    let is_active = move |p: &str| path() == p || (p == "/dashboard" && path() == "/dashboard/");
    
    let nav_class = move |p: &'static str| {
        move || {
            if is_active(p) {
                "flex items-center gap-sm p-sm rounded-xl transition-all bg-surface-variant/50 text-primary font-bold"
            } else {
                "flex items-center gap-sm p-sm rounded-xl transition-all text-on-surface-variant hover:bg-surface-variant/50"
            }
        }
    };

    let mob_nav_class = move |p: &'static str| {
        move || {
            if is_active(p) {
                "flex flex-col items-center gap-1 text-primary w-16 font-bold"
            } else {
                "flex flex-col items-center gap-1 text-on-surface-variant hover:text-primary transition-colors w-16 font-medium"
            }
        }
    };

    view! {
        <Transition fallback=move || {
            view! {
                <div class="p-8 text-center">{leptos_fluent::move_tr!("dashboard-loading")}</div>
            }
        }>
            {move || {
                onboard_resource
                    .get()
                    .map(|res| {
                        match res {
                            Ok(Some(streamer)) => {
                                provide_context(streamer.clone());

                                view! {
                                    <div class="bg-background text-on-surface font-body-md antialiased overflow-x-hidden">
                                        <Header />

                                        // SideNavBar
                                        <aside class="fixed left-0 top-20 h-[calc(100vh-80px)] w-64 bg-surface-container-low/40 backdrop-blur-md border-r border-white/10 hidden md:flex flex-col p-md gap-base z-40">
                                            <div class="mb-lg">
                                                <h2 class="text-headline-sm font-headline-sm text-primary">
                                                    {leptos_fluent::move_tr!("nav-creator-hub")}
                                                </h2>
                                                <p class="text-label-sm font-label-sm text-on-surface-variant">
                                                    {leptos_fluent::move_tr!("nav-manage-account")}
                                                </p>
                                            </div>
                                            <nav class="flex-1 flex flex-col gap-xs">
                                                <a class=nav_class("/dashboard") href="/dashboard">
                                                    <span class="material-symbols-outlined">"dashboard"</span>
                                                    <span class="text-label-md font-label-md">
                                                        {leptos_fluent::move_tr!("nav-dashboard")}
                                                    </span>
                                                </a>
                                                <a
                                                    class=nav_class("/dashboard/analytics")
                                                    href="/dashboard/analytics"
                                                >
                                                    <span class="material-symbols-outlined">"monitoring"</span>
                                                    <span class="text-label-md font-label-md">
                                                        {leptos_fluent::move_tr!("nav-analytics")}
                                                    </span>
                                                </a>
                                                <a
                                                    class=nav_class("/dashboard/payments")
                                                    href="/dashboard/payments"
                                                >
                                                    <span class="material-symbols-outlined">"payments"</span>
                                                    <span class="text-label-md font-label-md">
                                                        {leptos_fluent::move_tr!("nav-payments")}
                                                    </span>
                                                </a>
                                                <a
                                                    class=nav_class("/dashboard/settings")
                                                    href="/dashboard/settings"
                                                >
                                                    <span class="material-symbols-outlined">"settings"</span>
                                                    <span class="text-label-md font-label-md">
                                                        {leptos_fluent::move_tr!("nav-settings")}
                                                    </span>
                                                </a>
                                            </nav>
                                            <button class="bg-gradient-to-r from-primary-container to-primary text-on-primary-container py-sm rounded-xl font-bold flex items-center justify-center gap-xs active:scale-95 transition-all shadow-lg shadow-primary/20">
                                                <span class="material-symbols-outlined">"videocam"</span>
                                                <span>{leptos_fluent::move_tr!("nav-go-live")}</span>
                                            </button>
                                            <div class="pt-md border-t border-white/10 flex flex-col gap-xs">
                                                <a
                                                    class="flex items-center gap-sm p-sm text-on-surface-variant hover:bg-surface-variant/30 rounded-xl transition-all"
                                                    href="#"
                                                >
                                                    <span class="material-symbols-outlined">"help"</span>
                                                    <span class="text-label-md font-label-md">
                                                        {leptos_fluent::move_tr!("nav-help")}
                                                    </span>
                                                </a>
                                                <a
                                                    class="flex items-center gap-sm p-sm text-on-surface-variant hover:bg-surface-variant/30 rounded-xl transition-all"
                                                    href="/api/auth/logout"
                                                >
                                                    <span class="material-symbols-outlined">"logout"</span>
                                                    <span class="text-label-md font-label-md">
                                                        {leptos_fluent::move_tr!("nav-logout")}
                                                    </span>
                                                </a>
                                            </div>
                                        </aside>

                                        // Main Content & Footer inside the md:ml-64 container to prevent overlapping
                                        <div class="md:ml-64 flex flex-col min-h-screen pt-20">
                                            <main class="pt-12 px-margin-mobile md:px-margin-desktop pb-24 md:pb-xl flex-grow">
                                                <div class="max-w-7xl mx-auto">
                                                    <S3StatusBanner />
                                                    <Outlet />
                                                </div>
                                            </main>
                                            <Footer />
                                        </div>

                                        // Mobile Bottom Navigation
                                        <nav class="fixed bottom-0 w-full bg-surface-container-low/90 backdrop-blur-xl border-t border-white/10 flex justify-around items-center h-16 z-50 md:hidden pb-safe">
                                            <a href="/dashboard" class=mob_nav_class("/dashboard")>
                                                <span class="material-symbols-outlined text-[24px]">
                                                    "dashboard"
                                                </span>
                                                <span class="text-[10px] leading-none">
                                                    {leptos_fluent::move_tr!("nav-dashboard")}
                                                </span>
                                            </a>
                                            <a
                                                href="/dashboard/analytics"
                                                class=mob_nav_class("/dashboard/analytics")
                                            >
                                                <span class="material-symbols-outlined text-[24px]">
                                                    "monitoring"
                                                </span>
                                                <span class="text-[10px] leading-none">
                                                    {leptos_fluent::move_tr!("nav-analytics")}
                                                </span>
                                            </a>
                                            <button class="relative -top-5 w-12 h-12 bg-gradient-to-tr from-primary to-primary-container rounded-full shadow-lg shadow-primary/30 flex items-center justify-center text-on-primary active:scale-95 transition-transform border-4 border-background">
                                                <span class="material-symbols-outlined text-[24px]">
                                                    "videocam"
                                                </span>
                                            </button>
                                            <a
                                                href="/dashboard/payments"
                                                class=mob_nav_class("/dashboard/payments")
                                            >
                                                <span class="material-symbols-outlined text-[24px]">
                                                    "payments"
                                                </span>
                                                <span class="text-[10px] leading-none">
                                                    {leptos_fluent::move_tr!("nav-payments")}
                                                </span>
                                            </a>
                                            <a
                                                href="/dashboard/settings"
                                                class=mob_nav_class("/dashboard/settings")
                                            >
                                                <span class="material-symbols-outlined text-[24px]">
                                                    "settings"
                                                </span>
                                                <span class="text-[10px] leading-none">
                                                    {leptos_fluent::move_tr!("nav-settings")}
                                                </span>
                                            </a>
                                        </nav>

                                        // Interactive Layer: Atmospheric Glow
                                        <div class="fixed top-0 left-0 w-full h-full -z-10 pointer-events-none">
                                            <div class="js-glow absolute top-[10%] left-[20%] w-[500px] h-[500px] bg-primary/10 rounded-full blur-[120px] animate-pulse"></div>
                                            <div class="js-glow absolute bottom-[20%] right-[10%] w-[400px] h-[400px] bg-secondary/5 rounded-full blur-[100px]"></div>
                                        </div>

                                        <script>
                                            "document.querySelectorAll('.glass-card').forEach(card => {
                                                card.addEventListener('mouseenter', () => {
                                                    card.style.transform = 'translateY(-2px)';
                                                    card.style.transition = 'transform 0.2s ease-out';
                                                });
                                                card.addEventListener('mouseleave', () => {
                                                    card.style.transform = 'translateY(0px)';
                                                });
                                            });"
                                        </script>
                                    </div>
                                }
                                    .into_any()
                            }
                            Ok(None) => {
                                view! {
                                    <div class="p-8 text-center flex flex-col items-center justify-center min-h-[50vh] gap-md">
                                        <p class="text-on-surface-variant text-body-lg">
                                            {leptos_fluent::move_tr!("dashboard-must-login")}
                                        </p>
                                        <a
                                            href="/login"
                                            class="bg-primary text-on-primary px-xl py-md rounded-xl font-bold hover:brightness-110 active:scale-95 transition-all shadow-lg shadow-primary/20"
                                        >
                                            {leptos_fluent::move_tr!("dashboard-login-btn")}
                                        </a>
                                    </div>
                                }
                                    .into_any()
                            }
                            Err(e) => {
                                view! {
                                    <div class="p-8 text-center text-red-500">
                                        {format!("Error during onboarding: {:?}", e)}
                                    </div>
                                }
                                    .into_any()
                            }
                        }
                    })
            }}
        </Transition>
    }
}

#[component]
pub fn DashboardHome() -> impl IntoView {
    let streamer = use_context::<crate::db::DbStreamer>().expect("Streamer context missing");
    
    // Copy overlay link logic
    #[allow(unused_variables)]
    let token = streamer.overlay_token.clone();
    let copy_to_clipboard = move |_| {
        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(origin) = window.location().origin() {
                    let full_url = format!("{}/overlay/{}", origin, token);
                    let clipboard = window.navigator().clipboard();
                    if let Some(clipboard) = Some(clipboard) { // Using Some(clipboard) to avoid changing indentation or logic structure unnecessarily, but effectively unwrapping it
                        let _ = clipboard.write_text(&full_url);
                        if let Some(_document) = window.document() {
                            let _ = window.alert_with_message("Overlay link copied to clipboard! Paste it as a Browser Source in OBS.");
                        }
                    }
                }
            }
        }
    };

    // Overlay Status polling
    let tick = RwSignal::new(0);
    #[cfg(feature = "hydrate")]
    {
        use gloo_timers::callback::Interval;
        let poll_interval: StoredValue<Option<Interval>, leptos::prelude::LocalStorage> = StoredValue::new_local(None);
        let interval = Interval::new(5000, move || {
            tick.update(|t| *t += 1);
        });
        poll_interval.set_value(Some(interval));
    }
    
    let status_resource = Resource::new(move || tick.get(), |_| async move {
        crate::app::get_overlay_status().await.unwrap_or(false)
    });

    let test_action = ServerAction::<crate::app::TestOverlayDonation>::new();
    let pause_action = ServerAction::<crate::app::ToggleOverlayPause>::new();
    let sound_action = ServerAction::<crate::app::ToggleOverlaySound>::new();
    let (is_paused, set_is_paused) = signal(streamer.overlay_paused);
    let (is_sound, set_is_sound) = signal(streamer.overlay_sound_enabled);

    view! {
        <div class="flex flex-col md:flex-row md:items-end justify-between gap-md mb-lg">
            <div>
                <h1
                    data-testid="streamer-dashboard-header"
                    class="text-headline-lg font-headline-lg text-on-surface"
                >
                    {leptos_fluent::move_tr!("dashboard-streamer-dashboard")}
                </h1>
                <p class="text-body-md font-body-md text-on-surface-variant">
                    {leptos_fluent::move_tr!("dashboard-welcome-back")}
                    {streamer.display_name.clone()}
                    {leptos_fluent::move_tr!("dashboard-creator-hub-ready")}
                </p>
            </div>
            <div class="flex flex-col md:items-end gap-2">
                <div class="flex items-center gap-2">
                    <Suspense fallback=move || {
                        view! {
                            <span class="text-on-surface-variant text-label-sm">
                                "Checking status..."
                            </span>
                        }
                    }>
                        {move || {
                            let is_active = status_resource.get().unwrap_or(false);
                            if is_active {
                                view! {
                                    <div
                                        data-testid="overlay-status-active"
                                        class="flex items-center gap-1.5 px-2.5 py-1 bg-green-500/20 text-green-400 rounded-full text-label-sm font-semibold border border-green-500/30"
                                    >
                                        <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
                                        "Overlay Active"
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <div
                                        data-testid="overlay-status-inactive"
                                        class="flex items-center gap-1.5 px-2.5 py-1 bg-red-500/20 text-red-400 rounded-full text-label-sm font-semibold border border-red-500/30"
                                    >
                                        <div class="w-2 h-2 rounded-full bg-red-500"></div>
                                        "Overlay Inactive"
                                    </div>
                                }
                                    .into_any()
                            }
                        }}
                    </Suspense>
                </div>

                <div class="flex flex-wrap items-center gap-3 mb-3">
                    <button
                        type="button"
                        data-testid="toggle-pause-btn"
                        on:click=move |_| {
                            let new_state = !is_paused.get();
                            set_is_paused.set(new_state);
                            pause_action
                                .dispatch(crate::app::ToggleOverlayPause {
                                    paused: new_state,
                                });
                        }
                        class=move || {
                            format!(
                                "group relative flex items-center gap-2 px-5 py-2.5 rounded-full font-bold transition-all duration-300 border overflow-hidden {}",
                                if is_paused.get() {
                                    "bg-orange-500/15 text-orange-400 border-orange-500/40 hover:bg-orange-500/25 shadow-[0_0_15px_rgba(249,115,22,0.15)]"
                                } else {
                                    "bg-surface-container-highest text-on-surface hover:bg-surface-variant border-white/10 shadow-sm"
                                },
                            )
                        }
                    >
                        <span class="material-symbols-outlined text-[20px] transition-transform group-active:scale-90">
                            {move || if is_paused.get() { "pause_circle" } else { "play_circle" }}
                        </span>
                        <span class="text-sm tracking-wide">
                            {move || {
                                if is_paused.get() { "Overlay Paused" } else { "Pause Overlay" }
                            }}
                        </span>
                    </button>

                    <button
                        type="button"
                        data-testid="toggle-sound-btn"
                        on:click=move |_| {
                            let new_state = !is_sound.get();
                            set_is_sound.set(new_state);
                            sound_action
                                .dispatch(crate::app::ToggleOverlaySound {
                                    enabled: new_state,
                                });
                        }
                        class=move || {
                            format!(
                                "group relative flex items-center gap-2 px-5 py-2.5 rounded-full font-bold transition-all duration-300 border overflow-hidden {}",
                                if is_sound.get() {
                                    "bg-blue-500/15 text-blue-400 border-blue-500/40 hover:bg-blue-500/25 shadow-[0_0_15px_rgba(59,130,246,0.15)]"
                                } else {
                                    "bg-surface-container-highest text-on-surface/70 hover:bg-surface-variant hover:text-on-surface border-white/10 shadow-sm"
                                },
                            )
                        }
                    >
                        <span class="material-symbols-outlined text-[20px] transition-transform group-active:scale-90">
                            {move || if is_sound.get() { "volume_up" } else { "volume_off" }}
                        </span>
                        <span class="text-sm tracking-wide">
                            {move || if is_sound.get() { "Sound ON" } else { "Sound OFF" }}
                        </span>
                    </button>
                </div>
                <div class="flex items-center gap-2">
                    <ActionForm action=test_action>
                        <button
                            type="submit"
                            data-testid="test-overlay-btn"
                            class=move || {
                                format!(
                                    "inline-flex items-center gap-xs px-md py-sm rounded-xl bg-surface-container-highest text-on-surface font-bold hover:bg-surface-container-highest/80 transition-all border border-white/10 {}",
                                    if test_action.pending().get() { "opacity-50" } else { "" },
                                )
                            }
                            disabled=move || test_action.pending().get()
                        >
                            <span class="material-symbols-outlined text-[18px]">"science"</span>
                            <span>"Test Overlay"</span>
                        </button>
                    </ActionForm>

                    <button
                        data-testid="copy-overlay-link-btn"
                        on:click=copy_to_clipboard
                        class="inline-flex items-center gap-xs px-md py-sm rounded-xl bg-primary-container text-on-primary-container font-bold hover:brightness-110 transition-all active:scale-[0.98]"
                    >
                        <span class="material-symbols-outlined text-[18px]">"content_copy"</span>
                        <span>"Copy Overlay Link"</span>
                    </button>

                    <a
                        href=format!("/streamer/{}", streamer.username)
                        target="_blank"
                        class="inline-flex items-center gap-xs px-md py-sm rounded-xl bg-secondary text-on-secondary-container font-bold hover:brightness-110 transition-all active:scale-[0.98]"
                    >
                        <span class="material-symbols-outlined text-[18px]">"open_in_new"</span>
                        <span>"Open Donate Page"</span>
                    </a>
                </div>
            </div>
        </div>

        <DonationHistory streamer_id=streamer.id />
    }
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    let streamer = use_context::<crate::db::DbStreamer>().expect("Streamer context missing");
    let update_action = ServerAction::<UpdateStreamerProfile>::new();
    let action_value = update_action.value();
    let streamer_for_avatar = streamer.clone();
    
    view! {
        <div class="flex flex-col gap-lg max-w-2xl">
            <h1 class="text-headline-lg font-headline-lg text-on-surface">
                {leptos_fluent::move_tr!("settings-title")}
            </h1>

            <ActionForm action=update_action>
                <div class="flex flex-col gap-md bg-surface-container-low/40 backdrop-blur-md border border-white/10 rounded-2xl p-lg">
                    <div class="flex flex-col gap-xs">
                        <label class="text-label-md font-bold text-on-surface">
                            {leptos_fluent::move_tr!("settings-streamer-name")}
                        </label>
                        <input
                            type="text"
                            name="new_display_name"
                            data-testid="settings-display-name-input"
                            class="bg-surface-variant/30 border-none rounded-lg px-md py-sm text-on-surface focus:ring-2 focus:ring-primary"
                            value=streamer.display_name.clone()
                            required
                        />
                    </div>
                    <div class="flex flex-col gap-xs">
                        <label class="text-label-md font-bold text-on-surface">
                            {leptos_fluent::move_tr!("settings-description")}
                        </label>
                        <textarea
                            name="new_bio"
                            data-testid="settings-bio-input"
                            class="bg-surface-variant/30 border-none rounded-lg px-md py-sm text-on-surface focus:ring-2 focus:ring-primary h-24"
                            required
                        >
                            {streamer.bio.clone()}
                        </textarea>
                    </div>
                    <div class="flex flex-col gap-xs">
                        <label class="text-label-md font-bold text-on-surface">
                            {leptos_fluent::move_tr!("settings-link-id")}
                        </label>
                        <input
                            type="text"
                            name="new_username"
                            data-testid="settings-username-input"
                            class="bg-surface-variant/30 border-none rounded-lg px-md py-sm text-on-surface focus:ring-2 focus:ring-primary"
                            value=streamer.username.clone()
                            required
                            pattern="^[a-zA-Z0-9_]+$"
                            title="Only letters, numbers, and underscores allowed"
                        />
                        <span class="text-label-sm text-on-surface-variant">
                            {leptos_fluent::move_tr!("settings-public-path")}
                        </span>
                    </div>

                    {streamer
                        .payment_methods
                        .clone()
                        .into_iter()
                        .map(|pm| {
                            view! {
                                <input
                                    type="hidden"
                                    name="payment_methods[]"
                                    value=pm.to_string()
                                />
                            }
                        })
                        .collect_view()}

                    <Suspense fallback=|| ()>
                        {move || match action_value.get() {
                            Some(Ok(_)) => {
                                view! {
                                    <div
                                        data-testid="settings-success-message"
                                        class="text-secondary font-bold"
                                    >
                                        {leptos_fluent::move_tr!("settings-success-msg")}
                                    </div>
                                }
                                    .into_any()
                            }
                            Some(Err(e)) => {
                                view! {
                                    <div
                                        data-testid="settings-error-message"
                                        class="text-error font-bold"
                                    >
                                        {format!("Error: {}", e)}
                                    </div>
                                }
                                    .into_any()
                            }
                            None => view! {}.into_any(),
                        }}
                    </Suspense>

                    <div class="flex justify-end mt-sm">
                        <button
                            type="submit"
                            data-testid="settings-save-button"
                            class=move || {
                                format!(
                                    "bg-primary text-on-primary font-bold px-lg py-sm rounded-xl transition-all flex items-center gap-xs {}",
                                    if update_action.pending().get() {
                                        "opacity-70 cursor-not-allowed"
                                    } else {
                                        "hover:brightness-110 active:scale-95"
                                    },
                                )
                            }
                            disabled=move || update_action.pending().get()
                        >
                            {move || {
                                if update_action.pending().get() {
                                    view! {
                                        <span class="material-symbols-outlined text-[18px] animate-spin">
                                            "progress_activity"
                                        </span>
                                    }
                                        .into_any()
                                } else {
                                    view! {}.into_any()
                                }
                            }}
                            {leptos_fluent::move_tr!("settings-btn-save")}
                        </button>
                    </div>
                </div>
            </ActionForm>

            <crate::avatar::AvatarSettingsSection streamer=streamer_for_avatar />
            <MediaSettingsSection />
        </div>
    }
}

#[component]
pub fn PaymentsPage() -> impl IntoView {
    let streamer = use_context::<crate::db::DbStreamer>().expect("Streamer context missing");
    let update_action = ServerAction::<crate::app::UpdateStreamerProfile>::new();
    let action_value = update_action.value();

    view! {
        <div class="mb-lg">
            <h1
                data-testid="streamer-payments-header"
                class="text-headline-lg font-headline-lg text-on-surface"
            >
                {leptos_fluent::move_tr!("payments-title")}
            </h1>
            <p class="text-body-md font-body-md text-on-surface-variant">
                {leptos_fluent::move_tr!("payments-subtitle")}
            </p>
        </div>

        <div class="bg-surface-container-low/40 backdrop-blur-md border border-white/10 rounded-2xl p-md md:p-xl max-w-2xl">
            <ActionForm action=update_action>
                <div class="flex flex-col gap-lg">
                    <input
                        type="hidden"
                        name="new_display_name"
                        value=streamer.display_name.clone()
                    />
                    <input type="hidden" name="new_bio" value=streamer.bio.clone() />
                    <input type="hidden" name="new_username" value=streamer.username.clone() />

                    <div class="flex flex-col gap-xs">
                        <label class="text-label-md font-bold text-on-surface">
                            {leptos_fluent::move_tr!("payments-supported-methods")}
                        </label>
                        <div class="flex flex-col gap-sm">
                            <label class="flex items-center gap-sm cursor-pointer">
                                <input
                                    type="checkbox"
                                    name="payment_methods[]"
                                    value="Mock Auto"
                                    checked=streamer
                                        .payment_methods
                                        .contains(&PaymentMethod::MockAuto)
                                    class="w-5 h-5 accent-primary bg-surface-variant/30 border-none rounded"
                                />
                                <span class="text-body-md text-on-surface">
                                    {leptos_fluent::move_tr!("payments-mock-auto")}
                                </span>
                            </label>
                            <label class="flex items-center gap-sm cursor-pointer">
                                <input
                                    type="checkbox"
                                    name="payment_methods[]"
                                    value="Mock Manual"
                                    checked=streamer
                                        .payment_methods
                                        .contains(&PaymentMethod::MockManual)
                                    class="w-5 h-5 accent-primary bg-surface-variant/30 border-none rounded"
                                />
                                <span class="text-body-md text-on-surface">
                                    {leptos_fluent::move_tr!("payments-mock-manual")}
                                </span>
                            </label>
                        </div>
                    </div>

                    <Suspense fallback=|| ()>
                        {move || match action_value.get() {
                            Some(Ok(_)) => {
                                view! {
                                    <div
                                        data-testid="payments-success-message"
                                        class="text-secondary font-bold"
                                    >
                                        {leptos_fluent::move_tr!("payments-success-msg")}
                                    </div>
                                }
                                    .into_any()
                            }
                            Some(Err(e)) => {
                                view! {
                                    <div class="text-error font-bold">
                                        {format!("Error: {}", e)}
                                    </div>
                                }
                                    .into_any()
                            }
                            None => view! {}.into_any(),
                        }}
                    </Suspense>

                    <div class="flex justify-end pt-sm border-t border-white/10">
                        <button
                            type="submit"
                            data-testid="save-payments-btn"
                            class=move || {
                                format!(
                                    "bg-primary text-on-primary font-bold py-sm px-xl rounded-xl transition-all shadow-lg shadow-primary/20 flex items-center gap-xs {}",
                                    if update_action.pending().get() {
                                        "opacity-70 cursor-not-allowed"
                                    } else {
                                        "hover:brightness-110 active:scale-95"
                                    },
                                )
                            }
                            disabled=move || update_action.pending().get()
                        >
                            {move || {
                                if update_action.pending().get() {
                                    view! {
                                        <span class="material-symbols-outlined text-[18px] animate-spin">
                                            "progress_activity"
                                        </span>
                                    }
                                        .into_any()
                                } else {
                                    view! {}.into_any()
                                }
                            }}
                            {leptos_fluent::move_tr!("payments-btn-save")}
                        </button>
                    </div>
                </div>
            </ActionForm>
        </div>
    }
}


#[component]
pub fn DonationHistory(streamer_id: i32) -> impl IntoView {
    let (page, set_page) = signal(1i64);
    let (page_size, set_page_size) = signal(10i64);
    let (auto_reload, set_auto_reload) = signal(true);
    let (reload_interval, set_reload_interval) = signal(5u64);
    let (reload_trigger, _set_reload_trigger) = signal(0);

    #[cfg(feature = "hydrate")]
    {
        use leptos::prelude::Effect;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;

        Effect::new(move |_| {
            if auto_reload.get() {
                let interval = reload_interval.get();
                let set_rt = _set_reload_trigger;
                
                let window = web_sys::window().unwrap();
                let closure = Closure::wrap(Box::new(move || {
                    set_rt.update(|n| *n += 1);
                }) as Box<dyn FnMut()>);
                
                let id = window.set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    (interval * 1000) as i32,
                ).unwrap();
                
                closure.forget();
                
                leptos::prelude::on_cleanup(move || {
                    web_sys::window().unwrap().clear_interval_with_handle(id);
                });
            }
        });
    }

    let tx_resource = LocalResource::new(move || {
        let p = page.get();
        let ps = page_size.get();
        let trig = reload_trigger.get();
        async move {
            get_dashboard_transactions(streamer_id, p, ps, trig).await
        }
    });

    let mark_action = ServerAction::<crate::app::MarkTransactionViewed>::new();

    let total_count = move || {
        tx_resource.get().and_then(|r| r.ok()).map(|(_, count)| count).unwrap_or(0)
    };
    
    let _total_pages = Memo::new(move |_| {
        let count = total_count();
        let size = page_size.get();
        if size == 0 { 1 } else { ((count as f64) / (size as f64)).ceil() as i64 }
    });

    let start_idx = move || {
        let count = total_count();
        if count == 0 { 0 } else { (page.get() - 1) * page_size.get() + 1 }
    };
    
    let end_idx = move || {
        let end = page.get() * page_size.get();
        let count = total_count();
        if end > count { count } else { end }
    };

    view! {
        <div class="glass-card p-lg rounded-xl flex flex-col gap-lg mb-lg">
            <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-md">
                <div>
                    <h3 class="text-headline-md font-headline-md text-on-surface">
                        {leptos_fluent::move_tr!("dashboard-donation-history")}
                    </h3>
                    <p class="text-body-md font-body-md text-on-surface-variant">
                        {leptos_fluent::move_tr!("dashboard-donation-history-sub")}
                    </p>
                </div>

                <div class="flex flex-wrap gap-sm w-full md:w-auto items-center">
                    <label class="flex items-center gap-2 text-label-md text-on-surface cursor-pointer">
                        <input
                            type="checkbox"
                            class="accent-primary w-4 h-4 cursor-pointer"
                            prop:checked=move || auto_reload.get()
                            on:change=move |ev| set_auto_reload.set(event_target_checked(&ev))
                        />
                        {leptos_fluent::move_tr!("dashboard-auto-reload")}
                    </label>
                    <div class="flex items-center gap-2 ml-2">
                        <select
                            class="bg-surface-variant/30 border-none rounded-lg px-2 py-1 text-label-sm text-on-surface focus:ring-1 focus:ring-primary cursor-pointer"
                            on:change=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<u64>() {
                                    set_reload_interval.set(val);
                                }
                            }
                        >
                            <option value="5" selected=move || reload_interval.get() == 5>
                                "5s"
                            </option>
                            <option value="10" selected=move || reload_interval.get() == 10>
                                "10s"
                            </option>
                            <option value="30" selected=move || reload_interval.get() == 30>
                                "30s"
                            </option>
                            <option value="60" selected=move || reload_interval.get() == 60>
                                "60s"
                            </option>
                        </select>
                    </div>
                    <div class="flex items-center gap-2 ml-4">
                        <span class="text-label-sm text-on-surface-variant">
                            {leptos_fluent::move_tr!("dashboard-per-page")}
                        </span>
                        <select
                            class="bg-surface-variant/30 border-none rounded-lg px-2 py-1 text-label-sm text-on-surface focus:ring-1 focus:ring-primary cursor-pointer"
                            on:change=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<i64>() {
                                    set_page_size.set(val);
                                    set_page.set(1);
                                }
                            }
                        >
                            <option value="5" selected=move || page_size.get() == 5>
                                "5"
                            </option>
                            <option value="10" selected=move || page_size.get() == 10>
                                "10"
                            </option>
                            <option value="20" selected=move || page_size.get() == 20>
                                "20"
                            </option>
                            <option value="50" selected=move || page_size.get() == 50>
                                "50"
                            </option>
                        </select>
                    </div>
                </div>
            </div>

            <div class="flex justify-between items-center py-sm mb-md border-b border-white/10">
                <p class="text-label-sm font-label-sm text-on-surface-variant">
                    {leptos_fluent::move_tr!("dashboard-showing")} " " {move || start_idx()} " "
                    {leptos_fluent::move_tr!("dashboard-to")} " " {move || end_idx()} " "
                    {leptos_fluent::move_tr!("dashboard-of")} " " {move || total_count()} " "
                    {leptos_fluent::move_tr!("dashboard-donations")}
                </p>
                <div class="flex gap-xs">
                    <button
                        class="w-10 h-10 flex items-center justify-center rounded-lg bg-surface-variant/30 text-on-surface-variant hover:bg-surface-variant/50 disabled:opacity-30"
                        disabled=move || page.get() <= 1
                        on:click=move |_| {
                            set_page
                                .update(|p| {
                                    if *p > 1 {
                                        *p -= 1;
                                    }
                                })
                        }
                    >
                        <span class="material-symbols-outlined">"chevron_left"</span>
                    </button>
                    <span class="w-10 h-10 flex items-center justify-center rounded-lg bg-primary text-on-primary font-bold text-label-md">
                        {move || page.get()}
                    </span>
                    <button
                        class="w-10 h-10 flex items-center justify-center rounded-lg bg-surface-variant/30 text-on-surface-variant hover:bg-surface-variant/50 disabled:opacity-30"
                        disabled=move || page.get() >= _total_pages.get()
                        on:click=move |_| {
                            set_page
                                .update(|p| {
                                    if *p < _total_pages.get() {
                                        *p += 1;
                                    }
                                })
                        }
                    >
                        <span class="material-symbols-outlined">"chevron_right"</span>
                    </button>
                </div>
            </div>

            <div class="overflow-x-auto custom-scrollbar">
                <table class="w-full text-left border-collapse min-w-[700px]">
                    <thead>
                        <tr class="border-b border-white/10 text-label-sm font-label-sm text-on-surface-variant uppercase tracking-wider">
                            <th class="pb-md pr-md">
                                {leptos_fluent::move_tr!("dashboard-col-donor")}
                            </th>
                            <th class="pb-md px-md">
                                {leptos_fluent::move_tr!("dashboard-col-amount")}
                            </th>
                            <th class="pb-md px-md">
                                {leptos_fluent::move_tr!("dashboard-col-message")}
                            </th>
                            <th class="pb-md px-md">Status</th>
                            <th class="pb-md pl-md">
                                {leptos_fluent::move_tr!("dashboard-col-time")}
                            </th>
                        </tr>
                    </thead>
                    <tbody class="text-body-md">
                        <Suspense fallback=move || {
                            view! {
                                {(0..page_size.get_untracked())
                                    .map(|_| {
                                        view! {
                                            <tr class="border-b border-white/5 animate-pulse">
                                                <td class="py-md pr-md">
                                                    <div class="flex items-center gap-sm">
                                                        <div class="w-8 h-8 rounded-full bg-surface-variant/50"></div>
                                                        <div class="h-4 w-24 bg-surface-variant/50 rounded"></div>
                                                    </div>
                                                </td>
                                                <td class="py-md px-md">
                                                    <div class="h-4 w-16 bg-surface-variant/50 rounded"></div>
                                                </td>
                                                <td class="py-md px-md">
                                                    <div class="h-4 w-32 bg-surface-variant/50 rounded"></div>
                                                </td>
                                                <td class="py-md px-md">
                                                    <div class="h-4 w-16 bg-surface-variant/50 rounded"></div>
                                                </td>
                                                <td class="py-md pl-md">
                                                    <div class="h-4 w-20 bg-surface-variant/50 rounded"></div>
                                                </td>
                                            </tr>
                                        }
                                    })
                                    .collect_view()}
                            }
                        }>
                            {move || {
                                tx_resource
                                    .get()
                                    .map(|res| match res {
                                        Ok((txs, _)) => {
                                            if txs.is_empty() {
                                                view! {
                                                    <tr>
                                                        <td
                                                            colspan="4"
                                                            class="py-md text-center text-on-surface-variant"
                                                        >
                                                            {leptos_fluent::move_tr!("dashboard-no-donations")}
                                                        </td>
                                                    </tr>
                                                }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    {txs
                                                        .into_iter()
                                                        .map(|tx| {
                                                            view! {
                                                                <tr
                                                                    data-testid="donation-row"
                                                                    class="border-b border-white/5 hover:bg-white/5 transition-colors group"
                                                                >
                                                                    <td class="py-md pr-md">
                                                                        <div class="flex items-center gap-sm">
                                                                            <div class="w-8 h-8 rounded-full bg-primary/20 flex items-center justify-center">
                                                                                <span class="material-symbols-outlined text-sm text-primary">
                                                                                    "person"
                                                                                </span>
                                                                            </div>
                                                                            <span class="font-bold text-on-surface">
                                                                                {tx.donor_name}
                                                                            </span>
                                                                        </div>
                                                                    </td>
                                                                    <td class="py-md px-md font-bold text-secondary">
                                                                        "+$" {format!("{:.2}", tx.amount)}
                                                                    </td>
                                                                    <td class="py-md px-md text-on-surface-variant italic">
                                                                        {if let Some(msg) = tx.message.clone() {
                                                                            format!("\"{}\"", msg)
                                                                        } else {
                                                                            "".to_string()
                                                                        }}
                                                                    </td>
                                                                    <td class="py-md px-md">
                                                                        {if tx.status == TransactionStatus::ReadyForDisplay {
                                                                            view! {
                                                                                <div class="flex items-center gap-2">
                                                                                    <span class="px-2 py-0.5 rounded text-[10px] font-bold bg-primary/20 text-primary uppercase">
                                                                                        Ready
                                                                                    </span>
                                                                                    <ActionForm action=mark_action>
                                                                                        <input type="hidden" name="tx_id" value=tx.id />
                                                                                        <button
                                                                                            type="submit"
                                                                                            class="text-xs text-on-surface-variant hover:text-on-surface underline"
                                                                                        >
                                                                                            "Mark as Viewed"
                                                                                        </button>
                                                                                    </ActionForm>
                                                                                </div>
                                                                            }
                                                                                .into_any()
                                                                        } else {
                                                                            view! {
                                                                                <span class="px-2 py-0.5 rounded text-[10px] font-bold bg-surface-variant text-on-surface-variant uppercase">
                                                                                    Displayed
                                                                                </span>
                                                                            }
                                                                                .into_any()
                                                                        }}
                                                                    </td>
                                                                    <td class="py-md pl-md text-outline">{tx.created_at}</td>
                                                                </tr>
                                                            }
                                                        })
                                                        .collect_view()}
                                                }
                                                    .into_any()
                                            }
                                        }
                                        Err(_) => {
                                            view! {
                                                <tr>
                                                    <td colspan="4" class="py-md text-center text-error">
                                                        {leptos_fluent::move_tr!("dashboard-donations-error")}
                                                    </td>
                                                </tr>
                                            }
                                                .into_any()
                                        }
                                    })
                            }}
                        </Suspense>
                    </tbody>
                </table>
            </div>
        </div>
    }
}

pub use crate::analytics::AnalyticsPage;


#[component]
pub fn S3StatusBanner() -> impl IntoView {
    let s3_status_resource = Resource::new(
        || (),
        |_| async move { crate::app::check_s3_status().await.unwrap_or(false) }
    );
    
    view! {
        <Suspense fallback=|| ()>
            {move || {
                s3_status_resource
                    .get()
                    .map(|is_up| {
                        if !is_up {
                            view! {
                                <div class="bg-error/20 border border-error/50 text-error px-4 py-3 rounded-xl mb-6 flex items-center gap-3 shadow-lg shadow-error/10">
                                    <span class="material-symbols-outlined">"cloud_off"</span>
                                    <div>
                                        <h4 class="font-bold">"Media Server Unavailable"</h4>
                                        <p class="text-sm">
                                            "Custom media uploads are currently disabled. The overlay will automatically fallback to default media."
                                        </p>
                                    </div>
                                </div>
                            }
                                .into_any()
                        } else {
                            view! {}.into_any()
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn MediaSettingsSection() -> impl IntoView {
    let streamer = use_context::<crate::db::DbStreamer>().expect("Streamer context missing");
    
    let default_medias = Resource::new(|| (), |_| async move {
        crate::app::get_default_medias().await.unwrap_or_default()
    });
    
    let streamer_medias = Resource::new(|| (), |_| async move {
        crate::app::get_streamer_media().await.unwrap_or_default()
    });
    
    let save_media_action = ServerAction::<crate::app::SaveMediaSettings>::new();
    
    #[allow(unused_variables)]
    let (upload_pending, set_upload_pending) = signal(false);
    #[allow(unused_variables)]
    let (upload_result, set_upload_result) = signal::<Option<Result<(), String>>>(None);
    
    let (selected_media, _set_selected_media) = signal::<Option<uuid::Uuid>>(streamer.selected_media_id);
    let (fallback_media, _set_fallback_media) = signal::<String>(streamer.fallback_media_file.clone());

    view! {
        <div class="flex flex-col gap-lg bg-surface-container-low/40 backdrop-blur-md border border-white/10 rounded-2xl p-lg mt-lg max-w-2xl">
            <div class="mb-2">
                <h2 class="text-headline-sm font-headline-sm text-on-surface">"Media Settings"</h2>
                <p class="text-on-surface-variant text-sm">
                    "Upload custom media for your overlay and configure fallback options."
                </p>
            </div>

            <div class="bg-surface-variant/20 p-4 rounded-xl border border-white/5">
                <h3 class="font-bold mb-2 text-on-surface">"Upload New Media"</h3>
                <form on:submit=move |ev| {
                    ev.prevent_default();
                    #[cfg(feature = "hydrate")]
                    {
                        let target = event_target::<web_sys::HtmlFormElement>(&ev);
                        let elements = target.elements();
                        use wasm_bindgen::JsCast;
                        let file_input = elements
                            .named_item("file")
                            .unwrap()
                            .unchecked_into::<web_sys::HtmlInputElement>();
                        if let Some(files) = file_input.files() {
                            if let Some(file) = files.get(0) {
                                let file_name = file.name();
                                let content_type = file.type_();
                                let file_size = file.size() as i32;
                                set_upload_pending.set(true);
                                leptos::task::spawn_local(async move {
                                    use wasm_bindgen_futures::JsFuture;
                                    match crate::app::get_presigned_url(
                                            file_name.clone(),
                                            content_type,
                                        )
                                        .await
                                    {
                                        Ok((public_url, upload_url)) => {
                                            let window = web_sys::window().unwrap();
                                            let req_init = web_sys::RequestInit::new();
                                            req_init.set_method("PUT");
                                            req_init.set_body(&file);
                                            if let Ok(request) = web_sys::Request::new_with_str_and_init(
                                                &upload_url,
                                                &req_init,
                                            ) {
                                                if let Ok(resp_value) = JsFuture::from(
                                                        window.fetch_with_request(&request),
                                                    )
                                                    .await
                                                {
                                                    let resp: web_sys::Response = resp_value.into();
                                                    if resp.ok() {
                                                        let _ = crate::app::save_media_record(
                                                                file_name,
                                                                public_url,
                                                                file_size,
                                                            )
                                                            .await;
                                                        set_upload_result.set(Some(Ok(())));
                                                    } else {
                                                        set_upload_result
                                                            .set(Some(Err("Upload to S3 failed.".to_string())));
                                                    }
                                                } else {
                                                    set_upload_result
                                                        .set(Some(Err("Network error during upload".to_string())));
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            leptos::logging::error!(
                                                "Failed to get presigned URL: {:?}", e
                                            );
                                            set_upload_result
                                                .set(
                                                    Some(Err(format!("Failed to get presigned URL: {:?}", e))),
                                                );
                                        }
                                    }
                                    set_upload_pending.set(false);
                                });
                            }
                        }
                    }
                }>
                    <div class="flex flex-col gap-2">
                        <input
                            data-testid="media-file-input"
                            type="file"
                            name="file"
                            accept="audio/*,video/*"
                            class="text-on-surface text-sm file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-primary file:text-on-primary hover:file:bg-primary/80"
                            required
                        />
                        <button
                            data-testid="upload-media-button"
                            type="submit"
                            class=move || {
                                format!(
                                    "bg-primary text-on-primary font-bold px-4 py-2 rounded-xl mt-2 w-fit {}",
                                    if upload_pending.get() { "opacity-50" } else { "" },
                                )
                            }
                            disabled=move || upload_pending.get()
                        >
                            {move || {
                                if upload_pending.get() {
                                    "Uploading..."
                                } else {
                                    "Upload File (Max 2MB)"
                                }
                            }}
                        </button>
                    </div>
                </form>
                <Suspense fallback=|| ()>
                    {move || match upload_result.get() {
                        Some(Ok(_)) => {
                            view! {
                                <div
                                    data-testid="media-upload-success"
                                    class="text-green-400 mt-2 text-sm font-bold"
                                >
                                    "Upload successful! Please refresh the page to see it."
                                </div>
                            }
                                .into_any()
                        }
                        Some(Err(e)) => {
                            view! {
                                <div
                                    data-testid="media-upload-error"
                                    class="text-red-400 mt-2 text-sm font-bold"
                                >
                                    {format!("Error: {}", e)}
                                </div>
                            }
                                .into_any()
                        }
                        None => view! {}.into_any(),
                    }}
                </Suspense>
            </div>

            <ActionForm action=save_media_action>
                <div class="flex flex-col gap-6">
                    <div class="flex flex-col gap-2">
                        <label class="font-bold text-on-surface">"Primary Media"</label>
                        <select
                            name="selected_media_id"
                            class="bg-surface-variant/30 border-none rounded-lg px-4 py-2 text-on-surface focus:ring-2 focus:ring-primary"
                        >
                            <option value="" selected=move || selected_media.get().is_none()>
                                "None (Use Fallback)"
                            </option>
                            <Suspense fallback=move || {
                                view! { <option>"Loading..."</option> }
                            }>
                                {move || {
                                    streamer_medias
                                        .get()
                                        .unwrap_or_default()
                                        .into_iter()
                                        .map(|media| {
                                            let is_selected = selected_media.get() == Some(media.id);
                                            view! {
                                                <option value=media.id.to_string() selected=is_selected>
                                                    {format!(
                                                        "{} ({} bytes)",
                                                        media.file_name,
                                                        media.size_bytes,
                                                    )}
                                                </option>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </Suspense>
                        </select>
                        <p class="text-xs text-on-surface-variant">
                            "This media will be played on your overlay when S3 is online."
                        </p>
                    </div>

                    <div class="flex flex-col gap-2">
                        <label class="font-bold text-on-surface">"Fallback Media"</label>
                        <select
                            name="fallback_media_file"
                            class="bg-surface-variant/30 border-none rounded-lg px-4 py-2 text-on-surface focus:ring-2 focus:ring-primary"
                        >
                            <Suspense fallback=move || {
                                view! { <option>"Loading..."</option> }
                            }>
                                {move || {
                                    default_medias
                                        .get()
                                        .unwrap_or_default()
                                        .into_iter()
                                        .map(|file| {
                                            let is_selected = fallback_media.get() == file;
                                            let file_clone = file.clone();
                                            view! {
                                                <option value=file selected=is_selected>
                                                    {file_clone}
                                                </option>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </Suspense>
                        </select>
                        <p class="text-xs text-on-surface-variant">
                            "This default media will be played if your Primary Media fails to load or S3 is down."
                        </p>
                    </div>

                    <Suspense fallback=|| ()>
                        {move || match save_media_action.value().get() {
                            Some(Ok(_)) => {
                                view! {
                                    <div class="text-green-400 text-sm font-bold mt-2">
                                        "Settings saved successfully!"
                                    </div>
                                }
                                    .into_any()
                            }
                            Some(Err(e)) => {
                                view! {
                                    <div class="text-red-400 text-sm font-bold mt-2">
                                        {format!("Error: {}", e)}
                                    </div>
                                }
                                    .into_any()
                            }
                            None => view! {}.into_any(),
                        }}
                    </Suspense>

                    <div class="flex justify-end mt-4">
                        <button
                            type="submit"
                            class="bg-primary text-on-primary font-bold px-6 py-2 rounded-xl hover:brightness-110 active:scale-95 transition-all flex items-center gap-2"
                            disabled=move || save_media_action.pending().get()
                        >
                            "Save Media Settings"
                        </button>
                    </div>
                </div>
            </ActionForm>
        </div>
    }
}
