import { test, expect } from './fixtures';

test.describe('Viral Referral Tier Widget', () => {
  test('should display the referral tier progress on the dashboard based on real application flow', async ({ page }) => {
    // Navigate to the Tauri dashboard
    await page.goto('/dashboard.html');

    // Wait for the tier section to be visible
    const tierSection = page.locator('#referral-tier-section');
    await expect(tierSection).toBeVisible({ timeout: 15000 });

    // Ensure it correctly states Bronze tier
    await expect(page.locator('#referral-tier-text')).toContainText('You are on the Bronze Tier!');

    // Since we're using the fallback E2E tenant seeded with 0 conversions initially,
    // we expect 5 more referrals needed to reach Silver
    await expect(page.locator('#referral-tier-subtext')).toContainText('5 more referrals needed to reach Silver Tier.');

    // Check that progress bar is rendered
    const progressBar = page.locator('#referral-tier-progress');
    await expect(progressBar).toBeVisible();

    // Width should be 0% since 0 / 5 = 0
    await expect(progressBar).toHaveCSS('width', '0px');
  });
});
