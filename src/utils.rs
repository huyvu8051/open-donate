use std::future::Future;

pub async fn with_min_delay<T>(future: impl Future<Output = T>) -> T {
    #[cfg(target_arch = "wasm32")]
    let start = js_sys::Date::now();
    
    let result = future.await;
    
    #[cfg(target_arch = "wasm32")]
    {
        let elapsed = js_sys::Date::now() - start;
        let min_delay = 500.0;
        if elapsed < min_delay {
            let wait = (min_delay - elapsed) as u64;
            let fut = send_wrapper::SendWrapper::new(gloo_timers::future::sleep(std::time::Duration::from_millis(wait)));
            fut.await;
        }
    }
    
    result
}
