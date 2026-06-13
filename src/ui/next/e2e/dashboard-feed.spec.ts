import { test, expect } from '@playwright/test';

test.describe('Dashboard Actionable Feed (Mobile First)', () => {
  // Use a simulated 375px mobile viewport context for this specific suite
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display Unified Agent Feed instead of legacy panels', async ({ page }) => {
    await page.goto('/dashboard');

    // Wait for the feed to load
    await expect(page.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();

    // Verify Unified Agent Feed is present and functional
    await expect(page.locator('text="Proposals"').first()).toBeVisible();
    await expect(page.locator('text="Activity Feed"').first()).toBeVisible();

    // The legacy panels should have been removed to adhere to the owner-centric AI automation design
    await expect(page.locator('text="Operations Map"').first()).not.toBeVisible();
    await expect(page.locator('text="Recent Orders"').first()).not.toBeVisible();
    await expect(page.locator('text="Inbox Activity"').first()).not.toBeVisible();
  });
});
