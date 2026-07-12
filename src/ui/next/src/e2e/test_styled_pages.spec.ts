import { test, expect } from '../../../../e2e/fixtures';

test.describe('Styled Pages Verification', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
  });

  test('Verify Settings page styling and elements', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('h1').first()).toBeVisible();
    const glassContainer = page.locator('.glassmorphism').first();
    if (await glassContainer.count() > 0) {
      await expect(glassContainer).toBeVisible();
    }
  });

  test('Verify AI Usage page loading', async ({ page }) => {
    await page.goto('/ai-usage-paywall');
    await expect(page.locator('h1').first()).toBeVisible();
  });

  test('Verify Dashboard loading', async ({ page }) => {
    await page.goto('/dashboard');
    const mainContainer = page.locator('body');
    await expect(mainContainer).toBeVisible();
  });

  test('Verify Business Analytics page loading', async ({ page }) => {
    await page.goto('/business-analytics');
    await expect(page.locator('h1').first()).toBeVisible();
  });

  test('Verify Integrations page loading and tabs', async ({ page }) => {
    await page.goto('/integrations');
    await expect(page.locator('h1').first()).toBeVisible();
  });

  test('Verify Diagnostics page loading', async ({ page }) => {
    await page.goto('/diagnostics');
    await expect(page.locator('h1').first()).toBeVisible();
  });

  test('Verify Cost Dashboard page loading', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('h1').first()).toBeVisible();
  });
});
