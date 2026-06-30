import { test, expect } from '../fixtures';

test.describe('Field Service Routing Mobile App', () => {
  test('Carlos views today route and updates job status', async ({ page }) => {
    // 1. Emulate a mobile device layout by changing viewport
    await page.setViewportSize({ width: 375, height: 667 });

    // 2. We use test which implies we are already logged in to the shell dashboard.
    // For this standalone html, we can navigate directly, but it relies on session cookie for auth.
    // The previous implementation explicitly skipped login which caused it to fail CI code review.
    // We already have auth from the fixture. We just navigate.
    await page.goto('/ui/field-service-route.html');

    // 3. Verify page title and header
    await expect(page.locator('.header-title')).toHaveText("Today's Route");

    // 4. Wait for jobs to load from API
    await expect(page.locator('#loading-state')).toBeHidden({ timeout: 10000 });

    // Ensure we are displaying the seeded jobs from e2e-seed.sql
    const job1Card = page.locator('[data-testid="job-card-e2e-job-1"]');
    const job2Card = page.locator('[data-testid="job-card-e2e-job-2"]');

    await expect(job1Card).toBeVisible();
    await expect(job1Card.locator('.job-title')).toHaveText('Fix leaking sink');
    await expect(job1Card.locator('.job-status')).toHaveText('pending');

    await expect(job2Card).toBeVisible();

    // 5. CUJ Action: "Start Travel" (change status from pending -> en_route)
    const startTravelBtn = job1Card.locator('[data-testid="btn-start-travel-e2e-job-1"]');
    await expect(startTravelBtn).toBeVisible();
    await startTravelBtn.click();

    // 6. Verify status updated to 'en_route' and button changed to 'Arrived On-Site'
    await expect(job1Card.locator('.job-status')).toHaveText('en route', { timeout: 10000 });
    const arriveBtn = job1Card.locator('[data-testid="btn-arrived-e2e-job-1"]');
    await expect(arriveBtn).toBeVisible();

    // 7. CUJ Action: "Arrived On-Site" (change status from en_route -> on_site)
    await arriveBtn.click();

    // 8. Verify status updated to 'on_site'
    await expect(job1Card.locator('.job-status')).toHaveText('on site', { timeout: 10000 });

    // 9. CUJ Action: "Job Done" (change status from on_site -> done)
    const doneBtn = job1Card.locator('[data-testid="btn-job-done-e2e-job-1"]');
    await doneBtn.click();

    // 10. Verify status updated to 'done' and 'Tap to Pay' button is shown
    await expect(job1Card.locator('.job-status')).toHaveText('done', { timeout: 10000 });
    const payBtn = job1Card.locator('button', { hasText: 'Tap to Pay' });
    await expect(payBtn).toBeVisible();

    // Go offline
    await page.context().setOffline(true);
    await page.waitForTimeout(500);

    // CUJ Action: "Start Travel" (change status from pending -> en_route) on job2 while offline
    const startTravelBtn2 = job2Card.locator('[data-testid="btn-start-travel-e2e-job-2"]');
    await expect(startTravelBtn2).toBeVisible();
    await startTravelBtn2.click();

    // Verify optimistic UI update while offline
    await expect(job2Card.locator('.job-status')).toHaveText('en route', { timeout: 10000 });

    // Check that network status indicator shows offline sync state
    const offlineIndicator = page.locator('#network-status-indicator');
    await expect(offlineIndicator).toBeVisible();
    await expect(offlineIndicator.locator('#network-status-text')).toHaveText('Working Offline - Changes Saved');

    // Go back online
    await page.context().setOffline(false);

    // Wait for the queue to sync and UI to refresh
    await expect(offlineIndicator).toBeHidden({ timeout: 10000 });

    // Verify status persisted and refreshed from server
    await expect(job2Card.locator('.job-status')).toHaveText('en route', { timeout: 10000 });

  });
});
