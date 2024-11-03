import init, { hello_world, estimate_hashrate, estimate_hashrate2 } from "/xelis_captcha.js";

console.log("Loading WASM module...");
await init();
console.log("WASM module loaded!");
let msg = await hello_world();
console.log("Message: ", msg);
console.log("Estimating hashrate...");

document.body.appendChild(document.createTextNode("Starting..."));

let hashrate = await estimate_hashrate2(100);
document.body.appendChild(document.createTextNode("Hashrate: " + hashrate));

try {
    hashrate = await estimate_hashrate(100, navigator.hardwareConcurrency);
    console.log("Hashrate (multi-threads): ", hashrate);
    document.body.appendChild(document.createTextNode("Hashrate (" + navigator.hardwareConcurrency + "): " + hashrate));
} catch (e) {
    console.error(e);
    document.body.appendChild(document.createTextNode("Error: " + e));
}

