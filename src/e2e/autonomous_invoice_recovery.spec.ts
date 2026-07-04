import { test, expect } from './fixtures';
import { randomUUID } from 'crypto';

test.describe('Autonomous Billing & Invoice Recovery Agent E2E', () => {
  test('Agent detects overdue invoice and proposes a reminder', async ({ page, adminUser, request, loginAs }) => {
    // 1. Nora (Agency Principal) logs in.
    await loginAs(page, adminUser);

    // Setup: Inject an overdue invoice into the db directly through the backend harness test endpoints or wait
    // We'll create a feed item representing the invoice overdue since the background job is hard to test deterministically in E2E
    // without waiting 24 hours. The backend provides an endpoint to simulate receiving an event or creating a feed item directly.

    // As per the system prompt, we simulate an overdue invoice
    await page.goto('/feed');
    await expect(page.getByRole('heading', { name: 'Agent Feed' })).toBeVisible({ timeout: 15000 });

    const tenantId = adminUser.tenantId;

    // Simulate the Finance agent creating the feed item
    const feedItemPayload = {
      event_source: "finance",
      context_payload: {
        feature_type: "invoice_followup",
        invoice_id: "inv_12345",
        original_message: "Invoice inv_12345 is overdue.",
        generated_response: "Hi there, just checking in to see if you received invoice inv_12345. Let us know if you have any questions!",
        operational_action: "Draft personalized reminder",
        customer_id: "cust_12345"
      },
      proposed_action: {
        feature_type: "invoice_followup",
        invoice_id: "inv_12345",
        original_message: "Invoice inv_12345 is overdue.",
        generated_response: "Hi there, just checking in to see if you received invoice inv_12345. Let us know if you have any questions!",
        operational_action: "Draft personalized reminder",
        customer_id: "cust_12345"
      }
    };

    const feedRes = await request.post('/api/feed', {
      data: feedItemPayload
    });

    expect(feedRes.ok()).toBeTruthy();

    await page.reload();

    // Verify the Action Card appears in the feed
    await expect(page.locator('text=Invoice inv_12345 is overdue.')).toBeVisible({ timeout: 15000 });

    // Action buttons
    const approveBtn = page.getByRole('button', { name: 'Approve' }).first();
    await expect(approveBtn).toBeVisible();

    // The owner taps Approve
    await approveBtn.click();

    // The feed item should change state
    await expect(page.locator('text=Approved')).toBeVisible({ timeout: 10000 });

  });
});
