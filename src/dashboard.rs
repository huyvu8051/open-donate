use leptos::prelude::*;
use crate::app::get_or_create_streamer;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let onboard_resource = Resource::new(|| (), |_| async move {
        get_or_create_streamer().await
    });

    view! {
        <Transition fallback=move || view! { <div class="p-8 text-center">"Loading Dashboard..."</div> }>
            {move || {
                onboard_resource.get().map(|res| {
                    match res {
                        Ok(Some(_streamer)) => view! {
                            <div class="bg-background text-on-surface font-body-md antialiased overflow-x-hidden" inner_html=include_str!("dashboard_body.html")>
                            </div>
                        }.into_any(),
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
