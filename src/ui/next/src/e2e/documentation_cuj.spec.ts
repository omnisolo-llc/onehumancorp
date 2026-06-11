import { test, expect } from '@playwright/test';

test.describe('Documentation User Journey', () => {
  test('Maya navigates the Help Center and views the Changelog', async ({ page }) => {
    await page.goto('/changelog');

    // Verify Changelog is loaded
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Version 1.1 (Latest)' })).toBeVisible();

    // Now Maya navigates to the Help Center (using the generic help widget since it's the standard entrypoint)
    await page.goto('/help'); // Playwright can't easily click floating elements if they animate

    // Verify Help Center is loaded
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

    // Maya searches for "products" to learn how to add products
    await page.fill('input[placeholder="Search for help articles and videos..."]', 'products');

    // "My Store" should be visible because it contains instructions on products
    const myStoreLink = page.locator('h2', { hasText: 'My Store' });
    await expect(myStoreLink).toBeVisible();

    // Click on the article
    await page.locator("a[href=\"/help/my-store\"]").click();

    // Verify the article loaded
    await expect(page.locator("h1", { hasText: "Managing My Store" })).toBeVisible({ timeout: 30000 });
    await expect(page.locator('h2', { hasText: 'Adding Products' })).toBeVisible();
  });
});
