import { expect, test } from "./fixtures";

test.describe("Unified Agent Feed Mobile UX", () => {
  // Use a strictly 375px wide viewport as specified by the issue
  test.use({ viewport: { width: 375, height: 667 } });

  test("Renders and actions can be tapped on 375px mobile screen", async ({
    page,
    request,
  }) => {
    // 1. Seed some distinct approvals representing different departments,
    // including the Instagram DM custom cake scenario from the new agent_feed_items table.
    await request.post("/api/e2e/setup", {
      data: {
        query: `
          INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at)
          VALUES
            ('e2e-feed-test-1', 'e2e-tenant', 'operations', '3 new orders to fulfill', 'DRAFT', 'LOW', '{"feature_type": "fulfillment_batch", "message": "3 new orders to fulfill"}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            ('e2e-feed-test-2', 'e2e-tenant', 'marketing', 'Draft promo email?', 'DRAFT', 'LOW', '{"context": {"weekly_health_report": true}, "message": "Draft promo email?"}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
          ON CONFLICT (id) DO UPDATE SET status = 'DRAFT', updated_at = CURRENT_TIMESTAMP;

          INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
          VALUES
            ('e2e-feed-test-3', 'e2e-tenant', 'instagram_dm', '{"customer_message": "Do you make custom vegan cakes?", "feature_type": "instagram_dm", "draft_reply": "Yes we do! Here is a booking link: https://ohc.page/book"}'::jsonb, null, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
          ON CONFLICT (id) DO UPDATE SET lifecycle_state = 'PENDING_APPROVAL', updated_at = CURRENT_TIMESTAMP;
        `,
      },
    });

    // 2. Load the dashboard on mobile
    await page.goto("/dashboard");

    // 3. Ensure the unified feed tab is visible
    await expect(page.locator("text=Activity Feed").first()).toBeVisible({ timeout: 15000 });

    // 4. Verify the seeded cards are rendered
    const opsCard = page.locator("text=3 new orders to fulfill").first();
    const marketingCard = page.locator("text=Draft promo email?").first();
    const igCard = page.locator("text=Do you make custom vegan cakes?").first();

    await expect(opsCard).toBeVisible();
    await expect(marketingCard).toBeVisible();
    await expect(igCard).toBeVisible();

    // Verify Instagram DM specific UI elements
    await expect(page.locator("text=Instagram DM").first()).toBeVisible();
    await expect(page.locator("text=Yes we do! Here is a booking link").first()).toBeVisible();

    // 5. Verify touch targets on the Instagram DM specific button
    const approveIgButton = page.locator('button[data-testid="approve-instagram-dm"]').first();
    await expect(approveIgButton).toBeVisible();
    await approveIgButton.click();

    // The Instagram DM item should optimistically disappear
    await expect(page.locator("text=Do you make custom vegan cakes?").first()).not.toBeVisible();

    // Verify touch targets on the default Approve button
    const approveButton = page.locator('button[data-testid="approve-proposal"]').first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    await expect(page.locator("text=3 new orders to fulfill").first()).not.toBeVisible();

    // Test a specific payload button
    const draftButton = page.locator('button[data-testid="approve-draft"]').first();
    await expect(draftButton).toBeVisible();
    await draftButton.click();

    await expect(page.locator("text=Draft promo email?").first()).not.toBeVisible();
  });
});
