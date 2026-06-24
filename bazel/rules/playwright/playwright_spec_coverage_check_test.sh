#!/usr/bin/env bash
set -euo pipefail

SCRIPT="$1"
TMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMP_ROOT"' EXIT

assert_fails_with() {
  local expected="$1"
  shift
  local log="$TMP_ROOT/output.log"
  if "$@" >"$log" 2>&1; then
    echo "Expected command to fail: $*" >&2
    cat "$log" >&2
    exit 1
  fi
  if ! grep -Fq "$expected" "$log"; then
    echo "Expected failure output to contain: $expected" >&2
    cat "$log" >&2
    exit 1
  fi
}

source_root="$TMP_ROOT/source"
runfiles_root="$TMP_ROOT/runfiles"
mkdir -p "$source_root/src/e2e/playwright" "$runfiles_root/src/e2e"
cat >"$source_root/src/e2e/included.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('included', async () => {});
SPEC
cat >"$source_root/src/e2e/playwright/missing.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('missing from runfiles', async () => {});
SPEC
cp "$source_root/src/e2e/included.spec.ts" "$runfiles_root/src/e2e/included.spec.ts"

assert_fails_with \
  "missing from Bazel runfiles: src/e2e/playwright/missing.spec.ts" \
  env SOURCE_REPO_ROOT="$source_root" RUNFILES_ROOT="$runfiles_root" "$SCRIPT" --scan-runfiles

skip_root="$TMP_ROOT/skip-source"
mkdir -p "$skip_root/src/e2e"
cat >"$skip_root/src/e2e/skipped.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test.describe.skip('temporarily disabled flow', () => {
  test('does not run', async () => {});
});
SPEC

assert_fails_with \
  "skipped Playwright tests are forbidden: src/e2e/skipped.spec.ts" \
  env SOURCE_REPO_ROOT="$skip_root" "$SCRIPT" \
    --all "$skip_root/src/e2e/skipped.spec.ts" \
    --ci "$skip_root/src/e2e/skipped.spec.ts"

echo "Playwright coverage check tests passed."
