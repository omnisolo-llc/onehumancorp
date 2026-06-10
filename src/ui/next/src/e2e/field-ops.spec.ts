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

    const completeBtn = page.locator('button:has-text("Complete Job")').first();
    await completeBtn.click();

    // Verify UI updates locally
    await expect(page.locator('text=Saved Notes:')).toBeVisible();
    await expect(page.locator('text=\"Found a leak under the sink, requires immediate pipe replacement quote.\"')).toBeVisible();
    await expect(page.locator('text=Sales Agent will draft an estimate based on these notes once online.')).toBeVisible();

    // Simulate going back online
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Sync is handled by SyncManager in background - we're verifying the offline UX flow
    await expect(page.locator('text=Offline Mode')).not.toBeVisible();
  });
});
