import { test, expect } from './fixtures';
import { executeSql } from './db_utils';
import { E2E_ADMIN_USER } from './identities';

test.describe('Subscription Health & Churn Prevention', () => {
  const tenantId = E2E_ADMIN_USER.organizationId;

  test.beforeEach(async () => {
    try {
      await executeSql(`DELETE FROM ohc_job_queue WHERE id = 'job_health_1'`);
      await executeSql(`DELETE FROM agent_action_requests WHERE tenant_id = '${tenantId}' AND (description LIKE '%sub_health_test_1%' OR payload::text LIKE '%sub_health_test_1%')`);
      await executeSql(`DELETE FROM subscribers WHERE id = 'sub_health_test_1'`);
      await executeSql(`DELETE FROM subscription_plans WHERE id = 'plan_health_1'`);
    } catch {
      // Ignore cleanup error
    }
  });

  test.afterEach(async () => {
    try {
      await executeSql(`DELETE FROM ohc_job_queue WHERE id = 'job_health_1'`);
      await executeSql(`DELETE FROM agent_action_requests WHERE tenant_id = '${tenantId}' AND (description LIKE '%sub_health_test_1%' OR payload::text LIKE '%sub_health_test_1%')`);
      await executeSql(`DELETE FROM subscribers WHERE id = 'sub_health_test_1'`);
      await executeSql(`DELETE FROM subscription_plans WHERE id = 'plan_health_1'`);
    } catch {
      // Ignore cleanup error
    }
  });

  test('Worker identifies at-risk subscriber and agent drafts win-back message', async ({ page, loginAs, adminUser }) => {
    await executeSql(`
      INSERT INTO subscription_plans (id, tenant_id, name, price_cents, frequency)
      VALUES ('plan_health_1', '${tenantId}', 'Music Lessons', 5000, 'monthly')
      ON CONFLICT (id) DO UPDATE SET price_cents = 5000
    `);

    await executeSql(`
      INSERT INTO subscribers (id, tenant_id, customer_id, subscription_plan_id, status, stripe_subscription_id, health_score)
      VALUES ('sub_health_test_1', '${tenantId}', 'cust_health_1', 'plan_health_1', 'PAST_DUE', 'sub_stripe_1', 100)
      ON CONFLICT (id) DO UPDATE SET status = 'PAST_DUE', health_score = 100
    `);

    await executeSql(`
      INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status)
      VALUES ('job_health_1', '${tenantId}', 'subscription_health', '{"subscriber_id":"sub_health_test_1","customer_id":"cust_health_1"}', 'PENDING')
      ON CONFLICT (id) DO UPDATE SET status = 'PENDING', next_retry_at = CURRENT_TIMESTAMP
    `);

    await expect(async () => {
      const check = await executeSql(`
        SELECT description, payload
        FROM agent_action_requests
        WHERE tenant_id = '${tenantId}' AND (department_type = 'customer_success' OR department_type = 'CustomerSuccess')
        ORDER BY created_at DESC LIMIT 1
      `);
      expect(check.length).toBeGreaterThan(0);
      expect(JSON.stringify(check[0])).toContain('sub_health_test_1');
    }).toPass({ timeout: 15000 });

    await loginAs(page, adminUser);
    await page.goto('/dashboard/unified-feed');
    const card = page.locator('[data-testid="agent-feed-card"]', { hasText: 'sub_health_test_1' }).first();
    await expect(card).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('identified subscriber sub_health_test_1 as at-risk')).toBeVisible({ timeout: 15000 });

    await card.getByTestId('feed-approve-btn').click();
    await expect(page.getByText('identified subscriber sub_health_test_1 as at-risk')).toBeHidden({ timeout: 10000 });
  });
});
