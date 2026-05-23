import re

with open('streamer-dashboard.html', 'r') as f:
    html = f.read()

# Extract body
match = re.search(r'<body[^>]*>(.*?)</body>', html, re.DOTALL | re.IGNORECASE)
if match:
    body = match.group(1)
else:
    body = html

# Save body to a separate HTML file to be included
with open('src/dashboard_body.html', 'w') as f:
    f.write(body)

template = f"""use leptos::prelude::*;
use crate::db::DbStreamer;

#[component]
pub fn DashboardPage() -> impl IntoView {{
    let streamer_resource = use_context::<Resource<Result<Option<DbStreamer>, ServerFnError>>>()
        .expect("Streamer resource should be provided");

    view! {{
        <div class="bg-background text-on-surface font-body-md antialiased overflow-x-hidden" inner_html=include_str!("dashboard_body.html")>
        </div>
    }}
}}
"""

with open('src/dashboard.rs', 'w') as f:
    f.write(template)
print("Converted to use inner_html")
