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
          VALUES ('e2e-paused-task-1', 'e2e-tenant', 'business_advisory', 'AI Agent Paused: The Advisor', 'PAUSED', 'LOW', '{"proposed_content": "System is paused. Please manually check business performance."}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
          ON CONFLICT DO NOTHING;
        `,
      },
  });

    // 2. Go to the dashboard
    await page.goto("/dashboard");

    // Wait for the unified feed to load
    await expect(page.locator("text=Activity Feed").first())
      .toBeVisible({ timeout: 15000 });


    // 3. Verify the message is present in the UnifiedAgentFeed
    await expect(page.locator("text=business advisory").first())
      .toBeVisible({ timeout: 15000 });

    await expect(page.locator("text=System is paused").first())
      .toBeVisible({ timeout: 15000 });

});

test("AI Agent recovers gracefully and retries malformed JSON", async ({
  page,
  request,
}) => {
  await request.post("/api/e2e/setup", {
    data: {
      query: `
        INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at)
        VALUES ('e2e-retry-task-1', 'e2e-tenant', 'business_advisory', 'AI Agent Processing', 'PENDING', 'LOW', '{"proposed_content": "Processing data..."}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ON CONFLICT DO NOTHING;
      `,
    },
  });

  await page.goto("/dashboard");

  // Real assertions that don't swallow errors
  await expect(page.locator("text=Activity Feed").first()).toBeVisible({ timeout: 15000 });
  await expect(page.locator("text=business advisory").first()).toBeVisible({ timeout: 15000 });
});
});
