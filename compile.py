import subprocess
import os

command = [
    "wasm-pack",
    "build",
    "xelis-captcha",
    "--target",
    "web",
    "--out-dir",
    "../public",
    "--no-typescript",
    "--no-package",
    "--",
    "-Z",
    "build-std=std,panic_abort",
]
env = {
    "RUSTUP_TOOLCHAIN": "nightly",
    "RUSTFLAGS": "-C target-feature=+atomics,+bulk-memory,+mutable-globals",
}
env.update(os.environ)
subprocess.run(command, env=env)
