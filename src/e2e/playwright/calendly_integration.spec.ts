import { test, expect } from '../fixtures';

test.describe('Autonomous AI Work Triage - Calendly Integration', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('receives calendly webhook and displays booking in daily work feed', async ({ page, request, loginAs, adminUser }) => {
    // 1. Log in to the application
    await loginAs(page, adminUser);

    // 2. Inject raw signal to simulate Calendly Webhook processing
    const tenantId = 'default';
    const payload = {
      event: "invitee.created",
      payload: {
        invitee: {
          email: "e2e_test@example.com",
          name: "E2E Test User",
          start_time: "2026-01-01T10:00:00Z",
          end_time: "2026-01-01T11:00:00Z"
        }
      }
    };

    const res = await request.post(`/api/v1/webhooks/calendly?tenant_id=${tenantId}`, {
      data: payload
    });
    expect(res.ok()).toBeTruthy();

    // 3. Navigate to Daily Work Feed at mobile resolution
    await page.goto('/dashboard/daily-work');

    // Wait for the feed to load
    await expect(page.locator('text=Loading your work feed...')).not.toBeVisible({ timeout: 10000 });

    // 4. Verify the surfaced actionable card for Calendly booking
    // It should have the intent 'new_booking' and show the customer's name
    const newBookingText = page.locator('text=new_booking');
    await expect(newBookingText.first()).toBeVisible({ timeout: 10000 });

    const customerNameText = page.locator('text=E2E Test User');
    await expect(customerNameText.first()).toBeVisible();

    // Check touch target for mobile (e.g. approve button)
    const approveButton = page.locator('button:has-text("Approve")').first();
    await expect(approveButton).toBeVisible();

    const boundingBox = await approveButton.boundingBox();
    expect(boundingBox?.width).toBeGreaterThanOrEqual(44);
    expect(boundingBox?.height).toBeGreaterThanOrEqual(44);
  });
});
