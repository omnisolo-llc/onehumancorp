import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtemp, writeFile, rm } from 'node:fs/promises';
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
