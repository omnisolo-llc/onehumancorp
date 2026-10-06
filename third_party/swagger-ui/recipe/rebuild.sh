#!/usr/bin/env bash
set -euo pipefail
recipe=$(cd "$(dirname "$0")" && pwd)
repository=$(cd "$recipe/../../.." && pwd)
work=${1:?Provide an absolute, new build directory}
[[ "$work" == /* && ! -e "$work" ]] || { echo 'Build directory must be absolute and must not already exist' >&2; exit 1; }
[[ $(node --version) == v24.21.0 && $(npm --version) == 11.19.0 ]] || { echo 'Use official Node 24.21.0 with bundled npm 11.19.0' >&2; exit 1; }
[[ $(node -p 'process.platform + "/" + process.arch') == linux/x64 ]] || { echo 'The verified recipe targets Linux x64' >&2; exit 1; }
git clone --depth 1 --branch v5.33.1 https://github.com/swagger-api/swagger-ui.git "$work"
[[ $(git -C "$work" rev-parse HEAD) == cac3d136b5e37bfbe3288c8591f6f4fcf9870599 ]] || { echo 'Unexpected upstream source commit' >&2; exit 1; }
git -C "$work" apply --unidiff-zero --check "$recipe/upstream.patch"
git -C "$work" apply --unidiff-zero "$recipe/upstream.patch"
cp "$recipe/package-lock.json" "$work/package-lock.json"
export SOURCE_DATE_EPOCH=1790846452 SCARF_ANALYTICS=false
cd "$work"
npm ci --ignore-scripts --no-audit --no-fund
npm run build:bundle -- --profile --json "$work/runtime-webpack-stats.json"
npm run build-stylesheets
node "$recipe/inventory.mjs" "$work" "$work/runtime-webpack-stats.json" "$work/inventory"
node --input-type=module - "$repository" "$work" <<'JS'
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
import path from 'node:path';
const [repository, work] = process.argv.slice(2);
const manifest = JSON.parse(readFileSync(path.join(repository, 'third_party/swagger-ui/manifest.json')));
assert.equal(createHash('sha256').update(readFileSync(path.join(work, 'inventory/runtime-inventory.json'))).digest('hex'), manifest.runtimeInventorySha256, 'Runtime inventory differs from reviewed emitted modules');
assert.equal(createHash('sha256').update(readFileSync(path.join(work, 'inventory/THIRD_PARTY_LICENSES.txt'))).digest('hex'), manifest.assets['THIRD_PARTY_LICENSES.txt'], 'Runtime license inventory changed');
for (const [name, expected] of Object.entries(manifest.assets)) {
  if (!name.startsWith('dist/')) continue;
  const actual = createHash('sha256').update(readFileSync(path.join(work, name))).digest('hex');
  assert.equal(actual, expected, `Nonreproducible Swagger asset: ${name}`);
}
console.log('Rebuilt Swagger distribution matches every pinned runtime asset hash');
JS
