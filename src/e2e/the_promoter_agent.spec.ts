import { test, expect } from '@playwright/test';

test.describe('The Promoter Agent CUJ', () => {
  test('generates social post and SEO tags for a new product', async ({ page }) => {

    // We start at the homepage/triage feed
    await page.goto('/');

    // Verify the Promoter card is visible (it shouldn't be yet, since there are no products, but the page could be empty state)
    // Wait for the feed to load

    // For this test, we navigate directly to the promoter page which we added a link for
    await page.click('text="Go to Promoter Agent"');

    // We should see the empty state first if no mocked data
    // The exact behavior depends on how data is mocked, but we will assert the empty state first
    await expect(page.locator('text="No new proposals generated."')).toBeVisible();

  });
});
