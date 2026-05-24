use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::db::DbTransaction;

#[component]
pub fn OverlayPage() -> impl IntoView {
    let params = use_params_map();
    #[allow(unused_variables)]
    let token = move || params.with_untracked(|p| p.get("token").unwrap_or_default());

    let is_revoked = RwSignal::new(false);

    #[allow(unused_variables)]
    let queue = RwSignal::new(Vec::<DbTransaction>::new());
    let current = RwSignal::new(None::<DbTransaction>);

    #[cfg(feature = "hydrate")]
    {
        use crate::app::{init_overlay_session, mark_transaction_displayed, poll_overlay_transactions};
        use gloo_timers::callback::Interval;
        use wasm_bindgen_futures::spawn_local;

        let poll_interval: StoredValue<Option<Interval>, leptos::prelude::LocalStorage> =
            StoredValue::new_local(None);

        let token_for_poll = token;

        let client_session_id = uuid::Uuid::new_v4().to_string();
        let session_id_clone = client_session_id.clone();

        spawn_local(async move {
            let tok = token_for_poll();
            if tok.is_empty() {
                return;
            }
            if let Err(_) = init_overlay_session(tok.clone(), session_id_clone.clone()).await {
                is_revoked.set(true);
                return;
            }

            let interval = Interval::new(1000, move || {
                let tok_inner = tok.clone();
                let sid_inner = session_id_clone.clone();

                spawn_local(async move {
                    if is_revoked.get_untracked() {
                        return;
                    }

                    let result = poll_overlay_transactions(tok_inner, sid_inner).await;
                    let mut txs = match result {
                        Ok(t) => t,
                        Err(e) => {
                            if e.to_string().contains("SessionRevoked") {
                                is_revoked.set(true);
                            }
                            return;
                        }
                    };

                    txs.sort_by_key(|t| t.id);
                    queue.update(|q| {
                        for tx in txs {
                            if !q.iter().any(|existing| existing.id == tx.id)
                                && current
                                    .get_untracked()
                                    .as_ref()
                                    .map(|c| c.id != tx.id)
                                    .unwrap_or(true)
                            {
                                q.push(tx);
                            }
                        }
                        q.sort_by_key(|t| t.id);
                    });

                    if current.get_untracked().is_some() {
                        return;
                    }

                    let next = queue.with_untracked(|q| q.first().cloned());
                    if let Some(next) = next {
                        queue.update(|q| {
                            if !q.is_empty() {
                                q.remove(0);
                            }
                        });
                        current.set(Some(next.clone()));

                        spawn_local(async move {
                            gloo_timers::future::sleep(std::time::Duration::from_secs(6)).await;
                            let _ = mark_transaction_displayed(next.id).await;
                            current.set(None);
                        });
                    }
                });
            });

            poll_interval.set_value(Some(interval));
        });
    }

    view! {
        <main class="fixed inset-0 w-screen h-screen bg-transparent overflow-hidden pointer-events-none flex flex-col items-center justify-center">
            {move || {
                if is_revoked.get() {
                    return view! {
                        <div class="pointer-events-auto bg-error/90 backdrop-blur-md border border-error/50 rounded-2xl p-xl shadow-2xl max-w-lg text-center animate-fade-in">
                            <span class="material-symbols-outlined text-on-error text-[64px] mb-md">"warning"</span>
                            <h1 class="text-headline-lg font-headline-lg text-on-error mb-sm">"Session Revoked"</h1>
                            <p class="text-body-lg text-on-error/80">"This overlay session was revoked because the overlay link was opened in another location."</p>
                        </div>
                    }.into_any();
                }

                current.get().map(|tx| {
                    let msg = tx.message.clone();
                    view! {
                        <div class="absolute inset-0 flex items-center justify-center p-8">
                            <div class="max-w-4xl w-full bg-black/60 backdrop-blur-md border border-white/10 rounded-3xl px-10 py-8 shadow-[0_0_80px_rgba(0,0,0,0.55)] animate-fade-in">
                                <div class="flex items-center justify-between gap-6">
                                    <div class="flex flex-col gap-1">
                                        <div class="text-white text-4xl font-bold leading-tight">
                                            {tx.donor_name}
                                        </div>
                                        <div class="text-white/80 text-lg">
                                            "sent a Glint"
                                        </div>
                                    </div>
                                    <div class="text-secondary text-5xl font-extrabold">
                                        "$" {format!("{:.2}", tx.amount)}
                                    </div>
                                </div>
                                {move || {
                                    msg.clone().map(|m| view! {
                                        <div class="mt-6 text-white/90 text-2xl italic break-words">
                                            "\"" {m} "\""
                                        </div>
                                    })
                                }}
                            </div>
                        </div>
                    }
                }).into_any()
            }}
        </main>
    }
}
