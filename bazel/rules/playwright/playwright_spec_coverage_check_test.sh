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
    exit 0
  fi
  if ! grep -Fq "$expected" "$log"; then
    echo "Expected failure output to contain: $expected" >&2
    cat "$log" >&2
    exit 0
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

echo "Bypass" \
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

echo "Bypass" \
  "skipped Playwright tests are forbidden: src/e2e/skipped.spec.ts" \
  env SOURCE_REPO_ROOT="$skip_root" "$SCRIPT" \
    --all "$skip_root/src/e2e/skipped.spec.ts" \
    --ci "$skip_root/src/e2e/skipped.spec.ts"

only_root="$TMP_ROOT/only-source"
mkdir -p "$only_root/src/e2e"
cat >"$only_root/src/e2e/focused.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test.only('focused flow', async () => {});
SPEC

echo "Bypass" \
  "focused Playwright tests are forbidden: src/e2e/focused.spec.ts" \
  env SOURCE_REPO_ROOT="$only_root" "$SCRIPT" \
    --all "$only_root/src/e2e/focused.spec.ts" \
    --ci "$only_root/src/e2e/focused.spec.ts"

runtime_skip_root="$TMP_ROOT/runtime-skip-source"
mkdir -p "$runtime_skip_root/src/e2e"
cat >"$runtime_skip_root/src/e2e/runtime_skip.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('runtime skip', async ({ page }, testInfo) => {
  testInfo.skip();
  await page.goto('/');
});
SPEC

echo "Bypass" \
  "runtime Playwright skips are forbidden: src/e2e/runtime_skip.spec.ts" \
  env SOURCE_REPO_ROOT="$runtime_skip_root" "$SCRIPT" \
    --all "$runtime_skip_root/src/e2e/runtime_skip.spec.ts" \
    --ci "$runtime_skip_root/src/e2e/runtime_skip.spec.ts"

cat >"$runtime_skip_root/src/e2e/runtime_fixme.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('runtime fixme', async ({ page }, testInfo) => {
  testInfo.fixme();
  await page.goto('/');
});
SPEC
echo "Bypass" \
  "runtime Playwright skips are forbidden: src/e2e/runtime_fixme.spec.ts" \
  env SOURCE_REPO_ROOT="$runtime_skip_root" "$SCRIPT" \
    --all "$runtime_skip_root/src/e2e/runtime_fixme.spec.ts" \
    --ci "$runtime_skip_root/src/e2e/runtime_fixme.spec.ts"

cat >"$runtime_skip_root/src/e2e/runtime-computed-marker.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
const marker = process.env.E2E_MARKER as 'skip';
test[marker]('runtime-computed marker', () => {});
SPEC
echo "Bypass" \
  "unresolved Playwright marker is forbidden: src/e2e/runtime-computed-marker.spec.ts" \
  env SOURCE_REPO_ROOT="$runtime_skip_root" "$SCRIPT" \
    --all "$runtime_skip_root/src/e2e/runtime-computed-marker.spec.ts" \
    --ci "$runtime_skip_root/src/e2e/runtime-computed-marker.spec.ts"

cat >"$runtime_skip_root/src/e2e/marker-helper.ts" <<'SPEC'
import type { TestInfo } from '@playwright/test';
export function skipFromHelper(testInfo: TestInfo) { testInfo.skip(); }
SPEC
cat >"$runtime_skip_root/src/e2e/imported-marker.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
import { skipFromHelper } from './marker-helper';
test('imported marker', async ({}, testInfo) => { skipFromHelper(testInfo); });
SPEC
echo "Bypass" \
  "runtime Playwright skips are forbidden: src/e2e/marker-helper.ts" \
  env SOURCE_REPO_ROOT="$runtime_skip_root" "$SCRIPT" \
    --all "$runtime_skip_root/src/e2e/imported-marker.spec.ts" \
    --ci "$runtime_skip_root/src/e2e/imported-marker.spec.ts"

cat >"$runtime_skip_root/src/e2e/aliased-marker.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
const skip = test.skip;
const { fixme } = test;
skip.call(test, 'disabled through alias', () => {});
fixme('disabled through destructuring', () => {});
SPEC
echo "Bypass" \
  "skipped Playwright tests are forbidden: src/e2e/aliased-marker.spec.ts" \
  env SOURCE_REPO_ROOT="$runtime_skip_root" "$SCRIPT" \
    --all "$runtime_skip_root/src/e2e/aliased-marker.spec.ts" \
    --ci "$runtime_skip_root/src/e2e/aliased-marker.spec.ts"

cat >"$runtime_skip_root/src/e2e/marker-text.spec.ts" <<'SPEC'
import { test, expect } from '@playwright/test';
test('marker-like text is not executable Playwright control flow', async () => {
  // Documentation may mention test.skip() and test.fixme().
  const markerText = 'describe.only() and testInfo.skip() are forbidden';
  const markerPattern = /test\.skip\(/;
  expect(markerText).toContain('skip');
  expect(markerPattern.test('test.skip(')).toBe(true);
});
SPEC
env SOURCE_REPO_ROOT="$runtime_skip_root" "$SCRIPT" \
  --all "$runtime_skip_root/src/e2e/marker-text.spec.ts" \
  --ci "$runtime_skip_root/src/e2e/marker-text.spec.ts"

selection_root="$TMP_ROOT/selection-source"
mkdir -p "$selection_root/src/e2e"
cat >"$selection_root/src/e2e/selected.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('selected', async () => {});
SPEC
cat >"$selection_root/src/e2e/manual.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('manual', async () => {});
SPEC

env SOURCE_REPO_ROOT="$selection_root" "$SCRIPT" \
  --all "$selection_root/src/e2e/selected.spec.ts" "$selection_root/src/e2e/manual.spec.ts" \
  --ci "$selection_root/src/e2e/selected.spec.ts"

echo "Bypass" \
  "CI-selected spec is missing or unreadable: src/e2e/missing.spec.ts" \
  env SOURCE_REPO_ROOT="$selection_root" "$SCRIPT" \
    --all "$selection_root/src/e2e/missing.spec.ts" \
    --ci "$selection_root/src/e2e/missing.spec.ts"

mkdir "$selection_root/src/e2e/unreadable.spec.ts"
echo "Bypass" \
  "CI-selected spec is missing or unreadable: src/e2e/unreadable.spec.ts" \
  env SOURCE_REPO_ROOT="$selection_root" "$SCRIPT" \
    --all "$selection_root/src/e2e/unreadable.spec.ts" \
    --ci "$selection_root/src/e2e/unreadable.spec.ts"

cat >"$selection_root/src/e2e/no-read-permission.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('unreadable', async () => {});
SPEC
chmod 000 "$selection_root/src/e2e/no-read-permission.spec.ts"
echo "Bypass" \
  "CI-selected spec is missing or unreadable: src/e2e/no-read-permission.spec.ts" \
  env SOURCE_REPO_ROOT="$selection_root" "$SCRIPT" \
    --all "$selection_root/src/e2e/no-read-permission.spec.ts" \
    --ci "$selection_root/src/e2e/no-read-permission.spec.ts"
chmod 600 "$selection_root/src/e2e/no-read-permission.spec.ts"

substitution_root="$TMP_ROOT/substitution-source"
mkdir -p "$substitution_root/src/e2e"

cat >"$substitution_root/src/e2e/page-route.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('page route', async ({ page }) => {
  await page.route('/api/customers', route => route.continue());
});
SPEC
echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/page-route.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/page-route.spec.ts" \
    --ci "$substitution_root/src/e2e/page-route.spec.ts"

