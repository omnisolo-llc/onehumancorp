import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { createRequire } from 'node:module';
import path from 'node:path';

// GHSA-wq5f-xc86-pv6w fixes the prebuilt librsvg dependency in sharp 0.35.5.
for (const directory of ['.', 'src/ui/next']) {
  test(`${directory} declares and resolves the reviewed patched image runtime`, async () => {
    const manifest = JSON.parse(await readFile(path.join(directory, 'package.json')));
    const lock = JSON.parse(await readFile(path.join(directory, 'package-lock.json')));
    assert.equal(manifest.overrides.sharp, '0.35.5');
    const entries = Object.entries(lock.packages).filter(([name]) => /(?:^|\/)node_modules\/sharp$/.test(name));
    assert.ok(entries.length > 0);
    for (const [name, entry] of entries) {
      assert.equal(entry.version, manifest.overrides.sharp, name);
      assert.ok(entry.integrity && entry.resolved === 'https://registry.npmjs.org/sharp/-/sharp-0.35.5.tgz');
    }
  });
  test(`${directory} actually decodes SVG with the patched prebuilt librsvg`, async () => {
    const require = createRequire(path.resolve(directory, 'package.json'));
    const sharp = require('sharp');
    assert.equal(sharp.versions.sharp, '0.35.5');
    assert.equal(sharp.versions.rsvg, '2.63.2', 'A globally supplied vulnerable librsvg must not conceal the package fix');
    const svg = Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="3" height="2"><rect width="3" height="2" fill="#ff0000"/></svg>');
    const { data, info } = await sharp(svg).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
    assert.equal(info.width, 3); assert.equal(info.height, 2); assert.equal(info.channels, 4);
    assert.deepEqual([...data], Array.from({ length: 6 }, () => [255, 0, 0, 255]).flat());
  });
}
