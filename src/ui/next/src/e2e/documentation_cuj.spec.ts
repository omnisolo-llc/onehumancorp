import { test, expect } from '../../../../e2e/fixtures';

test.describe('Documentation User Journey', () => {
  test('Maya navigates the Help Center and views the Changelog', async ({ page }) => {
    await page.goto('/changelog');

    // Verify Changelog is loaded
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();

    // Now Maya navigates to the Help Center (using the generic help widget since it's the standard entrypoint)
    await page.goto('/help'); // Playwright can't easily click floating elements if they animate

    // Verify Help Center is loaded
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

    // Verify Categories from the fallback we added
    await expect(page.locator('h2', { hasText: 'Getting Started' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'My Store' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Payments' })).toBeVisible();

    // Verify Videos list loads
    await expect(page.locator('h2', { hasText: 'Video Tutorials' })).toBeVisible({ timeout: 10000 });

    const searchInput = page.locator('input[placeholder="Search for help articles and videos..."]');
    const responsePromise = page.waitForResponse(response =>
        response.url().includes("/api/help/search") &&
        (response.status() === 200 || response.status() === 304)
    );

    // Maya searches for "products" to learn how to add products
    await searchInput.fill('products');
    await responsePromise;

    // Click on the article
    const myStoreLink = page.locator('h3', { hasText: 'Adding Products' });
    await expect(myStoreLink).toBeVisible({ timeout: 10000 });
  });
});
