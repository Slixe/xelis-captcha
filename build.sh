#!/bin/bash

# Make sure Cargo is installed and needed as a dependency to create the WASM lib
if command -v cargo &> /dev/null; then
  echo "Cargo is installed."
else 
  echo "Installing Cargo..."

  # Download & run rustup installer
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain stable -y

  # Source the Cargo env
  source "$HOME/.cargo/env"

  echo "Cargo installed successfully."
fi

# Make sure wasm-pack is installed
cargo install wasm-pack

# Compile using wasm-pack
python3 compile.py

echo "Build complete."
