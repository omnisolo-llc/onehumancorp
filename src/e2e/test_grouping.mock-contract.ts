import { expect, test } from "./fixtures";

test.describe("Unified Agent Feed Grouping", () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test.beforeEach(async ({ page, request }) => {
    await request.post("/api/v1/e2e/setup", {
      data: {
        query: `
          DELETE FROM agent_feed_items WHERE id IN ('e2e-group-1', 'e2e-group-2', 'e2e-group-3');

          INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
          VALUES
            ('e2e-group-1', 'e2e-tenant', 'instagram_dm', '{"customer_message": "Can I order a vegan cake?", "group_key": "cake_inquiries"}'::jsonb, '{"description": "Draft reply for vegan cake", "draft_reply": "Yes we do! Here is a booking link."}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            ('e2e-group-2', 'e2e-tenant', 'instagram_dm', '{"customer_message": "Do you make chocolate cakes?", "group_key": "cake_inquiries"}'::jsonb, '{"description": "Draft reply for chocolate cake", "draft_reply": "Yes we do!"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            ('e2e-group-3', 'e2e-tenant', 'instagram_dm', '{"customer_message": "How much for a wedding cake?", "group_key": "cake_inquiries"}'::jsonb, '{"description": "Draft reply for wedding cake", "draft_reply": "Wedding cakes start at $200."}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
          ON CONFLICT (id) DO UPDATE SET lifecycle_state = 'PENDING_APPROVAL', updated_at = CURRENT_TIMESTAMP;
        `
      }
    });
  });

  const performLogin = async (page: any) => {
    await page.goto("/login");
    await page.getByPlaceholder("Email or Username").first().fill("e2e-user");
    await page.locator('input[type="password"]').first().fill("password123");
    await page.getByRole("button", { name: "Log In" }).click();
    await page.waitForTimeout(1000);
  };

  test("Grouping and Approve All", async ({ page }) => {
    await performLogin(page);

    await page.goto("/dashboard");
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const groupHeader = page.locator("text=3 new cake inquiries").first();
    await expect(groupHeader).toBeVisible();

    await groupHeader.click();

    const approveAllBtn = page.getByRole('button', { name: 'Approve All' }).first();
    await expect(approveAllBtn).toBeVisible();

    await approveAllBtn.click();

    await expect(groupHeader).not.toBeVisible();
  });
});
