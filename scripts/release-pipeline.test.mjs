import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

test('official desktop outputs and draft-first release verification', () => {
  const result = spawnSync('python3', [fileURLToPath(new URL('./release_pipeline_test.py', import.meta.url))], {
    encoding: 'utf8', timeout: 30000,
  });
  assert.ifError(result.error);
  assert.equal(result.status, 0, result.stdout + result.stderr);
});
