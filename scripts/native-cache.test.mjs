import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

test('CI setup actions preserve cache scopes and all required correctness gates', () => {
  const result = spawnSync('python3', [fileURLToPath(new URL('../.github/scripts/check_native_cache_test.py', import.meta.url))], {
    encoding: 'utf8', timeout: 30000,
  });
  assert.ifError(result.error);
  assert.equal(result.status, 0, result.stdout + result.stderr);
});
