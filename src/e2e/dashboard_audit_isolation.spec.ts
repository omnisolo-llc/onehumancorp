import { test, expect } from './fixtures';
import { e2eDbQuery } from './db_utils';
import { createDashboardAuditCase, prepareClickAuditState } from './support/dashboard_audit_fixture';
import { tagClickTargets } from './support/ui_click_audit';
import { assertSameClickInventory } from '../../scripts/ui-audit-fixture.cjs';
import { scopeClickInventory } from '../../scripts/ui-audit-inventory.cjs';

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

for (const route of ['/unified-feed', '/dashboard/unified-feed', '/feed', '/action-center']) {
  test(`a persisted action on ${route} cannot erase the next case's coverage`, async ({ browser, baseURL }) => {
    if (!baseURL) throw new Error('The isolated app base URL is required');
    const first = await createDashboardAuditCase(browser, baseURL, { width: 1280, height: 720 });
    try {
      await first.navigate(route);
      const discover = () => tagClickTargets(first.page, first.actor.namespace, first.actor.canonicalIds);
      const baseline = (await discover()).map(target => target.key);
      const mutation = first.page.waitForResponse(response =>
        ['PUT', 'POST'].includes(response.request().method())
        && /^\/api\/v1\/(?:agent-feed|agents\/approvals)\//.test(new URL(response.url()).pathname));
      await first.page.getByRole('button', { name: route.includes('unified-feed') ? 'Reject' : 'Dismiss', exact: true }).first().click();
      const response = await mutation;
      expect(response.status()).toBe(200);
      await response.finished();
      const approvalId = route === '/action-center'
        ? decodeURIComponent(new URL(response.url()).pathname.split('/').at(-1)!) : undefined;
      if (approvalId) {
        expect(await response.json()).toEqual({ success: true });
        expect(await e2eDbQuery('SELECT lifecycle_state FROM agent_feed_items WHERE id=$1 AND tenant_id=$2', [approvalId, first.actor.tenantId]))
          .toEqual([{ lifecycle_state: 'REJECTED' }]);
      }
      const reloadedApprovals = approvalId ? first.page.waitForResponse(result =>
        new URL(result.url()).origin === new URL(baseURL).origin
        && new URL(result.url()).pathname === '/api/v1/agents/approvals'
        && result.request().method() === 'GET') : undefined;
      // Verify the mutation survived a real reload before comparing inventories.
      await first.navigate(route);
      if (reloadedApprovals) {
        const reloaded = await reloadedApprovals;
        expect(reloaded.status()).toBe(200);
        const body = await reloaded.json();
        expect(body.pending_approvals.map((approval: { id: string }) => approval.id)).not.toContain(approvalId);
      }
      await expect.poll(async () => (await discover()).length).toBeLessThan(baseline.length);
      const changed = (await discover()).map(target => target.key);
      expect(() => assertSameClickInventory(baseline, changed)).toThrow('missing=');
      const second = await createDashboardAuditCase(browser, baseURL, { width: 1280, height: 720 });
      try {
        await second.navigate(route);
        expect(second.actor.tenantId).not.toBe(first.actor.tenantId);
        assertSameClickInventory(baseline, (await tagClickTargets(second.page, second.actor.namespace, second.actor.canonicalIds)).map(target => target.key));
      } finally { await second.close(); }
    } finally { await first.close(); }
  });
}

for (const route of ['/builder', '/website-builder']) {
  test(`local draft progress on ${route} preserves entry and started-state audit coverage`, async ({ browser, baseURL }) => {
    if (!baseURL) throw new Error('The isolated app base URL is required');
    const first = await createDashboardAuditCase(browser, baseURL, { width: 1280, height: 720 });
    try {
      await first.navigate(route);
      const baseline = (await tagClickTargets(first.page, first.actor.namespace, first.actor.canonicalIds)).map(target => target.key);
      await prepareClickAuditState(first.page, route, 'started-draft');
      const started = (await tagClickTargets(first.page, first.actor.namespace, first.actor.canonicalIds)).map(target => target.key);
      expect(started.some(key => !baseline.includes(key))).toBe(true);
      const entryKeys = scopeClickInventory(baseline.map(key => ({ key })), 'entry').map(target => target.key);
      const startedKeys = scopeClickInventory(started.map(key => ({ key })), 'started-draft').map(target => target.key);
      expect(new Set([...entryKeys, ...startedKeys]).size).toBe(baseline.length + started.length);
      await first.navigate(route);
      assertSameClickInventory(started, (await tagClickTargets(first.page, first.actor.namespace, first.actor.canonicalIds)).map(target => target.key));
      expect(() => assertSameClickInventory(baseline, started)).toThrow('missing=');
      const second = await createDashboardAuditCase(browser, baseURL, { width: 1280, height: 720 });
      try {
        await second.navigate(route);
        assertSameClickInventory(baseline, (await tagClickTargets(second.page, second.actor.namespace, second.actor.canonicalIds)).map(target => target.key));
        await prepareClickAuditState(second.page, route, 'started-draft');
        assertSameClickInventory(started, (await tagClickTargets(second.page, second.actor.namespace, second.actor.canonicalIds)).map(target => target.key));
      } finally { await second.close(); }
    } finally { await first.close(); }
  });
}

for (const route of ['/onboarding', '/share-card']) {
  test(`persisted setup Back and Skip on ${route} cannot erase another owner's Upload Image coverage`, async ({ browser, baseURL }) => {
    if (!baseURL) throw new Error('The isolated app base URL is required');
    const first = await createDashboardAuditCase(browser, baseURL, { width: 1280, height: 720 });
    try {
      await first.navigate(route);
      await expect(first.page.getByRole('button', { name: 'Upload Image', exact: true })).toBeVisible();
      const baseline = (await tagClickTargets(first.page, first.actor.namespace, first.actor.canonicalIds)).map(target => target.key);
      await prepareClickAuditState(first.page, route, 'intro');
      expect(await e2eDbQuery('SELECT state_json->>\'step\' AS selected_step FROM onboarding_state WHERE tenant_id=$1 AND user_id=$2', [first.actor.tenantId, first.actor.userId])).toEqual([{ selected_step: '-2' }]);
      const changed = (await tagClickTargets(first.page, first.actor.namespace, first.actor.canonicalIds)).map(target => target.key);
      expect(() => assertSameClickInventory(baseline, changed)).toThrow('missing=');
      await expect(first.page.getByRole('button', { name: 'Upload Image', exact: true })).toHaveCount(0);
      const skipped = first.page.waitForResponse(response => new URL(response.url()).origin === new URL(baseURL).origin
        && new URL(response.url()).pathname === '/api/v1/onboarding/state' && response.request().method() === 'POST'
        && response.request().postDataJSON()?.skipped === true);
      await first.page.getByRole('button', { name: 'Skip setup', exact: true }).click();
      expect((await skipped).status()).toBe(204);
      await expect(first.page).toHaveURL(new URL('/dashboard', baseURL).href);
      expect(await e2eDbQuery('SELECT state_json->>\'skipped\' AS skipped FROM onboarding_state WHERE tenant_id=$1 AND user_id=$2', [first.actor.tenantId, first.actor.userId])).toEqual([{ skipped: 'true' }]);
      const second = await createDashboardAuditCase(browser, baseURL, { width: 1280, height: 720 });
      try {
        await second.navigate(route);
        expect(second.actor.tenantId).not.toBe(first.actor.tenantId);
        await expect(second.page).toHaveURL(new URL('/onboarding', baseURL).href);
        await expect(second.page.getByRole('button', { name: 'Upload Image', exact: true })).toBeVisible();
        assertSameClickInventory(baseline, (await tagClickTargets(second.page, second.actor.namespace, second.actor.canonicalIds)).map(target => target.key));
        expect(await e2eDbQuery('SELECT state_json->>\'skipped\' AS skipped FROM onboarding_state WHERE tenant_id=$1 AND user_id=$2', [first.actor.tenantId, first.actor.userId])).toEqual([{ skipped: 'true' }]);
      } finally { await second.close(); }
    } finally { await first.close(); }
  });
}
