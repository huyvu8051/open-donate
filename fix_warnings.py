# Fix app.rs unused imports
with open('src/app.rs', 'r') as f:
    content = f.read()

imports = """
#[allow(unused_imports)]
use crate::db::{TransactionStatus, PaymentMethod};
#[allow(unused_imports)]
use leptos_router::hooks::{use_location, use_params_map};
"""

content = imports + content
with open('src/app.rs', 'w') as f:
    f.write(content)
