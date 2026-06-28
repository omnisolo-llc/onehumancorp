import { test, expect } from './fixtures';

test.describe('Offline Field Service Sync', () => {
  test('should optimistically update job status and sync when back online', async ({ page, context }) => {
    // Navigate to the field service route page
    await page.goto('/field-service-route.html');

    // Wait for jobs to load
    await expect(page.getByTestId('job-card-e2e-job-1')).toBeVisible({ timeout: 10000 });

    // Disconnect network
    await context.setOffline(true);

    // Verify offline indicator appears (give it a moment for offline event)
    await expect(page.locator('#network-status-indicator')).toBeVisible();
    await expect(page.locator('#network-status-text')).toHaveText(/Offline/);

    // Click "Start Travel" which updates status to 'en_route'
    const startTravelBtn = page.getByTestId('job-card-e2e-job-1').getByRole('button', { name: /Start Travel/i });
    await startTravelBtn.click();

    // Optimistic UI update: should now show 'en route' or the Arrived On-Site button
    await expect(page.getByTestId('job-card-e2e-job-1').getByRole('button', { name: /Arrived On-Site/i })).toBeVisible();

    // Reconnect network
    const [response] = await Promise.all([
      page.waitForResponse(res => res.url().includes('/api/v1/field-service-routing/jobs/e2e-job-1/status') && res.request().method() === 'POST'),
      context.setOffline(false)
    ]);

    expect(response.status()).toBe(200);

    // Verify it synced successfully
    await expect(page.locator('#network-status-indicator')).toBeHidden();

    // The data should remain 'en_route' after the real fetch overrides optimistic update
    await expect(page.getByTestId('job-card-e2e-job-1').getByRole('button', { name: /Arrived On-Site/i })).toBeVisible();
  });
});