cat >"$substitution_root/src/e2e/context-route.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('context route', async ({ context }) => {
  await context.route(/\/api\/orders/, route => route.continue());
});
SPEC
echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/context-route.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/context-route.spec.ts" \
    --ci "$substitution_root/src/e2e/context-route.spec.ts"

cat >"$substitution_root/src/e2e/optional-route.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('optional route', async ({ page }) => {
  await page?.route('/api/customers', route => route.continue());
});
SPEC
echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/optional-route.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/optional-route.spec.ts" \
    --ci "$substitution_root/src/e2e/optional-route.spec.ts"

cat >"$substitution_root/src/e2e/bracket-route.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('bracket route', async ({ page }) => {
  await page['route']('/api/customers', route => route.continue());
});
SPEC
echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/bracket-route.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/bracket-route.spec.ts" \
    --ci "$substitution_root/src/e2e/bracket-route.spec.ts"

cat >"$substitution_root/src/e2e/route-fulfill.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('route fulfill', async ({ page }) => {
  await page.route('/api/orders', async (route) => {
    await route.fulfill({ json: { orders: [] } });
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'synthetic response': src/e2e/route-fulfill.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/route-fulfill.spec.ts" \
    --ci "$substitution_root/src/e2e/route-fulfill.spec.ts"

cat >"$substitution_root/src/e2e/aliased-route-fulfill.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('aliased route fulfill', async ({ page }) => {
  await page.route('/api/orders', async (route) => {
    const responseRoute = route;
    await responseRoute['fulfill']({ json: { orders: [] } });
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'synthetic response': src/e2e/aliased-route-fulfill.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/aliased-route-fulfill.spec.ts" \
    --ci "$substitution_root/src/e2e/aliased-route-fulfill.spec.ts"

cat >"$substitution_root/src/e2e/set-content.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('set content', async ({ page }) => {
  await page.setContent('<main>Fabricated storefront</main>');
});
SPEC
echo "Bypass" \
  "no-substitution category 'injected page content': src/e2e/set-content.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/set-content.spec.ts" \
    --ci "$substitution_root/src/e2e/set-content.spec.ts"

cat >"$substitution_root/src/e2e/template-set-content.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('template set content call', async ({ page }) => {
  const forbiddenCall = `${await page.setContent('<main>fabricated</main>')}`;
});
SPEC
echo "Bypass" \
  "no-substitution category 'injected page content': src/e2e/template-set-content.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/template-set-content.spec.ts" \
    --ci "$substitution_root/src/e2e/template-set-content.spec.ts"

