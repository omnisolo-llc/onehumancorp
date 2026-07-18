#!/bin/bash
set -euo pipefail

if [[ "$#" -ne 3 ]]; then
  echo "usage: $0 <MODULE.bazel> <package-lock.json> <playwright_test.sh>" >&2
  exit 2
fi

module_file="$1"
package_lock="$2"
runner="$3"

module_version="$(sed -n 's#.*cdn.playwright.dev/builds/cft/\([^/]*\)/.*#\1#p' "$module_file" | head -n 1)"
package_version="$(python3 - "$package_lock" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as source:
    lock = json.load(source)

print(lock["packages"]["node_modules/playwright-core"]["version"])
PY
)"

if [[ -z "$module_version" ]]; then
  echo "MODULE.bazel does not pin a Chrome-for-Testing browser" >&2
  exit 1
fi

if ! grep -Fq 'https://cdn.playwright.dev/builds/cft/' "$module_file"; then
  echo "Bazel Chromium does not use Playwright's Chrome-for-Testing CDN" >&2
  exit 1
fi

if grep -Fq 'PLAYWRIGHT_BROWSERS_PATH="$HOME/.cache/ms-playwright"' "$runner"; then
  echo "Playwright runner discards the hermetic Bazel browser path" >&2
  exit 1
fi

echo "npm Playwright $package_version uses hermetic Chrome for Testing $module_version"
