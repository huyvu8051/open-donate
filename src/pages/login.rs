use crate::components::layout::Header;
use leptos::prelude::*;
use crate::auth::LoginWithEmail;

#[component]
pub fn LoginPage() -> impl IntoView {
    let login_action = ServerAction::<LoginWithEmail>::new();

    let error_msg = move || {
        login_action.value().get().and_then(|res| res.err()).map(|e| e.to_string().replace("error: ", ""))
    };

    Effect::new(move |_| {
        if let Some(Ok(_)) = login_action.value().get() {
            let _ = leptos::prelude::window().location().set_href("/dashboard");
        }
    });

    view! {
        <Header />
        <div class="min-h-screen pt-[120px] flex flex-col items-center justify-center px-margin-mobile">
            <div class="w-full max-w-[480px] bg-surface-container/50 backdrop-blur-xl p-md md:p-lg rounded-3xl border border-white/10 shadow-2xl">
                <h1 class="text-headline-xl font-headline-xl text-center text-on-surface mb-lg">
                    "Welcome Back"
                </h1>

                <ActionForm action=login_action>
                    <div class="flex flex-col gap-md">
                        {move || {
                            error_msg()
                                .map(|err| {
                                    view! {
                                        <div class="bg-error-container text-on-error-container p-sm rounded-lg text-body-md text-center">
                                            {err}
                                        </div>
                                    }
                                })
                        }} <div class="flex flex-col gap-xs">
                            <label class="text-body-md text-on-surface-variant font-medium">
                                "Email"
                            </label>
                            <input
                                type="email"
                                name="email"
                                data-testid="email-input"
                                required
                                class="bg-surface-container-highest border border-white/10 rounded-xl p-sm text-body-md text-on-surface focus:border-primary focus:outline-none transition-colors w-full"
                                placeholder="your@email.com"
                            />
                        </div> <div class="flex flex-col gap-xs">
                            <label class="text-body-md text-on-surface-variant font-medium">
                                "Password"
                            </label>
                            <input
                                type="password"
                                name="password"
                                data-testid="password-input"
                                required
                                class="bg-surface-container-highest border border-white/10 rounded-xl p-sm text-body-md text-on-surface focus:border-primary focus:outline-none transition-colors w-full"
                                placeholder="••••••••"
                            />
                        </div>
                        <button
                            type="submit"
                            data-testid="auth-submit-btn"
                            class=move || {
                                format!(
                                    "mt-md w-full bg-primary text-on-primary py-sm rounded-xl text-headline-md font-bold transition-all shadow-lg shadow-primary/20 {}",
                                    if login_action.pending().get() {
                                        "opacity-70 cursor-not-allowed"
                                    } else {
                                        "hover:brightness-110 active:scale-95"
                                    },
                                )
                            }
                            disabled=move || login_action.pending().get()
                        >
                            {move || {
                                if login_action.pending().get() {
                                    "Signing In..."
                                } else {
                                    "Sign In"
                                }
                            }}
                        </button>
                    </div>
                </ActionForm>

                <div class="mt-lg text-center">
                    <p class="text-on-surface-variant text-body-md">
                        "Don't have an account? "
                        <a href="/register" class="text-primary hover:underline font-bold">
                            "Sign Up"
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}
