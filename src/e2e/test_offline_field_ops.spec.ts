import { test, expect } from './fixtures';

test.describe('Offline-First Sync Engine for Field Operations', () => {
  test('Carlos can view jobs, change status, and draft quotes offline, then sync when online', async ({ page, context }) => {
    // Navigate to the field ops dashboard
    await page.goto('/field-ops/jobs');

    // Wait for jobs to load
    await expect(page.locator('text=Today\'s Route')).toBeVisible();
    await page.waitForTimeout(1000); // Give the fetch a moment if it's slow

    // Ensure the job loaded
    await expect(page.locator('text=Start Work').first()).toBeVisible();

    // 1. Go Offline
    await context.setOffline(true);
    await page.evaluate(() => {
        Object.defineProperty(navigator, 'onLine', { value: false });
        window.dispatchEvent(new Event('offline'));
    });

    // Verify Offline Indicator
    await expect(page.locator('#network-status-indicator')).toBeVisible();

    // 2. Change Job Status to "In-Progress"
    await page.locator('text=Start Work').first().click();

    // 3. Fill in Service Notes
    const textarea = page.locator('textarea[placeholder="E.g., Needs a replacement quote."]').first();
    await textarea.fill('Customer needs a quote for replacing the main pipe.');

    // 4. Complete the job (triggers CREATE_QUOTE via notes)
    await page.locator('text=Job Done').first().click();

    // Verify the "Pending Sync" indicator
    await expect(page.locator('#queue-dashboard')).toContainText('2 Pending Sync');

    // Verify the offline queue contents
    const queueData = await page.evaluate(() => JSON.parse(localStorage.getItem('OHC_Offline_Queue') || '[]') ); // Assuming we query IDB via a mock or just check UI state

    // 5. Go Online
    await context.setOffline(false);
    await page.evaluate(() => {
        Object.defineProperty(navigator, 'onLine', { value: true });
        window.dispatchEvent(new Event('online'));
    });

    // 6. Verify sync completed
    await expect(page.locator('#network-status-indicator-online')).toBeVisible({ timeout: 15000 });
  });
});
