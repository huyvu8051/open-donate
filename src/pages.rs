//! Static content pages: About, FAQ, Privacy Policy, Terms of Service.

use leptos::prelude::*;
use crate::app::{Header, Footer};

// ─── Shared helpers ───────────────────────────────────────────────────────────

#[component]
fn InfoCard(
    icon: &'static str,
    icon_color: &'static str,
    title: String,
    body: String,
) -> impl IntoView {
    view! {
        <div class="bg-surface-container-low/40 backdrop-blur-md border border-white/10 rounded-2xl p-xl flex flex-col gap-md hover:border-white/20 transition-colors">
            <div class=format!("w-12 h-12 rounded-xl {} flex items-center justify-center", icon_color)>
                <span class=format!("material-symbols-outlined {}", icon_color.replace("/20", ""))>{icon}</span>
            </div>
            <h2 class="text-headline-md font-headline-md text-on-surface">{title}</h2>
            <p class="text-body-md text-on-surface-variant leading-relaxed">{body}</p>
        </div>
    }
}

// ─── About Page ──────────────────────────────────────────────────────────────

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="bg-background text-on-surface font-body-md antialiased min-h-screen flex flex-col">
            <Header/>
            <main class="flex-1 pt-32 pb-24">
                // Hero
                <section class="relative py-xl px-margin-mobile md:px-margin-desktop text-center overflow-hidden">
                    <div class="absolute top-0 left-1/4 w-96 h-96 bg-primary/10 rounded-full blur-[120px] pointer-events-none"></div>
                    <div class="absolute bottom-0 right-1/4 w-80 h-80 bg-secondary/10 rounded-full blur-[100px] pointer-events-none"></div>
                    <div class="relative max-w-3xl mx-auto">
                        <span class="inline-block px-sm py-xs bg-surface-container-highest/40 backdrop-blur-md rounded-full border border-white/10 text-secondary text-label-md font-label-md mb-md">
                            {leptos_fluent::move_tr!("about-subtitle")}
                        </span>
                        <h1 class="text-display-lg font-display-lg text-on-surface mb-md">
                            {leptos_fluent::move_tr!("about-title")}
                        </h1>
                    </div>
                </section>

                // Cards
                <section class="max-w-5xl mx-auto px-margin-mobile md:px-margin-desktop grid grid-cols-1 md:grid-cols-2 gap-lg mt-xl">
                    <div class="bg-surface-container-low/40 backdrop-blur-md border border-white/10 rounded-2xl p-xl flex flex-col gap-md hover:border-primary/30 transition-colors">
                        <div class="w-12 h-12 rounded-xl bg-primary/20 flex items-center justify-center">
                            <span class="material-symbols-outlined text-primary">"rocket_launch"</span>
                        </div>
                        <h2 class="text-headline-md font-headline-md text-on-surface">
                            {leptos_fluent::move_tr!("about-mission-title")}
                        </h2>
                        <p class="text-body-md text-on-surface-variant leading-relaxed">
                            {leptos_fluent::move_tr!("about-mission-body")}
                        </p>
                    </div>

                    <div class="bg-surface-container-low/40 backdrop-blur-md border border-white/10 rounded-2xl p-xl flex flex-col gap-md hover:border-secondary/30 transition-colors">
                        <div class="w-12 h-12 rounded-xl bg-secondary/20 flex items-center justify-center">
                            <span class="material-symbols-outlined text-secondary">"visibility"</span>
                        </div>
                        <h2 class="text-headline-md font-headline-md text-on-surface">
                            {leptos_fluent::move_tr!("about-vision-title")}
                        </h2>
                        <p class="text-body-md text-on-surface-variant leading-relaxed">
                            {leptos_fluent::move_tr!("about-vision-body")}
                        </p>
                    </div>

                    <div class="md:col-span-2 bg-gradient-to-br from-primary/5 to-secondary/5 backdrop-blur-md border border-white/10 rounded-2xl p-xl flex flex-col gap-md hover:border-tertiary/30 transition-colors">
                        <div class="w-12 h-12 rounded-xl bg-tertiary/20 flex items-center justify-center">
                            <span class="material-symbols-outlined text-tertiary">"groups"</span>
                        </div>
                        <h2 class="text-headline-md font-headline-md text-on-surface">
                            {leptos_fluent::move_tr!("about-team-title")}
                        </h2>
                        <p class="text-body-md text-on-surface-variant leading-relaxed max-w-2xl">
                            {leptos_fluent::move_tr!("about-team-body")}
                        </p>
                    </div>
                </section>

                // CTA
                <section class="max-w-2xl mx-auto px-margin-mobile md:px-margin-desktop mt-xl text-center">
                    <a
                        href="/api/auth/login"
                        class="inline-flex items-center gap-sm px-xl py-md bg-primary text-on-primary rounded-xl font-bold text-headline-md hover:brightness-110 active:scale-[0.98] transition-all shadow-lg shadow-primary/30"
                    >
                        <span class="material-symbols-outlined">"bolt"</span>
                        {leptos_fluent::move_tr!("about-cta")}
                    </a>
                </section>
            </main>
            <Footer/>
        </div>
    }
}

