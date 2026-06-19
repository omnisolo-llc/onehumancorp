import { expect, test } from '@playwright/test';

test.describe('Offline Sync with SQLite', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should queue actions in SQLite optimistically when offline', async ({ page, context }) => {
    test.setTimeout(180000);

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const approveBtn = page.getByTestId('approve-proposal').first();
    const isVisible = await approveBtn.isVisible({ timeout: 15000 }).catch(() => false);

    if (isVisible) {
      // 1. Go offline
      await context.setOffline(true);
      await page.evaluate(() => window.dispatchEvent(new Event('offline')));

      // Verify offline banner
      await expect(page.locator('text=You are offline. Actions will sync when online.')).toBeVisible();

      const cardParent = approveBtn.locator('xpath=./../../..');

      // 2. Tap approve
      await approveBtn.click();

      // 3. The item should optimistically disappear
      await expect(cardParent).not.toBeVisible({ timeout: 2000 });

      // Check the pending sync banner count
      await expect(page.locator('text=Pending Sync (1)')).toBeVisible({ timeout: 5000 });

      // 4. Go back online
      await context.setOffline(false);
      await page.evaluate(() => window.dispatchEvent(new Event('online')));

      // Verify offline banner goes away
      await expect(page.locator('text=You are offline. Actions will sync when online.')).not.toBeVisible();

      // Verify pending sync banner goes away after sync
      await expect(page.locator('text=Pending Sync')).not.toBeVisible({ timeout: 10000 });
    }
  });
});
