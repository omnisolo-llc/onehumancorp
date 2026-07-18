import { test, expect } from './fixtures';

test.describe('The Promoter Agent CUJ', () => {
  test('generates social post and SEO tags for a new product', async ({ page, loginAs, unlimitedAdminUser }) => {
    // Login to ensure we have access
    await loginAs(page, unlimitedAdminUser);

    // We start at the homepage/triage feed
    await page.goto('/dashboard.html');

    // Wait for the dashboard to load
    await expect(page.locator('text="The Promoter Agent"')).toBeVisible();

    // For this test, we navigate directly to the promoter page which we added a link for
    await page.click('text="Promote New Product"');

    // The button has ID generate-btn
    await expect(page.locator('#generate-btn')).toBeVisible();

    // Fill in product details
    await page.fill('#product-name', 'Vegan Chocolate Cake');
    await page.fill('#product-desc', 'Delicious vegan chocolate cake with organic ingredients.');

    // Click generate
    await page.click('#generate-btn');

    // Wait for the results section to be visible
    await expect(page.locator('#results-section')).toBeVisible({ timeout: 15000 });

    // Assert that at least one variant card was generated
    const variantCards = page.locator('.variant-card');
    await expect(variantCards.first()).toBeVisible();
    expect(await variantCards.count()).toBeGreaterThan(0);
  });
});
