import { expect, test } from "./fixtures";

test.describe("Promoter Agent Proactive Flow", () => {
  // Use a strictly 375px wide viewport as specified by the issue
  test.use({ viewport: { width: 375, height: 667 } });

  test("Product creation triggers Promoter card in Agent Feed and allows approval", async ({
    page,
    request,
  }) => {
    const productName = `Limited Edition Mug ${Math.floor(Math.random() * 1000)}`;

    // 1. Login and go to dashboard
    await page.goto("/dashboard");
    await expect(page.locator('section[aria-label="Unified Agent Feed"]')).toBeVisible();

    // 2. Create a product via the internal API (simulating ProductCreated event)
    // In a real flow, this could be done via UI, but for speed we trigger the event logic.
    // Based on src/server/api/catalog.rs:handle_create_product
    const createProductRes = await request.post("/api/v1/catalog/product", {
      data: {
        name: productName,
        description: "A beautiful handcrafted mug for your morning coffee.",
        item_type: "physical"
      },
      headers: {
        "x-tenant-id": "e2e-tenant",
        "x-user-id": "default",
      }
    });
    expect(createProductRes.ok()).toBeTruthy();

    // 3. Refresh dashboard to see the new card
    // The MarketingAgent handles the event and calls execute_action which inserts into agent_approvals
    await page.reload();

    // 4. Look for the Promoter card in the Unified Agent Feed
    const promoterCard = page.locator(`text=New product detected! Schedule a post to drive sales?`).first();
    await expect(promoterCard).toBeVisible({ timeout: 15000 });

    // 5. Verify the captions variants are present
    await expect(page.locator('text=TikTok')).toBeVisible();
    await expect(page.locator('text=Instagram')).toBeVisible();
    await expect(page.locator('text=Facebook')).toBeVisible();

    // 6. Approve & Schedule
    const approveBtn = page.locator('button[data-testid="approve-promoter"]').first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 7. Verify the card disappears (optimistic UI)
    await expect(promoterCard).not.toBeVisible();

    // 8. Switch to Activity Feed and verify it appears there
    await page.click('button:has-text("Activity Feed")');
    await expect(page.locator(`text=New product detected! Schedule a post to drive sales?`)).toBeVisible();
    await expect(page.locator('text=APPROVED')).toBeVisible();
  });
});
