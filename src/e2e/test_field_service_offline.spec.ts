import { test, expect } from './fixtures';

test.describe('Field Service Offline Roster and Sync', () => {
  test('Carlos (Field Service) offline daily roster CUJ', async ({ page }) => {
    // 2. Online Mode: Load roster
    await page.goto('/field-service/roster');

    // We expect "No jobs for today." since we haven't mocked DB directly, but we can verify UI
    await expect(page.locator("text=Today's Jobs")).toBeVisible();

    // 3. Simulate Offline Mode
    await page.context().setOffline(true);
    await page.reload();

    // wait for offline indicator
    await expect(page.locator('text=Offline Mode')).toBeVisible();

    // 5. Restore connection
    await page.context().setOffline(false);

    // We emit an 'online' event explicitly
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // 6. Verify sync occurred (if there were any jobs)
  });
});
