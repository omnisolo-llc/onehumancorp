'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const os = require('node:os');

// Preserve all twelve service/browser lifecycles. Balance actual wall times
// across runs 37209169837, 37218212302 and 37220871509. All modeled bins fit
// 15–20 active minutes; failed-run timings do not certify correctness.
const BROWSER_BUDGET_MS = 60 * 60 * 1000;
const GROUPS = [[1, 8, 9, 10, 11, 12], [2, 3], [4, 5, 6, 7]];
const identity = ({ id, title, file }) => ({ id, title, file });
function inventory(items) {
  assert.ok(Array.isArray(items) && items.length, 'missing complete browser inventory');
  const map = new Map();
  for (const item of items) {
    assert.ok(item && typeof item.id === 'string' && item.id && typeof item.title === 'string' && typeof item.file === 'string', 'invalid test identity');
    assert.ok(!map.has(item.id), 'duplicate test identity');
    map.set(item.id, identity(item));
  }
  return map;
}
function sameInventory(actual, expected) {
  const left = inventory(actual), right = inventory(expected);
  assert.equal(left.size, right.size, 'browser inventory size mismatch');
  for (const [id, item] of left) assert.deepEqual(item, right.get(id), 'missing or changed browser identity');
}
function groupInventory(full, slices, groups = GROUPS) {
  assert.deepEqual(groups.flat().slice().sort((a,b) => a-b), Array.from({ length: 12 }, (_,i) => i+1), 'every logical shard must occur exactly once');
  assert.equal(slices.length, 12, 'all twelve logical inventories are required');
  sameInventory(slices.flat(), full);
  const selections = groups.map(group => group.flatMap(index => slices[index-1]));
  sameInventory(selections.flat(), full);
  return selections;
}
function validateProof(proof, context, selected) {
  assert.equal(proof?.schemaVersion, 1, 'invalid grouped inventory schema');
  assert.deepEqual(proof.source, { commit: context.commit, sourceDigest: context.sourceDigest }, 'stale grouped inventory source');
  assert.ok(Number.isInteger(proof.shard?.current) && proof.shard.current >= 1 && proof.shard.current <= 3 && proof.shard.total === 3, 'invalid grouped shard');
  assert.deepEqual(proof.groups, GROUPS, 'changed logical grouping');
  groupInventory(proof.inventory, proof.slices, proof.groups);
  assert.ok(Number.isInteger(proof.logicalShard) && GROUPS[proof.shard.current-1].includes(proof.logicalShard), 'invalid logical shard assignment');
  sameInventory(selected, proof.slices[proof.logicalShard-1]);
  return { current: proof.logicalShard, total: 12 };
}
function validateGroupedReceipts(receipts, context, required = false) {
  if (required) assert.ok(receipts.every(receipt => receipt.groupedInventory), 'missing required full discovery proof');
  if (!receipts.some(receipt => receipt.groupedInventory)) return;
  assert.equal(receipts.length, 12, 'all twelve logical receipts required');
  assert.deepEqual(receipts.map(receipt => receipt.shard.index).sort((a,b)=>a-b), Array.from({length:12},(_,i)=>i+1), 'every logical receipt required exactly once');
  const full = receipts[0].groupedInventory?.inventory;
  for (const receipt of receipts) {
    validateProof(receipt.groupedInventory, context, receipt.selection);
    assert.deepEqual(receipt.shard, { index: receipt.groupedInventory.logicalShard, total: 12 }, 'receipt logical identity mismatch');
    sameInventory(receipt.groupedInventory.inventory, full);
    sameInventory(receipt.tests, receipt.selection);
  }
  sameInventory(receipts.flatMap(receipt => receipt.tests), full);
}
async function readDiscovery(execute) {
  const directory = await fs.mkdtemp(path.join(os.tmpdir(), 'ohc-browser-inventory-'));
  const filename = path.join(directory, 'discovery.json');
  try {
    await execute(filename);
    const result = JSON.parse(await fs.readFile(filename, 'utf8'));
    inventory(result);
    return result;
  } finally {
    await fs.rm(directory, { recursive: true, force: true });
  }
}
async function prepareGroupedShard({ index, source, list }) {
  assert.ok(Number.isInteger(index) && index >= 1 && index <= 3, 'invalid grouped shard');
  const full = await list([]);
  const slices = [];
  for (let logical = 1; logical <= 12; logical++) slices.push(await list([`--shard=${logical}/12`]));
  const selected = groupInventory(full, slices)[index-1];
  const directory = await fs.mkdtemp(path.join(os.tmpdir(), 'ohc-browser-group-'));
  try {
    const filename = path.join(directory, 'tests.txt');
    for (const item of selected) assert.ok(typeof item.selector === 'string' && !/[\r\n]/.test(item.selector), 'invalid test selector');
    await fs.writeFile(filename, selected.map(item => item.selector).join('\n')+'\n', { mode: 0o600 });
    // Prove that the supported --test-list parser selects the exact identities,
    // rather than trusting selector text or comparing only counts.
    sameInventory(await list(['--test-list', filename]), selected);
    const proof = { schemaVersion: 1, source, shard: { current: index, total: 3 }, groups: GROUPS,
      inventory: full.map(identity), slices: slices.map(slice => slice.map(identity)) };
    const proofFile = path.join(directory, 'inventory.json');
    await fs.writeFile(proofFile, JSON.stringify(proof), { mode: 0o600 });
    console.log(`Grouped browser inventory ${index}/3: ${selected.length}/${full.length} exact identities; logical shards ${GROUPS[index-1].join(',')}`);
    return { args: ['--test-list', filename], proofFile, directory };
  } catch (error) {
    await fs.rm(directory, {recursive:true,force:true});
    throw error;
  }
}
async function runGroupedUnits({index, source, list, runUnit, signal = new AbortController().signal,
  budgetMs = BROWSER_BUDGET_MS, now = () => performance.now()}) {
  signal.throwIfAborted();
  const prepared = await prepareGroupedShard({index, source, list});
  let remaining = budgetMs;
  const failures = [];
  try {
    const proof = JSON.parse(await fs.readFile(prepared.proofFile, 'utf8'));
    for (const logical of GROUPS[index-1]) {
      signal.throwIfAborted();
      assert.ok(remaining > 0, 'cumulative browser budget exhausted');
      const proofFile = path.join(prepared.directory, `logical-${logical}.json`);
      await fs.writeFile(proofFile, JSON.stringify({...proof, logicalShard:logical}), {mode:0o600,flag:'wx'});
      const runBrowser = async execute => {
        signal.throwIfAborted();
        assert.ok(remaining > 0, 'cumulative browser budget exhausted');
        const start = now();
        try { return await execute({timeoutMs:Math.max(1,Math.floor(remaining)), signal}); }
        finally { remaining -= Math.max(0, now()-start); }
      };
      console.log(`Starting isolated logical shard ${logical}/12 in physical runner ${index}/3`);
      try { await runUnit(logical, {proofFile, signal, runBrowser, artifactSuffix:`logical-${logical}`}); }
      catch (error) {
        failures.push(error);
        console.error(`Logical shard ${logical}/12 failed: ${error.message}`);
      }
      // An ordinary assertion failure still permits the other distinct units;
      // owner cancellation and budget exhaustion never launch another unit.
      signal.throwIfAborted();
      assert.ok(remaining > 0, 'cumulative browser budget exhausted');
    }
    if (failures.length) throw new AggregateError(failures, failures.map(error=>error.message).join('\n'));
  } finally { await fs.rm(prepared.directory, {recursive:true,force:true}); }
}
module.exports = { BROWSER_BUDGET_MS, GROUPS, sameInventory, groupInventory, validateProof, validateGroupedReceipts, readDiscovery, prepareGroupedShard, runGroupedUnits };
