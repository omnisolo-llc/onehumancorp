import test from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
const ci = JSON.parse(execFileSync('python3', ['-c',
  "import yaml,json; print(json.dumps(yaml.safe_load(open('.github/workflows/ci.yml'))))"], { encoding: 'utf8' }));
const jobs = ci.jobs;
const runs = job => (job?.steps ?? []).map(step => step.run ?? '').join('\n');

test('Rust lint and tests occupy independent required runners', () => {
  assert.ok(jobs['native-lint'], 'Rust lint requires its own job');
  assert.match(runs(jobs['native-lint']), /make lint-backend/);
  assert.doesNotMatch(runs(jobs['native-test']), /make lint-backend|cargo clippy/);
  assert.deepEqual(jobs['native-lint'].needs, ['check-changes']);
  assert.deepEqual(jobs['native-test'].needs, ['check-changes']);
  assert.ok(jobs['ci-required'].needs.includes('native-lint'));
});
test('Node lint and unit suites occupy independent required runners', () => {
  assert.ok(jobs['native-node-lint'], 'Node lint requires its own job');
  assert.match(runs(jobs['native-node-lint']), /npm run lint:node/);
  assert.match(runs(jobs['native-node-lint']), /npm run typecheck:web/);
  assert.match(runs(jobs['native-node-lint']), /npm --prefix src\/cli run typecheck/);
  assert.doesNotMatch(runs(jobs['native-node']), /lint:node|typecheck/);
  assert.match(runs(jobs['native-node']), /make test-node/);
  assert.ok(jobs['ci-required'].needs.includes('native-node-lint'));
});
test('isolated browser shards exploit the quota without depending on lint', () => {
  const browser = jobs['native-e2e'];
  assert.equal(browser.strategy.matrix.shard.length, 32);
  assert.equal(browser.strategy['max-parallel'], 32);
  assert.equal(browser.strategy['fail-fast'], false);
  assert.ok(browser.strategy.matrix.shard.length + Object.keys(jobs).length - 1 <= 60);
  for (const prerequisite of ['native-lint', 'native-node-lint', 'native-test']) {
    assert.ok(!browser.needs.includes(prerequisite));
  }
  assert.match(runs(browser), /--retries=0/);
  assert.match(runs(browser), /--global-timeout=1200000/);
  assert.match(readFileSync('playwright.config.ts', 'utf8'), /workers: process.env.CI \? 1/);
});
test('full unsharded discovery and merged execution evidence remain required', () => {
  assert.match(runs(jobs['native-e2e-discovery']), /test:e2e -- --ci --list/);
  assert.doesNotMatch(runs(jobs['native-e2e-discovery']), /--shard|--grep/);
  assert.ok(jobs['native-e2e-report'].needs.includes('native-e2e'));
  assert.ok(jobs['native-e2e-report'].needs.includes('native-e2e-discovery'));
  assert.match(runs(jobs['native-e2e-report']), /ci-coverage.mjs .*--shards 32/);
  for (const job of ['native-e2e-discovery', 'native-e2e-report']) {
    assert.ok(jobs['ci-required'].needs.includes(job));
  }
});
test('independent lanes cannot be made optional by moving them', () => {
  assert.equal(jobs['ci-required'].if, '${{ always() }}');
  for (const name of ['native-lint', 'native-node-lint', 'native-test', 'native-node', 'native-e2e', 'native-e2e-report']) {
    const job = jobs[name];
    assert.ok(job, name);
    assert.ok(!job['continue-on-error'], name);
    for (const step of job.steps) assert.ok(!step['continue-on-error'], step.name);
  }
});
