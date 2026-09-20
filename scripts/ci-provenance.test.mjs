import test from 'node:test';
import assert from 'node:assert/strict';
import { browserCiEnvironment, testEnvironment, validateCiDiscovery, discoveryFilename } from './native-e2e.mjs';
const source = { GITHUB_ACTIONS: 'true', GITHUB_SHA: 'a'.repeat(40), GITHUB_RUN_ID: '123',
  GITHUB_RUN_ATTEMPT: '2', GITHUB_TOKEN: 'do-not-forward', DATABASE_URL: 'do-not-forward' };
test('only validated hosted identity reaches browser evidence', () => {
  const env = browserCiEnvironment(source);
  assert.deepEqual(env, { OMNISOLO_CI_IDENTITY: JSON.stringify({ sha: source.GITHUB_SHA, runId: '123', attempt: 2 }) });
  assert.equal(testEnvironment(source).GITHUB_TOKEN, undefined);
});
test('local runs do not impersonate hosted validation', () => {
  assert.deepEqual(browserCiEnvironment({ ...source, GITHUB_ACTIONS: undefined }), {});
});
for (const key of ['GITHUB_SHA', 'GITHUB_RUN_ID', 'GITHUB_RUN_ATTEMPT']) test(`missing ${key} fails hosted provenance`, () => {
  assert.throws(() => browserCiEnvironment({ ...source, [key]: '' }), /identity/);
});

// Real --list output can be truncated when the complete suite is large. Hosted
// preflight must use the reporter's actual selection, not a console substring.

test('structured selection validates even without a console Total line', () => {
  const identity = { sha: source.GITHUB_SHA, runId: '123', attempt: 2 };
  assert.equal(validateCiDiscovery({ schemaVersion: 1, mode: 'discovery', ...identity,
    shardIndex: 1, shardTotal: 1, selectedIds: ['nested:a', 'root:b'] }, identity, []), 2);
  for (const bad of [{}, { schemaVersion: 1, mode: 'discovery', ...identity,
    shardIndex: 1, shardTotal: 1, selectedIds: [] }]) {
    assert.throws(() => validateCiDiscovery(bad, identity, []));
  }
});
test('stale or wrong-shard discovery never certifies current preflight', () => {
  const identity = { sha: source.GITHUB_SHA, runId: '123', attempt: 2 };
  const report = { schemaVersion: 1, mode: 'discovery', ...identity,
    shardIndex: 1, shardTotal: 32, selectedIds: ['a'] };
  assert.equal(validateCiDiscovery(report, identity, ['--shard=1/32']), 1);
  assert.throws(() => validateCiDiscovery(report, identity, ['--shard=2/32']));
  assert.throws(() => validateCiDiscovery({ ...report, attempt: 1 }, identity, ['--shard=1/32']));
  assert.throws(() => validateCiDiscovery({ ...report, selectedIds: ['a', 'a'] }, identity, ['--shard=1/32']));
});
test('discovery filenames follow the exact command selection', () => {
  assert.equal(discoveryFilename([]), 'selection-all.json');
  assert.equal(discoveryFilename(['--shard=3/32']), 'selection-3-of-32.json');
  assert.equal(discoveryFilename(['--shard', '3/32']), 'selection-3-of-32.json');
  for (const args of [['--shard=0/32'], ['--shard=33/32'], ['--shard'], ['--shard=1/2', '--shard=2/2']]) {
    assert.throws(() => discoveryFilename(args));
  }
});
