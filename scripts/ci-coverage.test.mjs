import test from 'node:test';
import assert from 'node:assert/strict';
import { verifyCoverage } from './ci-coverage.mjs';

const identity = { sha: 'a'.repeat(40), runId: '42', attempt: 1 };
const passed = id => ({ id, outcome: 'passed', attempts: 1 });
const report = (shardIndex, selectedIds) => ({
  schemaVersion: 1, ...identity, shardIndex, shardTotal: 2,
  selectedIds, finished: selectedIds.map(passed), complete: true,
});
const pair = () => [report(1, ['a']), report(2, ['b'])];
const check = reports => verifyCoverage(['a', 'b'], reports, identity, 2);

test('complete disjoint coverage is accepted', () => {
  assert.deepEqual(check(pair()), { selected: 2, passed: 2, shards: 2 });
});
test('rejects a lost shard even when every available result passed', () => {
  assert.throws(() => check([pair()[0]]), /shard/i);
});
test('rejects a selected test missing from execution', () => {
  const reports = pair(); reports[1].finished = [];
  assert.throws(() => check(reports), /unfinished/i);
});
test('rejects discovery lost through a narrowed test directory', () => {
  assert.throws(() => verifyCoverage(['a', 'b', 'nested-test'], pair(), identity, 2), /missing/i);
});
test('rejects an unknown test result', () => {
  const reports = pair(); reports[1].finished.push(passed('not-selected'));
  assert.throws(() => check(reports), /unexpected/i);
});
test('rejects duplicate test identity across shards', () => {
  const reports = pair(); reports[1] = report(2, ['a', 'b']);
  assert.throws(() => check(reports), /duplicate/i);
});
test('rejects duplicate expected identity', () => {
  assert.throws(() => verifyCoverage(['a', 'a'], pair(), identity, 2), /duplicate/i);
});
test('rejects duplicate results rather than treating a retry as another test', () => {
  const reports = pair(); reports[1].finished.push(passed('b'));
  assert.throws(() => check(reports), /duplicate/i);
});
test('rejects duplicate shard indices', () => {
  const reports = pair(); reports[1].shardIndex = 1;
  assert.throws(() => check(reports), /duplicate/i);
});
test('rejects inconsistent or reduced shard counts', () => {
  const reports = pair(); reports[1].shardTotal = 1;
  assert.throws(() => check(reports), /shard/i);
});
for (const [field, value] of [['sha', 'b'.repeat(40)], ['runId', '43'], ['attempt', 2]]) {
  test(`rejects a result from a different ${field}`, () => {
    const reports = pair(); reports[1][field] = value;
    assert.throws(() => check(reports), /identity/i);
  });
}
for (const outcome of ['failed', 'timedOut', 'interrupted', 'skipped', 'flaky', 'unknown']) {
  test(`rejects ${outcome} tests`, () => {
    const reports = pair(); reports[1].finished[0].outcome = outcome;
    assert.throws(() => check(reports), /did not pass/i);
  });
}
test('rejects a passing retry as first-attempt qualification', () => {
  const reports = pair(); reports[1].finished[0].attempts = 2;
  assert.throws(() => check(reports), /first attempt/i);
});
test('rejects interrupted or incomplete reports', () => {
  const reports = pair(); reports[0].complete = false;
  assert.throws(() => check(reports), /incomplete/i);
});
test('rejects zero-test qualification', () => {
  assert.throws(() => verifyCoverage([], [report(1, [])], identity, 1), /empty/i);
});
test('rejects missing, malformed and non-hosted identity', () => {
  for (const bad of [{}, { ...identity, sha: 'main' }, { ...identity, runId: 'local' }, { ...identity, attempt: 0 }]) {
    assert.throws(() => verifyCoverage(['a', 'b'], pair(), bad, 2), /identity/i);
  }
});
test('rejects invalid shard index and report schema', () => {
  for (const patch of [{ shardIndex: 0 }, { shardIndex: 3 }, { schemaVersion: 0 }]) {
    const reports = pair(); Object.assign(reports[0], patch);
    assert.throws(() => check(reports));
  }
});
test('rejects a discovery-only report used as completed execution', () => {
  const reports = pair(); delete reports[0].finished;
  assert.throws(() => check(reports), /results/i);
});
