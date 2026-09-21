import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { readFile } from 'node:fs/promises';

function workflow() {
  const parsed = spawnSync('python3', ['-c',
    "import json,yaml; print(json.dumps(yaml.safe_load(open('.github/workflows/ci.yml'))))"],
  { encoding: 'utf8', timeout: 10000 });
  assert.ifError(parsed.error);
  assert.equal(parsed.status, 0, parsed.stderr);
  return JSON.parse(parsed.stdout).jobs;
}

test('strict Rust lint is an independent required job, not a predecessor of tests', () => {
  const jobs = workflow();
  assert.ok(jobs['native-lint'], 'missing independent native-lint job');
  assert.ok(jobs['native-lint'].steps.some(step => step.run === 'make lint-backend'));
  assert.ok(!jobs['native-test'].steps.some(step => /lint-backend/.test(step.run ?? '')));
  assert.ok(!jobs['native-test'].needs.includes('native-lint'));
  assert.ok(jobs['ci-required'].needs.includes('native-lint'));
});

test('browser evaluation uses isolated one-worker shards within the 60-runner quota', async () => {
  const jobs = workflow();
  const e2e = jobs['native-e2e'];
  assert.deepEqual(e2e.strategy.matrix.shard, Array.from({ length: 8 }, (_, i) => i + 1));
  assert.equal(e2e.strategy['max-parallel'], 8);
  assert.equal(e2e.strategy['fail-fast'], false);
  assert.ok(e2e.steps.some(step => /--shard=\$\{\{ matrix\.shard \}\}\/8 --workers=1 --retries=0/.test(step.run ?? '')));
  assert.ok(Object.keys(jobs).length + 7 <= 24, 'per-PR maximum should leave substantial runner headroom');
  const config = await readFile('playwright.config.ts', 'utf8');
  assert.match(config, /workers: process\.env\.CI \? 1/);
});

test('complete source-bound discovery and merged results are required before qualification', () => {
  const jobs = workflow();
  assert.ok(jobs['native-web'].steps.some(step => /ci-coverage.mjs discover/.test(step.run ?? '')));
  const report = jobs['native-e2e-report'];
  assert.ok(report, 'missing complete-results gate');
  assert.ok(report.needs.includes('native-e2e'));
  assert.ok(report.steps.some(step => /ci-coverage.mjs verify/.test(step.run ?? '')));
  assert.ok(jobs['ci-required'].needs.includes('native-e2e-report'));
  for (const name of ['native-lint', 'native-e2e', 'native-e2e-report']) {
    assert.notEqual(jobs[name]['continue-on-error'], true, name);
    for (const step of jobs[name].steps) assert.notEqual(step['continue-on-error'], true, name);
  }
});

test('preflight uses a complete JSON file, while execution retains first-attempt failure diagnostics', async () => {
  const runner = await readFile('scripts/native-e2e.mjs', 'utf8');
  const config = await readFile('playwright.config.ts', 'utf8');
  assert.match(runner, /discoverBrowserInventory\(args, env\)/);
  const discovery = await readFile('scripts/ci-coverage.mjs', 'utf8');
  assert.match(discovery, /PLAYWRIGHT_JSON_OUTPUT_FILE: output/);
  assert.match(discovery, /JSON\.parse\(await readFile\(output, 'utf8'\)\)/);
  assert.match(config, /'json'.*native-results\.json/);
  assert.match(config, /'blob'/);
  assert.match(config, /trace: 'retain-on-failure'/);
});
