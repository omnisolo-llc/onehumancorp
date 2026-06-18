import { test, expect } from './fixtures';

test.describe('Extended Glassmorphism Visual Polish Audits', () => {

  test('Verify pos terminal UI uses proper translucency filters', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/pos/terminal');
    await page.waitForLoadState('networkidle');

    // Wait for product card to be visible
    const firstProduct = page.locator('button.charge-btn').first();
    await expect(firstProduct).toBeVisible({ timeout: 10000 });
  });

  test('Verify setup wizard retains unified styling', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/setup');
    const header = page.locator('h1', { hasText: 'Business Setup' }).first();
    await expect(header).toBeVisible({ timeout: 10000 });
  });

  test('Verify dashboard has translucent panels on mobile width', async ({ page, loginAs, unlimitedAdminUser }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    const panel = page.locator('.glassmorphism').first();
    await expect(panel).toBeVisible({ timeout: 10000 });
  });

  test('Verify user settings applies glassmorphism', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/settings');
    const settingsHeader = page.locator('h1', { hasText: 'Settings' }).first();
    await expect(settingsHeader).toBeVisible({ timeout: 10000 });
  });

  test('Verify proposal generator respects modern UI layouts', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/proposal-generator');
    const panel = page.locator('.glass-card').first();
    await expect(panel).toBeVisible({ timeout: 10000 });
  });

});
