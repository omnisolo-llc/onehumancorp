import { test } from 'node:test';
import assert from 'node:assert/strict';
import { verifyCoverage } from './ci-coverage.mjs';

const identity = { sha: 'a'.repeat(40), run: '123', attempt: '1' };
const inventory = { kind: 'inventory', identity, complete: true, index: 0, total: 0,
  selected: ['root::case', 'src/ui/next/src/e2e/nested::case'] };
function reports() {
  return inventory.selected.map((id, index) => ({ kind: 'results', identity, complete: true,
    index: index + 1, total: 2, selected: [id], results: [{ id, status: 'passed', retry: 0 }] }));
}

test('complete nested and root discovery is accounted for exactly once', () => {
  assert.equal(verifyCoverage(inventory, reports(), identity, 2), 2);
});

test('rejects missing shards, nested tests, duplicate selections and unstarted tests', () => {
  const r = reports();
  for (const broken of [r.slice(0, 1), [r[0], r[0]], [r[0], { ...r[1], results: [] }],
    [r[0], { ...r[1], selected: [], results: [] }],
    [r[0], { ...r[1], selected: r[0].selected, results: r[0].results }]]) {
    assert.throws(() => verifyCoverage(inventory, broken, identity, 2));
  }
});

test('rejects wrong SHA, run, attempt, status, retry and incomplete finalization', () => {
  for (const key of ['sha', 'run', 'attempt']) {
    const r = reports(); r[1] = { ...r[1], identity: { ...identity, [key]: 'wrong' } };
    assert.throws(() => verifyCoverage(inventory, r, identity, 2));
  }
  for (const status of ['failed', 'timedOut', 'skipped', 'interrupted']) {
    const r = reports(); r[1].results[0].status = status;
    assert.throws(() => verifyCoverage(inventory, r, identity, 2));
  }
  const retried = reports(); retried[1].results[0].retry = 1;
  assert.throws(() => verifyCoverage(inventory, retried, identity, 2));
  const incomplete = reports(); incomplete[1].complete = false;
  assert.throws(() => verifyCoverage(inventory, incomplete, identity, 2));
  const duplicate = reports(); duplicate[1].results.push(duplicate[1].results[0]);
  assert.throws(() => verifyCoverage(inventory, duplicate, identity, 2));
});

test('empty, partial or foreign inventory cannot certify a run', () => {
  for (const broken of [{ ...inventory, selected: [] }, { ...inventory, complete: false },
    { ...inventory, index: 1 }, { ...inventory, identity: { ...identity, attempt: '2' } }]) {
    assert.throws(() => verifyCoverage(broken, reports(), identity, 2));
  }
});
