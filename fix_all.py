import re

# 1. Fix dashboard.rs imports
with open('src/dashboard.rs', 'r') as f:
    content = f.read()
content = content.replace(
    'use crate::app::{get_or_create_streamer, get_dashboard_transactions, Header, Footer, UpdateStreamerProfile};',
    'use crate::app::{get_or_create_streamer, get_dashboard_transactions, UpdateStreamerProfile};\nuse crate::components::layout::{Header, Footer};'
)
with open('src/dashboard.rs', 'w') as f:
    f.write(content)

# 2. Fix login.rs imports
with open('src/pages/login.rs', 'r') as f:
    content = f.read()
content = content.replace('<crate::app::Header />', '<Header />')
content = 'use crate::components::layout::Header;\n' + content
with open('src/pages/login.rs', 'w') as f:
    f.write(content)

# 3. Fix register.rs imports
with open('src/pages/register.rs', 'r') as f:
    content = f.read()
content = content.replace('<crate::app::Header />', '<Header />')
content = 'use crate::components::layout::Header;\n' + content
with open('src/pages/register.rs', 'w') as f:
    f.write(content)

# 4. Fix streamer.rs missing imports and braces
with open('src/pages/streamer.rs', 'r') as f:
    lines = f.readlines()

# find where pub fn StreamerPage() ends. It's when we see #[server(GetStreamer, "/api")]
idx = 0
for i, line in enumerate(lines):
    if '#[server(GetStreamer' in line:
        idx = i
        break

if idx > 0 and '}' not in lines[idx-1]:
    # Insert } before #[server
    lines.insert(idx, '}\n')

# Also wait, the closing brace at the end of the file added by fix_streamer.py should be removed.
if lines[-1].strip() == '}':
    lines.pop()

# Add missing imports
imports = """use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
"""
lines.insert(0, imports)

with open('src/pages/streamer.rs', 'w') as f:
    f.writelines(lines)

