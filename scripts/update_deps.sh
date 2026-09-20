#!/usr/bin/env bash
set -euo pipefail

# Explicit maintenance command: review and commit resulting native lockfile changes.
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "Refreshing native dependency lockfiles..."
cargo update
npm install --package-lock-only --ignore-scripts --no-audit --no-fund
npm --prefix src/ui/next install --package-lock-only --ignore-scripts --no-audit --no-fund

echo "Dependencies up to date."
