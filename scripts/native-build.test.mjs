import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, mkdir, writeFile, readFile, rm } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { packageWeb, distributablePath, webLayout, sourceDigest, validateWebArtifact, recordSuccessfulBuild } from './package-web.mjs';

test('native web bundle excludes environment files and build caches', () => {
  for (const name of ['.env', '.env.local', '.env.production', '.env.production.local']) {
    assert.equal(distributablePath(path.join('/app', name)), false);
  }
  assert.equal(distributablePath(path.join('/app', '.next', 'cache', 'build')), false);
  assert.equal(distributablePath(path.join('/app', 'node_modules', 'dotenv', 'index.js')), true);
});

test('packaging refuses missing fresh build instead of using tracked exports', async () => {
  const dir = await mkdtemp(path.join(os.tmpdir(), 'ohc-package-'));
  try {
    await mkdir(path.join(dir, 'src/ui/tauri/next_out'), { recursive: true });
    await writeFile(path.join(dir, 'src/ui/tauri/next_out/index.html'), 'old export');
    await assert.rejects(packageWeb(dir));
  } finally { await rm(dir, { recursive: true, force: true }); }
});

test('fresh standalone package preserves server routes and assets, not stale files', async () => {
  const dir = await mkdtemp(path.join(os.tmpdir(), 'ohc-package-'));
  try {
    const layout = webLayout(dir);
    const server = path.join(layout.standalone, layout.relativeServer);
    await mkdir(path.dirname(server), { recursive: true });
    await writeFile(server, '/* test server build artifact */');
    await writeFile(path.join(layout.source, '.next/BUILD_ID'), 'verified-build-id');
    await writeFile(path.join(layout.source, 'package-lock.json'), '{"lockfileVersion":3}');
    await mkdir(path.join(layout.source, '.next/static'), { recursive: true });
    await writeFile(path.join(layout.source, '.next/static/app.js'), 'asset');
    await writeFile(path.join(layout.standalone, '.env.production'), 'must-not-ship');
    await mkdir(path.join(layout.source, 'public'), { recursive: true });
    await writeFile(path.join(layout.source, 'public', '.env.local'), 'public-directory-secret-must-not-ship');
    await writeFile(path.join(layout.source, '.next/static', '.env.production'), 'static-directory-secret-must-not-ship');
    await writeFile(path.join(layout.source, 'public', 'robots.txt'), 'User-agent: *');
    await mkdir(layout.destination, { recursive: true });
    await writeFile(path.join(layout.destination, 'stale.html'), 'stale');
    await assert.rejects(packageWeb(dir), /build proof is missing/);
    // Explicit test compiler output, not a claimed production build.
    await recordSuccessfulBuild(await sourceDigest(dir), dir);
    await packageWeb(dir);
    assert.equal(existsSync(path.join(layout.destination, 'stale.html')), false);
    assert.equal(existsSync(path.join(layout.destination, '.env.production')), false);
    assert.equal(existsSync(path.join(layout.destination, 'src/ui/next/public/.env.local')), false);
    assert.equal(existsSync(path.join(layout.destination, 'src/ui/next/.next/static/.env.production')), false);
    assert.equal(await readFile(path.join(layout.destination, 'src/ui/next/public/robots.txt'), 'utf8'), 'User-agent: *');
    assert.equal(existsSync(path.join(layout.destination, layout.relativeServer)), true);
    assert.equal(await readFile(path.join(layout.destination, 'src/ui/next/.next/static/app.js'), 'utf8'), 'asset');
    const manifest = JSON.parse(await readFile(path.join(layout.destination, 'build-manifest.json'), 'utf8'));
    assert.equal(manifest.buildId, 'verified-build-id');
    assert.equal(manifest.node, process.versions.node);
    assert.match(manifest.packageLockSha256, /^[0-9a-f]{64}$/);
    assert.equal(manifest.sourceSha256, await sourceDigest(dir));
    await validateWebArtifact(layout.destination, dir);
    await assert.rejects(validateWebArtifact(layout.destination, dir, {
      platform: process.platform, arch: 'wrong-architecture', versions: process.versions,
    }), /does not match/);
    await writeFile(path.join(layout.source, 'changed-page.tsx'), 'new source with the same dependency lock');
    await assert.rejects(validateWebArtifact(layout.destination, dir), /does not match/);
    await assert.rejects(packageWeb(dir), /Stale or foreign/);
    await assert.rejects(recordSuccessfulBuild(manifest.sourceSha256, dir), /changed during build/);
  } finally { await rm(dir, { recursive: true, force: true }); }
});
