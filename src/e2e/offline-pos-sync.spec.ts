import { test, expect } from '@playwright/test';
import { memberPage as page } from './fixtures';

test.describe('Offline POS Sync', () => {
  test('should handle offline sales and sync upon reconnection', async ({ page, context }) => {
    // 1. Navigate to POS as Priya
    await page.goto('/ui/pos.html');

    // Wait for the POS to load and a product to be visible
    await expect(page.locator('.ohc-premium-card').first()).toBeVisible();

    // 2. Simulate going offline
    await context.setOffline(true);
    // Give it a moment to detect offline status
    await page.waitForTimeout(500);

    // Assert that the offline indicator is shown
    const networkIndicator = page.locator('#network-status-indicator');
    await expect(networkIndicator).toBeVisible();
    await expect(networkIndicator).toContainText('Offline - Sync Pending');

    // Ensure the glassmorphism styling is applied
    const bg = await networkIndicator.evaluate((el) => {
      return window.getComputedStyle(el).backgroundColor;
    });
    // rgba(255, 255, 255, 0.65)
    expect(bg).toContain('rgba(255, 255, 255, 0.65)');

    // 3. Process an in-store sale while offline
    // Assuming there's a button to mark as sold out or process transaction
    const firstProduct = page.locator('.ohc-premium-card').first();
    const actionButton = firstProduct.locator('button').first();
    await actionButton.click();

    // 4. Simulate reconnecting
    await context.setOffline(false);
    await page.waitForTimeout(1000);

    // Assert that it says Syncing... and then disappears or changes
    await expect(networkIndicator).toContainText('Syncing...');

    // Wait for sync to complete (indicator should be hidden)
    await expect(networkIndicator).toBeHidden({ timeout: 10000 });
  });
});
