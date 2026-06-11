import { test, expect } from './fixtures';

test.describe('Growth Viral Loop', () => {
  test('revenue milestone detection and celebration', async ({ page }) => {
    // Mock the milestones API to simulate reaching $1k revenue
    await page.route('**/api/v1/growth/milestones/check*', async route => {
      const json = {
        milestones: [
          {
            id: 'revenue_1k',
            title: '💰 Four-Figure Club',
            description: 'Your business has surpassed $1,000 in total revenue!',
            reached: true
          }
        ]
      };
      await route.fulfill({ json });
    });

    await page.goto('/milestones');

    // Verify milestone is reached and title is correct
    await expect(page.locator('h3:has-text("Four-Figure Club")')).toBeVisible();

    // Verify share payload contains the new incentive
    await expect(page.locator('text=Join OHC & get 14 days of Pro free')).toBeVisible();
  });

  test('referral reward attribution', async ({ page }) => {
    await page.route('**/api/v1/growth/referrals/generate', async route => {
      await route.fulfill({
        json: { referral_link: 'https://ohc.app/ref/test-code' }
      });
    });

    await page.goto('/settings/referrals');
    await expect(page.locator('text=https://ohc.app/ref/test-code')).toBeVisible();
  });
});
