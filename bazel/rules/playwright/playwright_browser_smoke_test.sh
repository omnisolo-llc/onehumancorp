#!/bin/bash
set -euo pipefail

if [[ "$#" -ne 1 ]]; then
  echo "usage: $0 <browser-executable>" >&2
  exit 2
fi

browser="$1"
if [[ ! -x "$browser" ]]; then
  echo "Chromium headless shell is not executable: $browser" >&2
  exit 1
fi

output="$TEST_TMPDIR/chromium-smoke.html"
timeout 15s "$browser" \
  --headless \
  --no-sandbox \
  --disable-gpu \
  --disable-dev-shm-usage \
  --dump-dom 'data:text/html,<title>bazel-playwright-browser-ok</title>' \
  >"$output"

grep -Fq '<title>bazel-playwright-browser-ok</title>' "$output"
echo "Bazel Chromium headless shell launched successfully"
