import { readFile, readdir } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

function unique(values, label) {
  if (!Array.isArray(values) || values.some((v) => typeof v !== 'string' || !v)) throw new Error(`Invalid ${label}`);
  const result = new Set(values);
  if (result.size !== values.length) throw new Error(`Duplicate ${label}`);
  return result;
}
function sameIdentity(actual, expected) {
  return actual && ['sha', 'run', 'attempt'].every((key) => actual[key] === expected[key]);
}

export function verifyCoverage(inventory, reports, identity, total) {
  if (!Number.isInteger(total) || total < 1 || total > 60 || inventory.kind !== 'inventory' ||
    inventory.complete !== true || inventory.index !== 0 || inventory.total !== 0 ||
    !sameIdentity(inventory.identity, identity)) throw new Error('Invalid full discovery identity');
  const expected = unique(inventory.selected, 'discovery IDs');
  if (!expected.size || !Array.isArray(reports) || reports.length !== total) throw new Error('Empty discovery or missing shards');
  const seen = new Set(), indexes = new Set();
  for (const report of reports) {
    if (report.kind !== 'results' || report.complete !== true || report.total !== total ||
      !sameIdentity(report.identity, identity) || !Number.isInteger(report.index) ||
      report.index < 1 || report.index > total || indexes.has(report.index)) throw new Error('Incomplete, duplicate or wrong-source shard');
    indexes.add(report.index);
    const selected = unique(report.selected, 'selected IDs');
    if (!Array.isArray(report.results) || report.results.length !== selected.size) throw new Error('Unstarted or duplicate test results');
    const actual = new Set();
    for (const test of report.results) {
      if (test.status !== 'passed' || test.retry !== 0 || !selected.has(test.id) || actual.has(test.id)) {
        throw new Error(`Failed, retried, skipped or unexpected test: ${test.id}`);
      }
      actual.add(test.id);
    }
    for (const id of selected) {
      if (!expected.has(id) || seen.has(id)) throw new Error(`Duplicate or undiscovered test: ${id}`);
      seen.add(id);
    }
  }
  if (seen.size !== expected.size) throw new Error(`Missing ${expected.size - seen.size} selected tests`);
  return seen.size;
}

async function main() {
  const [directory, count] = process.argv.slice(2);
  if (!directory || !/^\d+$/.test(count || '')) throw new Error('Usage: node scripts/ci-coverage.mjs DIRECTORY SHARD_COUNT');
  const identity = { sha: execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(),
    run: process.env.GITHUB_RUN_ID, attempt: process.env.GITHUB_RUN_ATTEMPT };
  if (!/^[a-f0-9]{40}$/.test(identity.sha) || !/^\d+$/.test(identity.run || '') || !/^[1-9]\d*$/.test(identity.attempt || '')) {
    throw new Error('Hosted source/run identity is required');
  }
  const inventory = JSON.parse(await readFile(path.join(directory, 'inventory-0.json'), 'utf8'));
  const names = (await readdir(directory)).filter((name) => /^results-\d+\.json$/.test(name));
  const reports = await Promise.all(names.map(async (name) => JSON.parse(await readFile(path.join(directory, name), 'utf8'))));
  const passed = verifyCoverage(inventory, reports, identity, Number(count));
  console.log(`Complete browser qualification: ${passed} first-attempt passes across ${count} shards at ${identity.sha}.`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  main().catch((error) => { console.error(error.message); process.exitCode = 1; });
}
