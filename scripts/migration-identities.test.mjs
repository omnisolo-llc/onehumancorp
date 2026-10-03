import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtemp, writeFile, rm, readFile, readdir } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = fileURLToPath(new URL('..', import.meta.url));
const checker = path.join(root, 'src/server/migrations/sqlx_migration_contract_test.sh');

test('canonical runtime migrations have unique SQLx versions and preserve schema contracts', () => {
  const result = spawnSync('bash', [checker], { cwd: root, encoding: 'utf8', timeout: 10_000 });
  assert.ifError(result.error);
  assert.equal(result.status, 0, result.stderr);
});

test('the POS collision repair preserves historical SQLx checksums', async () => {
  const directory = path.join(root, 'src/server/migrations');
  const names = await readdir(directory);
  // SQLx records SHA-384 over the original SQL bytes. Change schema through an
  // additive migration rather than rewriting either upstream chat migration.
  const checksums = {
    '233_chat_omnichannel.sql': '97321c6b3f5ab689eeffea31a3fa5c3d396563337774a3825d32faaa048cfbfdd3b4c3e5714977ad7f8caec34f0bf789',
    '236_pos_offline_request_identity.sql': 'fb36c12db63a07f133ac6be6037beb896292d62d16f612177d831e7d1b241503e2b58bfd27719e8dc69b1255f4a6b20a',
    '1009_native_omnichannel_chat.sql': 'aaa15a53375f57bb82011b2aae513f05335fb7e543e3f1136cd5cb5decbc7a4a3d9b8fdc46f248de33de664da33c8b00',
    '1025_usage_accounting.sql': '239d56c1459bf57717aeb184d01e6087889b728c99cbd62ee2329f7083a4a1591c6e764978871c613d8880362568b3b8',
  };
  for (const [name, checksum] of Object.entries(checksums)) {
    assert.ok(names.includes(name), `preserve the migration identity ${name}`);
    const sql = await readFile(path.join(directory, name));
    assert.equal(createHash('sha384').update(sql).digest('hex'), checksum, name);
  }
});

test('original chat DDL is not replayed under a second migration identity', async () => {
  const directory = path.join(root, 'src/server/migrations');
  const original = await readFile(path.join(directory, '233_chat_omnichannel.sql'));
  const originalChecksum = createHash('sha384').update(original).digest('hex');
  const copies = [];
  for (const name of (await readdir(directory)).filter((name) => name.endsWith('.sql'))) {
    const sql = await readFile(path.join(directory, name));
    if (createHash('sha384').update(sql).digest('hex') === originalChecksum) copies.push(name);
  }
  // Replaying this SQL fails at its unconditional CREATE POLICY statements.
  // Moving a duplicate to a free version would hide the identity collision
  // while still breaking startup; the original migration remains authoritative.
  assert.deepEqual(copies, ['233_chat_omnichannel.sql']);
});

test('POS and chat additions retain SQLx numeric dependency order after integration', async () => {
  const names = await readdir(path.join(root, 'src/server/migrations'));
  const relevant = names.filter((name) => /_(?:pos_offline_transactions|chat_omnichannel|pos_offline_request_identity|native_omnichannel_chat|chat_sender_identity_text)\.sql$/.test(name));
  relevant.sort((left, right) => Number.parseInt(left, 10) - Number.parseInt(right, 10));
  assert.deepEqual(relevant, [
    '076_pos_offline_transactions.sql',
    '233_chat_omnichannel.sql',
    '236_pos_offline_request_identity.sql',
    '1009_native_omnichannel_chat.sql',
    '1021_chat_sender_identity_text.sql',
  ]);
});

for (const names of [
  ['1012_first.sql', '1012_second.sql'],
  ['1_first.sql', '001_second.sql'],
  ['7_simple.sql', '007_other.up.sql'],
]) {
  test(`duplicate identities fail before database access: ${names.join(', ')}`, async () => {
    const dir = await mkdtemp(path.join(os.tmpdir(), 'ohc-migrations-'));
    try {
      for (const name of names) await writeFile(path.join(dir, name), 'SELECT 1;\n');
      const result = spawnSync('bash', [checker, dir], { cwd: root, encoding: 'utf8', timeout: 10_000 });
      assert.ifError(result.error);
      assert.equal(result.status, 1);
      assert.match(result.stderr, /duplicate numeric version/);
    } finally { await rm(dir, { recursive: true, force: true }); }
  });
}

test('invalid and out-of-range versions are rejected before SQL execution', async () => {
  for (const name of ['unversioned.sql', '9223372036854775808_overflow.sql']) {
    const dir = await mkdtemp(path.join(os.tmpdir(), 'ohc-migrations-'));
    try {
      await writeFile(path.join(dir, name), 'SELECT 1;\n');
      const result = spawnSync('bash', [checker, dir], { cwd: root, encoding: 'utf8', timeout: 10_000 });
      assert.ifError(result.error);
      assert.equal(result.status, 1);
      assert.match(result.stderr, /Invalid SQLx migration identity/);
    } finally { await rm(dir, { recursive: true, force: true }); }
  }
});


test('native chat migration stays in the source actually embedded and watched by the server', async () => {
  const db = await readFile(path.join(root, 'src/server/db.rs'), 'utf8');
  const build = await readFile(path.join(root, 'src/server/build.rs'), 'utf8');
  const embedded = db.match(/static POSTGRES_MIGRATOR[^;]+sqlx::migrate!\("([^"]+)"\)/);
  assert.ok(embedded, 'the actual embedded migration source must be discoverable');
  const directory = path.resolve(root, embedded[1]);
  assert.match(build, /cargo:rerun-if-changed=src\/server\/migrations/);
  const sql = await readFile(path.join(directory, '1009_native_omnichannel_chat.sql'), 'utf8');
  for (const table of ['inboxes','channels','contacts','conversations','messages']) {
    assert.ok(sql.includes(`CREATE TABLE IF NOT EXISTS chat_${table}`), table);
    assert.ok(sql.includes(`ALTER TABLE chat_${table} ENABLE ROW LEVEL SECURITY`), table);
  }
});
