use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
pub type CropperType = wasm_bindgen::JsValue;
#[cfg(not(feature = "hydrate"))]
pub type CropperType = ();
#[wasm_bindgen(inline_js = r#"
export function init_cropper(img_id) {
    let img = document.getElementById(img_id);
    if (!img) return null;
    return new window.Cropper(img, {
        aspectRatio: 1,
        viewMode: 1,
    });
}
export function get_cropped_blob(cropper) {
    return new Promise((resolve) => {
        let canvas = cropper.getCroppedCanvas({ width: 400, height: 400 });
        canvas.toBlob((blob) => {
            resolve(blob);
        }, 'image/jpeg', 0.9);
    });
}
"#)]
#[cfg(feature = "hydrate")]
extern "C" {
    pub fn init_cropper(img_id: &str) -> wasm_bindgen::JsValue;
    pub fn get_cropped_blob(cropper: &wasm_bindgen::JsValue) -> js_sys::Promise;
}

#[component]
pub fn AvatarSettingsSection(streamer: crate::db::DbStreamer) -> impl IntoView {
    use leptos_meta::{Script, Stylesheet};
    
    let update_avatar = ServerAction::<crate::app::UpdateAvatarUrl>::new();
    let (random_avatars, set_random_avatars) = signal::<Vec<String>>(vec![]);
    let (current_avatar, set_current_avatar) = signal(streamer.avatar_url.clone());
    
    let generate_random = move |_| {
        #[cfg(feature = "hydrate")]
        {
            let styles = ["avataaars", "bottts", "micah", "notionists", "lorelei"];
            let mut avatars = vec![];
            for _ in 0..10 {
                let style = styles[(js_sys::Math::random() * (styles.len() as f64)) as usize];
                let seed = (js_sys::Math::random() * 1000000.0).to_string();
                avatars.push(format!("https://api.dicebear.com/9.x/{}/svg?seed={}", style, seed));
            }
            set_random_avatars.set(avatars);
        }
    };

    let (is_cropping, set_is_cropping) = signal(false);
    let (selected_data_url, set_selected_data_url) = signal(None::<String>);
    let cropper_instance = StoredValue::new(None::<CropperType>);
    
    let on_file_change = move |ev| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsCast;
            let target = event_target::<web_sys::HtmlInputElement>(&ev);
            if let Some(files) = target.files() {
                if let Some(file) = files.get(0) {
                    let reader = web_sys::FileReader::new().unwrap();
                    let reader_clone = reader.clone();
                    let closure = Closure::wrap(Box::new(move || {
                        if let Some(result) = reader_clone.result().ok() {
                            if let Some(data_url) = result.as_string() {
                                set_selected_data_url.set(Some(data_url));
                                set_is_cropping.set(true);
                                
                                // Initialize cropper after a short delay so image is rendered
                                gloo_timers::callback::Timeout::new(100, move || {
                                    if let Some(c) = cropper_instance.get_value() {
                                        // clean up old
                                    }
                                    let cropper = init_cropper("avatar-crop-img");
                                    cropper_instance.set_value(Some(cropper));
                                }).forget();
                            }
                        }
                    }) as Box<dyn FnMut()>);
                    reader.set_onload(Some(closure.as_ref().unchecked_ref()));
                    closure.forget();
                    let _ = reader.read_as_data_url(&file);
                }
            }
        }
    };
    
    let (upload_status, set_upload_status) = signal("".to_string());
    
    let on_crop_save = move |_| {
        #[cfg(feature = "hydrate")]
        {
            set_upload_status.set("Processing...".to_string());
            if let Some(cropper) = cropper_instance.get_value() {
                leptos::task::spawn_local(async move {
                    use wasm_bindgen_futures::JsFuture;
                    if let Ok(blob_val) = JsFuture::from(get_cropped_blob(&cropper)).await {
                        let blob = blob_val.unchecked_into::<web_sys::Blob>();
                        let file_name = format!("avatar_{}.jpg", (js_sys::Math::random() * 1000000.0) as i32);
                        
                        match crate::app::get_presigned_url(file_name, "image/jpeg".to_string()).await {
                            Ok((public_url, upload_url)) => {
                                let window = web_sys::window().unwrap();
                                let req_init = web_sys::RequestInit::new();
                                req_init.set_method("PUT");
                                req_init.set_body(&blob);
                                if let Ok(request) = web_sys::Request::new_with_str_and_init(&upload_url, &req_init) {
                                    if let Ok(resp_value) = JsFuture::from(window.fetch_with_request(&request)).await {
                                        let resp: web_sys::Response = resp_value.into();
                                        if resp.ok() {
                                            let _ = update_avatar.dispatch(crate::app::UpdateAvatarUrl { new_avatar_url: public_url.clone() });
                                            set_current_avatar.set(public_url);
                                            set_upload_status.set("Upload successful!".to_string());
                                            set_is_cropping.set(false);
                                        } else {
                                            set_upload_status.set("S3 Upload Failed".to_string());
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                set_upload_status.set("Failed to get presigned URL".to_string());
                            }
                        }
                    }
                });
            }
        }
    };

    view! {
        <Stylesheet href="https://cdnjs.cloudflare.com/ajax/libs/cropperjs/1.6.1/cropper.min.css"/>
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/cropperjs/1.6.1/cropper.min.js"/>
        
        <div class="bg-surface-variant/20 p-6 rounded-2xl border border-white/5 mb-8">
            <h2 class="text-headline-sm font-headline-sm text-on-surface mb-4">"Profile Avatar"</h2>
            <div class="flex flex-col gap-4">
                <div class="flex items-center gap-4">
                    <img src=move || current_avatar.get() class="w-24 h-24 rounded-full object-cover bg-surface-container-highest border-2 border-primary/50"/>
                    <div class="flex flex-col gap-2">
                        <label class="px-4 py-2 bg-primary text-on-primary rounded-xl font-bold cursor-pointer hover:bg-primary/90 transition-colors text-center inline-block">
                            "Upload Custom Avatar"
                            <input type="file" class="hidden" accept="image/*" on:change=on_file_change/>
                        </label>
                        <button class="px-4 py-2 bg-surface-variant text-on-surface-variant rounded-xl font-bold hover:text-on-surface transition-colors" on:click=generate_random>
                            "Generate Random"
                        </button>
                    </div>
                </div>
                
                <Show when=move || !random_avatars.get().is_empty()>
                    <div class="mt-4 p-4 bg-surface-container rounded-xl border border-white/5">
                        <h3 class="text-label-md font-bold mb-2">"Select a random avatar:"</h3>
                        <div class="grid grid-cols-5 gap-2">
                            {move || random_avatars.get().into_iter().map(|url| {
                                let url_clone = url.clone();
                                let url_for_action = url.clone();
                                view! {
                                    <img src=url_clone class="w-16 h-16 rounded-full cursor-pointer hover:scale-110 transition-transform bg-white/5" on:click=move |_| {
                                        update_avatar.dispatch(crate::app::UpdateAvatarUrl { new_avatar_url: url_for_action.clone() });
                                        set_current_avatar.set(url_for_action.clone());
                                        set_random_avatars.set(vec![]);
                                    }/>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </Show>
                
                <Show when=move || is_cropping.get()>
                    <div class="fixed inset-0 bg-black/80 z-50 flex items-center justify-center p-4">
                        <div class="bg-surface-container p-6 rounded-2xl max-w-2xl w-full flex flex-col gap-4">
                            <h3 class="text-title-lg font-bold">"Crop Avatar"</h3>
                            <div class="w-full h-[500px] bg-black rounded overflow-hidden flex items-center justify-center">
                                {move || selected_data_url.get().map(|url| view! {
                                    <img id="avatar-crop-img" src=url class="max-w-full max-h-full block" />
                                })}
                            </div>
                            <div class="text-sm text-primary">{move || upload_status.get()}</div>
                            <div class="flex justify-end gap-2">
                                <button class="px-4 py-2 bg-surface-variant rounded-xl font-bold" on:click=move |_| set_is_cropping.set(false)>"Cancel"</button>
                                <button class="px-4 py-2 bg-primary text-on-primary rounded-xl font-bold" on:click=on_crop_save>"Save Avatar"</button>
                            </div>
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    }
}
