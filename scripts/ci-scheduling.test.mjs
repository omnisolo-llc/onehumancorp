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

// Deliberately support only this workflow's simple conjunctions. Without an
// explicit status function, GitHub adds success() for all needed jobs.
function allowsDocker(job, results, { cancelled = false, markdownOnly = 'false' } = {}) {
  const expression = job.if.match(/^\$\{\{ (.*) \}\}$/)?.[1];
  assert.ok(expression, 'expected an explicit job condition');
  const clauses = expression.split(' && ');
  const decisions = clauses.map(clause => {
    if (clause === '!cancelled()') return !cancelled;
    if (clause === "needs.check-changes.outputs.markdown-only == 'false'") return markdownOnly === 'false';
    const match = clause.match(/^needs\.([a-z-]+)\.result == 'success'$/);
    assert.ok(match, `unmodeled Docker condition: ${clause}`);
    return results[match[1]] === 'success';
  });
  const implicitSuccess = clauses.includes('!cancelled()') || needs(job).every(name => results[name] === 'success');
  return implicitSuccess && decisions.every(Boolean);
}

test('all possible CI job antichains stay within seven active runners', async () => {
  const bound = runnerBound((await workflow()).jobs);
  assert.ok(bound.maximum <= 7, `maximum ${bound.maximum}: ${bound.witness.join(', ')}`);
});
test('three grouped browser runners keep the complete workflow within seven runners', async () => {
  const { jobs } = await workflow();
  assert.equal(runnerBound(jobs).maximum, 7);
  jobs['native-e2e'].strategy.matrix.shard.push(4);
  jobs['native-e2e'].strategy['max-parallel'] = 4;
  assert.equal(runnerBound(jobs).maximum, 8);
});
test('browser waves overlap independent PostgreSQL checks while the final gate still requires them', async () => {
  const { jobs } = await workflow(); const bound = runnerBound(jobs);
  assert.ok(!bound.ancestors.get('native-e2e').has('postgres-security'));
  assert.ok(bound.ancestors.get('native-e2e').has('native-desktop'));
  for (const prerequisite of ['native-e2e', 'native-click-coverage', 'postgres-security', 'native-desktop']) assert.ok(bound.ancestors.get('ci-required').has(prerequisite), prerequisite);
  assert.ok(jobs['ci-required'].steps.some(step => step.run?.includes('require_success "postgres-security"')));
});
test('Node completion fences Docker instead of delaying browser shards', async () => {
  const { jobs } = await workflow(); const bound = runnerBound(jobs);
  assert.ok(!bound.ancestors.get('native-e2e').has('native-node'));
  assert.ok(bound.ancestors.get('docker-e2e').has('native-node'));
  assert.ok(!bound.ancestors.get('kind-e2e').has('native-node'));
  assert.ok(bound.ancestors.get('ci-required').has('native-node'));
  assert.ok(!jobs['native-node'].outputs);
  assert.ok(!jobs['native-node'].steps.some(step => step.uses?.startsWith('actions/upload-artifact@')));
});
test('removing the Docker completion fence is detected as an eight-runner graph', async () => {
  const { jobs } = await workflow();
  jobs['docker-e2e'].needs = needs(jobs['docker-e2e']).filter(name => name !== 'native-node');
  assert.equal(runnerBound(jobs).maximum, 8);
});
test('Docker still runs after Node failure but never after failed producers or cancellation', async () => {
  const { jobs } = await workflow(); const docker = jobs['docker-e2e'];
  assert.deepEqual(needs(docker), ['check-changes', 'native-images', 'native-build', 'native-node']);
  assert.ok(docker.if.includes('!cancelled()'), 'avoid the implicit success-only dependency gate');
  assert.ok(!docker.if.includes('needs.native-node.result'));
  const success = Object.fromEntries(needs(docker).map(name => [name, 'success']));
  for (const nodeResult of ['success', 'failure', 'cancelled', 'skipped']) {
    const results = { ...success, 'native-node': nodeResult };
    assert.ok(allowsDocker(docker, results), nodeResult);
    assert.ok(!allowsDocker(docker, results, { cancelled: true }));
    for (const markdownOnly of ['true', '']) {
      assert.ok(!allowsDocker(docker, results, { markdownOnly }));
    }
    for (const producer of ['check-changes', 'native-images', 'native-build']) {
      for (const result of ['failure', 'cancelled', 'skipped', undefined]) {
        assert.ok(!allowsDocker(docker, { ...results, [producer]: result }), `${producer}: ${result}`);
      }
    }
  }
  const implicit = { ...docker, if: docker.if.replace('!cancelled() && ', '') };
  assert.ok(!allowsDocker(implicit, { ...success, 'native-node': 'failure' }), 'the regression must detect implicit success()');
});
test('removing the desktop scheduling fence is detected as an eight-runner graph', async () => {
  const { jobs } = await workflow();
  jobs['native-e2e'].needs = needs(jobs['native-e2e']).filter(name => !['native-desktop','postgres-security'].includes(name));
  assert.equal(runnerBound(jobs).maximum, 8);
});
test('all three grouped browser runners, action budgets and real artifact producers remain required', async () => {
  const { jobs } = await workflow(); const browser = jobs['native-e2e']; const bound = runnerBound(jobs);
  assert.deepEqual(browser.strategy.matrix.shard, [1, 2, 3]);
  assert.equal(browser.strategy['max-parallel'], 3); assert.equal(browser.strategy['fail-fast'], false); assert.equal(browser['timeout-minutes'], 65);
  const downloads = browser.steps.filter(step => step.uses?.startsWith('actions/download-artifact@')).map(step => step.with.name);
  assert.deepEqual(downloads, ['native-linux-binaries','native-web']);
  for (const artifact of downloads) {
    const producers = Object.entries(jobs).filter(([,job]) => job.steps?.some(step => step.uses?.startsWith('actions/upload-artifact@') && step.with.name === artifact));
    assert.equal(producers.length, 1, artifact); assert.ok(bound.ancestors.get('native-e2e').has(producers[0][0]), artifact);
  }
  assert.match(JSON.stringify(browser), /--ci --grouped-shard=\$\{\{ matrix\.shard \}\}\/3 --workers=2 --retries=0/);
  assert.match(JSON.stringify(jobs['ci-required']), /--budget-minutes 60/);
  assert.equal(jobs['ci-required']['timeout-minutes'], 5, 'reporting has separate timeout headroom');
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

for (const name of ['native-test', 'postgres-security']) {
  test(`${name} preserves the approved one-hour test job budget`, async () => {
    const { jobs } = await workflow();
    assert.equal(jobs[name]['timeout-minutes'], 60,
      `${name} must not cancel the approved one-hour test run early`);
  });
}
