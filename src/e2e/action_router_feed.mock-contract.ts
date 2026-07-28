import { expect, test } from "./fixtures";

test.describe("Agent Feed - Dynamic Action Router Protocol", () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test.beforeEach(async ({ page, request }) => {
    // 1. Seed the database with a test inbox message and a feed item that proposes to reply to it
    await request.post("/api/v1/chaos/sql", {
      data: {
        query: `
          DELETE FROM agent_feed_items WHERE id = 'e2e-action-router-feed-test';
          DELETE FROM chat_messages WHERE id = 'e2e-action-router-inbox-test';

          INSERT INTO chat_messages (id, tenant_id, source, customer_id, text, status, created_at, updated_at)
          VALUES ('e2e-action-router-inbox-test', 'e2e-tenant', 'instagram', 'test-cust', 'How much is the blue cake?', 'unread', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
          ON CONFLICT DO NOTHING;

          INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, content_preview, state, created_at, updated_at)
          VALUES (
            'e2e-action-router-feed-test',
            'e2e-tenant',
            'instagram_dm',
            '{"customer_message": "How much is the blue cake?", "feature_type": "instagram_dm", "inbox_message_id": "e2e-action-router-inbox-test", "draft_reply": "The blue cake is $50. Let me know if you want to order!"}'::jsonb,
            '{"description": "How much is the blue cake?", "title": "Instagram DM from Customer"}'::jsonb,
            'PENDING_APPROVAL',
            CURRENT_TIMESTAMP,
            CURRENT_TIMESTAMP
          )
          ON CONFLICT DO NOTHING;
        `,
      },
    });

    await page.goto("/login");
    await page.fill('input[type="email"]', "e2e@example.com");
    await page.fill('input[type="password"]', "password");
    await page.click('button[type="submit"]');
    await page.waitForURL("/dashboard");

    // Disable smooth scrolling to prevent flaky tests
    await page.evaluate(() => {
        document.documentElement.style.scrollBehavior = 'auto';
    });
  });

  test("Approving a feed item triggers the dynamic action router and updates the target table", async ({ page, request }) => {
    // 1. Verify feed item is visible
    const feedItem = page.locator('div[data-testid^="feed-card-"]').filter({ hasText: 'How much is the blue cake?' });
    await expect(feedItem).toBeVisible();

    // 2. Tap "Approve & Send"
    const approveBtn = feedItem.locator('button[data-testid="feed-approve-btn"]');
    await approveBtn.click();

    // 3. Verify it disappears from the feed
    await expect(feedItem).toHaveCount(0);

    // 4. Verify the database was actually updated by the new Action Router
    const verifyResp = await request.post("/api/v1/chaos/sql", {
      data: {
        query: `
          SELECT status, draft_reply
          FROM chat_messages
          WHERE id = 'e2e-action-router-inbox-test' AND tenant_id = 'e2e-tenant';
        `,
      },
    });
    const result = await verifyResp.json();

    // The status should now be 'sent' and draft_reply populated, confirming the Action Router ran
    expect(result.data[0].status).toBe('sent');
    expect(result.data[0].draft_reply).toBe('The blue cake is $50. Let me know if you want to order!');
  });
});
