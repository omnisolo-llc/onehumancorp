import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';

const [rootArgument, statsFile, output] = process.argv.slice(2);
assert.ok(rootArgument && statsFile && output, 'Usage: inventory.mjs UPSTREAM STATS OUTPUT_DIRECTORY');
const root = path.resolve(rootArgument);
const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim();
assert.equal(sourceCommit, 'cac3d136b5e37bfbe3288c8591f6f4fcf9870599');
const stats = JSON.parse(fs.readFileSync(statsFile));
const lock = JSON.parse(fs.readFileSync(path.join(root, 'package-lock.json')));
assert.deepEqual(stats.errors, []);
assert.deepEqual(stats.warnings, []);
const packages = new Map();
const externals = new Set();
let includedModuleEntries = 0;
function visit(module, parentIncluded = false) {
  const included = parentIncluded || module.chunks?.length > 0;
  if (included) {
    includedModuleEntries += 1;
    const identifier = (module.identifier || '').split('!').at(-1);
    const offset = identifier.indexOf(root + '/node_modules/');
    if (offset >= 0) {
      const value = identifier.slice(offset).split('|')[0];
      const last = value.lastIndexOf('/node_modules/');
      const parts = value.slice(last + '/node_modules/'.length).split('/');
      const count = parts[0].startsWith('@') ? 2 : 1;
      const directory = value.slice(0, last + '/node_modules/'.length) + parts.slice(0, count).join('/');
      const manifest = JSON.parse(fs.readFileSync(path.join(directory, 'package.json')));
      const relative = path.relative(root, directory);
      const pinned = lock.packages[relative];
      assert.equal(pinned?.version, manifest.version, `Unbound dependency: ${relative}`);
      assert.ok(pinned.integrity && pinned.resolved.startsWith('https://registry.npmjs.org/'));
      const key = manifest.name + '@' + manifest.version;
      let entry = packages.get(key);
      if (!entry) {
        entry = { name: manifest.name, version: manifest.version, license: manifest.license ?? manifest.licenses,
          integrity: pinned.integrity, resolved: pinned.resolved, sourcePackages: [], modules: [] };
        packages.set(key, entry);
      }
      if (!entry.sourcePackages.includes(relative)) entry.sourcePackages.push(relative);
      entry.modules.push(path.relative(root, value));
    } else if (identifier.startsWith('external ')) externals.add(identifier);
  }
  for (const child of module.modules || []) visit(child, included);
}
for (const module of stats.modules || []) visit(module);
assert.equal(externals.size, 0, 'The distribution must be self-contained');
const inventory = { sourceCommit, webpack: stats.version, errors: 0, warnings: [], includedModuleEntries,
  externals: [...externals], packages: [...packages.values()].sort((a, b) => a.name.localeCompare(b.name) || a.version.localeCompare(b.version)) };
fs.mkdirSync(output, { recursive: true });
fs.writeFileSync(path.join(output, 'runtime-inventory.json'), JSON.stringify(inventory, null, 2) + '\n');
const notices = ['Third-party notices for the emitted Swagger UI runtime.\nSource identities and exact versions: runtime-inventory.json.\nOriginal published license/notice wording follows; line endings and trailing whitespace are normalized.\nWhere a published tarball omits a license file, its declared metadata and applicable standard license text are retained explicitly.\n'];
for (const entry of inventory.packages) {
  notices.push('\n' + '='.repeat(80) + '\n' + entry.name + '@' + entry.version + '\n' + entry.resolved + '\nLicense: ' + JSON.stringify(entry.license) + '\n');
  for (const relative of entry.sourcePackages) {
    const directory = path.join(root, relative);
    const files = fs.readdirSync(directory).filter(name => /^(licen[cs]e|copying|notice)([.-]|$)/i.test(name) && fs.statSync(path.join(directory, name)).isFile()).sort();
    if (files.length) {
      for (const name of files) notices.push(`\n--- ${relative}/${name} ---\n` + fs.readFileSync(path.join(directory, name), 'utf8'));
    } else {
      const manifest = JSON.parse(fs.readFileSync(path.join(directory, 'package.json')));
      notices.push('\nPublished tarball contains no standalone license/notice file. Published attribution metadata:\n' + JSON.stringify({ author: manifest.author, contributors: manifest.contributors, license: manifest.license, licenses: manifest.licenses, repository: manifest.repository }, null, 2) + '\n');
      if (entry.name === 'format') notices.push(fs.readFileSync(path.join(directory, 'format.js'), 'utf8').split(';(function()')[0]);
      if (entry.license === 'Apache-2.0') notices.push('\nApache-2.0 license text (also provided with Swagger UI):\n' + fs.readFileSync(path.join(root, 'LICENSE'), 'utf8'));
      else if (entry.license === 'MIT' || entry.license?.some?.(license => license.type === 'MIT')) {
        // The MIT permission grant is invariant. Keep attribution separately rather
        // than inventing a copyright year absent from the published package.
        const mit = fs.readFileSync(path.join(root, 'node_modules/react/LICENSE'), 'utf8');
        notices.push('\nStandard MIT permission grant; published attribution above applies:\n' + mit.slice(mit.indexOf('Permission is hereby granted')));
      } else throw new Error(`Missing license text for ${entry.name}`);
    }
  }
}
fs.writeFileSync(path.join(output, 'THIRD_PARTY_LICENSES.txt'), notices.join('\n').replace(/\r\n/g, '\n').split('\n').map(line => line.trimEnd()).join('\n').trimEnd() + '\n');
console.log(`Inventoried ${inventory.packages.length} emitted package versions and preserved their license notices`);
