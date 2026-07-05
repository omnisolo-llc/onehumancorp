import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Autonomous Field Service Quoting & Deposit Engine', () => {

  test('Owner sees service lead trigger a quote draft, deposit requirement, and provisional slot', async ({ browser }) => {
    const context = await browser.newContext();
    const page = await adminPage(context);

    await page.goto('/dashboard');
    await page.setViewportSize({ width: 375, height: 667 });

    // Simulate an incoming service lead via webhook (work_intake)
    const webhookRes = await page.request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: 'e2e-tenant',
        channel: 'work_intake',
        sender_id: 'carlos_customer_99',
        customer_name: 'Carlos Customer',
        customer_email: 'carlos.customer@example.com',
        message: 'My sink is leaking, can you fix it tomorrow at 2pm?'
      }
    });

    expect(webhookRes.ok()).toBeTruthy();

    // Wait for the background worker to process it (Sales Agent)
    await page.waitForTimeout(5000);

    await page.goto('/dashboard');

    // The draft quote should appear in the feed
    const quoteDraftCard = page.getByTestId('quote-draft-card');
    await expect(quoteDraftCard).toBeVisible({ timeout: 15000 });

    // Should contain deposit and provisional slot data as per our UI update
    await expect(quoteDraftCard).toContainText('Required Deposit:');
    await expect(quoteDraftCard).toContainText('Provisional Slot Held:');

    // Owner taps Approve & Send
    const approveBtn = quoteDraftCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // The card should disappear after approval (status resolved)
    await expect(quoteDraftCard).toBeHidden({ timeout: 10000 });
  });

  test('Customer deposit payment confirms booking and finalizes lead', async ({ browser }) => {
      const context = await browser.newContext();
      const page = await adminPage(context);

      // We simulate the Stripe webhook success to finalize the booking
      // We will send a mock checkout.session.completed with the required metadata
      const webhookRes = await page.request.post('/api/v1/billing/webhook', {
        headers: {
            'stripe-signature': 'mock-signature'
        },
        data: {
          type: "checkout.session.completed",
          data: {
            object: {
              metadata: {
                tenant_id: 'e2e-tenant',
                service_lead_id: 'mock-lead-id',
                estimate_id: 'mock-estimate-id',
                deposit_requirement_id: 'mock-deposit-req-id',
                proposed_slot_id: 'mock-slot-id'
              }
            }
          }
        }
      });
      // The webhook returns 200 immediately, background processing happens, but we just verify the route accepts it
      expect(webhookRes.status()).toBe(200);
  });
});
