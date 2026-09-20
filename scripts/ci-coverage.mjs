import { readFile, readdir } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

function requireIdentity(identity) {
  if (!identity || !/^[a-f0-9]{40}$/.test(identity.sha)
      || typeof identity.runId !== 'string' || !/^[1-9]\d*$/.test(identity.runId)
      || !Number.isSafeInteger(identity.attempt) || identity.attempt < 1) {
    throw new Error('A complete hosted source/run/attempt identity is required');
  }
}

function sameIdentity(actual, expected) {
  if (['sha', 'runId', 'attempt'].some(key => actual?.[key] !== expected[key])) {
    throw new Error('Report identity does not match this source/run/attempt');
  }
}

function uniqueIds(values, label) {
  if (!Array.isArray(values) || values.some(id => typeof id !== 'string' || !id)) {
    throw new Error(`Invalid ${label} test identities`);
  }
  const ids = new Set(values);
  if (ids.size !== values.length) throw new Error(`Duplicate ${label} test identity`);
  return ids;
}

/** Reconcile actual discovery against all shards, not against a smoke allowlist. */
export function verifyCoverage(expectedIds, reports, identity, shardTotal) {
  requireIdentity(identity);
  const expected = uniqueIds(expectedIds, 'expected');
  if (!expected.size) throw new Error('Empty test selection cannot qualify');
  if (!Number.isSafeInteger(shardTotal) || shardTotal < 1
      || !Array.isArray(reports) || reports.length !== shardTotal) {
    throw new Error('Missing or invalid execution shard count');
  }
  const shards = new Set(), selected = new Set();
  let passed = 0;
  for (const report of reports) {
    sameIdentity(report, identity);
    if (report.schemaVersion !== 1) throw new Error('Unsupported execution report schema');
    if (report.shardTotal !== shardTotal || !Number.isSafeInteger(report.shardIndex)
        || report.shardIndex < 1 || report.shardIndex > shardTotal) {
      throw new Error('Invalid or inconsistent execution shard identity');
    }
    if (shards.has(report.shardIndex)) throw new Error('Duplicate execution shard');
    shards.add(report.shardIndex);
    if (report.complete !== true) throw new Error(`Incomplete execution shard ${report.shardIndex}`);
    const ids = uniqueIds(report.selectedIds, 'selected');
    if (!Array.isArray(report.finished)) throw new Error('Missing execution results');
    const finished = new Set();
    for (const result of report.finished) {
      if (!result || !ids.has(result.id)) throw new Error('Unexpected execution result');
      if (finished.has(result.id)) throw new Error('Duplicate execution result');
      finished.add(result.id);
      if (result.outcome !== 'passed') throw new Error(`Test did not pass: ${result.id} (${result.outcome})`);
      if (result.attempts !== 1) throw new Error(`Test did not pass on its first attempt: ${result.id}`);
      passed++;
    }
    for (const id of ids) {
      if (selected.has(id)) throw new Error(`Duplicate selection across shards: ${id}`);
      if (!expected.has(id)) throw new Error(`Unexpected selected test: ${id}`);
      if (!finished.has(id)) throw new Error(`Unfinished selected test: ${id}`);
      selected.add(id);
    }
  }
  const missing = [...expected].filter(id => !selected.has(id));
  if (missing.length) throw new Error(`Missing ${missing.length} selected tests: ${missing.slice(0, 5).join(', ')}`);
  return { selected: selected.size, passed, shards: shards.size };
}

async function executionReports(directory) {
  const reports = [];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const filename = path.join(directory, entry.name);
    if (entry.isDirectory()) reports.push(...await executionReports(filename));
    else if (entry.isFile() && /^execution-\d+-of-\d+\.json$/.test(entry.name)) {
      reports.push(JSON.parse(await readFile(filename, 'utf8')));
    }
  }
  return reports;
}

async function main(args) {
  const options = new Map();
  if (args.length !== 6) throw new Error('Usage: ci-coverage.mjs --expected FILE --reports DIRECTORY --shards COUNT');
  for (let index = 0; index < args.length; index += 2) {
    if (!['--expected', '--reports', '--shards'].includes(args[index]) || options.has(args[index])) {
      throw new Error('Unknown or repeated coverage argument');
    }
    options.set(args[index], args[index + 1]);
  }
  const identity = { sha: process.env.GITHUB_SHA, runId: process.env.GITHUB_RUN_ID,
    attempt: Number(process.env.GITHUB_RUN_ATTEMPT) };
  requireIdentity(identity);
  const expected = JSON.parse(await readFile(options.get('--expected'), 'utf8'));
  sameIdentity(expected, identity);
  if (expected.schemaVersion !== 1 || expected.mode !== 'discovery' || expected.shardTotal !== 1) {
    throw new Error('Expected inventory must be complete, unsharded Playwright discovery');
  }
  const result = verifyCoverage(expected.selectedIds, await executionReports(options.get('--reports')),
    identity, Number(options.get('--shards')));
  console.log(`Verified ${result.passed}/${result.selected} first-attempt passes across ${result.shards} source-bound shards.`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  main(process.argv.slice(2)).catch(error => { console.error(error.message); process.exitCode = 1; });
}
