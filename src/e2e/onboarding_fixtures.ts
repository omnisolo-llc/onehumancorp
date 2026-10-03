import { test as base, expect } from './fixtures';
import { createGrowthOwner } from './growth_owner';

/** Each fresh-setup journey owns its real tenant and progress, including retries.
 * Reuse the isolated database guard and normalized-role actor creator. Never
 * reset the shared admin's preparation or another test's durable draft.
 */
export const test = base.extend<{ onboardingOwner: Awaited<ReturnType<typeof createGrowthOwner>> }>({
  onboardingOwner: [async ({ page, baseURL }, use) => {
    await use(await createGrowthOwner(page, baseURL));
  }, { auto: true }],
});
export { expect };
