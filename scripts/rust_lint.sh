#!/usr/bin/env bash
set -euo pipefail

cd "$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# Arguments are Cargo options, for example --release.
exec cargo clippy --locked --workspace --exclude app --all-targets "$@" -- -D warnings
