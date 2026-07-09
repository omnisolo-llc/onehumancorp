import { test, expect } from '@playwright/test';


test.describe('Viral Coupon Unlock Loop (Next.js)', () => {
  test('should generate a share-to-unlock coupon loop', async ({ page }) => {
    // Navigate to the Next.js version
    await page.goto('/viral-coupon-unlock');

    // Wait for the viral coupon unlock page to load
    await expect(page.locator('h1', { hasText: 'Share-to-Unlock Coupon 🎁' })).toBeVisible();

    // Fill in the form
    await page.fill('input[placeholder="e.g. 20% Off Your First Order"]', '50% Off Lifetime Pro');
    await page.fill('input[placeholder="e.g. WELCOME20"]', 'PRO50LIFETIME');
    await page.fill('input[type="number"]', '5');

    // Check that the preview updates
    await expect(page.locator('h2', { hasText: '50% Off Lifetime Pro' })).toBeVisible();
    await expect(page.locator('p', { hasText: 'PRO50LIFETIME' })).toBeVisible();
    await expect(page.locator('p', { hasText: 'Unlock this exclusive coupon by sharing with 5 friends!' })).toBeVisible();

    // Test the copy link functionality
    await page.click('button:has-text("Copy Link")');
    await expect(page.locator('button:has-text("Copied!")')).toBeVisible();
  });
});
