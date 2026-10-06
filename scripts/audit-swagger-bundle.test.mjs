import test from 'node:test';
import assert from 'node:assert/strict';
import { cp, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { advisoryEndpoint, auditSwaggerRuntime, verifySwaggerAssets } from './audit-swagger-bundle.mjs';

async function fixture(t) {
  const root = await mkdtemp(path.join(tmpdir(), 'swagger-audit-'));
  t.after(() => rm(root, { recursive: true, force: true }));
  for (const relative of ['third_party/swagger-ui', 'src/ui/next/public/vendor/swagger-ui']) await cp(relative, path.join(root, relative), { recursive: true });
  return root;
}
const json = (root, relative) => readFile(path.join(root, relative), 'utf8').then(JSON.parse);
const manifestPath = 'third_party/swagger-ui/manifest.json';
const inventoryPath = 'third_party/swagger-ui/runtime-inventory.json';
async function changeInventory(root, mutate) {
  const inventory = await json(root, inventoryPath); mutate(inventory);
  const bytes = JSON.stringify(inventory);
  await writeFile(path.join(root, inventoryPath), bytes);
  const manifest = await json(root, manifestPath);
  manifest.runtimeInventorySha256 = createHash('sha256').update(bytes).digest('hex');
  await writeFile(path.join(root, manifestPath), JSON.stringify(manifest));
}
test('audits all source-bound emitted versions, including the embedded sanitizer', async () => {
  let called = 0;
  const count = await auditSwaggerRuntime({ request: async (url, options) => {
    called += 1;
    assert.equal(url, advisoryEndpoint);
    assert.equal(options.method, 'POST');
    const versions = JSON.parse(options.body);
    assert.deepEqual(versions.dompurify, ['3.4.16']);
    assert.deepEqual(versions['swagger-ui'], ['5.33.1']);
    assert.equal(versions.argparse, undefined);
    assert.equal(versions['sprintf-js'], undefined);
    assert.equal(Object.values(versions).flat().length, 113);
    return { ok: true, json: async () => ({}) };
  } });
  assert.equal(count, 113); assert.equal(called, 1);
});
for (const [name, response, expected] of [
  ['HTTP failure', { ok: false, status: 503 }, /HTTP 503/],
  ['malformed success', { ok: true, json: async () => [] }, /Invalid/],
  ['invalid advisory records', { ok: true, json: async () => ({ dompurify: {} }) }, /Invalid/],
  ['unrequested package records', { ok: true, json: async () => ({ imaginary: [] }) }, /Invalid/],
  ['upstream Swagger advisories', { ok: true, json: async () => ({ 'swagger-ui': [{ severity: 'moderate' }] }) }, /advisories remain/],
  ['even low severity advisories', { ok: true, json: async () => ({ dompurify: [{ severity: 'low' }] }) }, /advisories remain/],
]) test(`refuses ${name}`, async () => { await assert.rejects(auditSwaggerRuntime({ request: async () => response }), expected); });
test('refuses unavailable advisory service', async () => { await assert.rejects(auditSwaggerRuntime({ request: async () => { throw new Error('offline'); } }), /offline/); });
for (const relative of ['src/ui/next/public/vendor/swagger-ui/dist/swagger-ui-bundle.js', 'third_party/swagger-ui/recipe/upstream.patch', inventoryPath]) test(`refuses altered ${relative}`, async t => {
  const root = await fixture(t); await writeFile(path.join(root, relative), 'altered');
  await assert.rejects(verifySwaggerAssets(root), /changed|differs/);
});
for (const [name, mutate] of [
  ['unbound package source', inventory => { inventory.packages[0].resolved = 'https://unknown.test/code.tgz'; }],
  ['external modules', inventory => { inventory.externals.push('external dependency'); }],
  ['missing sanitizer', inventory => { inventory.packages = inventory.packages.filter(entry => entry.name !== 'dompurify'); }],
  ['unpatched sanitizer', inventory => { inventory.packages.find(entry => entry.name === 'dompurify').version = '3.4.13'; }],
  ['CLI-only vulnerable graph', inventory => { inventory.packages[0].name = 'sprintf-js'; }],
]) test(`refuses ${name} even with an updated inventory hash`, async t => {
  const root = await fixture(t); await changeInventory(root, mutate); await assert.rejects(verifySwaggerAssets(root));
});

test('refuses an upstream identity that disagrees with the integrity-bound source lock', async t => {
  const root = await fixture(t); const manifest = await json(root, manifestPath);
  manifest.upstream.version = '5.33.2';
  await writeFile(path.join(root, manifestPath), JSON.stringify(manifest));
  await assert.rejects(verifySwaggerAssets(root), /source package identity/);
});
