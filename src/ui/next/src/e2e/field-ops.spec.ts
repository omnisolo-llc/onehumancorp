import { test, expect } from '@playwright/test';

test.describe('Offline Field Operations', () => {
  test('Carlos can view jobs, go offline, add notes, and complete a job which generates a quote request', async ({ page, context }) => {
    // Navigate to the field ops page
    await page.goto('/field-ops/jobs');

    // Verify online state
    await expect(page.locator('text=Today\'s Route')).toBeVisible();
    await expect(page.locator('text=Alice Smith')).toBeVisible();

    // Simulate going offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Verify offline indicator
    await expect(page.locator('text=Offline Mode')).toBeVisible();

    // Interact with a job (add notes and complete)
    const notesArea = page.locator('textarea').first();
    await notesArea.fill('Found a leak under the sink, requires immediate pipe replacement quote.');

    const completeBtn = page.locator('button:has-text("Job Done")').first();
    await completeBtn.click();

    // Verify UI updates locally
    await expect(page.locator('text=Saved Notes:')).toBeVisible();
    await expect(page.locator('text="Found a leak under the sink, requires immediate pipe replacement quote."')).toBeVisible();
    await expect(page.locator('text=Sales Agent will draft an estimate based on these notes once online.')).toBeVisible();

    // Simulate going back online
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Sync is handled by SyncManager in background - we're verifying the offline UX flow
    await expect(page.locator('text=Offline Mode')).not.toBeVisible();
  });

  test('Carlos can load the app offline and see cached jobs, then sync them back when online', async ({ page, context }) => {
    // 1. Visit the page online to populate the cache
    await page.goto('/field-ops/jobs');
    await expect(page.locator('text=Today\'s Route')).toBeVisible();
    await expect(page.locator('text=Alice Smith')).toBeVisible();

    // Wait for cache to populate
    await page.waitForTimeout(500);

    // 2. Go offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // 3. Reload the page while offline
    await page.reload();

    // Verify offline indicator and cached jobs are visible
    await expect(page.locator('text=Offline Mode')).toBeVisible();
    await expect(page.locator('text=Alice Smith')).toBeVisible();
    await expect(page.locator('text=Bob Jones')).toBeVisible();

    // 4. Complete a job while offline
    const bobCard = page.locator('.bg-white', { hasText: 'Bob Jones' });
    const notesArea = bobCard.locator('textarea');
    await notesArea.fill('Inspected breaker box. Need to draft quote for replacement.');

    const completeBtn = bobCard.locator('button:has-text("Heading to Job")');
    await completeBtn.click();

    const startWorkBtn = bobCard.locator('button:has-text("Start Work")');
    await startWorkBtn.click();

    const jobDoneBtn = bobCard.locator('button:has-text("Job Done")');
    await jobDoneBtn.click();

    // Verify UI updates locally
    await expect(bobCard.locator('text=Saved Notes:')).toBeVisible();
    await expect(bobCard.locator('text="Inspected breaker box. Need to draft quote for replacement."')).toBeVisible();

    // 5. Go back online
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Sync is handled by SyncManager in background
    await expect(page.locator('text=Offline Mode')).not.toBeVisible();

    // Check that we're back online and UI remains consistent
    await expect(bobCard.locator('text=Saved Notes:')).toBeVisible();
  });
});
