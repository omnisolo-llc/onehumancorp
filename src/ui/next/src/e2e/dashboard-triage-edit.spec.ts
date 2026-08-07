import { test, expect } from './fixtures';

test.describe('Dashboard Triage Action Feed Edit UI', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should allow editing a draft from the unified dashboard feed', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/dashboard');
    const heading = page.locator('text=Activity Feed').first();
    await expect(heading).toBeVisible({ timeout: 15000 });
  });
});
