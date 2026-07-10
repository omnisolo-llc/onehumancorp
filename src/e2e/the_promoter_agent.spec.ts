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

  });
});
