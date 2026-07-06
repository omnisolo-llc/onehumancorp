import { test, expect } from '@playwright/test';
import { setupTestTenant, cleanupTestTenant, executeSql } from './db_utils';

test.describe('Subscription Health & Churn Prevention', () => {
  let tenantId: string;

  test.beforeEach(async () => {
    tenantId = await setupTestTenant('sub-health-tester');
  });

  test.afterEach(async () => {
    await cleanupTestTenant(tenantId);
  });

  test('Worker identifies at-risk subscriber and agent drafts win-back message', async ({ page }) => {
    await executeSql(`
      INSERT INTO subscription_plans (id, tenant_id, name, price_cents, frequency)
      VALUES ('plan_health_1', '${tenantId}', 'Music Lessons', 5000, 'monthly')
    `);

    await executeSql(`
      INSERT INTO subscribers (id, tenant_id, customer_id, subscription_plan_id, status, stripe_subscription_id, health_score)
      VALUES ('sub_health_test_1', '${tenantId}', 'cust_health_1', 'plan_health_1', 'PAST_DUE', 'sub_stripe_1', 100)
    `);

    await executeSql(`
      INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status)
      VALUES ('job_health_1', '${tenantId}', 'subscription_health', '{"subscriber_id":"sub_health_test_1","customer_id":"cust_health_1"}', 'PENDING')
    `);

    await expect(async () => {
      const check = await executeSql(`
        SELECT description, payload
        FROM agent_action_requests
        WHERE tenant_id = '${tenantId}' AND department_type = 'CustomerSuccess'
        ORDER BY created_at DESC LIMIT 1
      `);
      expect(check.length).toBeGreaterThan(0);
    }).toPass({ timeout: 15000 });

    await page.goto(`/login?test_tenant=${tenantId}`);
    await page.goto('/dashboard/unified-feed');
    await expect(page.getByText('identified subscriber sub_health_test_1 as at-risk')).toBeVisible({ timeout: 15000 });

    await page.getByTestId('feed-approve-btn').first().click();
    await expect(page.getByText('identified subscriber sub_health_test_1 as at-risk')).toBeHidden();
  });
});
