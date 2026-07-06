import { expect, test } from "./fixtures";

test.describe("Unified Agent Feed Mobile UX", () => {
  // Use a strictly 375px wide viewport as specified by the issue
  test.use({ viewport: { width: 375, height: 667 } });

  test.beforeEach(async ({ page, request }) => {
    // Let's set up the database
    // Wait for the backend proxy? The setup endpoint in the mock test works.
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
            ('e2e-feed-test-3', 'e2e-tenant', 'instagram_dm', '{"customer_message": "Do you make custom vegan cakes?", "feature_type": "instagram_dm", "draft_reply": "Yes we do! Here is a booking link: https://ohc.page/book", "summary": "Do you make custom vegan cakes?"}'::jsonb, '{"description": "Do you make custom vegan cakes?", "title": "Do you make custom vegan cakes?"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
          ON CONFLICT (id) DO UPDATE SET lifecycle_state = 'PENDING_APPROVAL', updated_at = CURRENT_TIMESTAMP;
        `
      }
    });
  });

  const performLogin = async (page: any) => {
    await page.goto("/login");
    // Next.js login page:
    await page.getByPlaceholder("Email or Username").first().fill("e2e-user");
    await page.locator('input[type="password"]').first().fill("password123");
    await page.getByRole("button", { name: "Log In" }).click();
    await page.waitForTimeout(1000);
  };

  test("1b. Check specific seeded cards", async ({ page }) => {
    await performLogin(page);

    await page.goto("/dashboard");
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000); // give time for ws

    // The NextJS /feed UI
    const opsCard = page.locator("text=3 new orders to fulfill").first();
    const marketingCard = page.locator("text=Draft promo email?").first();
    const igCard = page.locator("text=Do you make custom vegan cakes?").first();

    await expect(opsCard).toBeVisible({ timeout: 15000 });
    await expect(marketingCard).toBeVisible();
    await expect(igCard).toBeVisible();
  });

  test("2. Tapping Approve & Send on Instagram DM dismisses the card", async ({ page }) => {
    await performLogin(page);

    await page.goto("/dashboard");
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const igCard = page.locator("text=Do you make custom vegan cakes?").first();
    await expect(igCard).toBeVisible({ timeout: 15000 });

    const parent = igCard.locator('..').locator('..');
    const reviewBtn = parent.locator('button[data-testid="approve-instagram-dm"]').first();
    await expect(reviewBtn).toBeVisible({ timeout: 15000 });
    await reviewBtn.click();

    const bottomSheet = page.getByTestId("bottom-sheet");
    await expect(bottomSheet).toBeVisible({ timeout: 15000 });

    const approveBtn = bottomSheet.locator('button[data-testid="feed-approve-btn"]').first();
    await expect(approveBtn).toBeVisible({ timeout: 15000 });
    await approveBtn.click();

    await expect(igCard).not.toBeVisible();
  });

  test("3. Tapping Approve on default proposal dismisses the card", async ({ page }) => {
    await performLogin(page);

    await page.goto("/dashboard");
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const opsCard = page.locator("text=3 new orders to fulfill").first();
    await expect(opsCard).toBeVisible({ timeout: 15000 });

    const parent = opsCard.locator('..').locator('..');
    const approveButton = parent.locator('button[data-testid="feed-approve-btn"]').first();
    await expect(approveButton).toBeVisible({ timeout: 15000 });
    await approveButton.click();

    await expect(opsCard).not.toBeVisible();
  });

  test("4. Tapping 'Yes, draft it!' on a draft proposal dismisses the card", async ({ page }) => {
    await performLogin(page);

    await page.goto("/dashboard");
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const marketingCard = page.locator("text=Draft promo email?").first();
    await expect(marketingCard).toBeVisible({ timeout: 15000 });

    const parent = marketingCard.locator('..').locator('..');
    const draftButton = parent.locator('button[data-testid="feed-approve-btn"]').first();
    await expect(draftButton).toBeVisible({ timeout: 15000 });
    await draftButton.click();

    const bottomSheet = page.getByTestId("bottom-sheet");
    await expect(bottomSheet).toBeVisible({ timeout: 15000 });

    const approveBtn = bottomSheet.locator('button[data-testid="feed-approve-btn"]').first();
    await expect(approveBtn).toBeVisible({ timeout: 15000 });
    await approveBtn.click();

    await expect(marketingCard).not.toBeVisible();
  });

  test("5. Tapping Dismiss on a draft proposal dismisses the card", async ({ page }) => {
    await performLogin(page);

    await page.goto("/dashboard");
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const marketingCard = page.locator("text=Draft promo email?").first();
    await expect(marketingCard).toBeVisible({ timeout: 15000 });

    const parent = marketingCard.locator('..').locator('..');
    const dismissButton = parent.locator('button[data-testid="feed-dismiss-btn"]').first();
    await expect(dismissButton).toBeVisible({ timeout: 15000 });
    await dismissButton.click();

    await expect(marketingCard).not.toBeVisible();
  });
});
