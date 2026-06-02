use leptos::prelude::*;

pub async fn sleep_send(ms: u64) {
    let (tx, rx) = futures::channel::oneshot::channel::<()>();
    set_timeout(
        move || { let _ = tx.send(()); },
        std::time::Duration::from_millis(ms)
    );
    let _ = rx.await;
}
