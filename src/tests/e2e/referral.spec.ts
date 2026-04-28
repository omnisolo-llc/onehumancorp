import { test, expect } from '@playwright/test';

test.describe('Referral Program', () => {
  test('user can generate and copy referral link', async ({ page }) => {
    // Navigate to the user management page
    await page.goto('/user_management');

    // Verify the widget is present
    await expect(page.locator('text="Get 1 Month Free Pro"')).toBeVisible();
    await expect(page.locator('text="Share OHC with a friend, both get 1 month free Pro."')).toBeVisible();

    // Check for the referral link
    const referralLink = page.locator('text=/ohc:\\/\\/join\\?ref=/');
    await expect(referralLink).toBeVisible();

    // Verify copy button
    const copyButton = page.locator('text="Copy"');
    await expect(copyButton).toBeVisible();

    // Verify share button
    const shareButton = page.locator('text="Share via Message"');
    await expect(shareButton).toBeVisible();
  });
});
