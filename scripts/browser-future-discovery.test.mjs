import test from 'node:test';
import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import { appendFile, mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { runNativeCommand } from './native-process.mjs';
import { testEnvironment } from './native-e2e.mjs';
import shards from './browser-shards.cjs';

const root = fileURLToPath(new URL('..', import.meta.url));
const require = createRequire(import.meta.url);

test('fresh Playwright discovery includes appended tests and nested new files exactly once', async t => {
  const directory = await mkdtemp(path.join(tmpdir(), 'ohc-future-discovery-'));
  const fixture = path.join(directory, 'specs');
  const groupsBefore = structuredClone(shards.GROUPS);
  const source = { commit: 'a'.repeat(40), sourceDigest: 'b'.repeat(64) };
  const addedTitles = ['appended test', 'new nested file'];
  const environment = { ...testEnvironment(), PLAYWRIGHT_TEST_DIR: fixture };
  const cli = require.resolve('@playwright/test/cli');
  const testImport = `import { test } from ${JSON.stringify(require.resolve('@playwright/test'))};\n`;
  const declaration = title => `test(${JSON.stringify(title)}, () => { throw new Error('Discovery fixture must never execute'); });\n`;
  let latestFull;
  let discoveries = 0;
  let prepared;
  const list = async selection => {
    const items = await shards.readDiscovery(filename => {
      discoveries++;
      return runNativeCommand(process.execPath, [cli, 'test', '--config', path.join(root, 'playwright.config.ts'),
        '--list', '--reporter', path.join(root, 'scripts/browser-inventory-reporter.cjs'),
        '--workers=2', '--retries=0', ...selection], {
        cwd: root, env: { ...environment, OHC_BROWSER_INVENTORY_OUTPUT: filename }, quiet: true, timeoutMs: 30_000,
      });
    });
    if (selection.length === 0) latestFull = items;
    return items;
  };
  try {
    await mkdir(fixture);
    const existing = path.join(fixture, 'existing.spec.ts');
    // Keep every logical slice nonempty without depending on the product's test count.
    await writeFile(existing, testImport + Array.from({ length: shards.GROUPS.flat().length * 2 }, (_, i) => declaration(`existing ${i}`)).join(''));
    const baseline = await list([]);
    const baselineIds = new Set(baseline.map(item => item.id));

    await appendFile(existing, declaration(addedTitles[0]));
    await mkdir(path.join(fixture, 'nested'));
    await writeFile(path.join(fixture, 'nested', 'new.spec.ts'), testImport + declaration(addedTitles[1]));

    // This production entry point freshly lists the full suite and all twelve
    // logical slices, then round-trips group 1 through Playwright's --test-list.
    prepared = await shards.prepareGroupedShard({ index: 1, source, list });
    const proof = JSON.parse(await readFile(prepared.proofFile, 'utf8'));
    const { inventory: full, slices } = proof;
    assert.equal(full.length, baseline.length + addedTitles.length);
    shards.sameInventory(full.filter(item => baselineIds.has(item.id)), baseline);
    const added = full.filter(item => !baselineIds.has(item.id));
    assert.deepEqual(added.map(item => item.title).sort(), [...addedTitles].sort());
    assert.ok(added.find(item => item.title === addedTitles[0]).file.endsWith('/existing.spec.ts'));
    assert.ok(added.find(item => item.title === addedTitles[1]).file.endsWith('/nested/new.spec.ts'));

    const groups = shards.groupInventory(full, slices);
    const selectors = new Map(latestFull.map(item => [item.id, item.selector]));
    // Round-trip the remaining physical groups using actual discovered selectors.
    for (let index = 1; index < groups.length; index++) {
      const filename = path.join(directory, `group-${index + 1}.txt`);
      await writeFile(filename, groups[index].map(item => selectors.get(item.id)).join('\n') + '\n');
      groups[index] = await list(['--test-list', filename]);
      shards.sameInventory(groups[index], proof.groups[index].flatMap(logical => slices[logical - 1]));
    }
    shards.sameInventory(groups.flat(), full);
    assert.deepEqual(shards.GROUPS, groupsBefore);
    assert.equal(slices.length, 12);
    assert.equal(groups.length, 3);
    for (const item of added) {
      assert.equal(slices.flat().filter(test => test.id === item.id).length, 1);
      assert.equal(groups.flat().filter(test => test.id === item.id).length, 1);
      const logical = slices.findIndex(slice => slice.some(test => test.id === item.id)) + 1;
      const physical = groups.findIndex(group => group.some(test => test.id === item.id));
      assert.ok(shards.GROUPS[physical].includes(logical));
      const omitted = structuredClone(slices);
      omitted[logical - 1] = omitted[logical - 1].filter(test => test.id !== item.id);
      assert.throws(() => shards.groupInventory(full, omitted), /browser inventory size mismatch/);
      const duplicated = structuredClone(slices);
      duplicated[logical - 1].push(item);
      assert.throws(() => shards.groupInventory(full, duplicated), /duplicate test identity/);
    }

    // These are identity-only receipt fixtures, not claimed browser executions.
    const receipts = shards.GROUPS.flatMap((group, index) => group.map(logical => ({
      shard: { index: logical, total: 12 }, selection: slices[logical - 1], tests: structuredClone(slices[logical - 1]),
      groupedInventory: { ...proof, shard: { current: index + 1, total: 3 }, logicalShard: logical },
    })));
    shards.validateGroupedReceipts(receipts, source, true);
    for (const item of added) {
      const omitted = structuredClone(receipts);
      const original = omitted.find(receipt => receipt.tests.some(test => test.id === item.id));
      original.tests = original.tests.filter(test => test.id !== item.id);
      assert.throws(() => shards.validateGroupedReceipts(omitted, source, true), /browser inventory size mismatch/);
      const duplicated = structuredClone(receipts);
      duplicated.find(receipt => receipt.tests.some(test => test.id === item.id)).tests.push(item);
      assert.throws(() => shards.validateGroupedReceipts(duplicated, source, true), /duplicate test identity/);
    }
    t.diagnostic(`${discoveries} real Playwright discoveries: ${baseline.length} -> ${full.length} identities; both additions occur once across 12 logical shards and 3 physical groups`);
  } finally {
    if (prepared) await rm(prepared.directory, { recursive: true, force: true });
    await rm(directory, { recursive: true, force: true });
  }
});
