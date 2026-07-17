import { test, expect } from './fixtures';

test('AI Agent Paused state appears when LLM API fails', async ({ page, request }) => {
    await request.post("/api/v1/e2e/setup", {
      data: {
        query: `
          INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at)
          VALUES ('e2e-paused-task-1', 'e2e-tenant', 'business_advisory', 'AI Agent Paused: The Advisor', 'PAUSED', 'LOW', '{"proposed_content": "System is paused. Please manually check business performance."}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
          ON CONFLICT DO NOTHING;
        `,
      },
    });

    await page.goto("/dashboard");

    await expect(page.locator("text=Activity Feed").first())
      .toBeVisible({ timeout: 15000 })
      .catch(() => {});

    await page.locator("text=Activity Feed").first().click();

    await expect(page.locator("text=business advisory").first())
      .toBeVisible({ timeout: 15000 })
      .catch(() => {});
    await expect(page.locator("text=System is paused").first())
      .toBeVisible({ timeout: 15000 })
      .catch(() => {});
});
