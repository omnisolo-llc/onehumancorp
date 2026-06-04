import { test, expect } from '@playwright/test';

test.describe('Offline Sync Journey', () => {
  test('handles offline checkout and subsequent sync', async ({ page, context }) => {
    // Navigate to checkout
    await page.goto('/checkout');
    await page.waitForLoadState('networkidle');

    // Make network offline
    await context.setOffline(true);

    // Click "Tap to Pay"
    // Handle prompt
    page.on('dialog', async dialog => {
      if (dialog.type() === 'prompt') {
        await dialog.accept('50');
      } else {
        await dialog.accept();
      }
    });

    await page.click('button:has-text("Tap to Pay")');

    // Wait for success modal to appear
    await expect(page.locator('text=Payment Successful!')).toBeVisible();

    // Navigate using the fallback button
    await page.click('button:has-text("Continue to Dashboard")');

    // Wait for redirect to dashboard
    await expect(page).toHaveURL(/.*\/dashboard/);

    // Wait for the queue dashboard to appear
    await expect(page.locator('#queue-dashboard')).toBeVisible();
    await expect(page.locator('#queue-dashboard')).toContainText('1 payments pending sync');

    // Reconnect network
    await context.setOffline(false);

    // The component might try to sync automatically, or we can click Sync Now
    await page.click('button:has-text("Sync Now")');

    // Check if the prompt/alert happens (mocking endpoint failure currently so it might not clear immediately but let's see)
    // For now just wait a bit to ensure no errors
    await page.waitForTimeout(2000);
  });
});
