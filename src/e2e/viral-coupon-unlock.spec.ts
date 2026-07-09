import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Coupon Unlock Loop', () => {
  test('should generate a share-to-unlock coupon loop', async ({ page }) => {
    // Navigate to dashboard
    await adminPage(page);

    // Click the new Viral Coupon Unlock link
    await page.click('a#viral-coupon-unlock-link');

    // Wait for the viral coupon unlock page to load
    await expect(page.locator('h1', { hasText: 'Share-to-Unlock Coupon 🎁' })).toBeVisible();

    // Fill in the form
    await page.fill('input#offerName', '50% Off Lifetime Pro');
    await page.fill('input#couponCode', 'PRO50LIFETIME');
    await page.fill('input#sharesReq', '5');

    // Check that the preview updates
    await expect(page.locator('#previewTitle')).toHaveText('50% Off Lifetime Pro');
    await expect(page.locator('#previewCode')).toHaveText('PRO50LIFETIME');
    await expect(page.locator('#shareCountText')).toHaveText('5');
    await expect(page.locator('#previewShares')).toHaveText('5');

    // Test the copy link functionality
    await page.click('button#copyBtn');
    await expect(page.locator('button#copyBtn')).toHaveText('Copied!');
  });
});
