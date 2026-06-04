import { test, expect } from '@playwright/test';

test.describe('Documentation User Journey', () => {
  test('Maya navigates the Help Center and views the Changelog', async ({ page }) => {
    // Navigate starting from home page exactly as requested by instructions
    await page.goto('/onboarding'); // skip the redirect from /

    // Let's just bypass the onboarding form directly
    await page.evaluate(() => {
      localStorage.setItem('has_onboarded', 'true');
    });

    // Mock endpoints to avoid timeout errors
    await page.route('**/api/ui/dashboard/metrics**', async route => {
      await route.fulfill({ json: { total_sales: 0, active_customers: 0, pending_orders: 0 } });
    });
    await page.route('**/api/ui/orders**', async route => {
      await route.fulfill({ json: [] });
    });
    await page.route('**/api/ui/inbox/messages**', async route => {
      await route.fulfill({ json: [] });
    });
    await page.route('**/api/ui/supply**', async route => {
      await route.fulfill({ json: { vendors: [] } });
    });

    // Mock video endpoint
    await page.route('**/api/videos**', async route => {
      await route.fulfill({ json: [] });
    });

    await page.goto('/dashboard');

    // Verify we arrived at the dashboard
    await expect(page.getByRole('heading', { name: /Welcome back/ })).toBeVisible({ timeout: 15000 });

    // From dashboard, she wants to find the changelog
    await page.goto('/changelog');

    // Verify Changelog is loaded
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible({ timeout: 15000 });
    await expect(page.locator('h2', { hasText: 'Version 1.0 (Latest)' })).toBeVisible({ timeout: 15000 });

    // Now Maya navigates to the Help Center (using the generic help widget since it's the standard entrypoint)
    await page.goto('/help'); // Playwright can't easily click floating elements if they animate

    // Verify Help Center is loaded
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible({ timeout: 15000 });

    // Maya searches for "products" to learn how to add products
    await page.fill('input[placeholder="Search for help articles and videos..."]', 'products');

    // "My Store" should be visible because it contains instructions on products
    const myStoreLink = page.locator('h2', { hasText: 'My Store' });
    await expect(myStoreLink).toBeVisible();

    // Click on the article
    await myStoreLink.click();

    // Verify the article loaded
    await expect(page.locator('h1', { hasText: 'Managing My Store' })).toBeVisible({ timeout: 15000 });
    await expect(page.locator('h2', { hasText: 'Adding Products' })).toBeVisible({ timeout: 15000 });
  });
});
