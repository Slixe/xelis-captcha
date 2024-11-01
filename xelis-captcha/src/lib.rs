use tokio_with_wasm::task::spawn_blocking;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn hello_world() -> String {
    let mut data = "hello world".to_string();
    spawn_blocking(move || {
        data.push_str("aaaaa");
        data
    }).await.unwrap()
}