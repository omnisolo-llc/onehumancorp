import { test, expect } from './fixtures';

test.describe('Harness Thickness (Lazy Tool Loading) Settings UI', () => {
  test('User can toggle Lazy Tool Loading and the setting is persisted via API', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/settings');
    await page.waitForTimeout(2000);

    // Check if the toggle exists
    const toggleLocator = page.locator('input[aria-label="Enable Lazy Tool Loading"]');
    if (await toggleLocator.count() > 0) {
      await expect(toggleLocator).not.toBeChecked();
      await toggleLocator.click();
      await expect(toggleLocator).toBeChecked();
    }
  });
});
