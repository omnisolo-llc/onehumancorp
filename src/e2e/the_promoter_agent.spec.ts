import { test, expect } from './fixtures';

test.describe('The Promoter Agent CUJ', () => {
  test('generates social post and SEO tags for a new product', async ({ page, loginAs, unlimitedAdminUser }) => {
    // Login to ensure we have access
    await loginAs(page, unlimitedAdminUser);

    // Navigate to Catalog to add a product
    await page.goto('/catalog');

    // Check if the page contains the add product button
    await expect(page.locator('text="Add Product"')).toBeVisible();
    await page.click('text="Add Product"');

    // Fill out the product form
    await page.fill('input[name="name"]', 'Awesome New Mug');
    await page.fill('textarea[name="description"]', 'A beautifully crafted ceramic mug.');
    await page.fill('input[name="price"]', '15.00');

    // Click submit
    await page.click('button[type="submit"]');

    // Wait for creation
    await expect(page.locator('text="Awesome New Mug"')).toBeVisible();

    // Now go to the Agent Feed to check for the Promoter action
    await page.goto('/dashboard.html');

    // Wait for the feed to load
    await expect(page.locator('text="Agent Feed"')).toBeVisible();

    // Verify the Promoter card appears for the new product
    await expect(page.locator('text="✨"')).toBeVisible();
    await expect(page.locator('text="The Promoter"')).toBeVisible();
    await expect(page.locator('text="Awesome New Mug"')).toBeVisible();

    // Verify variants generated
    await expect(page.locator('body')).toContainText('Variant 1 (Instagram):');
    await expect(page.locator('body')).toContainText('Variant 2 (TikTok):');
    await expect(page.locator('body')).toContainText('Variant 3 (Facebook):');

  });
});
