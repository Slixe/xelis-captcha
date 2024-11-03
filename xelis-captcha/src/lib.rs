use rand::{rngs::OsRng, RngCore};
use tokio_with_wasm::task::spawn_blocking;
use xelis_common::{
    block::{Algorithm, BLOCK_WORK_SIZE},
    crypto,
    utils::format_hashrate
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn estimate_hashrate(iterations: u32, n_threads: u32) -> String {
    let mut handles = Vec::new();
    let start = web_time::Instant::now();
    for _ in 0..n_threads {
        let handle = spawn_blocking(move || {
            let mut input = [0u8; BLOCK_WORK_SIZE];
            OsRng.fill_bytes(&mut input);
            for _ in 0..iterations {
                let _ = crypto::pow_hash(&input, Algorithm::V2).unwrap();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let elapsed = start.elapsed().as_millis();
    format_hashrate(1000f64 / (elapsed as f64 / (n_threads*iterations) as f64))
}

#[wasm_bindgen]
pub async fn estimate_hashrate2(iterations: u32) -> String {
    let start = web_time::Instant::now();

    let mut input = [0u8; BLOCK_WORK_SIZE];
    OsRng.fill_bytes(&mut input);
    for _ in 0..iterations {
        let _ = crypto::pow_hash(&input, Algorithm::V2).unwrap();
    }

    let elapsed = start.elapsed().as_millis();
    format_hashrate(1000f64 / (elapsed as f64 / iterations as f64))
}

#[wasm_bindgen]
pub async fn hello_world() -> String {
    let mut data = "hello world".to_string();
    spawn_blocking(move || {
        data.push_str("aaaaa");
        data
    }).await.unwrap()
}