#[component]
pub fn S3StatusBanner() -> impl IntoView {
    let s3_status_resource = Resource::new(
        || (),
        |_| async move { crate::utils::with_min_delay(crate::app::check_s3_status()).await.unwrap_or(false) }
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
        crate::utils::with_min_delay(crate::app::get_default_medias()).await.unwrap_or_default()
    });
    
    let streamer_medias = Resource::new(|| (), |_| async move {
        crate::utils::with_min_delay(crate::app::get_streamer_media()).await.unwrap_or_default()
    });
    
    let save_media_action = ServerAction::<crate::app::SaveMediaSettings>::new();
    let upload_action = ServerAction::<crate::app::UploadMedia>::new();
    
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
                <ActionForm action=upload_action enctype="multipart/form-data">
                    <div class="flex flex-col gap-2">
                        <input
                            type="file"
                            name="file"
                            accept="audio/*,video/*"
                            class="text-on-surface text-sm file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-primary file:text-on-primary hover:file:bg-primary/80"
                            required
                        />
                        <button
                            type="submit"
                            class=move || {
                                format!(
                                    "bg-primary text-on-primary font-bold px-4 py-2 rounded-xl mt-2 w-fit {}",
                                    if upload_action.pending().get() { "opacity-50" } else { "" },
                                )
                            }
                            disabled=move || upload_action.pending().get()
                        >
                            "Upload File (Max 2MB)"
                        </button>
                    </div>
                </ActionForm>
                <Suspense fallback=|| ()>
                    {move || match upload_action.value().get() {
                        Some(Ok(_)) => {
                            view! {
                                <div class="text-green-400 mt-2 text-sm font-bold">
                                    "Upload successful! Please refresh the page to see it."
                                </div>
                            }
                                .into_any()
                        }
                        Some(Err(e)) => {
                            view! {
                                <div class="text-red-400 mt-2 text-sm font-bold">
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
                                            view! {
                                                <option value=file.clone() selected=is_selected>
                                                    {file}
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