// ─── FAQ Item ─────────────────────────────────────────────────────────────────

#[component]
fn FaqItem(question: String, answer: String) -> impl IntoView {
    let (open, set_open) = signal(false);
    view! {
        <div class="bg-surface-container-low/40 backdrop-blur-md border border-white/10 rounded-2xl overflow-hidden hover:border-white/20 transition-colors">
            <button
                class="w-full flex items-center justify-between p-lg text-left gap-md"
                on:click=move |_| set_open.update(|v| *v = !*v)
            >
                <span class="text-headline-sm font-headline-sm text-on-surface">{question}</span>
                <span class=move || format!(
                    "material-symbols-outlined text-primary transition-transform duration-300 flex-shrink-0 {}",
                    if open.get() { "rotate-180" } else { "" }
                )>"expand_more"</span>
            </button>
            <div class=move || format!(
                "overflow-hidden transition-all duration-300 {}",
                if open.get() { "max-h-96 pb-lg px-lg" } else { "max-h-0" }
            )>
                <p class="text-body-md text-on-surface-variant leading-relaxed">{answer}</p>
            </div>
        </div>
    }
}

// ─── FAQ Page ─────────────────────────────────────────────────────────────────

#[component]
pub fn FaqPage() -> impl IntoView {
    view! {
        <div class="bg-background text-on-surface font-body-md antialiased min-h-screen flex flex-col">
            <Header/>
            <main class="flex-1 pt-32 pb-24">
                // Hero
                <section class="relative py-xl px-margin-mobile md:px-margin-desktop text-center overflow-hidden">
                    <div class="absolute top-0 right-1/4 w-96 h-96 bg-secondary/10 rounded-full blur-[120px] pointer-events-none"></div>
                    <div class="relative max-w-3xl mx-auto">
                        <h1 class="text-display-md font-display-md text-on-surface mb-sm">
                            {leptos_fluent::move_tr!("faq-title")}
                        </h1>
                        <p class="text-body-lg text-on-surface-variant">
                            {leptos_fluent::move_tr!("faq-subtitle")}
                        </p>
                    </div>
                </section>

                // FAQ accordion
                <section class="max-w-3xl mx-auto px-margin-mobile md:px-margin-desktop mt-xl flex flex-col gap-md">
                    <FaqItem
                        question=leptos_fluent::tr!("faq-q1")
                        answer=leptos_fluent::tr!("faq-a1")
                    />
                    <FaqItem
                        question=leptos_fluent::tr!("faq-q2")
                        answer=leptos_fluent::tr!("faq-a2")
                    />
                    <FaqItem
                        question=leptos_fluent::tr!("faq-q3")
                        answer=leptos_fluent::tr!("faq-a3")
                    />
                    <FaqItem
                        question=leptos_fluent::tr!("faq-q4")
                        answer=leptos_fluent::tr!("faq-a4")
                    />
                    <FaqItem
                        question=leptos_fluent::tr!("faq-q5")
                        answer=leptos_fluent::tr!("faq-a5")
                    />
                </section>

                // Contact CTA
                <section class="max-w-3xl mx-auto px-margin-mobile md:px-margin-desktop mt-xl">
                    <div class="bg-gradient-to-br from-primary/10 to-secondary/10 border border-white/10 rounded-2xl p-xl text-center">
                        <p class="text-headline-sm font-headline-sm text-on-surface mb-md">
                            {leptos_fluent::move_tr!("faq-contact")}
                        </p>
                        <a
                            href="mailto:support@glint.app"
                            class="inline-flex items-center gap-xs px-lg py-sm bg-primary text-on-primary rounded-xl font-bold hover:brightness-110 transition-all"
                        >
                            <span class="material-symbols-outlined text-[18px]">"mail"</span>
                            {leptos_fluent::move_tr!("faq-contact-link")}
                        </a>
                    </div>
                </section>
            </main>
            <Footer/>
        </div>
    }
}

