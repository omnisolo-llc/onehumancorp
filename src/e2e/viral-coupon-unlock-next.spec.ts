import { test, expect } from './fixtures';


test.describe('Viral Coupon Unlock Loop (Next.js)', () => {
  test('configures a coupon preview while publication and verification remain unavailable', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
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
    await expect(page.getByText('Configured target: 5 shares', { exact: true })).toBeVisible();
    await expect(page.getByText(/Share progress is unavailable/)).toBeVisible();

    // No publication receipt or referral verification exists for this editor.
    for (const name of ['Copy Link', 'Share on X', 'Share on WhatsApp']) {
      await expect(page.getByRole('button', { name, exact: true })).toBeDisabled();
    }
    await expect(page.getByText('🔒 Locked', { exact: true })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Copied!', exact: true })).toHaveCount(0);
  });
});
