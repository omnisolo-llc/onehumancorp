import { expect, test } from "./fixtures";

test.describe("Agentic Voice-to-Action Mobile Command Center", () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test("Allows user to execute a complex voice command and generates an Action Card", async ({ page, request }) => {
    // 1. Seed some distinct approvals representing different departments
    await request.post("/api/e2e/setup", {
      data: {
        query: `
          DELETE FROM agent_approvals WHERE id = 'voice-test-id';
        `,
      },
    });

    // 2. Load the dashboard on mobile
    await page.goto("/dashboard");

    // 3. Since we cannot mock the microphone easily on E2E without complex setup,
    // we directly call the endpoint to simulate what the mic would send, and then reload
    const response = await request.post('/api/v1/voice/command', {
      data: {
        mock_transcript: "Send a $150 repair quote to the last customer who called",
        tenant_id: "default"
      }
    });
    expect(response.ok()).toBeTruthy();

    await page.goto("/dashboard");

    // 4. Verify the new action card exists in the agent feed
    const quoteCard = page.locator("text=Send a $150 repair quote to the last customer who called").first();
    await expect(quoteCard).toBeVisible();

    // Check that it's actionable
    const approveButton = page.locator('button[data-testid="approve-proposal"]').first();
    await expect(approveButton).toBeVisible();
  });
});
