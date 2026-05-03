#!/bin/bash
set -e

WORKSPACE="$(cd "$(dirname "$0")/.." && pwd)"
RUSTUP_BIN="$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin"
DIST_DIR="/tmp/ohc-dist"
WBG="$HOME/.local/bin/wasm-bindgen"
WASM_OUT="$WORKSPACE/target/wasm32-unknown-unknown/debug/app.wasm"

FONTCONFIG_LIB="/nix/store/13438il5rqgaqa2cdddyyqf18hylhcb2-fontconfig-2.14.2-lib/lib"

# Set up fontconfig so the Slint build script can find fonts to embed
mkdir -p /tmp/fontconfig-cache
cat > /tmp/ohc-fonts.conf << 'XML'
<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <dir>/nix/store/76pr3pfwnskjbg7hmhbgpa90mrajqj12-freefont-ttf-20120503/share/fonts/truetype</dir>
  <dir>/nix/store/050qn98qj9j0iypyhcvhr3l2v17vwymy-dejavu-fonts-minimal-2.37/share/fonts/truetype</dir>
  <cachedir>/tmp/fontconfig-cache</cachedir>
</fontconfig>
XML

echo "[build-wasm] Building WASM lib target with Rust 1.95.0 (incremental after first run)..."

unset LD_AUDIT
export PATH="$RUSTUP_BIN:$PATH"
export CARGO_HOME="$HOME/.cargo"
export RUSTUP_HOME="$HOME/.rustup"
export LD_LIBRARY_PATH="$FONTCONFIG_LIB:${LD_LIBRARY_PATH:-}"
export FONTCONFIG_FILE="/tmp/ohc-fonts.conf"

cargo build \
    --target wasm32-unknown-unknown \
    --manifest-path "$WORKSPACE/src/app/Cargo.toml" \
    --lib 2>&1

echo "[build-wasm] Running wasm-bindgen..."
mkdir -p "$DIST_DIR"
"$WBG" --target web --out-dir "$DIST_DIR" "$WASM_OUT"

echo "[build-wasm] Copying index.html..."
cp "$WORKSPACE/src/app/dist-index.html" "$DIST_DIR/index.html"

echo "[build-wasm] Build complete. Files in $DIST_DIR:"
ls -lh "$DIST_DIR/"
