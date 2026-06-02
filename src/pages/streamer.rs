use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use leptos::prelude::*;
use crate::components::layout::{Header, Footer};
use crate::db::{TransactionStatus, PaymentMethod, DbStreamer, DbTransaction};
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
pub struct StreamerAnalytics {
    pub total_revenue: f64,
    pub donation_count: i64,
    pub avg_donation: f64,
    pub top_single_donation: f64,
    pub top_donors: Vec<(String, f64)>,
    pub revenue_over_time: Vec<(String, f64)>,
    pub payment_method_breakdown: Vec<(String, i64)>,
    pub amount_distribution: Vec<(String, i64)>,
    pub cumulative_revenue: Vec<(String, f64)>,
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
    let bio_expanded = RwSignal::new(false);

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
        |uname| async move { crate::utils::with_min_delay(get_streamer(uname)).await },
    );

    // Load recent transactions (SSR-friendly)
    let transactions_resource = Resource::new(
        move || (username(), transactions_trigger.get()),
        |(uname, _)| async move {
            if uname.is_empty() {
                return Ok(vec![]);
            }
            crate::utils::with_min_delay(get_recent_transactions(uname)).await
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
        <Suspense fallback=move || {
            view! { <Title text="Glint | Donate" /> }
        }>
            {move || {
                match streamer_resource.get() {
                    Some(Ok(Some(s))) => {
                        let display = if s.display_name.is_empty() {
                            s.username
                        } else {
                            s.display_name
                        };
                        let title = format!("Donate to {display} | Glint");
                        view! { <Title text=title /> }.into_any()
                    }
                    _ => view! { <Title text="Glint | Donate" /> }.into_any(),
                }
            }}
        </Suspense>
        <Header />
        <main class="pt-24 pb-xl px-margin-mobile md:px-margin-desktop max-w-5xl mx-auto flex-1 w-full flex flex-col gap-md animate-fade-in relative">
            <div
                aria-hidden="true"
                class="pointer-events-none absolute -top-24 -left-24 h-72 w-72 rounded-full bg-gradient-to-br from-primary/30 via-secondary/10 to-transparent blur-3xl"
            ></div>
            <div
                aria-hidden="true"
                class="pointer-events-none absolute -top-16 -right-20 h-72 w-72 rounded-full bg-gradient-to-br from-secondary/25 via-primary/10 to-transparent blur-3xl"
            ></div>
            // Streamer Profile Header
            <Suspense fallback=move || {
                view! {
                    <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl p-md mb-md relative overflow-hidden flex flex-col md:flex-row gap-md items-center md:items-center min-h-[140px] animate-pulse shadow-[0_20px_60px_rgba(0,0,0,0.25)] ring-1 ring-white/5">
                        <div class="w-24 h-24 md:w-28 md:h-28 rounded-2xl bg-white/5 border border-white/10"></div>
                        <div class="flex-1 flex flex-col gap-xs text-center md:text-left">
                            <div class="h-8 w-48 bg-white/5 rounded mx-auto md:mx-0"></div>
                            <div class="flex flex-col gap-2 mt-xs min-h-[3rem]">
                                <div class="h-4 w-full bg-white/5 rounded"></div>
                                <div class="h-4 w-5/6 bg-white/5 rounded mx-auto md:mx-0"></div>
                            </div>
                        </div>
                    </section>
                }
            }>
                {move || {
                    streamer_resource
                        .get()
                        .map(|res| {
                            match res {
                                Ok(Some(streamer)) => {
                                    let avatar = streamer.avatar_url.clone();
                                    let name = streamer.display_name.clone();
                                    let bio = streamer.bio.clone();
                                    let is_live = streamer.is_live;
                                    view! {
                                        <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl p-md mb-md relative overflow-hidden flex flex-col md:flex-row gap-md items-center md:items-center shadow-[0_20px_60px_rgba(0,0,0,0.25)] ring-1 ring-white/5">
                                            <div class="absolute inset-0 opacity-70 bg-gradient-to-br from-white/12 via-transparent to-transparent"></div>
                                            <div class="relative w-24 h-24 md:w-28 md:h-28 shrink-0">
                                                <div class="w-full h-full rounded-2xl overflow-hidden border border-white/15 shadow-[0_0_0_6px_rgba(255,255,255,0.03)]">
                                                    <img
                                                        alt="Streamer Avatar"
                                                        class="w-full h-full object-cover"
                                                        src=avatar
                                                    />
                                                </div>
                                                {if is_live {
                                                    view! {
                                                        <div class="absolute -bottom-1 -right-1 z-20 bg-error text-on-error text-[10px] font-bold px-2 py-0.5 rounded-full border-2 border-surface animate-pulse shadow-lg">
                                                            "LIVE"
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! {}.into_any()
                                                }}
                                            </div>
                                            <div class="flex-1 flex flex-col gap-xs text-center md:text-left relative z-10">
                                                <div class="flex items-center justify-center md:justify-start gap-sm">
                                                    <h1 class="text-headline-md md:text-headline-lg font-headline-lg text-on-surface">
                                                        {name}
                                                    </h1>
                                                </div>
                                                <div class="mt-xs">
                                                    {if bio.len() > 100 {
                                                        view! {
                                                            <div class="flex flex-col gap-1 items-center md:items-start">
                                                                <p class=move || if bio_expanded.get() {
                                                                    "text-on-surface-variant text-body-sm md:text-body-md font-body-md leading-snug max-w-prose mx-auto md:mx-0 min-h-[3rem]".to_string()
                                                                } else {
                                                                    "text-on-surface-variant text-body-sm md:text-body-md font-body-md leading-snug max-w-prose mx-auto md:mx-0 line-clamp-2 min-h-[3rem]".to_string()
                                                                }>
                                                                    {bio.clone()}
                                                                </p>
                                                                <button
                                                                    class="text-primary text-label-sm font-bold hover:underline"
                                                                    on:click=move |_| bio_expanded.update(|b| *b = !*b)
                                                                >
                                                                    {move || if bio_expanded.get() { "Show less" } else { "Read more" }}
                                                                </button>
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        view! {
                                                            <p class="text-on-surface-variant text-body-sm md:text-body-md font-body-md leading-snug max-w-prose mx-auto md:mx-0 line-clamp-2 min-h-[3rem]">
                                                                {bio.clone()}
                                                            </p>
                                                        }.into_any()
                                                    }}
                                                </div>
                                            </div>
                                        </section>
                                    }
                                        .into_any()
                                }
                                _ => {
                                    view! {
                                        <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl p-md mb-md text-center text-on-surface-variant">
                                            "Streamer profile not found in database."
                                        </section>
                                    }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Suspense>

            <div class="flex flex-col gap-md relative z-10">
                // Donation Form
                <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl p-md md:p-lg flex flex-col gap-md shadow-[0_20px_60px_rgba(0,0,0,0.25)] ring-1 ring-white/5">
                    <div class="flex items-center gap-sm">
                        <span
                            class="material-symbols-outlined text-secondary"
                            data-icon="volunteer_activism"
                        >
                            "volunteer_activism"
                        </span>
                        <h2 class="text-headline-sm md:text-headline-md font-headline-md text-on-surface">
                            {leptos_fluent::move_tr!("donate-send-a-glint")}
                        </h2>
                    </div>

                    // Your Name Input
                    <div class="flex flex-col gap-xs">
                        <label class="text-label-md font-label-md text-on-surface-variant">
                            {leptos_fluent::move_tr!("donate-your-name")}
                        </label>
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
                        <label class="text-label-md font-label-md text-on-surface-variant">
                            {leptos_fluent::move_tr!("donate-amount")}
                        </label>
                        <div class="grid grid-cols-2 md:grid-cols-4 gap-sm">
                            {
                                let amounts = vec!["5", "10", "25", "50"];
                                amounts
                                    .into_iter()
                                    .map(|amt| {
                                        let amt_clone = amt.to_string();
                                        view! {
                                            <button
                                                class=move || {
                                                    if amount.get() == amt_clone {
                                                        "bg-white/5 backdrop-blur-md border-2 px-md py-sm rounded-xl text-body-md font-semibold text-on-surface transition-all border-secondary shadow-[inset_0_0_15px_rgba(77,224,130,0.12)] hover:-translate-y-[1px]"
                                                            .to_string()
                                                    } else {
                                                        "bg-white/5 backdrop-blur-md border border-white/10 px-md py-sm rounded-xl text-body-md font-semibold text-on-surface hover:border-secondary transition-all hover:-translate-y-[1px]"
                                                            .to_string()
                                                    }
                                                }
                                                on:click=move |_| amount.set(amt.to_string())
                                            >
                                                "$"
                                                {amt}
                                            </button>
                                        }
                                    })
                                    .collect_view()
                            }
                        </div>
                        <div class="relative mt-sm">
                            <span class="absolute left-md top-1/2 -translate-y-1/2 text-on-surface-variant">
                                "$"
                            </span>
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
                        <label class="text-label-md font-label-md text-on-surface-variant">
                            {leptos_fluent::move_tr!("donate-message")}
                        </label>
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
                        <label class="text-label-md font-label-md text-on-surface-variant">
                            {leptos_fluent::move_tr!("donate-payment-method")}
                        </label>
                        <div class="grid grid-cols-2 sm:grid-cols-3 gap-sm">
                            <Suspense fallback=move || {
                                view! {
                                    <span class="text-on-surface-variant">
                                        "Loading methods..."
                                    </span>
                                }
                            }>
                                {move || {
                                    match streamer_resource.get() {
                                        Some(Ok(Some(s))) => {
                                            let methods = s.payment_methods.clone();
                                            if methods.is_empty() {
                                                view! {
                                                    <span class="text-on-surface-variant col-span-3">
                                                        "No payment methods available."
                                                    </span>
                                                }
                                                    .into_any()
                                            } else {
                                                methods
                                                    .into_iter()
                                                    .map(|pm| {
                                                        let icon = if pm == PaymentMethod::MockAuto {
                                                            "autorenew"
                                                        } else {
                                                            "pan_tool"
                                                        };
                                                        let pm_clone = pm.clone();
                                                        let pm_clone2 = pm.clone();
                                                        view! {
                                                            <label
                                                                class="cursor-pointer"
                                                                on:click=move |_| payment_method.set(pm_clone.clone())
                                                            >
                                                                <input
                                                                    checked=move || payment_method.get() == pm_clone2
                                                                    class="hidden peer"
                                                                    name="payment"
                                                                    type="radio"
                                                                />
                                                                <div class="flex items-center justify-center gap-xs bg-white/5 backdrop-blur-md px-sm py-sm rounded-xl text-center border border-white/10 peer-checked:border-primary peer-checked:bg-primary/5 transition-all hover:-translate-y-[1px] hover:border-white/20">
                                                                    <span class="material-symbols-outlined text-primary text-[18px]">
                                                                        {icon}
                                                                    </span>
                                                                    <span class="text-label-sm font-label-sm">
                                                                        {pm.to_string()}
                                                                    </span>
                                                                </div>
                                                            </label>
                                                        }
                                                    })
                                                    .collect_view()
                                                    .into_any()
                                            }
                                        }
                                        _ => view! {}.into_any(),
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
                        form_error
                            .get()
                            .map(|err| {
                                view! {
                                    <div class="mt-sm bg-error-container/20 border border-error/30 text-error rounded-xl px-md py-sm text-body-sm">
                                        {err}
                                    </div>
                                }
                            })
                    }}

                    {move || {
                        if show_payment_window.get() {
                            let status = mock_tx_status.get();
                            view! {
                                <div class="mt-sm bg-white/5 backdrop-blur-md border border-white/10 rounded-xl p-md flex flex-col gap-base">
                                    <div class="flex items-center justify-between">
                                        <div class="text-on-surface font-semibold">
                                            {leptos_fluent::move_tr!("streamer-mock-payment-required")}
                                        </div>
                                        <div class="text-label-sm text-on-surface-variant">
                                            {leptos_fluent::move_tr!("streamer-status-label")} " "
                                            <span class="text-on-surface">{status.to_string()}</span>
                                        </div>
                                    </div>

                                    {move || {
                                        if let Some(qr) = mock_display_qr.get() {
                                            view! {
                                                <div class="flex flex-col gap-xs">
                                                    <div class="text-label-sm text-on-surface-variant">
                                                        "QR (mock)"
                                                    </div>
                                                    <pre class="bg-surface-container-low/40 border border-white/10 rounded-xl p-sm text-body-sm overflow-x-auto text-on-surface">
                                                        {qr}
                                                    </pre>
                                                </div>
                                            }
                                                .into_any()
                                        } else if let Some(url) = mock_display_url.get() {
                                            let href = url.clone();
                                            view! {
                                                <div class="flex flex-col gap-xs">
                                                    <div class="text-label-sm text-on-surface-variant">
                                                        "Payment link (mock)"
                                                    </div>
                                                    <a
                                                        class="text-primary underline break-all"
                                                        href=href
                                                        target="_blank"
                                                        rel="noopener noreferrer"
                                                    >
                                                        {url}
                                                    </a>
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            view! {}.into_any()
                                        }
                                    }}

                                    <div class="flex flex-col gap-xs">
                                        <label class="text-label-sm text-on-surface-variant">
                                            "OTP (mock)"
                                        </label>
                                        <input
                                            data-testid="mock-otp-input"
                                            class="w-full bg-surface-container-low/40 border border-white/10 rounded-xl px-md py-sm text-body-md focus:outline-none focus:border-primary transition-all text-on-surface"
                                            placeholder="Enter OTP (any value)"
                                            type="text"
                                            prop:value=move || otp.get()
                                            on:input=move |ev| otp.set(event_target_value(&ev))
                                        />
                                    </div>

                                    {move || {
                                        if status == TransactionStatus::ReadyForDisplay {
                                            view! {
                                                <div
                                                    data-testid="payment-success-msg"
                                                    class="bg-secondary/10 border border-secondary/20 text-secondary rounded-xl px-md py-sm"
                                                >
                                                    {leptos_fluent::move_tr!("streamer-payment-success")}
                                                </div>
                                            }
                                                .into_any()
                                        } else if status == TransactionStatus::Rejected {
                                            view! {
                                                <div class="bg-error/10 border border-error/20 text-error rounded-xl px-md py-sm">
                                                    {leptos_fluent::move_tr!("streamer-payment-failed")}
                                                </div>
                                            }
                                                .into_any()
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
                                                        >
                                                            {leptos_fluent::move_tr!("streamer-btn-accept")}
                                                        </button>
                                                        <button
                                                            class="flex-1 bg-error text-on-error py-sm rounded-lg font-bold hover:brightness-110"
                                                            on:click=move |_| {
                                                                if let Some(id) = mock_tx_id.get() {
                                                                    reject_action.dispatch(id);
                                                                    mock_tx_status.set(TransactionStatus::Rejected);
                                                                }
                                                            }
                                                        >
                                                            {leptos_fluent::move_tr!("streamer-btn-reject")}
                                                        </button>
                                                    </div>
                                                }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <div class="text-on-surface-variant text-body-sm">
                                                        {leptos_fluent::move_tr!("streamer-waiting-payment")}
                                                    </div>
                                                }
                                                    .into_any()
                                            }
                                        }
                                    }}
                                </div>
                            }
                                .into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }}
                    <p class="text-center text-label-sm font-label-sm text-on-surface-variant">
                        {leptos_fluent::move_tr!("streamer-glint-matches")}
                    </p>
                </section>

                // Recent Tributes Section
                <section
                    data-testid="recent-tributes-section"
                    class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl p-md md:p-lg flex flex-col gap-md shadow-[0_20px_60px_rgba(0,0,0,0.25)] ring-1 ring-white/5"
                >
                    <div class="flex items-center gap-sm">
                        <span class="material-symbols-outlined text-primary" data-icon="history">
                            "history"
                        </span>
                        <h2 class="text-headline-sm md:text-headline-md font-headline-md text-on-surface">
                            {leptos_fluent::move_tr!("streamer-recent-tributes")}
                        </h2>
                    </div>

                    <Suspense fallback=move || {
                        view! {
                            <div class="text-on-surface-variant">
                                {leptos_fluent::move_tr!("streamer-loading-tributes")}
                            </div>
                        }
                    }>
                        {move || {
                            transactions_resource
                                .get()
                                .map(|res| {
                                    match res {
                                        Ok(txs) => {
                                            if txs.is_empty() {
                                                view! {
                                                    <p class="text-on-surface-variant text-center py-md">
                                                        {leptos_fluent::move_tr!("streamer-no-tributes")}
                                                    </p>
                                                }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <div class="flex flex-col gap-sm">
                                                        {txs
                                                            .into_iter()
                                                            .map(|tx| {
                                                                let msg = tx.message.clone();
                                                                view! {
                                                                    <div class="bg-white/5 border border-white/10 rounded-xl p-sm flex flex-col gap-xs transition-all hover:bg-white/10 hover:border-white/20">
                                                                        <div class="flex items-start gap-sm">
                                                                            <div class="shrink-0 w-9 h-9 rounded-full bg-gradient-to-br from-primary/25 to-secondary/15 border border-white/10 flex items-center justify-center">
                                                                                <span class="material-symbols-outlined text-primary text-[18px]">
                                                                                    "person"
                                                                                </span>
                                                                            </div>
                                                                            <div class="flex-1 min-w-0 flex flex-col gap-[2px]">
                                                                                <div class="flex items-center justify-between gap-sm">
                                                                                    <span class="text-on-surface font-semibold text-body-md truncate">
                                                                                        {tx.donor_name.clone()}
                                                                                    </span>
                                                                                    <span class="text-secondary font-bold text-body-lg shrink-0">
                                                                                        "$" {format!("{:.2}", tx.amount)}
                                                                                    </span>
                                                                                </div>
                                                                                <div class="flex items-center justify-between gap-sm text-label-sm text-on-surface-variant/80">
                                                                                    <span class="truncate">
                                                                                        {leptos_fluent::move_tr!("streamer-via")} " "
                                                                                        {tx.payment_method.to_string()}
                                                                                    </span>
                                                                                    <span class="shrink-0">{tx.created_at.clone()}</span>
                                                                                </div>
                                                                            </div>
                                                                        </div>
                                                                        {if let Some(msg_str) = msg {
                                                                            view! {
                                                                                <p class="text-on-surface-variant text-body-sm italic bg-surface-container-low/40 border border-white/5 rounded-lg p-sm mt-xs">
                                                                                    "\"" {msg_str} "\""
                                                                                </p>
                                                                            }
                                                                                .into_any()
                                                                        } else {
                                                                            view! {}.into_any()
                                                                        }}
                                                                    </div>
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
                                                <div class="text-on-surface-variant text-center">
                                                    {leptos_fluent::move_tr!("streamer-tributes-failed")}
                                                </div>
                                            }
                                                .into_any()
                                        }
                                    }
                                })
                        }}
                    </Suspense>
                </section>
            </div>
        </main>
        <Footer />
    }

}
#[server(GetStreamer, "/api")]
pub async fn get_streamer(username: String) -> Result<Option<DbStreamer>, ServerFnError> {
    let axum::Extension(pool) = leptos_axum::extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|e| ServerFnError::new(format!("Failed to extract DB pool: {:?}", e)))?;

    let row = sqlx::query(
        "SELECT id, username, display_name, avatar_url, bio, (last_online_ping IS NOT NULL AND last_online_ping >= NOW() - INTERVAL '300 seconds') AS is_live, user_id, overlay_token, active_overlay_session, payment_methods, overlay_paused, overlay_sound_enabled, selected_media_id, fallback_media_file FROM streamers WHERE username = $1"
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
                selected_media_id: r.try_get("selected_media_id").unwrap_or(None),
                fallback_media_file: r.try_get("fallback_media_file").unwrap_or_else(|_| "/default_donate.mp3".to_string()),
            }))
        }
        None => Ok(None),
    }
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

