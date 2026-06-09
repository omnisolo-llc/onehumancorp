import { expect, test } from '@playwright/test';

test.describe('Unified Agent Feed Mobile Test', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render properly and handle tabs', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Wait for the unified agent feed to load
    await expect(page.locator('button', { hasText: /Proposals/ }).first()).toBeVisible({ timeout: 15000 });
    await expect(page.locator('button', { hasText: 'Activity Feed' })).toBeVisible();

    // Switch tabs
    await page.locator('button', { hasText: 'Activity Feed' }).click();

    // Verify glassmorphism CSS
    const feedContainer = page.locator('.glassmorphism').first();
    await expect(feedContainer).toBeVisible();
    await expect(feedContainer).toHaveCSS('backdrop-filter', /blur\(30px\)/);

    // Switch back
    await page.locator('button', { hasText: /Proposals/ }).first().click();
  });
});
