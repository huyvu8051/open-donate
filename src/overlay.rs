use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::db::DbTransaction;

#[component]
pub fn OverlayPage() -> impl IntoView {
    let params = use_params_map();
    #[allow(unused_variables)]
    let token = move || params.with_untracked(|p| p.get("token").unwrap_or_default());

    let is_revoked = RwSignal::new(false);
    let interaction_required = RwSignal::new(false);
    
    let unlock_audio = move |_| {
        interaction_required.set(false);
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsCast;
            if let Some(audio) = leptos::prelude::document().get_element_by_id("audio-silence") {
                if let Ok(a) = audio.dyn_into::<web_sys::HtmlAudioElement>() {
                    let _ = a.play();
                }
            }
        }
    };

    #[allow(unused_variables)]
    let queue = RwSignal::new(Vec::<DbTransaction>::new());
    let current = RwSignal::new(None::<DbTransaction>);
    #[allow(unused_variables)]
    let is_playing = RwSignal::new(false);
    #[allow(unused_variables)]
    let is_sound_enabled = RwSignal::new(true);
    #[allow(unused_variables)]
    let media_cache = RwSignal::new(std::collections::HashMap::<i32, String>::new());
    
    let primary_media = RwSignal::new(None::<String>);
    let fallback_media = RwSignal::new(None::<String>);
    let fallback_level = RwSignal::new(0u8);

    #[cfg(feature = "hydrate")]
    {
        use crate::app::{init_overlay_session, prefetch_upcoming_transactions, lock_transaction};
        use wasm_bindgen_futures::spawn_local;
        use wasm_bindgen::JsCast;

        let token_for_poll = token;

        let client_session_id = uuid::Uuid::new_v4().to_string();
        let session_id_clone = client_session_id.clone();
        
        let session_id_clone2 = client_session_id.clone();
        let session_id_clone3 = client_session_id.clone();

        spawn_local(async move {
            let tok = token_for_poll();
            if tok.is_empty() {
                return;
            }
            if let Err(_) = init_overlay_session(tok.clone(), session_id_clone.clone()).await {
                is_revoked.set(true);
                return;
            }
            
            // Check autoplay policy on load
            spawn_local(async move {
                gloo_timers::future::sleep(std::time::Duration::from_millis(500)).await;
                if let Some(audio) = leptos::prelude::document().get_element_by_id("audio-silence") {
                    if let Ok(a) = audio.dyn_into::<web_sys::HtmlAudioElement>() {
                        if let Ok(promise) = a.play() {
                            if let Err(_) = wasm_bindgen_futures::JsFuture::from(promise).await {
                                interaction_required.set(true);
                            }
                        }
                    }
                }
            });

            let tok_inner = tok.clone();
            let sid_inner = session_id_clone2;
            
            // 1. Prefetch Loop (Every 2 seconds)
            spawn_local(async move {
                loop {
                    gloo_timers::future::sleep(std::time::Duration::from_secs(2)).await;
                    if is_revoked.get_untracked() {
                        break;
                    }
                    
                    let result = prefetch_upcoming_transactions(tok_inner.clone(), sid_inner.clone()).await;
                    if let Ok((txs, _overlay_paused, sound_enabled, primary, fallback)) = result {
                        is_sound_enabled.set(sound_enabled);
                        queue.set(txs.clone());
                        
                        // Update media settings
                        if primary_media.get_untracked() != primary {
                            primary_media.set(primary);
                            fallback_level.set(0);
                        }
                        if fallback_media.get_untracked() != Some(fallback.clone()) {
                            fallback_media.set(Some(fallback));
                        }
                        
                        // Here we would fetch actual media and store Blob URLs
                        media_cache.update(|cache| {
                            for tx in txs {
                                if !cache.contains_key(&tx.id) {
                                    // cache.insert(tx.id, "blob:...".to_string());
                                }
                            }
                        });
                    } else if let Err(e) = result {
                        if e.to_string().contains("SessionRevoked") {
                            is_revoked.set(true);
                            break;
                        }
                    }
                }
            });

            let tok_inner2 = tok.clone();
            let sid_inner2 = session_id_clone3;
            
            // 2. Display Loop (Continuous)
            spawn_local(async move {
                loop {
                    gloo_timers::future::sleep(std::time::Duration::from_millis(500)).await;
                    if is_revoked.get_untracked() {
                        break;
                    }
                    
                    if is_playing.get_untracked() {
                        continue;
                    }

                    let next_tx = queue.with_untracked(|q| q.first().cloned());
                    
                    if let Some(tx) = next_tx {
                        is_playing.set(true);
                        
                        // Try to lock it on the backend
                        let locked = lock_transaction(tok_inner2.clone(), sid_inner2.clone(), tx.id).await.unwrap_or(false);
                        
                        queue.update(|q| {
                            if !q.is_empty() {
                                q.remove(0);
                            }
                        });

                        if locked {
                            leptos::logging::log!("Displaying locked tx: {}", tx.id);
                            
                            // Play sound
                            if is_sound_enabled.get_untracked() {
                                if let Some(audio) = leptos::prelude::document().get_element_by_id("audio-silence") {
                                    if let Ok(a) = audio.dyn_into::<web_sys::HtmlAudioElement>() {
                                        if let Ok(promise) = a.play() {
                                            wasm_bindgen_futures::spawn_local(async move {
                                                if let Err(_) = wasm_bindgen_futures::JsFuture::from(promise).await {
                                                    interaction_required.set(true);
                                                }
                                            });
                                        }
                                    }
                                }
                                gloo_timers::future::sleep(std::time::Duration::from_secs(1)).await;
                                
                                if let Some(audio) = leptos::prelude::document().get_element_by_id("audio-donate") {
                                    if let Ok(a) = audio.dyn_into::<web_sys::HtmlAudioElement>() {
                                        if let Ok(promise) = a.play() {
                                            wasm_bindgen_futures::spawn_local(async move {
                                                if let Err(_) = wasm_bindgen_futures::JsFuture::from(promise).await {
                                                    interaction_required.set(true);
                                                }
                                            });
                                        }
                                    }
                                }
                            }

                            current.set(Some(tx.clone()));
                            gloo_timers::future::sleep(std::time::Duration::from_secs(6)).await;
                            
                            // Cleanup memory
                            media_cache.update(|c| {
                                if let Some(_url) = c.remove(&tx.id) {
                                    // web_sys::Url::revoke_object_url(&url).unwrap();
                                }
                            });
                            
                            current.set(None);
                        }
                        
                        // Give a small padding before the next one can start
                        gloo_timers::future::sleep(std::time::Duration::from_millis(500)).await;
                        is_playing.set(false);
                    }
                }
            });
        });
    }
    let current_audio_src = move || {
        let level = fallback_level.get();
        if level == 0 {
            if let Some(src) = primary_media.get() {
                return src;
            }
        }
        if level <= 1 {
            if let Some(src) = fallback_media.get() {
                return src;
            }
        }
        "/public/default_donate.mp3".to_string()
    };

    view! {
        <style>"body { background: transparent !important; }"</style>
        <audio id="audio-silence" src="/audio/silence.wav" preload="auto"></audio>
        <audio id="audio-donate" src=current_audio_src preload="auto" on:error=move |_| {
            let level = fallback_level.get();
            if level < 2 {
                fallback_level.set(level + 1);
            }
        }></audio>
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

                if interaction_required.get() {
                    return view! {
                        <div class="pointer-events-auto absolute inset-0 bg-black/80 z-50 flex items-center justify-center backdrop-blur-sm cursor-pointer" on:click=unlock_audio>
                            <div class="bg-surface/95 border border-white/20 rounded-[2rem] p-16 shadow-[0_0_80px_rgba(0,0,0,0.8)] max-w-4xl text-center animate-fade-in flex flex-col items-center">
                                <span class="material-symbols-outlined text-primary text-[120px] mb-8 animate-pulse drop-shadow-2xl">"volume_up"</span>
                                <h1 class="text-6xl font-extrabold text-white mb-6 drop-shadow-lg tracking-tight">"Click to enable Audio"</h1>
                                <p class="text-3xl text-white/80 leading-relaxed max-w-2xl">"Browsers require you to interact with the page before playing sound. Click anywhere to start!"</p>
                            </div>
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
                                        <div data-testid="mock-donor-name" class="text-white text-4xl font-bold leading-tight">
                                            {tx.donor_name}
                                        </div>
                                        <div class="text-white/80 text-lg">
                                            "sent a Glint"
                                        </div>
                                    </div>
                                    <div data-testid="mock-amount" class="text-secondary text-5xl font-extrabold">
                                        "$" {format!("{:.2}", tx.amount)}
                                    </div>
                                </div>
                                {move || {
                                    msg.clone().map(|m| view! {
                                        <div data-testid="mock-message" class="mt-6 text-white/90 text-2xl italic break-words">
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
