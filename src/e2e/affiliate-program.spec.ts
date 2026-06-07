import { test, expect } from '@playwright/test';

test.describe('Affiliate Program Growth Feature', () => {
  test('navigates to affiliate program and shows empty state', async ({ page }) => {
    // Start at dashboard
    await page.goto('/dashboard');

    // Find and click the Affiliate Program link
    await page.click('text=Affiliate Program');

    // Check URL
    await expect(page).toHaveURL(/.*\/affiliate-program/);

    // Verify headers
    await expect(page.locator('h1', { hasText: 'Affiliate Program' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Partner Program' })).toBeVisible();

    // Since there are no affiliates yet, we should see the empty state
    await expect(page.locator('text=No active affiliates yet')).toBeVisible();

    // Verify signup link generator is present
    await expect(page.locator('text=Invite Affiliates')).toBeVisible();
    await expect(page.locator('button', { hasText: 'Copy Link' })).toBeVisible();
  });
});