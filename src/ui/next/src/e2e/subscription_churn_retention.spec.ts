import { expect, test } from '../../../../e2e/fixtures';
import { seedDashboardAuditOwner } from '../../../../e2e/support/dashboard_audit_fixture';
import { e2eDbQuery } from '../../../../e2e/db_utils';

test.describe('Agentic Subscription Retention & Churn Prediction Feed E2E', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display subscription churn win-back recommendation in the feed and allow approval', async ({ anonymousPage, baseURL }) => {
    test.setTimeout(180000);
    const page = anonymousPage;
    if (!baseURL) throw new Error('The isolated app base URL is required');
    // Other feed journeys may approve the shared seed or move it off page one.
    // Reuse the canonical real database graph in a fresh tenant for this case.
    const actor = await seedDashboardAuditOwner(baseURL);
    const churnId = `${actor.namespace}-e2e-feed-churn`;

    // 1. Log in as this case's real Leo account through the same browser UI.
    await page.goto('/login');
    await page.getByRole('textbox', { name: 'Email or username' }).fill(`${actor.namespace}-leo@example.com`);
    await page.getByRole('textbox', { name: 'Password' }).fill(actor.password);
    await page.getByLabel(/Organization/).fill(actor.tenantId);
    await page.getByRole('button', { name: 'Log in' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });
    expect(await e2eDbQuery('SELECT lifecycle_state FROM agent_feed_items WHERE id=$1 AND tenant_id=$2',
      [churnId, actor.tenantId])).toEqual([{ lifecycle_state: 'PENDING_APPROVAL' }]);

    // Navigate to the unified agent feed
    await page.goto('/feed');

    // Wait for the feed items to populate
    await expect(page.getByTestId('agent-feed').first()).toBeVisible({ timeout: 25000 });

    // Assert that we see a churn risk Action Card
    const churnCard = page.locator('[data-testid="agent-feed-card"]').filter({ hasText: 'at risk of churning' }).first();
    await expect(churnCard).toBeVisible({ timeout: 15000 });

    const approveBtn = churnCard.locator('button', { hasText: 'Approve' }).first();
    await expect(approveBtn).toBeVisible({ timeout: 15000 });

    const decision = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/agent-feed/${churnId}/state`
      && response.request().method() === 'PUT');
    await approveBtn.click();
    expect((await decision).status()).toBe(200);

    // Assert that the card is removed after approval
    await expect(approveBtn).not.toBeVisible({ timeout: 15000 });
    expect(await e2eDbQuery('SELECT lifecycle_state FROM agent_feed_items WHERE id=$1 AND tenant_id=$2',
      [churnId, actor.tenantId])).toEqual([{ lifecycle_state: 'APPROVED' }]);
  });
});
