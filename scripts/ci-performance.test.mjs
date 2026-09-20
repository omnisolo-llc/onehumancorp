import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtemp, readFile, writeFile, rm } from 'node:fs/promises';
import path from 'node:path';
import os from 'node:os';
import { fileURLToPath } from 'node:url';

const script = fileURLToPath(new URL('./ci-performance.py', import.meta.url));
const job = (id, name, start, end, extra = {}) => ({ id, name, run_id: 42, run_attempt: 2,
  status: 'completed', conclusion: 'success', started_at: `2026-09-19T10:${start}:00Z`,
  completed_at: `2026-09-19T10:${end}:00Z`, ...extra });
const fixture = () => [{ total_count: 4, jobs: [
  job(1, 'Native backend binaries', '00', '10'), job(2, 'Native Next production build', '00', '03'),
  job(3, 'Native browser suite', '12', '20'),
  job(4, 'CI Required', '21', '21', { status: 'in_progress', conclusion: null }),
] }];
async function run(pages, budget = '30', prefix = '') {
  const dir = await mkdtemp(path.join(os.tmpdir(), 'ohc-ci-time-'));
  try {
    const input = path.join(dir, 'jobs.json'); await writeFile(input, JSON.stringify(pages));
    const result = spawnSync('python3', [script, '--jobs', input, '--run-id', '42', '--attempt', '2',
      '--budget-minutes', budget, '--job-prefix', prefix, '--cold', '--output', path.join(dir, 'report')], { encoding: 'utf8', timeout: 15000 });
    assert.ifError(result.error);
    let report;
    try { report = JSON.parse(await readFile(path.join(dir, 'report/performance.json'), 'utf8')); }
    catch { /* Invalid input must not generate success evidence. */ }
    return { ...result, report };
  } finally { await rm(dir, { recursive: true, force: true }); }
}
test('CI wall time includes dependent-job waits and is not the sum of parallel runtimes', async () => {
  const result = await run(fixture()); assert.equal(result.status, 0, result.stderr);
  assert.equal(result.report.elapsed_seconds, 1260);
  assert.equal(result.report.cache_mode_requested, 'disabled');
  assert.equal(result.report.cache_hit_proven, false);
  assert.equal(result.report.within_budget, true);
});
test('core build timing includes Tauri and dependency waits without replacing the full gate', async () => {
  const pages = fixture();
  pages[0].total_count = 5;
  pages[0].jobs.push(job(5, 'Native Tauri compile', '04', '12'));
  const result = await run(pages);
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.report.core_build.elapsed_seconds, 720);
  assert.equal(result.report.core_build.within_target, false);
  assert.equal(result.report.core_build.enforced, false);
  assert.equal(result.report.core_build.all_builds_succeeded, true);
  assert.equal(result.report.elapsed_seconds, 1260);
});
test('missing or skipped core builds do not become zero-minute success', async () => {
  const missing = await run(fixture());
  assert.equal(missing.report.core_build.elapsed_seconds, null);
  assert.deepEqual(missing.report.core_build.missing_jobs, ['Native Tauri compile']);
  const pages = fixture();
  pages[0].total_count = 5;
  pages[0].jobs.push(job(5, 'Native Tauri compile', '04', '12', { conclusion: 'skipped' }));
  const skipped = await run(pages);
  assert.equal(skipped.report.core_build.elapsed_seconds, null);
  assert.equal(skipped.report.core_build.all_builds_succeeded, false);
});

test('reused CI timing is scoped without counting concurrently running release jobs', async () => {
  const pages = fixture();
  const prefix = 'Release qualification / ';
  for (const row of pages[0].jobs) row.name = prefix + row.name;
  pages[0].jobs.push(job(99, 'Build signed desktop packages', '00', '59', { status: 'in_progress', conclusion: null }));
  pages[0].total_count++;
  const result = await run(pages, '30', prefix);
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.report.elapsed_seconds, 1260);
  assert.equal(result.report.jobs.length, 3);
  assert.equal((await run(pages, '30', 'Wrong caller / ')).status, 1);
  pages[0].total_count++;
  assert.equal((await run(pages, '30', prefix)).status, 1, 'prefix filtering cannot bypass pagination validation');
});

test('an over-budget run fails while preserving measured evidence', async () => {
  const result = await run(fixture(), '20'); assert.equal(result.status, 1);
  assert.equal(result.report.within_budget, false);
});
test('a failed build cannot become successful performance evidence', async () => {
  const pages = fixture(); pages[0].jobs[0].conclusion = 'failure';
  const result = await run(pages); assert.equal(result.status, 1);
  assert.equal(result.report.all_executed_jobs_succeeded, false);
});
for (const budget of ['NaN', 'Infinity', '0', '-1']) {
  test(`CI performance rejects invalid budget ${budget}`, async () => {
    const result = await run(fixture(), budget);
    assert.equal(result.status, 1);
    assert.equal(result.report, undefined);
  });
}

for (const [name, mutate] of [
  ['partial pagination', pages => { pages[0].total_count = 5; }],
  ['previous attempt', pages => { pages[0].jobs[0].run_attempt = 1; }],
  ['unknown outcome', pages => { pages[0].jobs[0].status = 'in_progress'; }],
  ['missing timestamps', pages => { pages[0].jobs[0].started_at = null; }],
  ['duplicate records', pages => { pages[0].jobs.push(pages[0].jobs[0]); }],
]) {
  test(`CI performance rejects ${name}`, async () => {
    const pages = fixture(); mutate(pages); const result = await run(pages);
    assert.equal(result.status, 1); assert.equal(result.report, undefined);
  });
}
