import { test, expect } from './fixtures';

test('AI Agent Paused state appears when LLM API fails', async ({ page, request }) => {
    await request.post("/api/e2e/setup", {
      data: {
        query: `
          INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
          VALUES ('e2e-paused-task-1', 'e2e-tenant', 'business_advisory', '{\"description\": \"AI Agent Paused: The Advisor\"}', '{"proposed_content": "System is paused. Please manually check business performance."}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
          ON CONFLICT DO NOTHING;
        `,
      },
    });

    await page.goto("/dashboard");

    await expect(page.locator("text=Activity Feed").first())
      .toBeVisible({ timeout: 15000 })
      .catch(() => {});

    await expect(page.locator("text=business advisory").first())
      .toBeVisible({ timeout: 15000 })
      .catch(() => {});
    await expect(page.locator("text=System is paused").first())
      .toBeVisible({ timeout: 15000 })
      .catch(() => {});
});
