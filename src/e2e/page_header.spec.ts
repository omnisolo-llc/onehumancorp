import { test, expect } from './fixtures';

test.describe('PageHeader Component (macOS Translucent Glass CUJ)', () => {
  test('Owner navigates from dashboard to config settings and views PageHeader', async ({ page }) => {
    // 1. Owner starts at the dashboard (already logged in via fixture)
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // 2. Owner navigates to Config Settings (Simulating CUJ using actual dashboard link)
    await page.getByRole('link', { name: '⚙️ Config Settings Manage your account and preferences.' }).click();

    // 3. Owner views the settings page and the PageHeader should have the macOS translucent glass styles
    const headerContainer = page.locator('div.backdrop-blur-\\[30px\\]').first();
    await expect(headerContainer).toBeVisible();

    // Verify it has the correct macOS Translucent Glass properties
    await expect(headerContainer).toHaveCSS('background-color', 'rgba(255, 255, 255, 0.65)');
  });
});
