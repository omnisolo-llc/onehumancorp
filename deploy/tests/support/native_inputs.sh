#!/usr/bin/env bash
set -euo pipefail
# Source after REPO_ROOT is set. An explicit local probe may be reused; otherwise
# Cargo fingerprints incrementally build it. Never reuse a stale Bazel binary.
prepare_native_probe() {
  if [[ -n "${OMNISOLO_GRPC_PROBE:-}" ]]; then
    GRPC_PROBE="$OMNISOLO_GRPC_PROBE"
    [[ "$GRPC_PROBE" == /* ]] || GRPC_PROBE="$REPO_ROOT/$GRPC_PROBE"
    [[ -x "$GRPC_PROBE" ]] || { echo 'Configured native gRPC probe is not executable' >&2; return 1; }
  else
    (cd "$REPO_ROOT" && cargo build --locked -p omnisolo --bin grpc-mtls-probe)
    GRPC_PROBE="${CARGO_TARGET_DIR:-$REPO_ROOT/target}/debug/grpc-mtls-probe"
    [[ "$GRPC_PROBE" = /* ]] || GRPC_PROBE="$REPO_ROOT/$GRPC_PROBE"
    [[ -x "$GRPC_PROBE" ]] || { echo 'Cargo did not produce the native gRPC probe' >&2; return 1; }
  fi
}
