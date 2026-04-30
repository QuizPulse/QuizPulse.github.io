#!/bin/bash
set -e

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
export PATH="$HOME/.cargo/bin:$PATH"

rustup target add wasm32-unknown-unknown

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

wasm-pack build

cd www && npm run build
