import { test, expect } from '../fixtures';

test.describe('Proactive Suggestions UI', () => {
  test('should display proactive suggestions in the unified agent feed when there are pending orders or unconfirmed bookings', async ({ page, request }) => {
    // 1. Seed unconfirmed booking logic in DB via API so the worker picks it up
    // In our test environment, we assume the backend worker is running
    // We create a booking to ensure the analysis catches it
    await request.post('/api/calendar/events', {
      data: {
        title: 'New Service Booking',
        description: 'Needs confirmation',
        start_time: new Date().toISOString(),
        end_time: new Date(Date.now() + 3600000).toISOString(),
        status: 'unconfirmed',
      }
    });

    // We also make sure the agent feed items have enough time to populate.
    // Navigate to the Dashboard
    await page.goto('/dashboard');

    // We expect the Triage Feed container to exist
    await expect(page.locator('text="Unified Agent Feed"').first()).toBeVisible();

    // Switch to proposals tab to ensure we are seeing agent feed items
    await page.getByRole('button', { name: /Proposals/i }).click();

    // Give time for SSE updates or polling in the feed
    await page.waitForTimeout(2000);

    // Verify the proactive analysis card renders based on the unconfirmed bookings seeded
    await expect(page.getByTestId('proactive-analysis-card')).toBeVisible();
    await expect(page.getByText('Proactive Operations Analysis')).toBeVisible();

    // Verify actions exist
    await expect(page.getByTestId('approve-proactive')).toBeVisible();
    await expect(page.getByTestId('dismiss-proactive')).toBeVisible();

    // Dismiss the suggestion
    await page.getByTestId('dismiss-proactive').click();

    // Wait for the card to disappear
    await expect(page.getByTestId('proactive-analysis-card')).not.toBeVisible();
  });
});
