import { test, expect } from '@playwright/test';

test.describe('Calendly Integration E2E', () => {
  const tenantId = 'e2e-calendly-tenant-' + Date.now();

  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    if (await page.locator('input[type="email"]').isVisible()) {
        await page.fill('input[type="email"]', 'owner@example.com');
        await page.fill('input[type="password"]', 'password123');
        await page.click('button:has-text("Sign in"), button:has-text("Log in")');
    }
  });

  test('Owner sees new Calendly booking in Unified Feed', async ({ page, request }) => {
    const webhookPayload = {
      event: 'invitee.created',
      payload: {
        tracking: { tenant_id: tenantId },
        event: {
          start_time: new Date(Date.now() + 86400000).toISOString(),
          end_time: new Date(Date.now() + 90000000).toISOString()
        },
        invitee: { email: 'client@example.com' }
      }
    };

    const response = await request.post('/api/v1/webhooks/calendly', {
      data: webhookPayload,
    });
    expect(response.status()).toBe(200);

    await page.route('/api/v1/agent-feed', async (route) => {
        const json = {
            items: [
                {
                    workItem: {
                        id: "mock-calendly-item-1",
                        tenant_id: tenantId,
                        source: "calendly_booking",
                        status: "open",
                        payload: {
                            title: "Calendly Booking: client@example.com",
                            description: "A new appointment has been scheduled for tomorrow."
                        }
                    },
                    lifecycle_state: "PENDING_APPROVAL",
                    created_at: new Date().toISOString(),
                    updated_at: new Date().toISOString()
                }
            ]
        };
        await route.fulfill({ json });
    });

    await page.goto('/unified-feed');

    await page.waitForSelector('text=Calendly Booking: client@example.com', { timeout: 10000 });

    const card = page.locator('text=Calendly Booking: client@example.com');
    await card.click();

    await expect(page.locator('text=Booking Details')).toBeVisible();

    const draftBtn = page.getByTestId('calendly-draft-follow-up');
    const rescheduleBtn = page.getByTestId('calendly-reschedule');
    const cancelBtn = page.getByTestId('calendly-cancel');

    await expect(draftBtn).toBeVisible();
    await expect(rescheduleBtn).toBeVisible();
    await expect(cancelBtn).toBeVisible();

    await page.locator('button:has(svg)').click();
    await expect(page.locator('text=Booking Details')).not.toBeVisible();
  });
});
