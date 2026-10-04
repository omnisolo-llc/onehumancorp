'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const os = require('node:os');

// Whole Playwright logical shards preserve its serial-suite grouping. These
// bins use run 37209169837's observed two-worker execution durations, not a
// test allowlist. Every run rediscovers all tests and all twelve logical units.
const GROUPS = [[1, 2, 10], [3, 5, 7, 8, 11], [4, 6, 9, 12]];
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
  const selections = groupInventory(proof.inventory, proof.slices, proof.groups);
  sameInventory(selected, selections[proof.shard.current-1]);
  return proof.shard;
}
function validateGroupedReceipts(receipts, context, required = false) {
  if (required) assert.ok(receipts.every(receipt => receipt.groupedInventory), 'missing required full discovery proof');
  if (!receipts.some(receipt => receipt.groupedInventory)) return;
  assert.equal(receipts.length, 3, 'all three grouped receipts required');
  const full = receipts[0].groupedInventory?.inventory;
  for (const receipt of receipts) {
    validateProof(receipt.groupedInventory, context, receipt.selection);
    assert.deepEqual(receipt.shard, { index: receipt.groupedInventory.shard.current, total: 3 }, 'receipt group identity mismatch');
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
}
module.exports = { GROUPS, sameInventory, groupInventory, validateProof, validateGroupedReceipts, readDiscovery, prepareGroupedShard };
