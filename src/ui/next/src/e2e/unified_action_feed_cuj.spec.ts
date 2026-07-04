import { expect, test } from './fixtures';

test.describe('Unified Action Feed UI Flow', () => {
  // Use a strictly 375px wide viewport as specified by the issue
  test.use({ viewport: { width: 375, height: 667 } });

  test.beforeEach(async ({ page, request }) => {
    // Set up the database to seed some AgentFeedItem records
    try {
      await request.post("/api/e2e/setup", {
        data: {
          query: `
            DELETE FROM agent_feed_items WHERE id IN ('e2e-action-feed-test-1', 'e2e-action-feed-test-2');

            INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
            VALUES
              ('e2e-action-feed-test-1', 'e2e-tenant', 'instagram_dm', '{"customer_message": "Can I get a custom cake next Tuesday?", "feature_type": "instagram_dm", "summary": "Can I get a custom cake next Tuesday?"}'::jsonb, '{"message": "Yes we do! Here is a booking link: https://ohc.page/book", "title": "Draft Reply"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
              ('e2e-action-feed-test-2', 'e2e-tenant', 'email', '{"customer_message": "Do you offer vegan options?", "feature_type": "email", "summary": "Do you offer vegan options?"}'::jsonb, '{"message": "Absolutely! We have a full vegan menu.", "title": "Draft Reply"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            ON CONFLICT (id) DO UPDATE SET lifecycle_state = 'PENDING_APPROVAL', updated_at = CURRENT_TIMESTAMP;
          `
        }
      });
    } catch(e) {}
  });

  test("Should render the unified action feed and allow tapping to approve", async ({ page, adminUser, loginAs }) => {
    // Navigate to the specific unified-feed page we implemented
    await loginAs(page, adminUser);

    await page.goto("/unified-feed");
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Verify it renders the WorkItemCard based on the test id
    const card1 = page.locator('[data-testid="triage-card-e2e-action-feed-test-1"]').first();
    const card2 = page.locator('[data-testid="triage-card-e2e-action-feed-test-2"]').first();

    await expect(card1).toBeVisible({ timeout: 15000 });
    await expect(card2).toBeVisible();

    // Verify the context text
    await expect(card1).toContainText("Can I get a custom cake next Tuesday?");

    // Verify the agent draft view text
    await expect(card1).toContainText("Yes we do! Here is a booking link");

    // Click approve on the first card
    const approveBtn = card1.getByTestId("feed-approve-btn");
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Optimistic UI updates - card1 should disappear
    await expect(card1).not.toBeVisible();

    // card2 should still be visible
    await expect(card2).toBeVisible();

    // Click dismiss on the second card
    const dismissBtn = card2.getByTestId("feed-dismiss-btn");
    await expect(dismissBtn).toBeVisible();
    await dismissBtn.click();

    // Optimistic UI updates - card2 should disappear
    await expect(card2).not.toBeVisible();

    // Empty state should appear
    await expect(page.locator('text=All caught up!')).toBeVisible();
  });
});
