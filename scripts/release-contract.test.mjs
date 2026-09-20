import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

test('release identity, locked version stamping and complete asset assembly', () => {
  const result = spawnSync('python3', [fileURLToPath(new URL('./release_contract_test.py', import.meta.url))], {
    encoding: 'utf8', timeout: 30000,
  });
  assert.ifError(result.error);
  assert.equal(result.status, 0, result.stdout + result.stderr);
});
