import subprocess

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
]
subprocess.run(command)
