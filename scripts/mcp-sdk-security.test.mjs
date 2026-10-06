import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const patchedVersion = '1.31.0';
const sdk = '@modelcontextprotocol/sdk';
const readJson = relative => readFile(new URL(`../${relative}`, import.meta.url), 'utf8').then(JSON.parse);

// GHSA-6qxp-vccf-f47h affects SDK 1.12.0 through 1.30.1.
// https://github.com/advisories/GHSA-6qxp-vccf-f47h
// Keep the independently installed npm applications on the same patched policy.
for (const directory of ['', 'src/ui/next/']) {
  test(`${directory || 'root/'}npm override pins the OAuth issuer-binding fix`, async () => {
    const manifest = await readJson(`${directory}package.json`);
    assert.equal(manifest.overrides[sdk], patchedVersion);
  });

  test(`${directory || 'root/'}npm lock cannot resolve an unpatched MCP SDK`, async () => {
    const lock = await readJson(`${directory}package-lock.json`);
    const entries = Object.entries(lock.packages).filter(([name]) => name.endsWith(`/node_modules/${sdk}`) || name === `node_modules/${sdk}`);
    if (!directory) assert.ok(entries.length > 0, 'the root GitHub MCP server must retain its SDK dependency');
    for (const [name, entry] of entries) assert.equal(entry.version, patchedVersion, name);
    if (lock.dependencies?.[sdk]) assert.equal(lock.dependencies[sdk].version, patchedVersion);
  });
}
