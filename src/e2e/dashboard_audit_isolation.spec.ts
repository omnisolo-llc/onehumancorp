import { test, expect } from './fixtures';
import { e2eDbQuery } from './db_utils';
import { createDashboardAuditCase } from './support/dashboard_audit_fixture';
import { tagClickTargets } from './support/ui_click_audit';
import { assertSameClickInventory } from '../../scripts/ui-audit-fixture.cjs';

test('a real dismiss in one dashboard case cannot erase another required control in the next case', async ({ browser, baseURL }) => {
  if (!baseURL) throw new Error('The isolated app base URL is required');
  const first = await createDashboardAuditCase(browser, baseURL, { width: 1280, height: 720 });
  try {
    // These factual records intentionally do not satisfy either active
    // operations rule. Workers remain enabled during the browser audit.
    expect(await e2eDbQuery(`SELECT
      (SELECT count(*)::int FROM raw_materials WHERE tenant_id LIKE $1
        AND reorder_threshold>0 AND current_quantity<reorder_threshold) AS low_stock,
      (SELECT count(*)::int FROM (
        SELECT tenant_id, customer_id FROM bookings WHERE tenant_id LIKE $1
        GROUP BY tenant_id, customer_id
        HAVING count(*)>1 AND max(start_time)<CURRENT_TIMESTAMP-INTERVAL '14 days'
      ) AS dormant_sources) AS dormant_customers`, [`${first.actor.namespace}-%`]))
      .toEqual([{ low_stock: 0, dormant_customers: 0 }]);
    await first.navigate();
    const baseline = (await tagClickTargets(first.page, first.actor.namespace, first.actor.canonicalIds)).map(target => target.key);
    const firstId = `${first.actor.namespace}-e2e-feed-churn`;
    const card = first.page.getByTestId(`triage-card-${firstId}`);
    await expect(card.getByRole('button', { name: 'Send Win-Back Offer', exact: true })).toBeVisible();
    const response = first.page.waitForResponse(result => new URL(result.url()).pathname === `/api/v1/agent-feed/${firstId}` && result.request().method() === 'PUT');
    await card.getByRole('button', { name: 'Dismiss', exact: true }).click();
    expect((await response).status()).toBe(200);
    await expect(card).toHaveCount(0);
    expect(await e2eDbQuery('SELECT lifecycle_state FROM agent_feed_items WHERE id=$1 AND tenant_id=$2', [firstId, first.actor.tenantId])).toEqual([{ lifecycle_state: 'DISMISSED' }]);
    const afterDismiss = (await tagClickTargets(first.page, first.actor.namespace, first.actor.canonicalIds)).map(target => target.key);
    expect(() => assertSameClickInventory(baseline, afterDismiss)).toThrow('missing=');

    const second = await createDashboardAuditCase(browser, baseURL, { width: 1280, height: 720 });
    try {
      await second.navigate();
      expect(second.actor.tenantId).not.toBe(first.actor.tenantId);
      assertSameClickInventory(baseline, (await tagClickTargets(second.page, second.actor.namespace, second.actor.canonicalIds)).map(target => target.key));
      const secondId = `${second.actor.namespace}-e2e-feed-churn`;
      expect(await e2eDbQuery('SELECT lifecycle_state FROM agent_feed_items WHERE id=$1 AND tenant_id=$2', [secondId, second.actor.tenantId])).toEqual([{ lifecycle_state: 'PENDING_APPROVAL' }]);
      const alternative = second.page.getByTestId(`triage-card-${secondId}`).getByRole('button', { name: 'Send Win-Back Offer', exact: true });
      await expect(alternative).toBeVisible();
      const accepted = second.page.waitForResponse(result => new URL(result.url()).pathname === `/api/v1/agent-feed/${secondId}` && result.request().method() === 'PUT');
      await alternative.click();
      expect((await accepted).status()).toBe(200);
      expect(await e2eDbQuery('SELECT lifecycle_state FROM agent_feed_items WHERE id=$1 AND tenant_id=$2', [secondId, second.actor.tenantId])).toEqual([{ lifecycle_state: 'APPROVED' }]);
      expect(await e2eDbQuery('SELECT lifecycle_state FROM agent_feed_items WHERE id=$1 AND tenant_id=$2', [firstId, first.actor.tenantId])).toEqual([{ lifecycle_state: 'DISMISSED' }]);
    } finally { await second.close(); }
  } finally { await first.close(); }
});
