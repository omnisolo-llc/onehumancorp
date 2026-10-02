import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import yaml from 'js-yaml';

const read = name => readFile(new URL(`../${name}`, import.meta.url), 'utf8');
const workflow = async () => yaml.load(await read('.github/workflows/ci.yml'));
const needs = job => job.needs === undefined ? [] : Array.isArray(job.needs) ? job.needs : [job.needs];
function runnerBound(jobs) {
  const names = Object.keys(jobs);
  assert.ok(names.length <= 24, 'reassess the bounded DAG proof when adding more jobs');
  const ancestors = new Map();
  function visit(name, active = new Set()) {
    assert.ok(jobs[name], `unknown dependency ${name}`);
    assert.ok(!active.has(name), 'workflow dependency cycle');
    if (ancestors.has(name)) return ancestors.get(name);
    const next = new Set(active).add(name); const result = new Set();
    for (const parent of needs(jobs[name])) { result.add(parent); for (const ancestor of visit(parent, next)) result.add(ancestor); }
    ancestors.set(name, result); return result;
  }
  for (const name of names) visit(name);
  const weight = name => {
    const strategy = jobs[name].strategy;
    if (!strategy?.matrix) return 1;
    assert.ok(!strategy.matrix.include && !strategy.matrix.exclude, 'explicitly account for matrix include/exclude before claiming a bound');
    let total = 1;
    for (const axis of Object.values(strategy.matrix)) { assert.ok(Array.isArray(axis) && axis.length > 0); total *= axis.length; }
    const maximum = strategy['max-parallel'] ?? total;
    assert.ok(Number.isSafeInteger(maximum) && maximum > 0);
    return Math.min(total, maximum);
  };
  let maximum = 0; let witness = [];
  function search(index, selected, total) {
    if (total > maximum) { maximum = total; witness = selected.slice(); }
    for (let i = index; i < names.length; i++) {
      const candidate = names[i];
      if (selected.every(other => !ancestors.get(candidate).has(other) && !ancestors.get(other).has(candidate))) search(i + 1, [...selected, candidate], total + weight(candidate));
    }
  }
  search(0, [], 0); return { maximum, witness, ancestors };
}

test('all possible CI job antichains stay within eight active runners', async () => {
  const bound = runnerBound((await workflow()).jobs);
  assert.ok(bound.maximum <= 8, `maximum ${bound.maximum}: ${bound.witness.join(', ')}`);
});
test('browser waves overlap independent PostgreSQL checks while the final gate still requires them', async () => {
  const { jobs } = await workflow(); const bound = runnerBound(jobs);
  assert.ok(!bound.ancestors.get('native-e2e').has('postgres-security'));
  assert.ok(bound.ancestors.get('native-e2e').has('native-desktop'));
  for (const prerequisite of ['native-e2e', 'native-click-coverage', 'postgres-security', 'native-desktop']) assert.ok(bound.ancestors.get('ci-required').has(prerequisite), prerequisite);
  assert.ok(jobs['ci-required'].steps.some(step => step.run?.includes('require_success "postgres-security"')));
});
test('removing the desktop scheduling fence is detected as a nine-runner graph', async () => {
  const { jobs } = await workflow();
  jobs['native-e2e'].needs = needs(jobs['native-e2e']).filter(name => !['native-desktop','postgres-security'].includes(name));
  assert.equal(runnerBound(jobs).maximum, 9);
});
test('all twelve browser shards, action budgets and real artifact producers remain required', async () => {
  const { jobs } = await workflow(); const browser = jobs['native-e2e']; const bound = runnerBound(jobs);
  assert.deepEqual(browser.strategy.matrix.shard, Array.from({ length: 12 }, (_, i) => i + 1));
  assert.equal(browser.strategy['max-parallel'], 4); assert.equal(browser.strategy['fail-fast'], false); assert.equal(browser['timeout-minutes'], 25);
  const downloads = browser.steps.filter(step => step.uses?.startsWith('actions/download-artifact@')).map(step => step.with.name);
  assert.deepEqual(downloads, ['native-linux-binaries','native-web']);
  for (const artifact of downloads) {
    const producers = Object.entries(jobs).filter(([,job]) => job.steps?.some(step => step.uses?.startsWith('actions/upload-artifact@') && step.with.name === artifact));
    assert.equal(producers.length, 1, artifact); assert.ok(bound.ancestors.get('native-e2e').has(producers[0][0]), artifact);
  }
  assert.match(JSON.stringify(browser), /--ci --shard=\$\{\{ matrix\.shard \}\}\/12 --workers=2 --retries=0/);
  assert.match(JSON.stringify(jobs['ci-required']), /--budget-minutes 30/);
});
test('real browser setup owns its PostgreSQL schema and consumes no PG-gate outputs', async () => {
  const { jobs } = await workflow();
  assert.ok(!jobs['postgres-security'].outputs);
  const runtime = await read('scripts/native-e2e.mjs');
  assert.match(runtime, /const pg = `ohc-e2e-pg-\$\{suffix\}`/);
  assert.match(runtime, /'POSTGRES_DB=ohc'/);
  assert.match(runtime, /DATABASE_URL: `postgres:\/\/ohc:ohc@127\.0\.0\.1:\$\{pgPort\}\/ohc`/);
  assert.match(runtime, /src\/server\/migrations\/sqlx_migration_contract_test\.sh/);
  assert.doesNotMatch(runtime, /needs\.postgres-security|postgres-security-evidence/);
});