// ─── Privacy Page ─────────────────────────────────────────────────────────────

#[component]
pub fn PrivacyPage() -> impl IntoView {
    view! {
        <div class="bg-background text-on-surface font-body-md antialiased min-h-screen flex flex-col">
            <Header/>
            <main class="flex-1 pt-32 pb-24">
                <article class="max-w-3xl mx-auto px-margin-mobile md:px-margin-desktop">
                    <div class="mb-xl border-b border-white/10 pb-lg">
                        <h1 class="text-display-md font-display-md text-on-surface mb-xs">
                            {leptos_fluent::move_tr!("privacy-title")}
                        </h1>
                        <p class="text-label-md text-on-surface-variant">
                            {leptos_fluent::move_tr!("privacy-subtitle")}
                        </p>
                    </div>

                    <p class="text-body-lg text-on-surface-variant leading-relaxed mb-xl">
                        {leptos_fluent::move_tr!("privacy-intro")}
                    </p>

                    <div class="flex flex-col gap-lg">
                        <div class="flex gap-md">
                            <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center flex-shrink-0 mt-1">
                                <span class="material-symbols-outlined text-primary text-[20px]">"database"</span>
                            </div>
                            <div class="flex flex-col gap-xs">
                                <h2 class="text-headline-sm font-headline-sm text-on-surface">{leptos_fluent::move_tr!("privacy-collect-title")}</h2>
                                <p class="text-body-md text-on-surface-variant leading-relaxed">{leptos_fluent::move_tr!("privacy-collect-body")}</p>
                            </div>
                        </div>
                        <div class="flex gap-md">
                            <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center flex-shrink-0 mt-1">
                                <span class="material-symbols-outlined text-primary text-[20px]">"manage_accounts"</span>
                            </div>
                            <div class="flex flex-col gap-xs">
                                <h2 class="text-headline-sm font-headline-sm text-on-surface">{leptos_fluent::move_tr!("privacy-use-title")}</h2>
                                <p class="text-body-md text-on-surface-variant leading-relaxed">{leptos_fluent::move_tr!("privacy-use-body")}</p>
                            </div>
                        </div>
                        <div class="flex gap-md">
                            <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center flex-shrink-0 mt-1">
                                <span class="material-symbols-outlined text-primary text-[20px]">"lock"</span>
                            </div>
                            <div class="flex flex-col gap-xs">
                                <h2 class="text-headline-sm font-headline-sm text-on-surface">{leptos_fluent::move_tr!("privacy-security-title")}</h2>
                                <p class="text-body-md text-on-surface-variant leading-relaxed">{leptos_fluent::move_tr!("privacy-security-body")}</p>
                            </div>
                        </div>
                        <div class="flex gap-md">
                            <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center flex-shrink-0 mt-1">
                                <span class="material-symbols-outlined text-primary text-[20px]">"verified_user"</span>
                            </div>
                            <div class="flex flex-col gap-xs">
                                <h2 class="text-headline-sm font-headline-sm text-on-surface">{leptos_fluent::move_tr!("privacy-rights-title")}</h2>
                                <p class="text-body-md text-on-surface-variant leading-relaxed">{leptos_fluent::move_tr!("privacy-rights-body")}</p>
                            </div>
                        </div>
                    </div>
                </article>
            </main>
            <Footer/>
        </div>
    }
}

// ─── Terms Page ───────────────────────────────────────────────────────────────

