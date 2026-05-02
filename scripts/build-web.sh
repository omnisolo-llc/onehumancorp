#!/bin/bash
set -e
echo "Building Slint WASM app..."
cd src/app
wasm-pack build --target web
echo "WASM build complete."
