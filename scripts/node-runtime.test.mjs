import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { NODE_VERSION, nodeDistribution, verifyArchive } from './node-runtime.mjs';

test('every supported native desktop has an exact pinned official distribution', () => {
  const hashes = new Set();
  for (const platform of ['linux', 'darwin', 'win32']) for (const architecture of ['x64', 'arm64']) {
    const d = nodeDistribution(NODE_VERSION, platform, architecture);
    assert.match(d.sha256, /^[0-9a-f]{64}$/);
    assert.equal(new URL(d.url).origin, 'https://nodejs.org');
    assert.equal(d.architecture, architecture);
    hashes.add(d.sha256);
  }
  assert.equal(hashes.size, 6);
  assert.throws(() => nodeDistribution('99.0.0', 'linux', 'x64'));
  assert.throws(() => nodeDistribution(NODE_VERSION, 'linux', 'riscv64'));
  assert.throws(() => nodeDistribution(NODE_VERSION, '../linux', 'x64'));
});

test('cache corruption and mismatched distributions fail before extraction', async () => {
  const directory = await mkdtemp(path.join(tmpdir(), 'ohc-node-integrity-'));
  try {
    const file = path.join(directory, 'archive');
    await writeFile(file, 'fixture');
    const digest = createHash('sha256').update('fixture').digest('hex');
    await verifyArchive(file, digest);
    await assert.rejects(verifyArchive(file, '0'.repeat(64)), /checksum mismatch/);
    await writeFile(file, '');
    await assert.rejects(verifyArchive(file, digest), /invalid size/);
  } finally { await rm(directory, { recursive: true, force: true }); }
});
