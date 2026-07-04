import { test, expect } from '@playwright/test';

test.describe('Fulfillment Orchestrator Agent Feed', () => {
  test('Owner can approve Master Proposal in one tap', async ({ page }) => {
    // 1. Intercept the feed API
    await page.route('**/api/agent-feed?*', async (route) => {
      const json = {
        items: [
          {
            id: 'mock-fulfillment-id',
            event_source: 'triage.inquiry',
            lifecycle_state: 'PENDING_APPROVAL',
            proposed_action: {
              feature_type: 'fulfillment_draft',
              service: 'Custom Vegan Cake',
              start_time: new Date().toISOString(),
              end_time: new Date().toISOString(),
              price: 55.0,
              is_surge: true,
              inbox_message_id: 'mock-msg'
            },
            context_payload: {
              customer_id: 'cust-123'
            },
            created_at: new Date().toISOString()
          }
        ]
      };
      await route.fulfill({ json });
    });

    await page.route('**/api/agents/approvals/mock-fulfillment-id/approve', async (route) => {
      await route.fulfill({ json: { success: true } });
    });

    await page.route('**/api/agent-feed/ws', async (route) => {
      await route.fulfill({ status: 101 }); // mock websocket
    });

    // 2. Load feed page
    await page.goto('/feed');

    // 3. Verify card exists
    await expect(page.locator('text=Fulfillment Draft: Custom Vegan Cake')).toBeVisible();
    await expect(page.locator('text=Spot reserved in calendar')).toBeVisible();
    await expect(page.locator('text=Surge pricing applied')).toBeVisible();

    // 4. Click Approve & Send
    await page.locator('button:has-text("Approve & Send")').click();

    // 5. Verify card disappears (optimistic UI update)
    await expect(page.locator('text=Fulfillment Draft: Custom Vegan Cake')).not.toBeVisible();
  });
});
