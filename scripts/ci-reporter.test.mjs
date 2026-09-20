import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import CiReporter from './playwright/ci-reporter.mjs';

const identity = { sha: 'a'.repeat(40), runId: '42', attempt: 1 };
function fixture(t, options = {}) {
  const directory = mkdtempSync(path.join(os.tmpdir(), 'ohc-reporter-test-'));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  const reporter = new CiReporter({ directory, identity, ...options });
  const read = (name = 'execution-1-of-2.json') => JSON.parse(readFileSync(path.join(directory, name), 'utf8'));
  return { reporter, read, directory };
}
const suite = (...ids) => ({ allTests: () => ids.map(id => ({ id })) });
const config = { shard: { current: 1, total: 2 } };

test('onBegin records an incomplete actual selection before any test executes', t => {
  const { reporter, read } = fixture(t);
  reporter.onBegin(config, suite('nested::case', 'root::case'));
  const report = read();
  assert.deepEqual(report.selectedIds, ['nested::case', 'root::case']);
  assert.deepEqual(report.finished, []);
  assert.equal(report.complete, false);
  assert.deepEqual([report.sha, report.runId, report.attempt], [identity.sha, '42', 1]);
});
test('success is recorded only after every selected test finishes', t => {
  const { reporter, read } = fixture(t);
  reporter.onBegin(config, suite('a', 'b'));
  reporter.onTestEnd({ id: 'a' }, { status: 'passed', retry: 0 });
  assert.equal(read().complete, false);
  reporter.onTestEnd({ id: 'b' }, { status: 'passed', retry: 0 });
  reporter.onEnd({ status: 'passed' });
  assert.equal(read().complete, true);
  assert.deepEqual(read().finished, [
    { id: 'a', outcome: 'passed', attempts: 1 }, { id: 'b', outcome: 'passed', attempts: 1 },
  ]);
});
test('a retry cannot replace a failed first attempt with a qualifying pass', t => {
  const { reporter, read } = fixture(t);
  reporter.onBegin(config, suite('a'));
  reporter.onTestEnd({ id: 'a' }, { status: 'failed', retry: 0 });
  reporter.onTestEnd({ id: 'a' }, { status: 'passed', retry: 1 });
  reporter.onEnd({ status: 'passed' });
  assert.deepEqual(read().finished, [{ id: 'a', outcome: 'failed', attempts: 2 }]);
});
for (const status of ['interrupted', 'timedout']) test(`run ${status} cannot qualify`, t => {
  const { reporter, read } = fixture(t);
  reporter.onBegin(config, suite('a'));
  reporter.onTestEnd({ id: 'a' }, { status: 'passed', retry: 0 });
  reporter.onEnd({ status });
  assert.equal(read().complete, false);
});
test('global errors cannot qualify even after tests pass', t => {
  const { reporter, read } = fixture(t);
  reporter.onBegin(config, suite('a'));
  reporter.onTestEnd({ id: 'a' }, { status: 'passed', retry: 0 });
  reporter.onError(new Error('global teardown failed'));
  reporter.onEnd({ status: 'failed' });
  assert.equal(read().complete, false);
});
test('a premature end preserves missing test evidence', t => {
  const { reporter, read } = fixture(t);
  reporter.onBegin(config, suite('a', 'b'));
  reporter.onTestEnd({ id: 'a' }, { status: 'passed', retry: 0 });
  reporter.onEnd({ status: 'passed' });
  assert.equal(read().complete, false);
});
test('discovery does not manufacture execution results', t => {
  const { reporter, read, directory } = fixture(t, { discovery: true });
  reporter.onBegin({ shard: null }, suite('nested::a', 'b'));
  reporter.onEnd({ status: 'passed' });
  const report = read('selection-all.json');
  assert.equal(report.mode, 'discovery');
  assert.equal(report.shardTotal, 1);
  assert.deepEqual(report.selectedIds, ['nested::a', 'b']);
  assert.equal(report.finished, undefined);
  assert.throws(() => readFileSync(path.join(directory, 'execution-1-of-1.json')), /ENOENT/);
});
test('sharded discovery cannot overwrite full discovery', t => {
  const { reporter, read } = fixture(t, { discovery: true });
  reporter.onBegin(config, suite('a'));
  assert.equal(read('selection-1-of-2.json').shardTotal, 2);
  assert.throws(() => read('selection-all.json'), /ENOENT/);
});
test('duplicate identities fail rather than being silently merged', t => {
  const { reporter } = fixture(t);
  assert.throws(() => reporter.onBegin(config, suite('a', 'a')), /Duplicate/);
});
test('unexpected results fail', t => {
  const { reporter } = fixture(t);
  reporter.onBegin(config, suite('a'));
  assert.throws(() => reporter.onTestEnd({ id: 'b' }, { status: 'passed', retry: 0 }), /Unexpected/);
});
test('missing provenance fails closed', t => {
  const directory = mkdtempSync(path.join(os.tmpdir(), 'ohc-reporter-test-'));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  assert.throws(() => new CiReporter({ directory, identity: {} }), /identity/);
});
