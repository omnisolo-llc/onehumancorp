import { createRequire } from 'node:module';
import { readFile, writeFile, mkdir, readdir, mkdtemp, rm } from 'node:fs/promises';
import { fileURLToPath, pathToFileURL } from 'node:url';
import path from 'node:path';
import assert from 'node:assert/strict';
import os from 'node:os';
import { runNativeCommand } from './native-process.mjs';

const root = fileURLToPath(new URL('..', import.meta.url));
const require = createRequire(new URL('../package.json', import.meta.url));

export function workflowIdentity(environment = process.env) {
  const identity = {
    sha: environment.GITHUB_SHA,
    runId: environment.GITHUB_RUN_ID,
    attempt: environment.GITHUB_RUN_ATTEMPT,
  };
  assert.match(identity.sha ?? '', /^[a-f0-9]{40}$/u, 'Missing source commit identity');
  assert.match(identity.runId ?? '', /^[1-9]\d*$/u, 'Missing workflow run identity');
  assert.match(identity.attempt ?? '', /^[1-9]\d*$/u, 'Missing workflow attempt identity');
  return identity;
}

function verifyIdentity(actual, expected) {
  for (const key of ['sha', 'runId', 'attempt']) {
    assert.equal(actual?.[key], expected[key], `Mismatched ${key}`);
  }
}

// Read Playwright's actual JSON suite tree, including nested application specs.
// A regular expression over test source cannot enumerate parameterized tests.
function testsFromReport(report) {
  assert.ok(Array.isArray(report?.suites), 'Malformed Playwright report');
  assert.ok(Array.isArray(report.errors) && report.errors.length === 0,
    'Playwright reported setup, discovery or reporter errors');
  const found = new Map();
  const visit = (suites) => {
    for (const suite of suites) {
      assert.ok(suite && typeof suite === 'object', 'Malformed suite');
      for (const spec of suite.specs ?? []) {
        assert.ok(typeof spec.id === 'string' && spec.id.length > 0, 'Missing test identity');
        assert.ok(Array.isArray(spec.tests) && spec.tests.length > 0, 'Spec has no project tests');
        for (const test of spec.tests) {
          assert.ok(typeof test.projectName === 'string' && test.projectName.length > 0,
            'Missing project identity');
          const id = JSON.stringify([test.projectName, spec.id]);
          assert.ok(!found.has(id), `Duplicate test identity: ${id}`);
          found.set(id, { ...test, file: spec.file });
        }
      }
      visit(suite.suites ?? []);
    }
  };
  visit(report.suites);
  return found;
}

export function inventoryFromReport(report, identity) {
  verifyIdentity(report.config?.metadata, identity);
  const tests = testsFromReport(report);
  const selectedIds = [...tests.keys()].sort();
  const files = [...new Set([...tests.values()].map(test => test.file))].sort();
  assert.ok(selectedIds.length > 0, 'Empty browser inventory');
  assert.ok(files.every(file => typeof file === 'string' && file.length > 0), 'Missing test file');
  return { ...identity, selectedIds, files };
}

export function verifyCoverage(inventory, reports, identity, shardTotal) {
  verifyIdentity(inventory, identity);
  assert.ok(Number.isSafeInteger(shardTotal) && shardTotal > 0, 'Invalid shard count');
  assert.ok(Array.isArray(inventory.selectedIds) && inventory.selectedIds.length > 0, 'Empty inventory');
  const expected = new Set(inventory.selectedIds);
  assert.equal(expected.size, inventory.selectedIds.length, 'Duplicate inventory identities');
  assert.ok(Array.isArray(reports), 'Missing reports');
  assert.equal(reports.length, shardTotal, 'Missing or extra shard reports');
  const indices = new Set(), seen = new Set();
  for (const report of reports) {
    verifyIdentity(report.config?.metadata, identity);
    const shard = report.config?.shard;
    assert.equal(shard?.total, shardTotal, 'Inconsistent shard totals');
    assert.ok(Number.isSafeInteger(shard.current) && shard.current >= 1 && shard.current <= shardTotal,
      'Invalid shard index');
    assert.ok(!indices.has(shard.current), 'Duplicate shard index');
    indices.add(shard.current);
    for (const [id, test] of testsFromReport(report)) {
      assert.ok(expected.has(id), `Unexpected test: ${id}`);
      assert.ok(!seen.has(id), `Test selected by multiple shards: ${id}`);
      seen.add(id);
      assert.equal(test.expectedStatus, 'passed', `Non-passing expectation: ${id}`);
      assert.equal(test.status, 'expected', `Unexpected or flaky test outcome: ${id}`);
      assert.ok(Array.isArray(test.results) && test.results.length === 1,
        `Unstarted test or retried first-attempt failure: ${id}`);
      assert.equal(test.results[0].status, 'passed', `Failed, skipped or interrupted test: ${id}`);
    }
  }
  assert.equal(seen.size, expected.size, 'Selected tests are missing terminal results');
  return { ...identity, selected: expected.size, passed: seen.size, shards: indices.size };
}

async function findReports(directory) {
  const found = [];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const filename = path.join(directory, entry.name);
    if (entry.isDirectory()) found.push(...await findReports(filename));
    else if (entry.name === 'native-results.json') {
      found.push(JSON.parse(await readFile(filename, 'utf8')));
    }
  }
  return found;
}

// Console output can be truncated when a large listing exits under pipe
// backpressure. Only the completed JSON file is authoritative for selection.
export async function discoverBrowserInventory(args, environment) {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'ohc-browser-discovery-'));
  const output = path.join(directory, 'discovery.json');
  const identity = {
    sha: environment.GITHUB_SHA || 'local', runId: environment.GITHUB_RUN_ID || 'local',
    attempt: environment.GITHUB_RUN_ATTEMPT || 'local',
  };
  try {
    await runNativeCommand(process.execPath, [require.resolve('@playwright/test/cli'),
      'test', '--config', 'playwright.config.ts', '--list',
      ...args.filter(arg => arg !== '--list'), '--reporter=json'], {
      cwd: root, env: { ...environment, PLAYWRIGHT_JSON_OUTPUT_FILE: output }, timeoutMs: 90000,
    });
    return inventoryFromReport(JSON.parse(await readFile(output, 'utf8')), identity);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}

async function main(args) {
  const identity = workflowIdentity();
  if (args[0] === 'discover' && args.length === 2) {
    const inventory = await discoverBrowserInventory([], { ...process.env, PLAYWRIGHT_TEST_DIR: './src' });
    await mkdir(path.dirname(args[1]), { recursive: true });
    await writeFile(args[1], `${JSON.stringify(inventory, null, 2)}\n`);
    console.log(`Discovered ${inventory.selectedIds.length} browser tests at ${identity.sha}`);
  } else if (args[0] === 'verify' && args.length === 4) {
    const inventory = JSON.parse(await readFile(args[1], 'utf8'));
    const reports = await findReports(args[2]);
    console.log(JSON.stringify(verifyCoverage(inventory, reports, identity, Number(args[3])), null, 2));
  } else {
    throw new Error('Usage: ci-coverage.mjs discover OUTPUT | verify INVENTORY REPORT_DIRECTORY SHARD_TOTAL');
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  main(process.argv.slice(2)).catch((error) => { console.error(error.message); process.exitCode = 1; });
}
