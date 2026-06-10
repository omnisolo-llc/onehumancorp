import { expect, test } from "./fixtures";

test.describe("Unified Agent Feed Mobile UX", () => {
  // Use a strictly 375px wide viewport as specified by the issue
  test.use({ viewport: { width: 375, height: 812 } });

  test("Renders and actions can be tapped on 375px mobile screen", async ({
    page,
    request,
  }) => {
    // 1. Seed some distinct approvals representing different departments
    await request.post("/api/e2e/setup", {
      data: {
        query: `
          INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at)
          VALUES
            ('e2e-feed-test-1', 'e2e-tenant', 'operations', '3 new orders to fulfill', 'DRAFT', 'LOW', '{"feature_type": "fulfillment_batch", "message": "Batch process 3 orders?"}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            ('e2e-feed-test-2', 'e2e-tenant', 'marketing', 'Draft promo email?', 'DRAFT', 'LOW', '{"context": {"weekly_health_report": true}, "message": "Send promo?"}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
          ON CONFLICT (id) DO UPDATE SET status = 'DRAFT', updated_at = CURRENT_TIMESTAMP;
        `,
      },
    });

    // 2. Load the dashboard on mobile
    await page.goto("/dashboard");

    // 3. Ensure the unified feed tab is visible
    const feedSection = page.locator(
      'section[aria-label="Unified Agent Feed"]',
    );
    await expect(feedSection).toBeVisible({ timeout: 15000 });

    // Ensure the feed is properly constrained to 375px or less visually (mobile constraints)
    const boundingBox = await feedSection.boundingBox();
    expect(boundingBox?.width).toBeLessThanOrEqual(375);

    // 4. Verify the seeded cards are rendered
    const opsCard = page.locator("text=3 new orders to fulfill").first();
    const marketingCard = page.locator("text=Draft promo email?").first();

    await expect(opsCard).toBeVisible();
    await expect(marketingCard).toBeVisible();

    // 5. Verify touch targets on the default Approve button (has min-h-[44px] class)
    // We verify the actual rendered bounds have a minimum 44x44 size
    const approveButton = page
      .locator('button[data-testid="approve-proposal"]')
      .first();
    await expect(approveButton).toBeVisible();

    const approveButtonBox = await approveButton.boundingBox();
    if (approveButtonBox) {
      expect(approveButtonBox.width).toBeGreaterThanOrEqual(44);
      expect(approveButtonBox.height).toBeGreaterThanOrEqual(44);
    }

    await approveButton.click();

    // The item should optimisticly disappear
    await expect(
      page.locator("text=3 new orders to fulfill").first(),
    ).not.toBeVisible();

    // Test a specific payload button (e.g., from weekly_health_report which shows "Yes, draft it!")
    const draftButton = page
      .locator('button[data-testid="approve-draft"]')
      .first();
    await expect(draftButton).toBeVisible();

    const draftButtonBox = await draftButton.boundingBox();
    if (draftButtonBox) {
      expect(draftButtonBox.width).toBeGreaterThanOrEqual(44);
      expect(draftButtonBox.height).toBeGreaterThanOrEqual(44);
    }

    await draftButton.click();

    await expect(
      page.locator("text=Draft promo email?").first(),
    ).not.toBeVisible();
  });
});
