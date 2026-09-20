import test from 'node:test';
import assert from 'node:assert/strict';
import { verifyCoverage, inventoryFromReport } from './ci-coverage.mjs';

const identity = { sha: 'a'.repeat(40), runId: '42', attempt: '1' };
const report = (index, ids, overrides = {}) => ({
  config: { metadata: { ...identity }, shard: { current: index, total: 2 } },
  errors: [],
  suites: [{ title: 'nested', suites: [{ title: 'browser', specs: ids.map(id => ({
    id, title: id, file: 'src/ui/next/src/e2e/nested.spec.ts',
    tests: [{ projectName: 'chromium', expectedStatus: 'passed', status: 'expected',
      results: [{ status: 'passed' }] }],
  })) }] }],
  ...overrides,
});
const inventory = () => inventoryFromReport(report(1, ['a', 'b']), identity);

test('all nested test identities must have exactly one passing result', () => {
  assert.equal(verifyCoverage(inventory(), [report(1, ['a']), report(2, ['b'])], identity, 2).passed, 2);
});
test('reject missing and duplicate tests even when every reported result passes', () => {
  for (const ids of [[], ['a'], ['b', 'extra']]) {
    assert.throws(() => verifyCoverage(inventory(), [report(1, ['a']), report(2, ids)], identity, 2));
  }
});
test('reject missing, duplicated or incorrectly numbered shards', () => {
  for (const reports of [[report(1, ['a'])], [report(1, ['a']), report(1, ['b'])],
    [report(1, ['a']), report(3, ['b'])]]) {
    assert.throws(() => verifyCoverage(inventory(), reports, identity, 2));
  }
});
test('source SHA, run and attempt cannot be mixed', () => {
  for (const key of ['sha', 'runId', 'attempt']) {
    const foreign = report(2, ['b']);
    foreign.config.metadata[key] = 'foreign';
    assert.throws(() => verifyCoverage(inventory(), [report(1, ['a']), foreign],
      { sha: 'a'.repeat(40), runId: '42', attempt: '1' }, 2));
  }
});
test('failed, timed out, interrupted, skipped and unstarted tests cannot pass', () => {
  for (const status of ['failed', 'timedOut', 'interrupted', 'skipped', null]) {
    const broken = report(2, ['b']);
    broken.suites[0].suites[0].specs[0].tests[0].results = status ? [{ status }] : [];
    assert.throws(() => verifyCoverage(inventory(), [report(1, ['a']), broken], identity, 2));
  }
});
test('a passing retry does not conceal a first-attempt failure', () => {
  const flaky = report(2, ['b']);
  flaky.suites[0].suites[0].specs[0].tests[0].results = [{ status: 'failed' }, { status: 'passed' }];
  assert.throws(() => verifyCoverage(inventory(), [report(1, ['a']), flaky], identity, 2));
});
test('reporter/setup errors, malformed discovery and duplicate identities fail closed', () => {
  assert.throws(() => inventoryFromReport({ suites: [] }, identity));
  assert.throws(() => inventoryFromReport(report(1, ['a', 'a']), identity));
  assert.throws(() => verifyCoverage(inventory(), [report(1, ['a']),
    report(2, ['b'], { errors: [{ message: 'setup failed' }] })], identity, 2));
});
test('all shards must agree on total and expected failure annotations do not qualify', () => {
  const bad = report(2, ['b']); bad.config.shard.total = 3;
  assert.throws(() => verifyCoverage(inventory(), [report(1, ['a']), bad], identity, 2));
  const expectedFailure = report(2, ['b']);
  expectedFailure.suites[0].suites[0].specs[0].tests[0].expectedStatus = 'failed';
  assert.throws(() => verifyCoverage(inventory(), [report(1, ['a']), expectedFailure], identity, 2));
});
