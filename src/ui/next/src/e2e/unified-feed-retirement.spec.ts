import { randomUUID } from 'node:crypto';
import { test, expect } from '../../../../e2e/fixtures';
import { e2eDbQuery, e2eDbTransaction } from '../../../../e2e/db_utils';

const aliases = [
  { legacy: '/unified-feed.html', state: 'APPROVED', button: 'Record approval' },
  { legacy: '/ui/unified-feed.html', state: 'DISMISSED', button: 'Reject' },
] as const;

for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
  for (const { legacy, state, button } of aliases) {
    test(`${legacy} reaches a persisted ${state.toLowerCase()} decision at ${viewport.width}px`, async ({ page, loginAs, adminUser, unlimitedAdminUser, baseURL }, testInfo) => {
      if (!baseURL) throw new Error('The isolated app base URL is required');
      const base = new URL(baseURL);
      if (!['http:', 'https:'].includes(base.protocol) || !['127.0.0.1', 'localhost', '[::1]'].includes(base.hostname)
          || base.username || base.password) throw new Error('Feed fixtures require the isolated local app');
      const tenantId = adminUser.organizationId;
      const owners = await e2eDbQuery(`SELECT u.id, u.tenant_id FROM users u
        JOIN identity_user_roles r ON r.user_id=u.id AND r.tenant_id=u.tenant_id
        WHERE u.username=$1 AND u.tenant_id=$2 AND u.active=true
          AND u.roles=ARRAY['ADMIN']::text[] AND r.role_name='ADMIN'`, [adminUser.email, tenantId]);
      expect(owners).toHaveLength(1);
      await page.setViewportSize(viewport);
      // The standard fixture restores real server-issued cached credentials.
      await loginAs(page, adminUser);
      const session = await page.request.get('/api/v1/auth/session-identity');
      expect(session.status()).toBe(200);
      expect(await session.json()).toMatchObject({ userId: owners[0].id, tenantId });
      const foreignTenantId = unlimitedAdminUser.organizationId;
      expect(foreignTenantId).not.toBe(tenantId);
      const id = `e2e-feed-retirement-${randomUUID()}`;
      const cacheRefreshId = `${id}-cache`;
      const ownedIds = [id, cacheRefreshId];
      const description = `Review the saved retirement decision ${id}`;
      const writes: string[] = [];
      page.on('request', request => {
        if (request.method() === 'PUT' && new URL(request.url()).pathname === `/api/v1/agent-feed/${id}`) writes.push(request.url());
      });
      let seeded = false;
      try {
        // Only unique case-owned rows are inserted in the disposable seed tenant.
        // Neither has a feature_type or incident source that admits provider work.
        await e2eDbTransaction(async query => {
          for (const ownedId of ownedIds) {
            await query(`INSERT INTO agent_feed_items
              (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
              VALUES ($1, $2, 'Operations', $3::jsonb, $4::jsonb, 'PENDING_APPROVAL', NOW(), NOW())`,
            [ownedId, tenantId, JSON.stringify({ description: ownedId === id ? description : `Cache refresh ${ownedId}` }), JSON.stringify({ action_type: 'proposal', summary: 'Review this recorded draft only.' })]);
          }
        });
        seeded = true;
        // SQL does not invalidate the real list cache. Publish a separate owned
        // row's pending state so the target still receives exactly one PUT.
        const refreshed = await page.request.put(`/api/v1/agent-feed/${cacheRefreshId}`, {
          headers: { origin: base.origin, 'sec-fetch-site': 'same-origin', 'x-ohc-expected-user': owners[0].id, 'x-ohc-expected-tenant': tenantId },
          data: { state: 'PENDING_APPROVAL' },
        });
        expect(refreshed.status(), `Refresh owned feed fixture: ${await refreshed.text()}`).toBe(200);
        expect(await refreshed.json()).toMatchObject({ id: cacheRefreshId, tenant_id: tenantId, lifecycle_state: 'PENDING_APPROVAL', dispatch: { status: 'NOT_REQUESTED', job_id: null } });
        const suffix = `?audit=unified-retirement&ref=first&ref=second&tenant_id=${encodeURIComponent(foreignTenantId)}#decision`;
        const redirected = page.waitForResponse(response => new URL(response.url()).pathname === legacy);
        const loaded = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/agent-feed' && response.request().method() === 'GET');
        await page.goto(`${legacy}${suffix}`, { waitUntil: 'domcontentloaded' });
        await expect(page).toHaveURL(new URL(`/unified-feed${suffix}`, baseURL).href);
        const redirect = await redirected;
        expect(redirect.status()).toBe(307);
        expect(redirect.headers()['cache-control']).toContain('no-store');
        const list = await loaded;
        expect(list.status()).toBe(200);
        const body = await list.json();
        expect(body.items.some((item: { id: string }) => item.id === id)).toBe(true);
        expect(body.items.every((item: { tenant_id: string }) => item.tenant_id === tenantId)).toBe(true);
        const foreignQuery = await page.request.get(`/api/v1/agent-feed?tenant_id=${encodeURIComponent(foreignTenantId)}`);
        expect(foreignQuery.status()).toBe(200);
        const foreignBody = await foreignQuery.json();
        expect(foreignBody.items.some((item: { id: string }) => item.id === id)).toBe(true);
        expect(foreignBody.items.every((item: { tenant_id: string }) => item.tenant_id === tenantId)).toBe(true);
        const card = page.getByTestId('agent-feed-card').filter({ hasText: description });
        await expect(card).toBeVisible();
        await expect(card.getByRole('button', { name: 'Record approval', exact: true })).toBeVisible();
        await expect(page.getByRole('button', { name: /approve.*send|^sending/i })).toHaveCount(0);
        await expect(page.getByText('Approving records a decision. Execution or delivery requires a separate verified outcome.', { exact: true })).toBeVisible();

        const committed = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/agent-feed/${id}` && response.request().method() === 'PUT');
        await card.getByRole('button', { name: button, exact: true }).click();
        const response = await committed;
        expect(response.status()).toBe(200);
        const receipt = await response.json();
        expect(receipt).toMatchObject({ id, tenant_id: tenantId, lifecycle_state: state, decision_recorded: true, dispatch: { status: 'NOT_REQUESTED', job_id: null } });
        await expect(card).toHaveCount(0);
        const notice = state === 'APPROVED' ? 'Approval recorded. Execution or delivery is not verified by this decision.' : 'Dismissal recorded.';
        await expect(page.getByRole('status', { name: 'Decision status' }).filter({ hasText: notice })).toBeVisible();
        expect(await e2eDbQuery('SELECT lifecycle_state FROM agent_feed_items WHERE id=$1 AND tenant_id=$2', [id, tenantId])).toEqual([{ lifecycle_state: state }]);
        expect(await e2eDbQuery('SELECT decision_state, dispatch_status, job_id FROM agent_feed_decisions WHERE action_id=$1 AND tenant_id=$2', [id, tenantId])).toEqual([{ decision_state: state, dispatch_status: 'NOT_REQUESTED', job_id: null }]);
        expect(await e2eDbQuery("SELECT count(*)::int AS count FROM ohc_job_queue WHERE tenant_id=$1 AND payload->>'action_id'=ANY($2::text[])", [tenantId, ownedIds])).toEqual([{ count: 0 }]);
        const readback = await page.request.get(`/api/v1/agent-feed/${id}/decision`);
        expect(readback.status()).toBe(200);
        expect(await readback.json()).toMatchObject({ id, tenant_id: tenantId, lifecycle_state: state, decision_recorded: true, dispatch: { status: 'NOT_REQUESTED', job_id: null } });

        await page.reload({ waitUntil: 'domcontentloaded' });
        await expect(page.getByText('Loading feed...', { exact: true })).toBeHidden();
        await expect(card).toHaveCount(0);
        await expect(page.getByRole('status', { name: 'Decision status' }).filter({ hasText: notice })).toBeVisible();
        expect(writes).toHaveLength(1);
        const head = await page.request.head(`${legacy}?ref=first&ref=second`, { maxRedirects: 0 });
        expect(head.status()).toBe(307);
        const destination = new URL(head.headers().location, head.url());
        expect(destination.pathname).toBe('/unified-feed');
        expect(destination.search).toBe('?ref=first&ref=second');
        const post = await page.request.post(legacy, { maxRedirects: 0, headers: { origin: base.origin, 'sec-fetch-site': 'same-origin' }, data: {} });
        expect([404, 405]).toContain(post.status());
        expect(post.headers().location).toBeUndefined();
        await testInfo.attach('recorded-decision', { body: await page.screenshot(), contentType: 'image/png' });
      } finally {
        if (seeded) {
          await e2eDbTransaction(async query => {
            await query('DELETE FROM agent_feed_decisions WHERE tenant_id=$1 AND action_id=ANY($2::text[])', [tenantId, ownedIds]);
            await query('DELETE FROM agent_feed_items WHERE tenant_id=$1 AND id=ANY($2::text[])', [tenantId, ownedIds]);
            expect(await query('SELECT id FROM agent_feed_items WHERE tenant_id=$1 AND id=ANY($2::text[])', [tenantId, ownedIds])).toEqual([]);
            expect(await query('SELECT action_id FROM agent_feed_decisions WHERE tenant_id=$1 AND action_id=ANY($2::text[])', [tenantId, ownedIds])).toEqual([]);
          });
        }
      }
    });
  }
}

for (const { legacy } of aliases) {
  test(`anonymous ${legacy} stays behind login`, async ({ anonymousPage }) => {
    const response = await anonymousPage.request.get(`${legacy}?ref=retirement`, { maxRedirects: 0 });
    expect(response.status()).toBe(307);
    expect(response.headers()['cache-control']).toContain('no-store');
    const target = new URL(response.headers().location, response.url());
    expect(target.pathname).toBe('/login');
    expect(target.searchParams.get('next')).toBe(`${legacy}?ref=retirement`);
  });
}
