import { test, expect } from '@playwright/test';

test.describe('Field Ops Offline Job Sync Flow', () => {
  test('toggle offline, complete job, toggle online, verify sync', async ({ page, context }) => {
    // Navigate to field ops jobs page
    await page.goto('/field-ops/jobs');

    // Make sure we have mock data seeded or loaded
    // Since we're doing e2e, wait for jobs to load
    await page.waitForSelector('.bg-white.rounded-xl.shadow-sm.border');

    // Simulate going offline using Playwright's network condition mock
    await context.setOffline(true);

    // Check we are offline via the UI top banner
    await expect(page.getByText('Offline Mode')).toBeVisible();

    // Click on the first "Start Work" or "Heading to Job" to transition to "In-Progress"
    const headingToJobButton = page.getByText('Heading to Job').first();
    if (await headingToJobButton.isVisible()) {
      await headingToJobButton.click();
    }
    const startWorkButton = page.getByText('Start Work').first();
    if (await startWorkButton.isVisible()) {
       await startWorkButton.click();
    }

    // Now it should be In-Progress, so we can complete it
    const markCompleteButton = page.getByText('Mark Complete').first();
    await expect(markCompleteButton).toBeVisible();

    // Let's add a note before completing
    const notesTextarea = page.locator('textarea[placeholder*="parts used"]').first();
    await notesTextarea.fill('Replaced the flux capacitor. All good now.');

    // Complete the job
    await markCompleteButton.click();

    // Wait for optimistic UI update (it should say COMPLETED on the pill)
    await expect(page.getByText('COMPLETED').first()).toBeVisible();
    await expect(page.getByText('Saved Notes:').first()).toBeVisible();

    // Now go back online
    await context.setOffline(false);

    // Wait for the sync to occur
    // SyncManager automatically retries when online. We can wait a bit or listen for the fetch call
    const response = await page.waitForResponse(response =>
      response.url().includes('/api/v1/sync/events') && response.request().method() === 'POST'
    );
    expect(response.status()).toBe(200);

    const data = await response.json();
    expect(data.success).toBe(true);
    expect(data.applied_count).toBeGreaterThan(0);
  });
});