#[component]
pub fn TermsPage() -> impl IntoView {
    view! {
        <div class="bg-background text-on-surface font-body-md antialiased min-h-screen flex flex-col">
            <Header/>
            <main class="flex-1 pt-32 pb-24">
                <article class="max-w-3xl mx-auto px-margin-mobile md:px-margin-desktop">
                    <div class="mb-xl border-b border-white/10 pb-lg">
                        <h1 class="text-display-md font-display-md text-on-surface mb-xs">
                            {leptos_fluent::move_tr!("terms-title")}
                        </h1>
                        <p class="text-label-md text-on-surface-variant">
                            {leptos_fluent::move_tr!("terms-subtitle")}
                        </p>
                    </div>

                    <p class="text-body-lg text-on-surface-variant leading-relaxed mb-xl">
                        {leptos_fluent::move_tr!("terms-intro")}
                    </p>

                    <div class="flex flex-col gap-md">
                        <div class="bg-surface-container-low/30 border border-white/5 rounded-2xl p-lg flex gap-md hover:border-white/15 transition-colors">
                            <div class="w-10 h-10 rounded-xl bg-tertiary/10 flex items-center justify-center flex-shrink-0 mt-1">
                                <span class="material-symbols-outlined text-tertiary text-[20px]">"gavel"</span>
                            </div>
                            <div class="flex flex-col gap-xs flex-1">
                                <div class="flex items-center gap-sm">
                                    <span class="text-label-sm font-bold text-tertiary/70">"01"</span>
                                    <h2 class="text-headline-sm font-headline-sm text-on-surface">{leptos_fluent::move_tr!("terms-use-title")}</h2>
                                </div>
                                <p class="text-body-md text-on-surface-variant leading-relaxed">{leptos_fluent::move_tr!("terms-use-body")}</p>
                            </div>
                        </div>
                        <div class="bg-surface-container-low/30 border border-white/5 rounded-2xl p-lg flex gap-md hover:border-white/15 transition-colors">
                            <div class="w-10 h-10 rounded-xl bg-tertiary/10 flex items-center justify-center flex-shrink-0 mt-1">
                                <span class="material-symbols-outlined text-tertiary text-[20px]">"article"</span>
                            </div>
                            <div class="flex flex-col gap-xs flex-1">
                                <div class="flex items-center gap-sm">
                                    <span class="text-label-sm font-bold text-tertiary/70">"02"</span>
                                    <h2 class="text-headline-sm font-headline-sm text-on-surface">{leptos_fluent::move_tr!("terms-content-title")}</h2>
                                </div>
                                <p class="text-body-md text-on-surface-variant leading-relaxed">{leptos_fluent::move_tr!("terms-content-body")}</p>
                            </div>
                        </div>
                        <div class="bg-surface-container-low/30 border border-white/5 rounded-2xl p-lg flex gap-md hover:border-white/15 transition-colors">
                            <div class="w-10 h-10 rounded-xl bg-tertiary/10 flex items-center justify-center flex-shrink-0 mt-1">
                                <span class="material-symbols-outlined text-tertiary text-[20px]">"payments"</span>
                            </div>
                            <div class="flex flex-col gap-xs flex-1">
                                <div class="flex items-center gap-sm">
                                    <span class="text-label-sm font-bold text-tertiary/70">"03"</span>
                                    <h2 class="text-headline-sm font-headline-sm text-on-surface">{leptos_fluent::move_tr!("terms-payments-title")}</h2>
                                </div>
                                <p class="text-body-md text-on-surface-variant leading-relaxed">{leptos_fluent::move_tr!("terms-payments-body")}</p>
                            </div>
                        </div>
                        <div class="bg-surface-container-low/30 border border-white/5 rounded-2xl p-lg flex gap-md hover:border-white/15 transition-colors">
                            <div class="w-10 h-10 rounded-xl bg-tertiary/10 flex items-center justify-center flex-shrink-0 mt-1">
                                <span class="material-symbols-outlined text-tertiary text-[20px]">"shield"</span>
                            </div>
                            <div class="flex flex-col gap-xs flex-1">
                                <div class="flex items-center gap-sm">
                                    <span class="text-label-sm font-bold text-tertiary/70">"04"</span>
                                    <h2 class="text-headline-sm font-headline-sm text-on-surface">{leptos_fluent::move_tr!("terms-liability-title")}</h2>
                                </div>
                                <p class="text-body-md text-on-surface-variant leading-relaxed">{leptos_fluent::move_tr!("terms-liability-body")}</p>
                            </div>
                        </div>
                        <div class="bg-surface-container-low/30 border border-white/5 rounded-2xl p-lg flex gap-md hover:border-white/15 transition-colors">
                            <div class="w-10 h-10 rounded-xl bg-tertiary/10 flex items-center justify-center flex-shrink-0 mt-1">
                                <span class="material-symbols-outlined text-tertiary text-[20px]">"edit_note"</span>
                            </div>
                            <div class="flex flex-col gap-xs flex-1">
                                <div class="flex items-center gap-sm">
                                    <span class="text-label-sm font-bold text-tertiary/70">"05"</span>
                                    <h2 class="text-headline-sm font-headline-sm text-on-surface">{leptos_fluent::move_tr!("terms-changes-title")}</h2>
                                </div>
                                <p class="text-body-md text-on-surface-variant leading-relaxed">{leptos_fluent::move_tr!("terms-changes-body")}</p>
                            </div>
                        </div>
                    </div>
                </article>
            </main>
            <Footer/>
        </div>
    }
}
