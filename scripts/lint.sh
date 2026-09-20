#!/usr/bin/env bash
set -euo pipefail

cd "$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "Running native Cargo Clippy for the headless workspace..."
exec cargo clippy --locked --workspace --exclude app --all-targets "$@"
