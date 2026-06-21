import { test, expect } from './fixtures';

test.describe('Growth Viral Loop', () => {
  test('revenue milestone detection and celebration', async ({ page }) => {
    // Rely on e2e-seed.sql to provide the milestone
    await page.goto('/milestones');

    // Verify milestone is reached and title is correct
    await expect(page.locator('h3:has-text("Four-Figure Club")')).toBeVisible();

    // Verify share payload contains the new incentive
    await expect(page.locator('text=Join OHC & get 14 days of Pro free')).toBeVisible();
  });

  test('referral reward attribution', async ({ page }) => {
    await page.goto('/settings/referrals');

    // We expect the button to exist and generate the real URL via real interaction
    await page.locator('button:has-text("Generate Referral Link")').click();

    // Wait for the URL to be displayed in the UI.
    await expect(page.locator('text=/https:\\/\\/ohc\\.app\\/ref\\/[a-zA-Z0-9_-]+/')).toBeVisible();
  });
});
