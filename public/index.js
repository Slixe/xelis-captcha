const { hello_world } = wasm_bindgen;

async function load() {
    console.log("Loading WASM module...");
    await wasm_bindgen();
    console.log("WASM module loaded!");
    let msg = await hello_world();
    console.log("Message: ", msg);
}

load();