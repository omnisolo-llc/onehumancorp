import { test, expect } from '@playwright/test';
import { setupAuthAndDatabase } from '../../../../e2e/test_setup';

test.describe('Agentic Subscription Retention CUJ', () => {
  let tenantId: string;

  test.beforeAll(async ({ request }) => {
    tenantId = await setupAuthAndDatabase(request);

    // Mock a subscription with low health score (no recent activity)
    const seedRes = await request.post('/api/test-db/seed-query', {
      headers: {
        'x-tenant-id': tenantId,
      },
      data: {
        query: `
          -- Add a test customer
          INSERT INTO customers (id, tenant_id, name, email)
          VALUES ('test-cust-churn', '${tenantId}', 'Alex Student', 'alex@example.com')
          ON CONFLICT DO NOTHING;

          -- Add a product and plan
          INSERT INTO products (id, tenant_id, name, type)
          VALUES ('test-prod-churn', '${tenantId}', 'Music Lessons', 'service')
          ON CONFLICT DO NOTHING;

          INSERT INTO subscription_plans (id, tenant_id, product_id, interval, interval_count, status)
          VALUES ('test-plan-churn', '${tenantId}', 'test-prod-churn', 'month', 1, 'active')
          ON CONFLICT DO NOTHING;

          -- Add an active subscription approaching renewal (next 7 days) and no recent activity (> 30 days old)
          INSERT INTO subscriptions (id, tenant_id, customer_id, plan_id, status, current_period_end, cancel_at_period_end, created_at)
          VALUES ('test-sub-churn', '${tenantId}', 'test-cust-churn', 'test-plan-churn', 'active', NOW() + INTERVAL '3 days', FALSE, NOW() - INTERVAL '35 days')
          ON CONFLICT DO NOTHING;

          -- Ensure there are no recent appointments for this customer
          DELETE FROM appointments WHERE customer_id = 'test-cust-churn';

          -- Add a mock Agent Feed Item to simulate what the job/orchestrator produces
          INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state)
          VALUES (
            'test-feed-churn',
            '${tenantId}',
            'CustomerSuccess',
            '{"feature_type":"subscription_churn_risk", "reason":"No recent activity in 30 days and approaching renewal", "customer_id":"test-cust-churn"}',
            '{"feature_type":"subscription_churn_risk", "generated_response":"Hi Alex! We noticed you haven''t booked a session lately. Want to schedule a free 15-minute catch-up to keep the momentum going?", "action_type":"DraftForReview"}',
            'PENDING_APPROVAL'
          )
          ON CONFLICT DO NOTHING;
        `,
      },
    });
    expect(seedRes.ok()).toBeTruthy();
  });

  test('Owner reviews and approves a win-back offer for an at-risk subscription', async ({ page }) => {
    // Navigate to the Dashboard / Unified Agent Feed
    await page.goto(`/dashboard?tenant=${tenantId}`);

    // Check if we are on the Command Center tab
    const commandCenterTab = page.getByRole('button', { name: /Proposals/i });
    if (await commandCenterTab.isVisible()) {
      await commandCenterTab.click();
    }

    // Wait for the feed to load
    await page.waitForTimeout(2000); // Give time for data to fetch

    // Look for the specific churn risk warning text
    await expect(page.locator('text=⚠️ High Churn Risk')).toBeVisible({ timeout: 10000 });

    // Verify the context and drafted message appear
    await expect(page.locator('text=Health score dropped due to inactivity.')).toBeVisible();
    await expect(page.locator('text="Hi Alex! We noticed you haven\'t booked a session lately. Want to schedule a free 15-minute catch-up to keep the momentum going?"')).toBeVisible();

    // Verify the buttons are visible
    const approveBtn = page.getByTestId('action-card-approve-test-feed-churn');
    const dismissBtn = page.getByTestId('action-card-dismiss-test-feed-churn');

    await expect(approveBtn).toBeVisible();
    await expect(dismissBtn).toBeVisible();

    // Approve the action
    await approveBtn.click();

    // Verify it disappears from the feed
    await expect(page.locator('text=⚠️ High Churn Risk')).toBeHidden({ timeout: 10000 });
  });
});
