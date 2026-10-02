import test from 'node:test';
import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
const { runFiniteClickInventory } = require('./ui-audit-inventory.cjs');

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
