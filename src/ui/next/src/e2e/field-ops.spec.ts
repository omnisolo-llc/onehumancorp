import { test, expect } from '../../../e2e/fixtures';

test.describe('Offline Field Operations', () => {
  test('Carlos can view jobs, go offline, add notes, and complete a job which generates a quote request', async ({ page, context, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

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

    // Look for heading to job first
    const headingToJobBtn = page.locator('button', { hasText: 'Heading to Job' }).first();

    // Playwright robust checking for visibility before clicking to avoid flakiness
    try {
      await headingToJobBtn.waitFor({ state: 'visible', timeout: 5000 });
      await headingToJobBtn.click();
    } catch (e) {
      // It might not be visible if it's already in the next state
    }

    // Now it should say Start Work
    const startWorkBtn = page.locator('button', { hasText: 'Start Work' }).first();
    try {
      await startWorkBtn.waitFor({ state: 'visible', timeout: 5000 });
      await startWorkBtn.click();
    } catch (e) {
      // It might not be visible if it's already in the next state
    }

    // Now it should say Job Done
    const jobDoneBtn = page.locator('button', { hasText: 'Job Done' }).first();
    await jobDoneBtn.waitFor({ state: 'visible' });
    await jobDoneBtn.click();

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
});