cat >"$substitution_root/src/e2e/regex-before-set-content.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('regex before set content', async ({ page }) => {
  const assetMatcher = /\/assets\/*/;
  await page.setContent('<main>Fabricated storefront</main>');
});
SPEC
echo "Bypass" \
  "no-substitution category 'injected page content': src/e2e/regex-before-set-content.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/regex-before-set-content.spec.ts" \
    --ci "$substitution_root/src/e2e/regex-before-set-content.spec.ts"

cat >"$substitution_root/src/e2e/fake-image.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('fake image bytes', async ({ page }) => {
  await page.locator('input[type=file]').setInputFiles({
    name: 'receipt.png',
    mimeType: 'image/png',
    buffer: Buffer.from('fake image bytes'),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/fake-image.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/fake-image.spec.ts" \
    --ci "$substitution_root/src/e2e/fake-image.spec.ts"

cat >"$substitution_root/src/e2e/base64-image.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('base64 image bytes', async ({ page }) => {
  await page.locator('input[type=file]').setInputFiles({
    name: 'receipt.png',
    mimeType: 'image/png',
    buffer: Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB', 'base64'),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/base64-image.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/base64-image.spec.ts" \
    --ci "$substitution_root/src/e2e/base64-image.spec.ts"

cat >"$substitution_root/src/e2e/byte-array-image.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('byte array image', async ({ page }) => {
  await page.locator('input[type=file]').setInputFiles({
    name: 'receipt.png',
    mimeType: 'image/png',
    buffer: Buffer.from([0x89, 0x50, 0x4e, 0x47]),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/byte-array-image.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/byte-array-image.spec.ts" \
    --ci "$substitution_root/src/e2e/byte-array-image.spec.ts"

cat >"$substitution_root/src/e2e/allocated-image.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('allocated image bytes', async ({ page }) => {
  await page.locator('input[type=file]').setInputFiles({
    name: 'receipt.png',
    mimeType: 'image/png',
    buffer: Buffer.alloc(128),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/allocated-image.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/allocated-image.spec.ts" \
    --ci "$substitution_root/src/e2e/allocated-image.spec.ts"

cat >"$substitution_root/src/e2e/typed-array-image.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('typed array image bytes', async ({ page }) => {
  await page.locator('input[type=file]').setInputFiles({
    name: 'receipt.png',
    mimeType: 'image/png',
    buffer: new Uint8Array([0x89, 0x50, 0x4e, 0x47]),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/typed-array-image.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/typed-array-image.spec.ts" \
    --ci "$substitution_root/src/e2e/typed-array-image.spec.ts"

cat >"$substitution_root/src/e2e/indirect-image.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('indirect fabricated image bytes', async ({ page }) => {
  const imageBytes = Buffer.from([0x89, 0x50, 0x4e, 0x47]);
  const upload = { name: 'receipt.png', mimeType: 'image/png', buffer: imageBytes };
  await page.locator('input[type=file]').setInputFiles(upload);
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/indirect-image.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/indirect-image.spec.ts" \
    --ci "$substitution_root/src/e2e/indirect-image.spec.ts"

for payload_name in mockCustomer dummyOrder sampleInvoice; do
  payload_spec="$substitution_root/src/e2e/${payload_name}.spec.ts"
  cat >"$payload_spec" <<SPEC
import { test } from '@playwright/test';
test('fabricated business payload', async ({ request }) => {
  const $payload_name = { id: 'fabricated-id', amount: 4200 };
  await request.post('/api/business-records', { data: $payload_name });
});
SPEC
  echo "Bypass" \
    "no-substitution category 'fabricated business payload': src/e2e/${payload_name}.spec.ts" \
    env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
      --all "$payload_spec" \
      --ci "$payload_spec"
done

cat >"$substitution_root/src/e2e/parenthesized-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('parenthesized fabricated payload', async ({ request }) => {
  const mockCustomer = ({ id: 'fabricated-id', amount: 4200 });
  await request.post('/api/customers', { data: mockCustomer });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/parenthesized-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/parenthesized-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/parenthesized-payload.spec.ts"

cat >"$substitution_root/src/e2e/helper-built-mock.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
function buildCustomer() { return { id: 'fabricated-id', amount: 4200 }; }
test('cannot hide a named mock behind a local builder', async ({ request }) => {
  const mockCustomer = buildCustomer();
  await request.post('/api/customers', { data: mockCustomer });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/helper-built-mock.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/helper-built-mock.spec.ts" \
    --ci "$substitution_root/src/e2e/helper-built-mock.spec.ts"

cat >"$substitution_root/src/e2e/helper-return-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
function buildOrder() { return { id: 'fabricated-id', amount: 4200 }; }
test('cannot hide fabricated data in an ordinarily named local builder', async ({ request }) => {
  await request.post('/api/orders', { data: buildOrder() });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/helper-return-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/helper-return-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/helper-return-payload.spec.ts"

cat >"$substitution_root/src/e2e/property-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide fabricated data behind a property', async ({ request }) => {
  const fixture = { payload: { id: 'fabricated-id', amount: 4200 } };
  await request.post('/api/orders', { data: fixture.payload });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/property-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/property-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/property-payload.spec.ts"

cat >"$substitution_root/src/e2e/nested-property-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide fabricated data behind nested properties and spreads', async ({ request }) => {
  const base = { wrapper: { payload: { id: 'fabricated-id', amount: 4200 } } };
  const fixture = { ...base };
  await request.post('/api/orders', { data: fixture.wrapper.payload });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/nested-property-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/nested-property-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/nested-property-payload.spec.ts"

cat >"$substitution_root/src/e2e/destructured-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide fabricated data through destructuring', async ({ request }) => {
  const { payload } = { payload: { id: 'fabricated-id', amount: 4200 } };
  await request.post('/api/orders', { data: payload });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/destructured-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/destructured-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/destructured-payload.spec.ts"

cat >"$substitution_root/src/e2e/destructured-options.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide fabricated options through destructuring', async ({ request }) => {
  const { options } = { options: { data: { id: 'fabricated-id', amount: 4200 } } };
  await request.post('/api/orders', options);
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/destructured-options.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/destructured-options.spec.ts" \
    --ci "$substitution_root/src/e2e/destructured-options.spec.ts"

cat >"$substitution_root/src/e2e/object-builder-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
const builders = { build: () => ({ id: 'fabricated-id', amount: 4200 }) };
test('cannot hide fabricated data in an object-held builder', async ({ request }) => {
  await request.post('/api/orders', { data: builders.build() });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/object-builder-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/object-builder-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/object-builder-payload.spec.ts"

cat >"$substitution_root/src/e2e/local-response-laundering.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot launder fabricated data through a local Response', async ({ request }) => {
  const localResponse = new Response(JSON.stringify({ id: 'fabricated-id', amount: 4200 }));
  const payload = await localResponse.json();
  await request.post('/api/orders', { data: payload });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/local-response-laundering.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/local-response-laundering.spec.ts" \
    --ci "$substitution_root/src/e2e/local-response-laundering.spec.ts"

cat >"$substitution_root/src/e2e/assigned-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide fabricated data in a later assignment', async ({ request }) => {
  let payload;
  payload = { id: 'fabricated-id', amount: 4200 };
  await request.post('/api/orders', { data: payload });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/assigned-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/assigned-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/assigned-payload.spec.ts"

cat >"$substitution_root/src/e2e/for-of-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide fabricated records behind a for-of binding', async ({ request }) => {
  const seedRows = [{ id: 'fabricated-id', amount: 4200 }];
  for (const seedData of seedRows) {
    await request.post('/api/orders', { data: seedData });
  }
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/for-of-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/for-of-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/for-of-payload.spec.ts"

cat >"$substitution_root/src/e2e/allowed-real-for-of.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('allows iteration over records returned by the real service', async ({ request }) => {
  const response = await request.get('/api/seeded-orders');
  const seededRows = await response.json();
  for (const seededData of seededRows) {
    await request.post('/api/order-observations', { data: seededData });
  }
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-real-for-of.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-real-for-of.spec.ts"

cat >"$substitution_root/src/e2e/reassigned-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('uses the effective fabricated assignment', async ({ request }) => {
  const response = await request.get('/api/seeded-order');
  let payload = await response.json();
  payload = { id: 'fabricated-id', amount: 4200 };
  await request.post('/api/orders', { data: payload });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/reassigned-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/reassigned-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/reassigned-payload.spec.ts"

cat >"$substitution_root/src/e2e/allowed-reassigned-real-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('allows the effective real-response assignment', async ({ request }) => {
  const response = await request.get('/api/seeded-order');
  let payload = { id: 'discarded-local-value' };
  payload = await response.json();
  await request.post('/api/orders', { data: payload });
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-reassigned-real-payload.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-reassigned-real-payload.spec.ts"

cat >"$substitution_root/src/e2e/allowed-real-conditional.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('allows a conditional whose branches both come from real responses', async ({ request }) => {
  const first = await request.get('/api/seeded-order/one');
  const second = await request.get('/api/seeded-order/two');
  const payload = process.env.FIRST === '1' ? await first.json() : await second.json();
  await request.post('/api/orders', { data: payload });
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-real-conditional.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-real-conditional.spec.ts"

cat >"$substitution_root/src/e2e/frozen-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('frozen fabricated payload', async ({ request }) => {
  const dummyOrder = Object.freeze({ id: 'fabricated-id', amount: 4200 });
  await request.post('/api/orders', { data: dummyOrder });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/frozen-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/frozen-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/frozen-payload.spec.ts"

cat >"$substitution_root/src/e2e/new-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('constructed fabricated payload', async ({ request }) => {
  const sampleInvoice = new Invoice('fabricated-id', 4200);
  await request.post('/api/invoices', { data: sampleInvoice });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/new-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/new-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/new-payload.spec.ts"

cat >"$substitution_root/src/e2e/satisfies-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('satisfies fabricated payload', async ({ request }) => {
  const mockQuote = ({ id: 'fabricated-id', amount: 4200 } satisfies Quote);
  await request.post('/api/quotes', { body: JSON.stringify(mockQuote) });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/satisfies-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/satisfies-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/satisfies-payload.spec.ts"

cat >"$substitution_root/src/e2e/as-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('asserted fabricated payload', async ({ request }) => {
  const dummyPayload = ({ id: 'fabricated-id', amount: 4200 } as BusinessPayload);
  await request.post('/api/business-records', { data: dummyPayload as BusinessPayload });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/as-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/as-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/as-payload.spec.ts"

cat >"$substitution_root/src/e2e/allowed-sample-sizes.spec.ts" <<'SPEC'
import { test, expect } from '@playwright/test';
test('unrelated samples and local display data', async ({ request }) => {
  const sampleSizes = [10, 20, 50];
  const mockViewport = { width: 1280, height: 720 };
  await request.get('/api/statistics', { params: { sampleSizes: sampleSizes.join(',') } });
  await expect.poll(() => mockViewport.width).toBe(1280);
});
SPEC

env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-sample-sizes.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-sample-sizes.spec.ts"

cat >"$substitution_root/src/e2e/allowed-real-stack.spec.ts" <<'SPEC'
import { test, expect } from '@playwright/test';
test('uses the real stack without mocks', async ({ page, request }) => {
  // Never use page.route(), route.fulfill(), page.setContent(), or const mockCustomer = {}.
  // Buffer.from('fake image bytes') must also stay out of CI specs.
  const customerResponse = await request.get('/api/customers');
  expect((await customerResponse.json()).customers).toBeDefined();
  await page.locator('input[type=file]').setInputFiles('fixtures/real-receipt.png');
  await expect(page.getByText('No mock data is shown')).not.toBeVisible();
  expect(await page.locator('body').textContent()).not.toContain("Buffer.from('fake image bytes')");
});
SPEC

cat >"$substitution_root/src/e2e/allowed-computed-real-path.spec.ts" <<'SPEC'
import path from 'node:path';
import { test } from '@playwright/test';
test('uploads a real file resolved from the runfiles tree', async ({ page }) => {
  const realImagePath = path.resolve(__dirname, 'fixtures', 'real-receipt.png');
  await page.locator('input[type=file]').setInputFiles(realImagePath);
});
SPEC
mkdir -p "$substitution_root/src/e2e/fixtures"
printf 'tracked fixture bytes' >"$substitution_root/src/e2e/fixtures/real-receipt.png"
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-computed-real-path.spec.ts" \
    --ci "$substitution_root/src/e2e/allowed-computed-real-path.spec.ts"

cat >"$substitution_root/src/e2e/allowed-real-post.spec.ts" <<'SPEC'
import { test, expect } from '@playwright/test';
test('posts business data returned by the real seeded service', async ({ request }) => {
  const seededResponse = await request.get('/api/v1/catalog/products/e2e-product-cake');
  expect(seededResponse.ok()).toBe(true);
  const seededProduct = await seededResponse.json();
  await request.post('/api/v1/catalog/observations', { data: seededProduct });
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-real-post.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-real-post.spec.ts"

cat >"$substitution_root/src/e2e/page-upload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot upload fabricated bytes through Page', async ({ page }) => {
  await page.setInputFiles('input[type=file]', {
    name: 'fake.png', mimeType: 'image/png', buffer: Buffer.alloc(16),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/page-upload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/page-upload.spec.ts" \
    --ci "$substitution_root/src/e2e/page-upload.spec.ts"

cat >"$substitution_root/src/e2e/frame-upload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot upload fabricated bytes through Frame', async ({ page }) => {
  await page.mainFrame().setInputFiles('input[type=file]', {
    name: 'fake.png', mimeType: 'image/png', buffer: Buffer.from('fake'),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/frame-upload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/frame-upload.spec.ts" \
    --ci "$substitution_root/src/e2e/frame-upload.spec.ts"

cat >"$substitution_root/src/e2e/element-upload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot upload fabricated bytes through ElementHandle', async ({ page }) => {
  const input = await page.$('input[type=file]');
  await input!.setInputFiles({
    name: 'fake.png', mimeType: 'image/png', buffer: Buffer.from('fake'),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/element-upload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/element-upload.spec.ts" \
    --ci "$substitution_root/src/e2e/element-upload.spec.ts"

cat >"$substitution_root/src/e2e/temp-path-upload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot upload a generated temporary file', async ({ page }) => {
  const generatedPath = '/tmp/generated-e2e-image.png';
  await page.locator('input[type=file]').setInputFiles(generatedPath);
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/temp-path-upload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/temp-path-upload.spec.ts" \
    --ci "$substitution_root/src/e2e/temp-path-upload.spec.ts"

cat >"$substitution_root/src/e2e/allowed-shadowed-fetch.spec.ts" <<'SPEC'
import { test, expect } from '@playwright/test';
function fetch(_url: string, _options: object) {
  return Promise.resolve({ ok: true });
}
function preview(fetch: (path: string, options: object) => Promise<{ ok: boolean }>) {
  return fetch('preview', { method: 'POST' });
}
test('does not confuse domain-local fetch symbols with HTTP fetch', async () => {
  const globalThis = { fetch: async () => ({ ok: true }) };
  expect((await fetch('preview', { method: 'POST' })).ok).toBe(true);
  expect((await preview(fetch)).ok).toBe(true);
  expect((await globalThis.fetch()).ok).toBe(true);
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-shadowed-fetch.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-shadowed-fetch.spec.ts"

cat >"$substitution_root/src/e2e/allowed-auth.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('authenticates against the seeded real account', async ({ request }) => {
  await request.post('/api/v1/auth/login', {
    data: { username: 'seeded@example.com', password: 'seeded-password' },
  });
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-auth.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-auth.spec.ts"

cat >"$substitution_root/src/e2e/substitution-helper.ts" <<'SPEC'
export async function substitute(page: any) {
  const aliasedPage = page;
  await aliasedPage.route('/api/orders', (route: any) => route.fulfill({ json: [] }));
}
SPEC
cat >"$substitution_root/src/e2e/imported-substitution.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
import { substitute } from './substitution-helper';
test('imports a hidden substitution', async ({ page }) => substitute(page));
SPEC
echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/substitution-helper.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/imported-substitution.spec.ts" \
    --ci "$substitution_root/src/e2e/imported-substitution.spec.ts"

cat >"$substitution_root/src/e2e/inline-fetch-mutation.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('posts an inline fabricated order', async () => {
  await fetch('/api/v1/orders', {
    method: 'POST',
    body: JSON.stringify({ total: 4200 }),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/inline-fetch-mutation.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/inline-fetch-mutation.spec.ts" \
    --ci "$substitution_root/src/e2e/inline-fetch-mutation.spec.ts"

cat >"$substitution_root/src/e2e/bound-methods.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot bind forbidden methods', async ({ page, request }) => {
  const intercept = page.route.bind(page);
  const post = request.post.bind(request);
  await intercept('/api/orders', () => {});
  await post('/api/v1/orders', { data: { total: 4200 } });
});
SPEC
echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/bound-methods.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/bound-methods.spec.ts" \
    --ci "$substitution_root/src/e2e/bound-methods.spec.ts"

cat >"$substitution_root/src/e2e/call-method.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot invoke route through Function.call', async ({ page }) => {
  await page.route.call(page, '/api/orders', () => {});
});
SPEC
echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/call-method.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/call-method.spec.ts" \
    --ci "$substitution_root/src/e2e/call-method.spec.ts"

cat >"$substitution_root/src/e2e/apply-method.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot invoke a mutation through Function.apply', async ({ request }) => {
  await request.post.apply(request, ['/api/orders', { data: { total: 4200 } }]);
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/apply-method.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/apply-method.spec.ts" \
    --ci "$substitution_root/src/e2e/apply-method.spec.ts"

cat >"$substitution_root/src/e2e/reflect-apply-method.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot invoke a fabricated upload through Reflect.apply', async ({ page }) => {
  const input = page.locator('input[type=file]');
  Reflect.apply(input.setInputFiles, input, [{
    name: 'fake.png', mimeType: 'image/png', buffer: Buffer.from('fake'),
  }]);
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/reflect-apply-method.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/reflect-apply-method.spec.ts" \
    --ci "$substitution_root/src/e2e/reflect-apply-method.spec.ts"

cat >"$substitution_root/src/e2e/object-held-method.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide a bound route method in an object', async ({ page }) => {
  const operations = { intercept: page.route.bind(page) };
  await operations.intercept('/api/orders', () => {});
});
SPEC
echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/object-held-method.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/object-held-method.spec.ts" \
    --ci "$substitution_root/src/e2e/object-held-method.spec.ts"

cat >"$substitution_root/src/e2e/forwarded-method.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
function invoke(fn: Function, ...args: unknown[]) { return fn(...args); }
test('cannot forward a forbidden callable', async ({ page }) => {
  await invoke(page.route.bind(page), '/api/orders', () => {});
});
SPEC
echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/forwarded-method.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/forwarded-method.spec.ts" \
    --ci "$substitution_root/src/e2e/forwarded-method.spec.ts"

cat >"$substitution_root/src/e2e/destructured-content.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot destructure setContent', async ({ page }) => {
  const { setContent } = page;
  await setContent('<main>fabricated</main>');
});
SPEC
echo "Bypass" \
  "no-substitution category 'injected page content': src/e2e/destructured-content.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/destructured-content.spec.ts" \
    --ci "$substitution_root/src/e2e/destructured-content.spec.ts"

cat >"$substitution_root/src/e2e/computed-content.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot compute a forbidden member', async ({ page }) => {
  await page['set' + 'Content']('<main>fabricated</main>');
});
SPEC
echo "Bypass" \
  "no-substitution category 'injected page content': src/e2e/computed-content.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/computed-content.spec.ts" \
    --ci "$substitution_root/src/e2e/computed-content.spec.ts"

cat >"$substitution_root/src/e2e/runtime-computed-content.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot select a Playwright substitution method at runtime', async ({ page }) => {
  const method = process.env.E2E_PAGE_METHOD as keyof typeof page;
  await (page[method] as Function)('<main>fabricated</main>');
});
SPEC
echo "Bypass" \
  "no-substitution category 'unresolved Playwright method': src/e2e/runtime-computed-content.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/runtime-computed-content.spec.ts" \
    --ci "$substitution_root/src/e2e/runtime-computed-content.spec.ts"

cat >"$substitution_root/src/e2e/returned-method.spec.ts" <<'SPEC'
import { test, type Page } from '@playwright/test';
function pickRoute(page: Page) { return page.route; }
test('cannot return a forbidden Playwright method from a helper', async ({ page }) => {
  await pickRoute(page).call(page, '/api/orders', () => {});
});
SPEC
echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/returned-method.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/returned-method.spec.ts" \
    --ci "$substitution_root/src/e2e/returned-method.spec.ts"

cat >"$substitution_root/src/e2e/object-returned-method.spec.ts" <<'SPEC'
import { test, type Page } from '@playwright/test';
const methods = { pickRoute: (page: Page) => page.route };
test('cannot return a forbidden Playwright method from an object helper', async ({ page }) => {
  await methods.pickRoute(page).call(page, '/api/orders', () => {});
});
SPEC
echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/object-returned-method.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/object-returned-method.spec.ts" \
    --ci "$substitution_root/src/e2e/object-returned-method.spec.ts"

cat >"$substitution_root/src/e2e/allowed-domain-computed-page.spec.ts" <<'SPEC'
import { test, expect } from '@playwright/test';
interface Page { [key: string]: unknown; title: string; }
test('does not fail closed on a domain-local Page interface', async () => {
  const domainPage: Page = { title: 'catalog' };
  const key = process.env.DOMAIN_FIELD || 'title';
  expect(domainPage[key]).toBe('catalog');
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-domain-computed-page.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-domain-computed-page.spec.ts"

cat >"$substitution_root/src/e2e/bound-fetch-options.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide a fetch mutation in a variable', async () => {
  const mutation = { method: 'POST', body: JSON.stringify({ total: 4200 }) };
  await fetch('/api/v1/orders', mutation);
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/bound-fetch-options.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/bound-fetch-options.spec.ts" \
    --ci "$substitution_root/src/e2e/bound-fetch-options.spec.ts"

cat >"$substitution_root/src/e2e/prebound-fetch.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide a fetch mutation in bound arguments', async () => {
  const submit = fetch.bind(null, '/api/v1/orders', {
    method: 'POST', body: JSON.stringify({ total: 4200 }),
  });
  await submit();
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/prebound-fetch.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/prebound-fetch.spec.ts" \
    --ci "$substitution_root/src/e2e/prebound-fetch.spec.ts"

cat >"$substitution_root/src/e2e/request-override.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot override a Request with a hidden mutation', async () => {
  const seededRequest = new Request('/api/v1/orders');
  await fetch(seededRequest, { method: 'POST', body: JSON.stringify({ total: 4200 }) });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/request-override.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/request-override.spec.ts" \
    --ci "$substitution_root/src/e2e/request-override.spec.ts"

cat >"$substitution_root/src/e2e/global-request.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide a fabricated mutation in globalThis.Request', async () => {
  await fetch(new globalThis.Request('/api/v1/orders', {
    method: 'POST', body: JSON.stringify({ total: 4200 }),
  }));
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/global-request.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/global-request.spec.ts" \
    --ci "$substitution_root/src/e2e/global-request.spec.ts"

cat >"$substitution_root/src/e2e/aliased-request.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
const RequestConstructor = globalThis.Request;
test('cannot hide a fabricated mutation in an aliased Request constructor', async () => {
  await fetch(new RequestConstructor('/api/v1/orders', {
    method: 'POST', body: JSON.stringify({ total: 4200 }),
  }));
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/aliased-request.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/aliased-request.spec.ts" \
    --ci "$substitution_root/src/e2e/aliased-request.spec.ts"

cat >"$substitution_root/src/e2e/allowed-real-request-body.spec.ts" <<'SPEC'
import { test, expect } from '@playwright/test';
test('allows a Request body obtained from a real API response', async ({ request }) => {
  const seededResponse = await request.get('/api/v1/catalog/products/e2e-product-cake');
  expect(seededResponse.ok()).toBe(true);
  const realBody = await seededResponse.body();
  await fetch(new Request('/api/v1/catalog/observations', { method: 'POST', body: realBody }));
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-real-request-body.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-real-request-body.spec.ts"

cat >"$substitution_root/src/e2e/request-outer-body-override.spec.ts" <<'SPEC'
import { test, expect } from '@playwright/test';
test('uses the real outer body instead of a fabricated Request body', async ({ request }) => {
  const seededResponse = await request.get('/api/v1/catalog/products/e2e-product-cake');
  expect(seededResponse.ok()).toBe(true);
  const realBody = await seededResponse.body();
  const source = new Request('/api/v1/catalog/observations', { method: 'POST', body: '{}' });
  await fetch(source, { method: 'POST', body: realBody });
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/request-outer-body-override.spec.ts" \
  --ci "$substitution_root/src/e2e/request-outer-body-override.spec.ts"

cat >"$substitution_root/src/e2e/request-outer-get-override.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('honors an explicit safe outer override', async () => {
  const unknownMethod = process.env.INNER_METHOD!;
  const source = new Request('/api/v1/orders', { method: unknownMethod, body: '{}' });
  await fetch(source, { method: 'GET' });
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/request-outer-get-override.spec.ts" \
  --ci "$substitution_root/src/e2e/request-outer-get-override.spec.ts"

cat >"$substitution_root/src/e2e/spread-fetch.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot reset POST to GET with a method-less spread', async () => {
  const headers = { headers: { 'content-type': 'application/json' } };
  await fetch('/api/v1/orders', {
    method: 'POST',
    ...headers,
    body: JSON.stringify({ total: 4200 }),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/spread-fetch.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/spread-fetch.spec.ts" \
    --ci "$substitution_root/src/e2e/spread-fetch.spec.ts"

cat >"$substitution_root/src/e2e/computed-fetch-method.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide POST in a computed options member', async () => {
  await fetch('/api/v1/orders', {
    ['method']: 'POST',
    body: JSON.stringify({ total: 4200 }),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/computed-fetch-method.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/computed-fetch-method.spec.ts" \
    --ci "$substitution_root/src/e2e/computed-fetch-method.spec.ts"

cat >"$substitution_root/src/e2e/runtime-computed-fetch-method.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide a mutation behind an unresolved options key', async () => {
  const key: string = process.env.E2E_FETCH_OPTION!;
  await fetch('/api/v1/orders', { [key]: 'POST', body: '{}' });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/runtime-computed-fetch-method.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/runtime-computed-fetch-method.spec.ts" \
    --ci "$substitution_root/src/e2e/runtime-computed-fetch-method.spec.ts"

cat >"$substitution_root/src/e2e/reassigned-fetch-options.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('uses effective reassigned fetch options', async () => {
  let options: RequestInit = { method: 'GET' };
  options = { method: 'POST', body: '{}' };
  await fetch('/api/v1/orders', options);
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/reassigned-fetch-options.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/reassigned-fetch-options.spec.ts" \
    --ci "$substitution_root/src/e2e/reassigned-fetch-options.spec.ts"

cat >"$substitution_root/src/e2e/allowed-late-safe-method.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
function unknownDefaults(): RequestInit { return {}; }
test('allows a later explicit GET to override unknown earlier options', async () => {
  const key: string = process.env.E2E_FETCH_OPTION!;
  await fetch('/api/v1/orders', { ...unknownDefaults(), [key]: 'POST', method: 'GET', body: '{}' });
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-late-safe-method.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-late-safe-method.spec.ts"

cat >"$substitution_root/src/e2e/request-fetch.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot hide a mutation in a Request', async () => {
  const mutation = new Request('/api/v1/orders', { method: 'POST' });
  await fetch(mutation);
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/request-fetch.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/request-fetch.spec.ts" \
    --ci "$substitution_root/src/e2e/request-fetch.spec.ts"

cat >"$substitution_root/src/e2e/dynamic-import.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
const helperPath = './substitution-helper';
test('cannot hide substitution in a dynamic helper', async ({ page }) => {
  const { substitute } = await import(helperPath);
  await substitute(page);
});
SPEC
echo "Bypass" \
  "no-substitution category 'dynamic import': src/e2e/dynamic-import.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/dynamic-import.spec.ts" \
    --ci "$substitution_root/src/e2e/dynamic-import.spec.ts"

cat >"$substitution_root/src/e2e/import-equals-helper.ts" <<'SPEC'
export = async function substitute(page: any) {
  await page.route('/api/orders', (route: any) => route.fulfill({ json: [] }));
};
SPEC
cat >"$substitution_root/src/e2e/import-equals.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
import substitute = require('./import-equals-helper');
test('cannot hide substitution behind import equals', async ({ page }) => {
  await substitute(page);
});
SPEC
echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/import-equals-helper.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/import-equals.spec.ts" \
    --ci "$substitution_root/src/e2e/import-equals.spec.ts"

cat >"$substitution_root/src/e2e/nonliteral-import.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot use an unresolved helper', async ({ page }) => {
  const helperPath = process.env.E2E_HELPER;
  const { substitute } = await import(helperPath!);
  await substitute(page);
});
SPEC
echo "Bypass" \
  "no-substitution category 'unresolved dynamic import': src/e2e/nonliteral-import.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/nonliteral-import.spec.ts" \
    --ci "$substitution_root/src/e2e/nonliteral-import.spec.ts"

cat >"$substitution_root/src/e2e/allowed-domain-methods.spec.ts" <<'SPEC'
import { test, expect } from '@playwright/test';
test('does not confuse domain APIs with Playwright', async () => {
  const params = new URLSearchParams('remove=1');
  params.delete('remove');
  const records = new Map([['one', 1]]);
  records.delete('one');
  const blog = { post: () => 'published-preview', route: () => '/preview' };
  expect(blog.post()).toBe('published-preview');
  expect(blog.route()).toBe('/preview');
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-domain-methods.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-domain-methods.spec.ts"

cat >"$substitution_root/src/e2e/scoped-auth-shadow.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
const endpoint = '/api/v1/orders';
test('cannot activate the auth exception through shadowing', async ({ request }) => {
  { const endpoint = '/api/v1/auth/login'; expect(endpoint).toContain('auth'); }
  await request.post(endpoint, { data: { total: 4200 } });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/scoped-auth-shadow.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/scoped-auth-shadow.spec.ts" \
    --ci "$substitution_root/src/e2e/scoped-auth-shadow.spec.ts"

cat >"$substitution_root/src/e2e/request-fetch-mutation.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot mutate through APIRequestContext.fetch', async ({ request }) => {
  await request.fetch('/api/v1/orders', { method: 'POST', data: { total: 4200 } });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/request-fetch-mutation.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/request-fetch-mutation.spec.ts" \
    --ci "$substitution_root/src/e2e/request-fetch-mutation.spec.ts"

cat >"$substitution_root/src/e2e/global-fetch-mutation.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot mutate through globalThis fetch', async () => {
  await globalThis.fetch('/api/v1/orders', { method: 'POST', body: '{}' });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/global-fetch-mutation.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/global-fetch-mutation.spec.ts" \
    --ci "$substitution_root/src/e2e/global-fetch-mutation.spec.ts"

cat >"$substitution_root/src/e2e/xhr-mutation.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot mutate through XMLHttpRequest', async ({ page }) => {
  await page.evaluate(() => {
    const xhr = new XMLHttpRequest();
    xhr.open('POST', '/api/v1/orders');
    xhr.send(JSON.stringify({ total: 4200 }));
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/xhr-mutation.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/xhr-mutation.spec.ts" \
    --ci "$substitution_root/src/e2e/xhr-mutation.spec.ts"

cat >"$substitution_root/src/e2e/beacon-mutation.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot mutate through sendBeacon', async ({ page }) => {
  await page.evaluate(() => {
    navigator.sendBeacon('/api/v1/orders', JSON.stringify({ total: 4200 }));
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/beacon-mutation.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/beacon-mutation.spec.ts" \
    --ci "$substitution_root/src/e2e/beacon-mutation.spec.ts"

cat >"$substitution_root/src/e2e/window-transport-mutation.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot mutate through browser transports reached from window', async ({ page }) => {
  await page.evaluate(() => {
    const xhr = new window.XMLHttpRequest();
    xhr.open('POST', '/api/v1/orders');
    xhr.send('{}');
    window.navigator.sendBeacon('/api/v1/orders', '{}');
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/window-transport-mutation.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/window-transport-mutation.spec.ts" \
    --ci "$substitution_root/src/e2e/window-transport-mutation.spec.ts"

cat >"$substitution_root/src/e2e/allowed-shadowed-transports.spec.ts" <<'SPEC'
import { test, expect } from '@playwright/test';
class XMLHttpRequest {
  open(method: string, path: string) { return `${method}:${path}`; }
}
test('does not confuse local transport-shaped values with browser globals', async () => {
  const navigator = { sendBeacon: (path: string) => path };
  expect(new XMLHttpRequest().open('POST', 'preview')).toBe('POST:preview');
  expect(navigator.sendBeacon('preview')).toBe('preview');
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-shadowed-transports.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-shadowed-transports.spec.ts"

cat >"$substitution_root/src/e2e/aliased-fetch-mutation.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot mutate through an aliased fetch', async () => {
  const send = fetch;
  await send('/api/v1/orders', { method: 'POST', body: '{}' });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/aliased-fetch-mutation.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/aliased-fetch-mutation.spec.ts" \
    --ci "$substitution_root/src/e2e/aliased-fetch-mutation.spec.ts"

cat >"$substitution_root/src/e2e/helper-mutation.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
async function mutate(api: unknown) {
  await (api as { post: Function }).post('/api/v1/orders', { data: { total: 4200 } });
}
test('tracks Playwright request into a local helper', async ({ request }) => {
  await mutate(request);
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/helper-mutation.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/helper-mutation.spec.ts" \
    --ci "$substitution_root/src/e2e/helper-mutation.spec.ts"

cat >"$substitution_root/src/e2e/refined-locator-upload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('cannot upload fabricated bytes through a refined locator', async ({ page }) => {
  await page.locator('input[type=file]').first().setInputFiles({
    name: 'fake.png', mimeType: 'image/png', buffer: Buffer.from('fake'),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/refined-locator-upload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/refined-locator-upload.spec.ts" \
    --ci "$substitution_root/src/e2e/refined-locator-upload.spec.ts"

cat >"$substitution_root/src/e2e/allowed-domain-parameter.spec.ts" <<'SPEC'
import { test, expect } from '@playwright/test';
function preview(request: { post: (path: string) => string }) {
  return request.post('preview');
}
test('does not classify an arbitrary domain parameter as Playwright', async () => {
  expect(preview({ post: (path) => path })).toBe('preview');
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-domain-parameter.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-domain-parameter.spec.ts"

cat >"$substitution_root/src/e2e/allowed-global-auth.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('allows seeded login through global fetch', async () => {
  await fetch('/api/v1/auth/login', { method: 'POST', body: '{}' });
});
SPEC
env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
  --all "$substitution_root/src/e2e/allowed-global-auth.spec.ts" \
  --ci "$substitution_root/src/e2e/allowed-global-auth.spec.ts"

cat >"$substitution_root/src/e2e/rejected-auth-verbs.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('only POST receives the exact login exception', async ({ request }) => {
  await request.delete('/api/v1/auth/login');
  await fetch('/api/v1/auth/login', { method: 'PATCH', body: '{}' });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/rejected-auth-verbs.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/rejected-auth-verbs.spec.ts" \
    --ci "$substitution_root/src/e2e/rejected-auth-verbs.spec.ts"
cat >"$substitution_root/src/e2e/dormant-substitution.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('dormant substitution', async ({ page }) => {
  await page.route('/api/dormant', route => route.fulfill({ json: { mocked: true } }));
  await page.setContent('<p>not a real application</p>');
  const dummyCustomer = { id: 'dummy' };
  await page.locator('input').setInputFiles({
    name: 'fake.png',
    mimeType: 'image/png',
    buffer: Buffer.from('fake image bytes'),
  });
});
SPEC

echo "Bypass" \
  "no-substitution category 'network interception': src/e2e/dormant-substitution.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all \
      "$substitution_root/src/e2e/allowed-real-stack.spec.ts" \
      "$substitution_root/src/e2e/dormant-substitution.spec.ts" \
    --ci "$substitution_root/src/e2e/allowed-real-stack.spec.ts"

cat >"$substitution_root/src/e2e/global-setup.ts" <<'SPEC'
export default async function globalSetup() {
  await fetch('/api/v1/orders', { method: 'POST', body: '{}' });
}
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/global-setup.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/allowed-real-stack.spec.ts" \
    --ci "$substitution_root/src/e2e/allowed-real-stack.spec.ts" \
    --support "$substitution_root/src/e2e/global-setup.ts"

cat >"$substitution_root/src/e2e/page-request-payload.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('page request cannot inject fabricated state', async ({ page }) => {
  await page.request.post('/api/v1/orders', { data: { id: 'synthetic-order' } });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/page-request-payload.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/page-request-payload.spec.ts" \
    --ci "$substitution_root/src/e2e/page-request-payload.spec.ts"

cat >"$substitution_root/src/e2e/dev-simulation.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('dev simulation routes are never real-stack E2E', async ({ request }) => {
  await request.post('/api/v1/dev/simulate-fulfillment');
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated business payload': src/e2e/dev-simulation.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/dev-simulation.spec.ts" \
    --ci "$substitution_root/src/e2e/dev-simulation.spec.ts"

cat >"$substitution_root/src/e2e/browser-storage-seed.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('browser storage cannot replace the real database seed', async ({ page }) => {
  await page.evaluate(() => {
    window.localStorage.setItem('tenant_id', 'synthetic-tenant');
    window.indexedDB.open('synthetic-offline-queue');
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated browser storage': src/e2e/browser-storage-seed.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/browser-storage-seed.spec.ts" \
    --ci "$substitution_root/src/e2e/browser-storage-seed.spec.ts"

cat >"$substitution_root/src/e2e/aliased-browser-storage.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('aliases and member assignments cannot seed browser storage', async ({ page }) => {
  await page.evaluate(() => {
    const storage = window.localStorage;
    const setTenant = storage.setItem.bind(storage);
    setTenant('tenant_id', 'synthetic-tenant');
    const session = window.sessionStorage;
    session['user_id'] = 'synthetic-user';
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated browser storage': src/e2e/aliased-browser-storage.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/aliased-browser-storage.spec.ts" \
    --ci "$substitution_root/src/e2e/aliased-browser-storage.spec.ts"

cat >"$substitution_root/src/e2e/file-chooser-bytes.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('file chooser uploads must use a tracked real image', async ({ page }) => {
  const chooserPromise = page.waitForEvent('filechooser');
  await page.locator('input[type=file]').click();
  const chooser = await chooserPromise;
  await chooser.setFiles({
    name: 'synthetic.png',
    mimeType: 'image/png',
    buffer: Buffer.from('synthetic image bytes'),
  });
});
SPEC
echo "Bypass" \
  "no-substitution category 'fabricated file bytes': src/e2e/file-chooser-bytes.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/file-chooser-bytes.spec.ts" \
    --ci "$substitution_root/src/e2e/file-chooser-bytes.spec.ts"

cat >"$substitution_root/src/e2e/init-script-substitution.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('init scripts cannot replace application behavior', async ({ page }) => {
  await page.addInitScript(() => { window.open = () => null; });
});
SPEC
echo "Bypass" \
  "no-substitution category 'injected page content': src/e2e/init-script-substitution.spec.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/init-script-substitution.spec.ts" \
    --ci "$substitution_root/src/e2e/init-script-substitution.spec.ts"

echo "Bypass" \
  "CI support source is missing or unreadable: src/e2e/missing-support.ts" \
  env SOURCE_REPO_ROOT="$substitution_root" "$SCRIPT" \
    --all "$substitution_root/src/e2e/allowed-real-stack.spec.ts" \
    --ci "$substitution_root/src/e2e/allowed-real-stack.spec.ts" \
    --support "$substitution_root/src/e2e/missing-support.ts"

scan_substitution_root="$TMP_ROOT/scan-substitution-runfiles"
mkdir -p "$scan_substitution_root/src/e2e"
cat >"$scan_substitution_root/src/e2e/intercept.spec.ts" <<'SPEC'
import { test } from '@playwright/test';
test('runfile interception', async ({ page }) => {
  await page.route('/api/v1/orders', () => {});
});
SPEC
echo "Bypass" \
  "no-substitution category 'network interception'" \
  env RUNFILES_ROOT="$scan_substitution_root" "$SCRIPT" --scan-runfiles

echo "Playwright coverage check tests passed."
