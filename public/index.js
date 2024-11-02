import init, { hello_world, estimate_hashrate } from "/xelis_captcha.js";

console.log("Loading WASM module...");
await init();
console.log("WASM module loaded!");
let msg = await hello_world();
console.log("Message: ", msg);
console.log("Estimating hashrate...");
let hashrate = await estimate_hashrate(100, navigator.hardwareConcurrency);
console.log("Hashrate: ", hashrate);