import { readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repository = fileURLToPath(new URL('..', import.meta.url));
export const advisoryEndpoint = 'https://registry.npmjs.org/-/npm/v1/security/advisories/bulk';

export async function verifySwaggerAssets(root = repository) {
  const directory = path.join(root, 'third_party/swagger-ui');
  const manifest = JSON.parse(await readFile(path.join(directory, 'manifest.json'), 'utf8'));
  const inventoryBytes = await readFile(path.join(directory, 'runtime-inventory.json'));
  const sha256 = bytes => createHash('sha256').update(bytes).digest('hex');
  if (sha256(inventoryBytes) !== manifest.runtimeInventorySha256) throw new Error('Swagger runtime inventory changed');
  const inventory = JSON.parse(inventoryBytes);
  if (inventory.sourceCommit !== manifest.upstream.commit || !/^[a-f0-9]{40}$/.test(inventory.sourceCommit)) throw new Error('Swagger source identity mismatch');
  if (!Array.isArray(inventory.packages) || inventory.packages.length === 0 || inventory.externals.length !== 0) throw new Error('Swagger runtime inventory is incomplete');
  if (!manifest.assets['dist/swagger-ui-bundle.js'] || !manifest.assets['dist/swagger-ui.css']) throw new Error('Swagger runtime assets are missing');
  for (const [name, hash] of Object.entries(manifest.assets)) {
    if (path.isAbsolute(name) || name.split('/').includes('..') || !/^[a-f0-9]{64}$/.test(hash)) throw new Error('Invalid Swagger asset identity');
    if (sha256(await readFile(path.join(root, 'src/ui/next/public/vendor/swagger-ui', name))) !== hash) throw new Error(`Swagger asset differs from the reviewed build: ${name}`);
  }
  if (!manifest.recipe?.['recipe/package-lock.json'] || !manifest.recipe?.['recipe/upstream.patch'] || !manifest.recipe?.['recipe/inventory.mjs'] || !manifest.recipe?.['recipe/rebuild.sh']) throw new Error('Swagger build recipe is missing');
  for (const [name, hash] of Object.entries(manifest.recipe)) {
    if (!name.startsWith('recipe/') || name.split('/').includes('..') || !/^[a-f0-9]{64}$/.test(hash)) throw new Error('Invalid Swagger recipe identity');
    if (sha256(await readFile(path.join(directory, name))) !== hash) throw new Error(`Swagger build recipe changed: ${name}`);
  }
  const source = JSON.parse(await readFile(path.join(directory, 'recipe/package-lock.json'), 'utf8'));
  if (manifest.upstream.name !== 'swagger-ui' || manifest.upstream.name !== source.name || manifest.upstream.version !== source.version || manifest.upstream.tag !== `v${source.version}`) throw new Error('Swagger source package identity mismatch');
  const request = Object.assign(Object.create(null), { [source.name]: [source.version] });
  for (const dependency of inventory.packages) {
    if (!dependency.name || !/^\d+\.\d+\.\d+(?:[-+].+)?$/.test(dependency.version) || !dependency.integrity || !dependency.resolved?.startsWith('https://registry.npmjs.org/')) throw new Error('Unbound Swagger runtime dependency');
    if (['argparse', 'sprintf-js'].includes(dependency.name)) throw new Error('The CLI-only vulnerable graph must not be shipped');
    (request[dependency.name] ??= []).push(dependency.version);
  }
  if (JSON.stringify(request.dompurify) !== JSON.stringify(['3.4.16'])) throw new Error('The reviewed patched DOMPurify must be present');
  return request;
}

export async function auditSwaggerRuntime({ root = repository, request = fetch } = {}) {
  const dependencies = await verifySwaggerAssets(root);
  const response = await request(advisoryEndpoint, {
    method: 'POST', headers: { 'content-type': 'application/json' },
    body: JSON.stringify(dependencies), signal: AbortSignal.timeout(30_000),
  });
  if (!response.ok) throw new Error(`Swagger runtime advisory check failed: HTTP ${response.status}`);
  const advisories = await response.json();
  if (!advisories || typeof advisories !== 'object' || Array.isArray(advisories) || Object.entries(advisories).some(([name, value]) => !Object.hasOwn(dependencies, name) || !Array.isArray(value))) throw new Error('Invalid Swagger advisory response');
  const affected = Object.entries(advisories).filter(([, entries]) => entries.length > 0).map(([name]) => name);
  if (affected.length) throw new Error(`Swagger runtime advisories remain: ${affected.join(', ')}`);
  return Object.values(dependencies).reduce((total, versions) => total + versions.length, 0);
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  auditSwaggerRuntime().then(count => console.log(`Audited ${count} source-bound Swagger runtime package versions: no advisories`)).catch(error => {
    console.error(error.message);
    process.exitCode = 1;
  });
}
