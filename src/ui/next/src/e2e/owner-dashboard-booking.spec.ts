import { test, expect } from '../../../../e2e/fixtures';

test.describe('Owner Dashboard Bookings', () => {
  test('Owner can navigate to bookings management view and see AI suggestions context', async ({ page }) => {
    await page.goto('/dashboard/bookings');
    await expect(page.getByTestId('owner-dashboard-bookings')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Bookings Management' })).toBeVisible();

    const feedLink = page.getByRole('link', { name: 'Go to Feed' });
    await expect(feedLink).toBeVisible();
    await feedLink.click();
    await expect(page).toHaveURL(/.*\/feed/);
  });

  test('Webhook from Calendly creates a task', async ({ page, request }) => {
    // Send mock Calendly webhook request
    const response = await request.post('/api/v1/webhooks/calendly?tenant_id=t1', {
      data: {
        event: "invitee.created",
        payload: { email: "test@calendly.com", name: "Test User", start_time: "2026-01-01T10:00:00Z", end_time: "2026-01-01T11:00:00Z" }
      }
    });
    expect(response.ok()).toBeTruthy();

    await page.goto('/feed');
    await expect(page.getByText('New Calendly Booking from Test User (test@calendly.com)')).toBeVisible({ timeout: 10000 });
  });
});
