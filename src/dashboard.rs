use leptos::prelude::*;
use crate::app::{get_or_create_streamer, get_dashboard_transactions, Header, Footer};

#[component]
pub fn DashboardPage() -> impl IntoView {
    let onboard_resource = LocalResource::new(move || {
        async move {
            get_or_create_streamer().await
        }
    });

    view! {
        <Transition fallback=move || view! { <div class="p-8 text-center">"Loading Dashboard..."</div> }>
            {move || {
                onboard_resource.get().map(|res| {
                    match res {
                        Ok(Some(streamer)) => {
                            let _profile_url = format!("/streamer/{}", streamer.username);
                            let overlay_url = format!("/overlay/{}", streamer.username);
                            let _donate_url = format!("http://localhost:3000/streamer/{}", streamer.username);
                            let _avatar_url = if streamer.avatar_url.is_empty() { 
                                "https://api.dicebear.com/9.x/avataaars/svg".to_string() 
                            } else { 
                                streamer.avatar_url.clone() 
                            };

                            view! {
                                <div class="bg-background text-on-surface font-body-md antialiased overflow-x-hidden">
                                    <Header />
                                    
                                    // SideNavBar
                                    <aside class="fixed left-0 top-20 h-[calc(100vh-80px)] w-64 bg-surface-container-low/40 backdrop-blur-md border-r border-white/10 hidden md:flex flex-col p-md gap-base z-40">
                                        <div class="mb-lg">
                                            <h2 class="text-headline-sm font-headline-sm text-primary">"Creator Hub"</h2>
                                            <p class="text-label-sm font-label-sm text-on-surface-variant">"Manage your Glint account"</p>
                                        </div>
                                        <nav class="flex-1 flex flex-col gap-xs">
                                            <a class="flex items-center gap-sm p-sm bg-primary-container text-on-primary-container rounded-xl font-bold translate-x-1 duration-200" href="#">
                                                <span class="material-symbols-outlined">"dashboard"</span>
                                                <span class="text-label-md font-label-md">"Dashboard"</span>
                                            </a>
                                            <a class="flex items-center gap-sm p-sm text-on-surface-variant hover:bg-surface-variant/50 rounded-xl transition-all" href="#">
                                                <span class="material-symbols-outlined">"monitoring"</span>
                                                <span class="text-label-md font-label-md">"Analytics"</span>
                                            </a>
                                            <a class="flex items-center gap-sm p-sm text-on-surface-variant hover:bg-surface-variant/50 rounded-xl transition-all" href="#">
                                                <span class="material-symbols-outlined">"payments"</span>
                                                <span class="text-label-md font-label-md">"Payments"</span>
                                            </a>
                                            <a class="flex items-center gap-sm p-sm text-on-surface-variant hover:bg-surface-variant/50 rounded-xl transition-all" href="#">
                                                <span class="material-symbols-outlined">"settings"</span>
                                                <span class="text-label-md font-label-md">"Settings"</span>
                                            </a>
                                        </nav>
                                        <button class="bg-gradient-to-r from-primary-container to-primary text-on-primary-container py-sm rounded-xl font-bold flex items-center justify-center gap-xs active:scale-95 transition-all shadow-lg shadow-primary/20">
                                            <span class="material-symbols-outlined">"videocam"</span>
                                            <span>"Go Live"</span>
                                        </button>
                                        <div class="pt-md border-t border-white/10 flex flex-col gap-xs">
                                            <a class="flex items-center gap-sm p-sm text-on-surface-variant hover:bg-surface-variant/30 rounded-xl transition-all" href="#">
                                                <span class="material-symbols-outlined">"help"</span>
                                                <span class="text-label-md font-label-md">"Help"</span>
                                            </a>
                                            <a class="flex items-center gap-sm p-sm text-on-surface-variant hover:bg-surface-variant/30 rounded-xl transition-all" href="/api/auth/logout">
                                                <span class="material-symbols-outlined">"logout"</span>
                                                <span class="text-label-md font-label-md">"Logout"</span>
                                            </a>
                                        </div>
                                    </aside>
                                    
                                    // Main Content
                                    <main class="md:ml-64 pt-32 px-margin-mobile md:px-margin-desktop pb-24 md:pb-xl min-h-screen">
                                        <div class="max-w-7xl mx-auto">
                                            // Welcome Header
                                            <div class="flex flex-col md:flex-row md:items-end justify-between gap-md mb-lg">
                                            <div>
                                                <h1 data-testid="streamer-dashboard-header" class="text-headline-lg font-headline-lg text-on-surface">"Streamer Dashboard"</h1>
                                                <p class="text-body-md font-body-md text-on-surface-variant">
                                                    "Welcome back, " {streamer.display_name.clone()} ". Your creator hub is ready."
                                                </p>
                                            </div>
                                            <a
                                                class="inline-flex items-center gap-xs px-md py-sm rounded-xl bg-primary-container text-on-primary-container font-bold hover:brightness-110 transition-all active:scale-[0.98]"
                                                href=overlay_url.clone()
                                                target="_blank"
                                                rel="noopener noreferrer"
                                            >
                                                <span class="material-symbols-outlined text-[20px]">"open_in_new"</span>
                                                <span>"Open Overlay"</span>
                                            </a>
                                        </div>

                                        <section class="bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl p-md md:p-xl mb-lg">
                                            <div class="flex flex-col gap-sm">
                                                <div class="flex items-center gap-sm">
                                                    <span class="material-symbols-outlined text-primary">"link"</span>
                                                    <h2 class="text-headline-md font-headline-md text-on-surface">"Overlay URL"</h2>
                                                </div>
                                                <p class="text-body-md text-on-surface-variant mb-sm">
                                                    "Copy this secure URL and paste it as a Browser Source in OBS. Only one instance can be active at a time. If you open it elsewhere, the previous session will be revoked."
                                                </p>
                                                <div class="flex items-center gap-sm bg-black/40 border border-white/10 rounded-xl p-sm">
                                                    <code class="flex-1 text-on-surface text-label-md font-mono overflow-hidden text-ellipsis whitespace-nowrap px-sm">
                                                        "http://localhost:3000" {overlay_url.clone()}
                                                    </code>
                                                </div>
                                            </div>
                                        </section>

                                        <DonationHistory streamer_id={streamer.id} />
                                    </div>
                                </main>

                                    // Mobile Bottom Navigation
                                    <nav class="fixed bottom-0 w-full bg-surface-container-low/90 backdrop-blur-xl border-t border-white/10 flex justify-around items-center h-16 z-50 md:hidden pb-safe">
                                        <a href="#" class="flex flex-col items-center gap-1 text-primary w-16">
                                            <span class="material-symbols-outlined text-[24px]">"dashboard"</span>
                                            <span class="text-[10px] font-bold leading-none">"Dashboard"</span>
                                        </a>
                                        <a href="#" class="flex flex-col items-center gap-1 text-on-surface-variant hover:text-primary transition-colors w-16">
                                            <span class="material-symbols-outlined text-[24px]">"monitoring"</span>
                                            <span class="text-[10px] font-medium leading-none">"Analytics"</span>
                                        </a>
                                        <button class="relative -top-5 w-12 h-12 bg-gradient-to-tr from-primary to-primary-container rounded-full shadow-lg shadow-primary/30 flex items-center justify-center text-on-primary active:scale-95 transition-transform border-4 border-background">
                                            <span class="material-symbols-outlined text-[24px]">"videocam"</span>
                                        </button>
                                        <a href="#" class="flex flex-col items-center gap-1 text-on-surface-variant hover:text-primary transition-colors w-16">
                                            <span class="material-symbols-outlined text-[24px]">"payments"</span>
                                            <span class="text-[10px] font-medium leading-none">"Payments"</span>
                                        </a>
                                        <a href="#" class="flex flex-col items-center gap-1 text-on-surface-variant hover:text-primary transition-colors w-16">
                                            <span class="material-symbols-outlined text-[24px]">"settings"</span>
                                            <span class="text-[10px] font-medium leading-none">"Settings"</span>
                                        </a>
                                    </nav>

                                    <Footer />

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
                            }.into_any()
                        },
                        Ok(None) => view! {
                            <div class="p-8 text-center">"You must be logged in to access the dashboard."</div>
                        }.into_any(),
                        Err(e) => view! {
                            <div class="p-8 text-center text-red-500">
                                {format!("Error during onboarding: {:?}", e)}
                            </div>
                        }.into_any(),
                    }
                })
            }}
        </Transition>
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
                    <h3 class="text-headline-md font-headline-md text-on-surface">"Donation History"</h3>
                    <p class="text-body-md font-body-md text-on-surface-variant">"Detailed view of your recent contributions"</p>
                </div>
                
                <div class="flex flex-wrap gap-sm w-full md:w-auto items-center">
                    <label class="flex items-center gap-2 text-label-md text-on-surface cursor-pointer">
                        <input type="checkbox" 
                            class="accent-primary w-4 h-4 cursor-pointer"
                            prop:checked=move || auto_reload.get()
                            on:change=move |ev| set_auto_reload.set(event_target_checked(&ev))
                        />
                        "Auto Reload"
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
                            <option value="5" selected=move || reload_interval.get() == 5>"5s"</option>
                            <option value="10" selected=move || reload_interval.get() == 10>"10s"</option>
                            <option value="30" selected=move || reload_interval.get() == 30>"30s"</option>
                            <option value="60" selected=move || reload_interval.get() == 60>"60s"</option>
                        </select>
                    </div>
                    <div class="flex items-center gap-2 ml-4">
                        <span class="text-label-sm text-on-surface-variant">"Per page:"</span>
                        <select 
                            class="bg-surface-variant/30 border-none rounded-lg px-2 py-1 text-label-sm text-on-surface focus:ring-1 focus:ring-primary cursor-pointer"
                            on:change=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<i64>() {
                                    set_page_size.set(val);
                                    set_page.set(1); 
                                }
                            }
                        >
                            <option value="5" selected=move || page_size.get() == 5>"5"</option>
                            <option value="10" selected=move || page_size.get() == 10>"10"</option>
                            <option value="20" selected=move || page_size.get() == 20>"20"</option>
                            <option value="50" selected=move || page_size.get() == 50>"50"</option>
                        </select>
                    </div>
                </div>
            </div>
            
            <div class="overflow-x-auto custom-scrollbar">
                <table class="w-full text-left border-collapse min-w-[700px]">
                    <thead>
                        <tr class="border-b border-white/10 text-label-sm font-label-sm text-on-surface-variant uppercase tracking-wider">
                            <th class="pb-md pr-md">"Supporter ID"</th>
                            <th class="pb-md px-md">"Amount"</th>
                            <th class="pb-md px-md">"Message"</th>
                            <th class="pb-md pl-md">"Timestamp"</th>
                        </tr>
                    </thead>
                    <tbody class="text-body-md">
                        <Suspense fallback=move || view! { <tr><td colspan="4" class="py-md text-center text-on-surface-variant">"Loading..."</td></tr> }>
                            {move || {
                                tx_resource.get().map(|res| match res {
                                    Ok((txs, _)) => {
                                        if txs.is_empty() {
                                            view! { <tr><td colspan="4" class="py-md text-center text-on-surface-variant">"No donations yet."</td></tr> }.into_any()
                                        } else {
                                            view! {
                                                {txs.into_iter().map(|tx| view! {
                                                    <tr data-testid="donation-row" class="border-b border-white/5 hover:bg-white/5 transition-colors group">
                                                        <td class="py-md pr-md">
                                                            <div class="flex items-center gap-sm">
                                                                <div class="w-8 h-8 rounded-full bg-primary/20 flex items-center justify-center">
                                                                    <span class="material-symbols-outlined text-sm text-primary">"person"</span>
                                                                </div>
                                                                <span class="font-bold text-on-surface">{tx.donor_name}</span>
                                                            </div>
                                                        </td>
                                                        <td class="py-md px-md font-bold text-secondary">"+$" {format!("{:.2}", tx.amount)}</td>
                                                        <td class="py-md px-md text-on-surface-variant italic">
                                                            {if let Some(msg) = tx.message { format!("\"{}\"", msg) } else { "".to_string() }}
                                                        </td>
                                                        <td class="py-md pl-md text-outline">{tx.created_at}</td>
                                                    </tr>
                                                }).collect_view()}
                                            }.into_any()
                                        }
                                    },
                                    Err(_) => view! { <tr><td colspan="4" class="py-md text-center text-error">"Error loading donations."</td></tr> }.into_any()
                                })
                            }}
                        </Suspense>
                    </tbody>
                </table>
            </div>
            
            <div class="flex justify-between items-center pt-md border-t border-white/10">
                <p class="text-label-sm font-label-sm text-on-surface-variant">
                    "Showing " {move || start_idx()} " to " {move || end_idx()} " of " {move || total_count()} " donations"
                </p>
                <div class="flex gap-xs">
                    <button
                        class="w-10 h-10 flex items-center justify-center rounded-lg bg-surface-variant/30 text-on-surface-variant hover:bg-surface-variant/50 disabled:opacity-30"
                        disabled={move || page.get() <= 1}
                        on:click=move |_| set_page.update(|p| if *p > 1 { *p -= 1 })
                    >
                        <span class="material-symbols-outlined">"chevron_left"</span>
                    </button>
                    <span class="w-10 h-10 flex items-center justify-center rounded-lg bg-primary text-on-primary font-bold text-label-md">
                        {move || page.get()}
                    </span>
                    <button
                        class="w-10 h-10 flex items-center justify-center rounded-lg bg-surface-variant/30 text-on-surface-variant hover:bg-surface-variant/50 disabled:opacity-30"
                        disabled={move || page.get() >= _total_pages.get()}
                        on:click=move |_| set_page.update(|p| if *p < _total_pages.get() { *p += 1 })
                    >
                        <span class="material-symbols-outlined">"chevron_right"</span>
                    </button>
                </div>
            </div>
        </div>
    }
}
