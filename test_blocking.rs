use leptos::prelude::*;
fn test() {
    let _r = Resource::new_blocking(|| (), |_| async { () });
}
