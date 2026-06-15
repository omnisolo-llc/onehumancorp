import { expect, test } from "./fixtures";

test.describe("Unified Agent Feed Mobile UX", () => {
  // Use a strictly 375px wide viewport as specified by the issue
  test.use({ viewport: { width: 375, height: 667 } });

  test.beforeEach(async ({ request }) => {
    // Seed some distinct approvals representing different departments,
    // including the Instagram DM custom cake scenario from the new agent_feed_items table.
    await request.post("/api/e2e/setup", {
      data: {
        query: `
          DELETE FROM agent_approvals WHERE id IN ('e2e-feed-test-1', 'e2e-feed-test-2');
          DELETE FROM agent_feed_items WHERE id IN ('e2e-feed-test-3');

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
  });

  const performLogin = async (page: any) => {
    await page.goto("/login");
    await expect(page.getByRole("heading", { name: "Login" })).toBeVisible();
    await page.getByPlaceholder("Email or Username").first().fill("e2e-user");
    await page.locator('input[type="password"]').first().fill("password");
    await page.getByRole("button", { name: "Log In" }).click();
    await expect(page.locator("text=Activity Feed").first()).toBeVisible({ timeout: 15000 });
  };

  test("1. Renders seeded cards and handles optimistic offline actions", async ({ page, context }) => {
    await performLogin(page);

    const opsCard = page.locator("text=3 new orders to fulfill").first();
    await expect(opsCard).toBeVisible();

    // Go offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Tap Approve while offline
    const approveButton = page.locator('button[data-testid="approve-proposal"]').first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Should optimistically disappear
    await expect(opsCard).not.toBeVisible();

    // Come back online
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Feed should remain correct (card is still gone)
    await expect(opsCard).not.toBeVisible({ timeout: 5000 });
  });

  test("1b. Renders seeded cards and tab navigation on 375px mobile screen", async ({ page }) => {
    await performLogin(page);

    const opsCard = page.locator("text=3 new orders to fulfill").first();
    const marketingCard = page.locator("text=Draft promo email?").first();
    const igCard = page.locator("text=Do you make custom vegan cakes?").first();

    await expect(opsCard).toBeVisible();
    await expect(marketingCard).toBeVisible();
    await expect(igCard).toBeVisible();

    await expect(page.locator("text=Instagram DM").first()).toBeVisible();
    await expect(page.locator("text=Yes we do! Here is a booking link").first()).toBeVisible();
  });

  test("2. Tapping Approve & Send on Instagram DM dismisses the card", async ({ page }) => {
    await performLogin(page);

    const approveIgButton = page.locator('button[data-testid="approve-instagram-dm"]').first();
    await expect(approveIgButton).toBeVisible();
    await approveIgButton.click();

    await expect(page.locator("text=Do you make custom vegan cakes?").first()).not.toBeVisible();
  });

  test("3. Tapping Approve on default proposal dismisses the card", async ({ page }) => {
    await performLogin(page);

    const approveButton = page.locator('button[data-testid="approve-proposal"]').first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    await expect(page.locator("text=3 new orders to fulfill").first()).not.toBeVisible();
  });

  test("4. Tapping 'Yes, draft it!' on a draft proposal dismisses the card", async ({ page }) => {
    await performLogin(page);

    const draftButton = page.locator('button[data-testid="approve-draft"]').first();
    await expect(draftButton).toBeVisible();
    await draftButton.click();

    await expect(page.locator("text=Draft promo email?").first()).not.toBeVisible();
  });

  test("5. Tapping Dismiss on a draft proposal dismisses the card", async ({ page }) => {
    await performLogin(page);

    const dismissButton = page.locator('button[data-testid="dismiss-draft"]').first();
    await expect(dismissButton).toBeVisible();
    await dismissButton.click();

    await expect(page.locator("text=Draft promo email?").first()).not.toBeVisible();
  });
});
