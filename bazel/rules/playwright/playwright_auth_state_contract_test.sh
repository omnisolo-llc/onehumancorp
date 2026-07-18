#!/bin/bash
set -euo pipefail

if [[ "$#" -ne 2 ]]; then
  echo "usage: $0 <playwright-runner> <playwright-config>" >&2
  exit 2
fi

runner="$1"
config="$2"

if ! grep -Fq 'export PLAYWRIGHT_STORAGE_STATE=' "$runner"; then
  echo "Playwright runner does not export a writable authentication state path" >&2
  exit 1
fi

if ! grep -Fq 'process.env.PLAYWRIGHT_STORAGE_STATE' "$config"; then
  echo "Playwright config does not consume the authentication state path" >&2
  exit 1
fi

echo "Playwright runner and config share the real authentication state path"
