use leptos::prelude::*;
use crate::components::layout::{Header, Footer};

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
                    {leptos_fluent::move_tr!("landing-title-start")}
                    <span class="text-primary italic">
                        {leptos_fluent::move_tr!("landing-title-glint")}
                    </span> {leptos_fluent::move_tr!("landing-title-end")}
                </h1>
                <p class="text-body-lg font-body-lg text-on-surface-variant/80 max-w-lg mx-auto italic">
                    {leptos_fluent::move_tr!("landing-trusted-by")}
                </p>
                <div class="flex flex-col md:flex-row items-center justify-center gap-md pt-base">
                    <button class="px-lg py-md bg-secondary text-on-secondary-container rounded-xl font-headline-md text-headline-md neon-glow-secondary hover:scale-105 transition-transform active:scale-95 duration-150">
                        {leptos_fluent::move_tr!("landing-donate-now")}
                    </button>
                    <a
                        href=move || format!("/{}/register", expect_context::<leptos_fluent::I18n>().language.get().id.to_string())
                        class="px-lg py-md bg-surface-container-highest/40 backdrop-blur-md border border-white/20 text-on-surface rounded-xl font-headline-md text-headline-md hover:bg-surface-container-highest/60 transition-all inline-block text-center"
                    >
                        {leptos_fluent::move_tr!("landing-start-creating")}
                    </a>
                </div>
            </div>
            <div class="mt-xl grid grid-cols-1 md:grid-cols-3 gap-md w-full max-w-5xl">
                <div class="glass-card flex-1 min-w-[200px] p-lg md:p-xl rounded-2xl flex flex-col items-center justify-center gap-xs">
                    <span class="text-display-sm md:text-display-md font-display-md font-bold text-primary">
                        "10K+"
                    </span>
                    <span class="text-label-md md:text-label-lg font-label-lg text-on-surface-variant font-medium tracking-wide uppercase">
                        {leptos_fluent::move_tr!("landing-active-creators")}
                    </span>
                </div>
                <div class="glass-card flex-1 min-w-[200px] p-lg md:p-xl rounded-2xl flex flex-col items-center justify-center gap-xs">
                    <span class="text-display-sm md:text-display-md font-display-md font-bold text-secondary">
                        "2M+"
                    </span>
                    <span class="text-label-md md:text-label-lg font-label-lg text-on-surface-variant font-medium tracking-wide uppercase">
                        {leptos_fluent::move_tr!("landing-live-viewers")}
                    </span>
                </div>
                <div class="glass-card flex-1 min-w-[200px] p-lg md:p-xl rounded-2xl flex flex-col items-center justify-center gap-xs">
                    <span class="text-display-sm md:text-display-md font-display-md font-bold text-tertiary">
                        "50M+"
                    </span>
                    <span class="text-label-md md:text-label-lg font-label-lg text-on-surface-variant font-medium tracking-wide uppercase">
                        {leptos_fluent::move_tr!("landing-total-glints")}
                    </span>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <Header />
        <main class="pt-20 text-left flex-1">
            <Hero />
            // For Streamers Section
            <section class="py-xl px-margin-mobile md:px-margin-desktop max-w-7xl mx-auto">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-lg items-center">
                    <div class="space-y-md">
                        <h2 class="text-headline-lg font-headline-lg text-secondary">
                            "For Streamers"
                        </h2>
                        <h3 class="text-headline-md font-headline-md text-on-surface">
                            "Accelerate your growth with tools built for speed."
                        </h3>
                        <ul class="space-y-base">
                            <li class="flex items-start gap-sm p-md rounded-xl bg-surface-container-low/40 backdrop-blur-md border border-white/5 hover:border-secondary/30 transition-colors">
                                <span
                                    class="material-symbols-outlined text-secondary"
                                    data-icon="bolt"
                                >
                                    "bolt"
                                </span>
                                <div>
                                    <p class="text-on-surface font-bold">"Real-time alerts"</p>
                                    <p class="text-on-surface-variant text-label-md">
                                        "Low-latency notifications that keep your community engaged instantly."
                                    </p>
                                </div>
                            </li>
                            <li class="flex items-start gap-sm p-md rounded-xl bg-surface-container-low/40 backdrop-blur-md border border-white/5 hover:border-secondary/30 transition-colors">
                                <span
                                    class="material-symbols-outlined text-secondary"
                                    data-icon="account_balance_wallet"
                                >
                                    "account_balance_wallet"
                                </span>
                                <div>
                                    <p class="text-on-surface font-bold">"Instant payouts"</p>
                                    <p class="text-on-surface-variant text-label-md">
                                        "No more waiting weeks. Your earnings are yours, immediately."
                                    </p>
                                </div>
                            </li>
                            <li class="flex items-start gap-sm p-md rounded-xl bg-surface-container-low/40 backdrop-blur-md border border-white/5 hover:border-secondary/30 transition-colors">
                                <span
                                    class="material-symbols-outlined text-secondary"
                                    data-icon="insights"
                                >
                                    "insights"
                                </span>
                                <div>
                                    <p class="text-on-surface font-bold">"Detailed analytics"</p>
                                    <p class="text-on-surface-variant text-label-md">
                                        "Deep dive into viewer behavior and contribution trends."
                                    </p>
                                </div>
                            </li>
                        </ul>
                    </div>
                    <div class="relative group">
                        <div class="absolute inset-0 bg-secondary/10 blur-3xl group-hover:bg-secondary/20 transition-all"></div>
                        <div class="relative bg-surface-container-highest/30 backdrop-blur-xl rounded-2xl border border-white/10 p-base overflow-hidden">
                            <img
                                alt="Streamer Dashboard"
                                class="rounded-xl w-full"
                                src="https://lh3.googleusercontent.com/aida-public/AB6AXuD0f6PmQqoeoKjRMaM9Rt_FTi6mYBDOuxzSzE6xLTkQ8pP8qR8z2hmpOdLQG0UMV7U1lH7UiZk35FgVwKKEv6pK7wYFwnpRE9VXdwzxAGitfXl8Q75e6DZhE6L1E_SUol1j8c8-AaKPPoVJiDFt_LwDu_q6SQrMvUXSUum3GsaMDmpyefq61E_KA0VG2WSA9mKS2kUg4bc6Y3FFeQO-Xd4HC_vUSTi_SxylwTLBZbAWTmayjnnNxkyr_bJ5JJdTidK42GHrlQREd-E"
                            />
                            <div class="absolute bottom-md left-md right-md p-md bg-background/80 backdrop-blur-md rounded-xl border border-secondary/30 flex justify-between items-center">
                                <div class="flex items-center gap-sm">
                                    <div class="w-10 h-10 rounded-full bg-secondary flex items-center justify-center">
                                        <span
                                            class="material-symbols-outlined text-on-secondary"
                                            data-icon="trending_up"
                                        >
                                            "trending_up"
                                        </span>
                                    </div>
                                    <span class="text-on-surface font-bold">
                                        "+24% revenue this week"
                                    </span>
                                </div>
                                <span class="text-secondary text-label-sm font-label-sm">
                                    "LIVE NOW"
                                </span>
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
                                    <span
                                        class="material-symbols-outlined text-primary text-headline-lg"
                                        data-icon="military_tech"
                                    >
                                        "military_tech"
                                    </span>
                                    <p class="text-on-surface font-bold mt-sm">"Loyalty Badges"</p>
                                    <p class="text-on-surface-variant text-label-sm">
                                        "Unlock exclusive status based on your support."
                                    </p>
                                </div>
                                <div class="bg-surface-container-highest/50 backdrop-blur-lg p-md rounded-2xl border border-white/10">
                                    <span
                                        class="material-symbols-outlined text-tertiary text-headline-lg"
                                        data-icon="forum"
                                    >
                                        "forum"
                                    </span>
                                    <p class="text-on-surface font-bold mt-sm">"Chat Highlights"</p>
                                    <p class="text-on-surface-variant text-label-sm">
                                        "Stand out with neon-bordered messages."
                                    </p>
                                </div>
                            </div>
                            <div class="space-y-md pt-lg">
                                <div class="bg-surface-container-highest/50 backdrop-blur-lg p-md rounded-2xl border border-white/10">
                                    <span
                                        class="material-symbols-outlined text-secondary text-headline-lg"
                                        data-icon="card_giftcard"
                                    >
                                        "card_giftcard"
                                    </span>
                                    <p class="text-on-surface font-bold mt-sm">
                                        "Personal Tributes"
                                    </p>
                                    <p class="text-on-surface-variant text-label-sm">
                                        "Send personalized gifts to your idols."
                                    </p>
                                </div>
                                <div class="bg-surface-container-highest/50 backdrop-blur-lg p-md rounded-2xl border border-white/10">
                                    <span
                                        class="material-symbols-outlined text-primary text-headline-lg"
                                        data-icon="verified_user"
                                    >
                                        "verified_user"
                                    </span>
                                    <p class="text-on-surface font-bold mt-sm">"Secure Vault"</p>
                                    <p class="text-on-surface-variant text-label-sm">
                                        "Your transactions are encrypted and private."
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="order-1 lg:order-2 space-y-md">
                        <h2 class="text-headline-lg font-headline-lg text-primary">"For Fans"</h2>
                        <h3 class="text-headline-md font-headline-md text-on-surface">
                            "Experience deeper connections with the creators you love."
                        </h3>
                        <p class="text-body-md text-on-surface-variant">
                            "Glint turns every transaction into a moment of interaction. Support isn't just a number; it's a personalized message, a badge of honor, and a direct impact on the content you enjoy."
                        </p>
                        <button class="px-lg py-md bg-surface-container-highest/40 backdrop-blur-md border border-primary/30 text-primary rounded-xl font-headline-md text-headline-md hover:bg-primary/10 transition-all flex items-center gap-sm">
                            "Find a Creator "
                            <span class="material-symbols-outlined" data-icon="chevron_right">
                                "chevron_right"
                            </span>
                        </button>
                    </div>
                </div>
            </section>

            // How it works
            <section class="py-xl px-margin-mobile md:px-margin-desktop max-w-7xl mx-auto text-center">
                <h2 class="text-headline-lg font-headline-lg text-on-surface mb-xl">
                    {leptos_fluent::move_tr!("landing-how-it-works")}
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-lg relative">
                    <div class="hidden md:block absolute top-1/2 left-0 w-full h-px bg-gradient-to-r from-transparent via-white/20 to-transparent -translate-y-1/2 z-0"></div>
                    <div class="relative z-10 group">
                        <div class="w-16 h-16 bg-surface-container-highest rounded-full flex items-center justify-center mx-auto border-2 border-primary neon-glow-primary mb-md group-hover:scale-110 transition-transform">
                            <span class="text-headline-md font-headline-md text-primary">"1"</span>
                        </div>
                        <h4 class="text-headline-md font-headline-md text-on-surface">
                            {leptos_fluent::move_tr!("how-step1-title")}
                        </h4>
                        <p class="text-on-surface-variant mt-sm">
                            {leptos_fluent::move_tr!("how-step1-desc")}
                        </p>
                    </div>
                    <div class="relative z-10 group">
                        <div class="w-16 h-16 bg-surface-container-highest rounded-full flex items-center justify-center mx-auto border-2 border-secondary neon-glow-secondary mb-md group-hover:scale-110 transition-transform">
                            <span class="text-headline-md font-headline-md text-secondary">
                                "2"
                            </span>
                        </div>
                        <h4 class="text-headline-md font-headline-md text-on-surface">
                            {leptos_fluent::move_tr!("how-step2-title")}
                        </h4>
                        <p class="text-on-surface-variant mt-sm">
                            {leptos_fluent::move_tr!("how-step2-desc")}
                        </p>
                    </div>
                    <div class="relative z-10 group">
                        <div class="w-16 h-16 bg-surface-container-highest rounded-full flex items-center justify-center mx-auto border-2 border-tertiary mb-md group-hover:scale-110 transition-transform">
                            <span class="text-headline-md font-headline-md text-tertiary">"3"</span>
                        </div>
                        <h4 class="text-headline-md font-headline-md text-on-surface">
                            {leptos_fluent::move_tr!("how-step3-title")}
                        </h4>
                        <p class="text-on-surface-variant mt-sm">
                            {leptos_fluent::move_tr!("how-step3-desc")}
                        </p>
                    </div>
                </div>
            </section>

            // CTA Section
            <section class="py-xl px-margin-mobile md:px-margin-desktop text-center">
                <div class="max-w-4xl mx-auto bg-gradient-to-br from-primary/10 to-secondary/10 backdrop-blur-xl p-xl rounded-[2rem] border border-white/10">
                    <h2 class="text-headline-lg md:text-headline-xl font-headline-xl text-on-surface mb-md">
                        {leptos_fluent::move_tr!("cta-title")}
                    </h2>
                    <p class="text-body-lg text-on-surface-variant mb-lg">
                        {leptos_fluent::move_tr!("cta-subtitle")}
                    </p>
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
        <Footer />

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
