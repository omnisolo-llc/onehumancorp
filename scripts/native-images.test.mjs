import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

test('production image artifacts retain source, checksum, tag and image-ID boundaries', () => {
  const result = spawnSync('python3', [fileURLToPath(new URL('./native-images-test.py', import.meta.url))], {
    encoding: 'utf8', timeout: 15000,
  });
  assert.ifError(result.error);
  assert.equal(result.status, 0, result.stdout + result.stderr);
  assert.match(result.stderr, /Ran 8 tests/);
  assert.match(result.stderr, /OK/);
});
