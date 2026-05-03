#!/bin/bash
NIX_CARGO="/nix/store/brzjqpcbk04hzmhsqlmp7vng4jdis2yc-rust-mixed/bin/cargo"
PROTOC=$(command -v protoc || echo "protoc")
exec PROTOC="$PROTOC" STANDALONE_MODE=true "$NIX_CARGO" run --bin server
