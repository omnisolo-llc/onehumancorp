import { test, expect } from './fixtures';

test.describe('Growth Loop: Business Health Gamification', () => {
  test('User can access business health score from dashboard and view action items', async ({ page }) => {
    // 1. Navigate to the dashboard
    await page.goto('/dashboard');

    // 2. Locate the "Business Health" widget link and click it
    const businessHealthLink = page.locator('a[href="/business-health"]');
    await expect(businessHealthLink).toBeVisible();
    await businessHealthLink.click();

    // 3. Verify navigation to the /business-health page
    await page.waitForURL('**/business-health');
    await expect(page.locator('h1', { hasText: 'Business Health' })).toBeVisible();

    // 4. Check for the score circle (e.g. 65 / 100)
    const scoreText = page.locator('span', { hasText: '65' });
    await expect(scoreText).toBeVisible();
    const scoreDenominator = page.locator('span', { hasText: '/ 100' });
    await expect(scoreDenominator).toBeVisible();

    // 5. Verify action items are present
    const actionItemsHeader = page.locator('h3', { hasText: 'Action Items to Improve' });
    await expect(actionItemsHeader).toBeVisible();

    // Check for a specific action item like "Add 3 more products"
    const actionItem = page.locator('h4', { hasText: 'Add 3 more products' });
    await expect(actionItem).toBeVisible();
  });
});
