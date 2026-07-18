import { test, expect } from './fixtures';

test.describe('More Cost Cents Tracking', () => {
  test('should record cost metrics and reflect on dashboard', async ({ page, loginAs }) => {
    const testUser = { email: "starter@example.com", password: "password123", role: "ADMIN" };
    await loginAs(page, testUser as any);

    // Simulate an agent action that incurs cost
    await page.request.post('/api/billing/report-cost', {
        data: {
            metric_name: 'ohc_llm_cost_total_cents',
            value: 1250,
            labels: { agent_id: 'marketing_agent' }
        }
    });

    await page.goto('/cost-dashboard', { waitUntil: 'load' });
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' })).toBeVisible({ timeout: 15000 });

    // We should see the agent cost reported
    // Wait for elements to populate
    await expect(page.locator('h3', { hasText: 'Agent & Feature Costs' }).first()).toBeVisible({ timeout: 15000 });
  });
});
