import { test, expect } from "./fixtures";

test("Autonomous Service Portfolio Generator", async ({ page, request }) => {
  // 1. User opens the app
  await page.goto("/");

  // Login via UI (from global-setup login structure) is implicit here or we use the authenticated context

  // 2. Trigger the job completion event through the webhook
  const response = await request.post("/api/agents/webhook", {
    data: {
      event_type: "tenant.job.completed",
      payload: {
        service_name: "Cedar Fence Install",
        media: ["https://example.com/finished-fence.jpg"]
      }
    }
  });

  // Verify the hook responded
  expect(response.status()).toBe(200);

  // Wait a short bit for the orchestration to create the draft
  await page.waitForTimeout(2000);

  // 3. Navigate to Team Inbox
  await page.goto("/team");

  // "The Promoter" has pending approvals indicator (e.g., a badge)
  const promoterCard = page.locator("text=The Promoter").locator("..");
  await expect(promoterCard).toBeVisible();

  // 4. User taps "The Promoter" department
  await promoterCard.click();

  // Verify approval inbox view for The Promoter containing the Case Study card
  await expect(page.locator("text=Portfolio Post Drafted")).toBeVisible();
  await expect(page.locator("text=Based on your recently completed job: Cedar Fence Install")).toBeVisible();

  // Verify image is displayed (checks the src directly for our mock image)
  const img = page.locator('img[alt="Project photo"]');
  await expect(img).toBeVisible();
  await expect(img).toHaveAttribute('src', 'https://example.com/finished-fence.jpg');

  // Verify description exists
  await expect(page.locator('text="Beautiful new cedar fence install completed recently. Completed on time and on budget."')).toBeVisible();

  // 5. User approves the action via the "Publish to Website" button
  const publishBtn = page.locator("button", { hasText: "Publish to Website" }).first();
  await publishBtn.click();

  // Wait for the action to be processed
  await expect(page.locator("text=Portfolio Post Drafted")).not.toBeVisible();
});
