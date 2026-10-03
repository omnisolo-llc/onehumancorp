const { assertSameClickInventory } = require('./ui-audit-fixture.cjs');
const FINITE_CLICK_CASE_BUDGET = 30000;

// A finite route inventory is work, not a convergence problem. Each case has
// its own bounded lookup/action/reset deadline and must preserve every key.
async function runFiniteClickInventory(baseline, operations) {
  const expected = baseline.map(target => target.key);
  for (const target of baseline) {
    await operations.step(target, async () => {
      assertSameClickInventory(expected, (await operations.discover()).map(item => item.key));
      // Never retry this callback: it may already have dispatched a mutation.
      await operations.visit(target);
      await operations.reset();
    }, FINITE_CLICK_CASE_BUDGET);
  }
  assertSameClickInventory(expected, (await operations.discover()).map(item => item.key));
}
// The dynamic crawler retains its discovery history across document resets.
async function runDynamicClickInventory(discoveredKeys, audited, operations) {
  const startedAt = Date.now();
  while (true) {
    const candidates = await operations.discover();
    for (const target of candidates) if (!discoveredKeys.includes(target.key)) discoveredKeys.push(target.key);
    const candidate = candidates.find(target => !audited.has(target.key));
    if (!candidate) {
      // Exhaustion is about every historical discovery, not just the current
      // document. A persisted mutation must never silently erase coverage.
      assertSameClickInventory(discoveredKeys, [...audited]);
      return;
    }
    if (Date.now() - startedAt > 90_000) {
      throw new Error(`Click target enumeration did not converge after ${audited.size} targets; next=${candidate.label}. No remaining coverage was silently skipped.`);
    }
    await operations.visit(candidate);
    await operations.reset();
  }
}
function scopeClickInventory(inventory, state) {
  return inventory.map(target => ({ ...target, sourceKey: target.key, key: JSON.stringify([state, target.key]) }));
}
module.exports = { runFiniteClickInventory, runDynamicClickInventory, scopeClickInventory, FINITE_CLICK_CASE_BUDGET };
