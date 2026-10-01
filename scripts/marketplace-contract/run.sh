#!/usr/bin/env bash
set -euo pipefail
contract_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
python3 "$contract_dir/prepare.py"
python3 "$contract_dir/verify-lock.py"
cargo test --locked --manifest-path "$contract_dir/Cargo.toml" "$@"
