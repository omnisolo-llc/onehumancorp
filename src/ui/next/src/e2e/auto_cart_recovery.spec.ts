import { test, expect } from '../../../../e2e/fixtures';

test.describe('Automated Cart Recovery Growth Loop', () => {
  test('Merchant enables auto-recovery via soft paywall trial extension', async ({ page, request }) => {
    // 1. Merchant navigates to cart recovery page
    await page.goto('/cart-recovery');

    // Check baseline: the toggle should be visible
    const automateHeader = page.locator('h3', { hasText: 'Automate with Agent Nova' });
    await expect(automateHeader).toBeVisible({ timeout: 10000 });

    const toggleBtn = page.locator('#auto-recovery-toggle');
    await expect(toggleBtn).toBeVisible();

    // Ensure it's not enabled initially
    await expect(page.locator('text=Auto-Recovery Enabled')).not.toBeVisible();

    // 2. Merchant tries to enable auto-recovery without Pro
    await toggleBtn.click();

    // 3. Soft paywall appears
    const upgradeHeader = page.locator('h2', { hasText: 'Upgrade to Pro' });
    await expect(upgradeHeader).toBeVisible();

    // 4. Merchant claims trial extension by sharing
    const shareBtn = page.locator('button', { hasText: 'Share on X to get 7 Days Free' });

    // We intercept the window.open call which happens in claimTrialExtension
    await page.evaluate(() => {
        window.open = function() { return null as any; };
    });

    await shareBtn.click();

    // 5. Verify the soft paywall closes and trial status shows
    await expect(upgradeHeader).not.toBeVisible();
    await expect(page.locator('text=Your 7-day Pro trial has been activated.')).toBeVisible();

    // 6. Verify auto-recovery is now enabled
    await expect(page.locator('text=Auto-Recovery Enabled')).toBeVisible();

    // Verify toggle reflects active state (orange background)
    await expect(toggleBtn).toHaveClass(/bg-orange-500/);

    // Also verify that generate draft was called successfully
    await expect(page.locator('text=✨ AI Generated Draft')).toBeVisible();
  });
});
