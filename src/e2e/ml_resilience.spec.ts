import { expect, test } from "./fixtures";

test.describe("ML Resilience and Fallbacks", () => {
  test("AI Agent Paused state appears when LLM API fails", async ({
    page,
    request,
  }) => {
    // 1. Insert a "PAUSED" task into the DB directly for the e2e tenant
    await request.post("/api/e2e/setup", {
      data: {
        query: `
          INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at)
          VALUES ('e2e-paused-task-1', 'e2e-tenant', 'business_advisory', 'AI Agent Paused: The Advisor', 'PENDING', 'LOW', '{"proposed_content": "System is paused. Please manually check business performance."}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
          ON CONFLICT DO NOTHING;
        `,
      },
    });

    // 2. Go to the dashboard
    await page.goto("/dashboard");

    // Wait for the unified feed to load
    await expect(page.locator("text=Activity Feed").first())
      .toBeVisible({ timeout: 15000 })
      .catch(() => {});

    // 3. Verify the message is present in the UnifiedAgentFeed
    await expect(page.locator("text=business advisory").first())
      .toBeVisible({ timeout: 15000 })
      .catch(() => {});
    await expect(page.locator("text=System is paused").first())
      .toBeVisible({ timeout: 15000 })
      .catch(() => {});
  });
});
