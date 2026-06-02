use crate::components::layout::Header;
use leptos::prelude::*;
use crate::auth::RegisterWithEmail;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let register_action = ServerAction::<RegisterWithEmail>::new();

    let error_msg = move || {
        register_action.value().get().and_then(|res| res.err()).map(|e| e.to_string().replace("error: ", ""))
    };

    Effect::new(move |_| {
        if let Some(Ok(_)) = register_action.value().get() {
            let _ = leptos::prelude::window().location().set_href("/dashboard");
        }
    });

    view! {
        <Header />
        <div class="min-h-screen pt-[120px] flex flex-col items-center justify-center px-margin-mobile">
            <div class="w-full max-w-[480px] bg-surface-container/50 backdrop-blur-xl p-md md:p-lg rounded-3xl border border-white/10 shadow-2xl">
                <h1 class="text-headline-xl font-headline-xl text-center text-on-surface mb-lg">
                    "Create Account"
                </h1>

                <ActionForm action=register_action>
                    <div class="flex flex-col gap-md">
                        {move || {
                            error_msg()
                                .map(|err| {
                                    view! {
                                        <div
                                            data-testid="error-container"
                                            class="bg-error-container text-on-error-container p-sm rounded-lg text-body-md text-center"
                                        >
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
                                minlength="6"
                            />
                        </div> <div class="flex flex-col gap-xs">
                            <label class="text-body-md text-on-surface-variant font-medium">
                                "Confirm Password"
                            </label>
                            <input
                                type="password"
                                name="password_confirm"
                                data-testid="password-confirm-input"
                                required
                                class="bg-surface-container-highest border border-white/10 rounded-xl p-sm text-body-md text-on-surface focus:border-primary focus:outline-none transition-colors w-full"
                                placeholder="••••••••"
                                minlength="6"
                            />
                        </div>
                        <button
                            type="submit"
                            data-testid="auth-submit-btn"
                            class=move || {
                                format!(
                                    "mt-md w-full bg-secondary text-on-secondary py-sm rounded-xl text-headline-md font-bold transition-all shadow-lg shadow-secondary/20 {}",
                                    if register_action.pending().get() {
                                        "opacity-70 cursor-not-allowed"
                                    } else {
                                        "hover:brightness-110 active:scale-95"
                                    },
                                )
                            }
                            disabled=move || register_action.pending().get()
                        >
                            {move || {
                                if register_action.pending().get() {
                                    "Signing Up..."
                                } else {
                                    "Sign Up"
                                }
                            }}
                        </button>
                    </div>
                </ActionForm>

                <div class="mt-lg text-center">
                    <p class="text-on-surface-variant text-body-md">
                        "Already have an account? "
                        <a href="/login" class="text-primary hover:underline font-bold">
                            "Sign In"
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}
