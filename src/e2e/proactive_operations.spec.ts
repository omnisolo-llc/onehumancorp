import { test, expect } from './fixtures';
import { createDashboardAuditCase } from './support/dashboard_audit_fixture';
import { e2eDbQuery } from './db_utils';

test.describe('Proactive Operations Task Feed', () => {
  test('an owner can review, assign and draft a schedule from recorded operations proposals', async ({ browser, baseURL }) => {
    if (!baseURL) throw new Error('The isolated application URL is required');
    const owned = await createDashboardAuditCase(browser, baseURL, { width: 1280, height: 720 });
    try {
      await owned.navigate();
      await expect(owned.page.getByRole('region', { name: 'Unified Agent Feed' })).toBeVisible();
      for (const [suffix, action] of [
        ['e2e-ops-checklist', 'Review Checklist'],
        ['e2e-ops-supplier', 'Assign to Staff'],
        ['e2e-ops-staffing', 'Draft Schedule Request'],
      ]) {
        const id = `${owned.actor.namespace}-${suffix}`;
        const card = owned.page.getByTestId(`triage-card-${id}`);
        await expect(card).toBeVisible();
        const accepted = owned.page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/agent-feed/${id}` && response.request().method() === 'PUT');
        await card.getByRole('button', { name: action, exact: true }).click();
        expect((await accepted).status()).toBe(200);
        await expect(card).toHaveCount(0);
        expect(await e2eDbQuery('SELECT lifecycle_state FROM agent_feed_items WHERE id=$1 AND tenant_id=$2', [id, owned.actor.tenantId])).toEqual([{ lifecycle_state: 'APPROVED' }]);
      }
    } finally { await owned.close(); }
  });
});
