import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile, readdir } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { NODE_VERSION, nodeDistribution } from './node-runtime.mjs';

const root = fileURLToPath(new URL('../', import.meta.url));
const tauriManifest = JSON.parse(execFileSync('python3', ['-c',
  'import json,tomllib; print(json.dumps(tomllib.load(open("src/ui/tauri/Cargo.toml", "rb"))))',
], { cwd: root, encoding: 'utf8' }));

test('native Node pin and all archive digests match the reviewed official release', async () => {
  // Snapshot retrieved over HTTPS from nodejs.org/dist/v22.23.3/SHASUMS256.txt.
  // Keep upstream evidence with the pin; do not fetch mutable data during tests.
  const sums = await readFile(new URL('./test-support/node-releases/v22.23.3-SHASUMS256.txt', import.meta.url), 'utf8');
  const official = new Map(sums.trim().split('\n').map((line) => {
    const [sha256, archive] = line.trim().split(/\s+/);
    return [archive, sha256];
  }));
  assert.equal(NODE_VERSION, '22.23.3');
  assert.equal((await readFile(new URL('../.node-version', import.meta.url), 'utf8')).trim(), NODE_VERSION);
  for (const platform of ['linux', 'darwin', 'win32']) {
    for (const architecture of ['x64', 'arm64']) {
      const distribution = nodeDistribution(NODE_VERSION, platform, architecture);
      assert.equal(distribution.sha256, official.get(distribution.archive), distribution.archive);
    }
  }
});

test('native URL parsing reuses Tauri without a direct HTTP client dependency', () => {
  assert.equal(tauriManifest.dependencies.reqwest, undefined);
});

test('updater dependency is limited to desktop targets', () => {
  assert.equal(tauriManifest.dependencies['tauri-plugin-updater'], undefined);
  assert.ok(tauriManifest.target['cfg(any(target_os = "macos", windows, target_os = "linux"))']
    .dependencies['tauri-plugin-updater']);
});

test('updater permissions remain desktop-only and unavailable to remote pages', async () => {
  const directory = new URL('../src/ui/tauri/capabilities/', import.meta.url);
  const capabilities = await Promise.all((await readdir(directory)).filter((name) => name.endsWith('.json'))
    .map(async (name) => JSON.parse(await readFile(new URL(name, directory), 'utf8'))));
  const updaterCapabilities = capabilities.filter((capability) => capability.permissions
    .some((permission) => (typeof permission === 'string' ? permission : permission.identifier).startsWith('updater:')));
  assert.ok(updaterCapabilities.length > 0, 'desktop retains updater access for bundled code');
  for (const capability of updaterCapabilities) {
    assert.deepEqual([...capability.platforms ?? []].sort(), ['linux', 'macOS', 'windows']);
    assert.equal(capability.remote, undefined, 'hosted and loopback pages must not receive updater IPC access');
    assert.deepEqual(capability.windows, ['main']);
  }
  const common = capabilities.find((capability) => capability.identifier === 'default');
  assert.ok(common.permissions.includes('core:default'));
  assert.equal(common.platforms, undefined, 'mobile retains its common core capability');
});

test('CLI DOM assertions are available only as development dependencies', async () => {
  const manifest = JSON.parse(await readFile(new URL('../src/cli/package.json', import.meta.url), 'utf8'));
  const lock = JSON.parse(await readFile(new URL('../src/cli/package-lock.json', import.meta.url), 'utf8'));
  assert.equal(manifest.dependencies['@testing-library/jest-dom'], undefined);
  assert.ok(manifest.devDependencies['@testing-library/jest-dom']);
  assert.equal(lock.packages['node_modules/@testing-library/jest-dom'].dev, true);
});
