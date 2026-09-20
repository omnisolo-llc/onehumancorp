import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

const read = (file) => readFile(new URL(`../${file}`, import.meta.url), 'utf8');

test('Windows SQLCipher uses a bundled crypto library, including GNU archives', async () => {
  const cargo = await read('Cargo.toml');
  assert.ok(cargo.includes("[target.'cfg(windows)'.dependencies.libsqlite3-sys]"));
  assert.ok(cargo.includes('bundled-sqlcipher-vendored-openssl'));
  const release = await read('.github/workflows/release.yml');
  assert.ok(release.includes('target: x86_64-pc-windows-gnu'));
  assert.ok(release.includes('msys2/setup-msys2@v2'));
});

test('every release build consumes the shared release identity', async () => {
  const workflow = await read('.github/workflows/release.yml');
  assert.match(workflow, /release-metadata:/);
  assert.doesNotMatch(workflow, /VERSION="\$\{GITHUB_REF_NAME#v\}"/);
  assert.match(workflow, /uses: \.\/\.github\/actions\/prepare-release/);
  assert.match(await read('.github/actions/prepare-release/action.yml'), /release_contract\.py prepare/);
});

test('publishing includes the complete updater before making the release public', async () => {
  const workflow = await read('.github/workflows/release.yml');
  assert.match(workflow, /release_contract\.py assemble/);
  assert.match(workflow, /target_commitish:/);
  assert.match(workflow, /fail_on_unmatched_files: true/);
  assert.doesNotMatch(workflow, /assemble-latest-json:/);
});

test('release publication is gated by the same complete CI workflow', async () => {
  const workflow = await read('.github/workflows/release.yml');
  const ci = await read('.github/workflows/ci.yml');
  assert.match(workflow, /uses: \.\/\.github\/workflows\/ci\.yml/);
  assert.match(ci, /workflow_call:/);
});
