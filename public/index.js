import init, { hello_world } from "/xelis_captcha.js";

console.log("Loading WASM module...");
await init();
console.log("WASM module loaded!");
let msg = await hello_world();
console.log("Message: ", msg);
