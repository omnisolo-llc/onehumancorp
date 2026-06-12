import { expect, test } from "./fixtures";

test.describe("Ambassador Intake to Unified Agent Feed Mobile UX", () => {
  // Use a strictly 375px wide viewport as specified by the issue
  test.use({ viewport: { width: 375, height: 667 } });

  test("ambassador receives Instagram DM, drafts a reply and surfaces action card", async ({
    page,
    request,
  }) => {
    // 1. Send the webhook payload to the newly created endpoint.
    const submitResponse = await request.post("/api/v1/webhooks/ambassador", {
      data: {
        tenant_id: "e2e-tenant",
        message: "Do you make custom vegan cakes?",
        source: "instagram_dm",
      },
    });

    expect(submitResponse.ok()).toBeTruthy();

    // 2. Load the dashboard on mobile
    await page.goto("/dashboard");

    // 3. Ensure the unified feed tab is visible
    await expect(page.locator("text=Activity Feed").first()).toBeVisible({ timeout: 15000 });

    // Wait for the ambassador instagram DM card to appear
    await expect(async () => {
      await page.reload();
      const igCard = page.locator("text=Do you make custom vegan cakes?").first();
      await expect(igCard).toBeVisible({ timeout: 5000 });
    }).toPass({
      intervals: [2000, 5000, 10000],
      timeout: 30000,
    });

    // 4. Verify Ambassador Reply specific UI elements (using feature_type ambassador_reply)
    await expect(page.locator("text=Customer Inquiry").first()).toBeVisible();
    await expect(page.locator("text=Thank you for your message.").first()).toBeVisible(); // LocalLLMClient mock response

    // 5. Verify touch targets on the Ambassador specific button
    const approveIgButton = page.locator('button[data-testid="approve-ambassador-reply"]').first();
    await expect(approveIgButton).toBeVisible();
    await approveIgButton.click();

    // The item should optimistically disappear
    await expect(page.locator("text=Do you make custom vegan cakes?").first()).not.toBeVisible();
  });
});
