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
module.exports = { runFiniteClickInventory, FINITE_CLICK_CASE_BUDGET };
