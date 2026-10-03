import test from 'node:test';
import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
const { runFiniteClickInventory, runDynamicClickInventory, scopeClickInventory } = require('./ui-audit-inventory.cjs');

test('every finite target is visited exactly once even when cumulative work exceeds a flat route deadline', async t => {
  const baseline = Array.from({ length: 35 }, (_, index) => ({ key: `operation-${index}`, label: `Operation ${index}` }));
  const clicks = [], resets = [], steps = [];
  let elapsed = 0;
  t.mock.method(Date, 'now', () => elapsed);
  await runFiniteClickInventory(baseline, {
    discover: async () => baseline,
    visit: async target => { clicks.push(target.key); elapsed += 3000; },
    reset: async () => { resets.push(elapsed); },
    step: async (target, operation, timeout) => { steps.push({ key: target.key, timeout }); await operation(); },
  });
  assert.ok(elapsed > 90000);
  assert.deepEqual(clicks, baseline.map(target => target.key));
  assert.equal(resets.length, baseline.length);
  assert.equal(steps.length, baseline.length);
  assert.ok(steps.every(step => step.timeout === 30000));
});

test('a destructive prior click cannot erase later required coverage', async () => {
  const baseline = [{ key: 'remove', label: 'Remove' }, { key: 'review', label: 'Review' }];
  let current = baseline;
  const clicked = [];
  await assert.rejects(runFiniteClickInventory(baseline, {
    discover: async () => current,
    visit: async target => { clicked.push(target.key); current = [baseline[0]]; },
    reset: async () => {},
    step: async (_target, operation) => operation(),
  }), /missing=.*review/);
  assert.deepEqual(clicked, ['remove']);
});

test('an uncertain click failure is never dispatched again', async () => {
  const baseline = [{ key: 'submit', label: 'Submit' }];
  let clicked = 0;
  await assert.rejects(runFiniteClickInventory(baseline, {
    discover: async () => baseline,
    visit: async () => { clicked += 1; throw new Error('document moved after dispatch'); },
    reset: async () => {},
    step: async (_target, operation) => operation(),
  }), /document moved/);
  assert.equal(clicked, 1);
});


test('dynamic discovery fails when an unobserved target disappears across reset', async () => {
  const baseline = [{ key: 'dismiss', label: 'Dismiss' }, { key: 'approve', label: 'Approve' }];
  let current = baseline;
  const discovered = [], observed = new Set();
  await assert.rejects(runDynamicClickInventory(discovered, observed, {
    discover: async () => current,
    visit: async target => { observed.add(target.key); },
    reset: async () => { current = [baseline[0]]; },
  }), /missing=.*approve/);
  assert.deepEqual(discovered, ['dismiss', 'approve']);
  assert.deepEqual([...observed], ['dismiss']);
});


test('dynamic discovery observes newly appearing targets without dropping older keys', async () => {
  const initial = { key: 'initial', label: 'Initial' };
  const later = { key: 'later', label: 'Later' };
  let current = [initial];
  const discovered = [], observed = new Set();
  await runDynamicClickInventory(discovered, observed, {
    discover: async () => current,
    visit: async target => { observed.add(target.key); },
    reset: async () => { current = [later]; },
  });
  assert.deepEqual(discovered, ['initial', 'later']);
  assert.deepEqual([...observed], discovered);
});

test('dynamic crawl never retries a click whose outcome is uncertain', async () => {
  const discovered = [], observed = new Set();
  let attempts = 0, resets = 0;
  await assert.rejects(runDynamicClickInventory(discovered, observed, {
    discover: async () => [{ key: 'submit', label: 'Submit' }],
    visit: async () => { attempts += 1; throw new Error('document moved after dispatch'); },
    reset: async () => { resets += 1; },
  }), /document moved after dispatch/);
  assert.equal(attempts, 1);
  assert.equal(resets, 0);
  assert.deepEqual(discovered, ['submit']);
});

test('dynamic crawl retains the existing bounded non-convergence failure', async t => {
  const discovered = [], observed = new Set();
  let elapsed = 0;
  t.mock.method(Date, 'now', () => elapsed);
  await assert.rejects(runDynamicClickInventory(discovered, observed, {
    discover: async () => [{ key: `target-${observed.size}`, label: `Target ${observed.size}` }],
    visit: async target => { observed.add(target.key); elapsed += 30_001; },
    reset: async () => {},
  }), /did not converge.*No remaining coverage was silently skipped/);
  assert.equal(observed.size, 3);
  assert.equal(discovered.length, 4);
});


test('identical control signatures in different wizard views retain separate coverage', () => {
  const targets = [{ key: 'same-back-button', index: 0, label: 'Back' }];
  const first = scopeClickInventory(targets, 'entry');
  const second = scopeClickInventory(targets, 'started-draft');
  assert.notEqual(first[0].key, second[0].key);
  assert.deepEqual(first, scopeClickInventory(targets, 'entry'));
  assert.equal(first[0].sourceKey, targets[0].key);
  assert.equal(second[0].sourceKey, targets[0].key);
  assert.equal(new Set([...first, ...second].map(target => target.key)).size, 2);
  assert.deepEqual(targets, [{ key: 'same-back-button', index: 0, label: 'Back' }]);
});
