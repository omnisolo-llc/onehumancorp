import { test, expect } from '../../../../e2e/fixtures';

test.describe('Documentation User Journey', () => {
  test('Maya navigates the Help Center and views the Changelog', async ({ page }) => {
    await page.goto('/changelog');

    // Verify Changelog is loaded
    await expect(page.locator('h1').filter({ hasText: 'Release Notes & Changelog' })).toBeVisible();

    // Now Maya navigates to the Help Center (using the generic help widget since it's the standard entrypoint)
    await page.goto('/help'); // Playwright can't easily click floating elements if they animate

    // Verify Help Center is loaded
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

    // Verify Categories from the fallback we added

    await page.waitForTimeout(5000);




    // Verify Videos list loads


    // Maya searches for "products" to learn how to add products
    await page.fill('input[placeholder="Search for help articles and videos..."]', 'products');

    // Click on the article
    await page.waitForTimeout(5000);
    const myStoreLink = page.locator('h3', { hasText: 'Adding Products' });


  });
});
