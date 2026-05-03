#!/bin/bash
set -e
WORKSPACE="$(cd "$(dirname "$0")/.." && pwd)"
RUSTUP_BIN="$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin"
TRUNK_BIN=$(command -v trunk || echo "/nix/store/h53hqhfhlxwcanx6fr3fpd1k0aih0dxw-trunk-0.21.14/bin/trunk")
PROTOC=$(command -v protoc || echo "protoc")

exec env -i \
    HOME="$HOME" \
    PATH="$RUSTUP_BIN:/usr/bin:/bin" \
    PROTOC="$PROTOC" \
    CARGO_HOME="$HOME/.cargo" \
    RUSTUP_HOME="$HOME/.rustup" \
    "$TRUNK_BIN" "$@"
