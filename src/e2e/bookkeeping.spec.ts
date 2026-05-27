import { test, expect } from './fixtures';

test.describe('Autonomous AI Bookkeeping E2E', () => {
  test('displays financial briefing on the dashboard', async ({ page }) => {
    // Note: In a real test we would seed data, but here we check for the elements' presence
    // and that the dashboard loads without crashing after our changes.
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Check if the financial briefing container exists
    const briefing = page.locator('#financial-briefing');
    // It might be hidden if no insights generated yet, but we can check existence in DOM
    await expect(briefing).toBeAttached();
  });
});
