import { test, expect } from './fixtures';
import { createGrowthOwner } from './growth_owner';
import { seedFeedItem } from './feed-fixtures';
import { e2eDbQuery, e2eDbTransaction } from './db_utils';

test.describe('Mobile Unified Feed MVP', () => {
  // Use a mobile viewport to simulate 375px
  test.use({ viewport: { width: 375, height: 667 } });

  test('should load the unified feed and process agent cards', async ({ anonymousPage: page, baseURL }, testInfo) => {
    if (!baseURL) throw new Error('The isolated local app URL is required');
    const base = new URL(baseURL);
    if (!['http:', 'https:'].includes(base.protocol) || !['127.0.0.1', 'localhost', '[::1]'].includes(base.hostname)
        || base.username || base.password) throw new Error('Mobile feed fixtures require the isolated local app');
    const owner = await createGrowthOwner(page, baseURL);
    const ids: string[] = [];
    const receipts: Record<string, unknown> = { owner, decisions: [] };
    const writes: string[] = [];
    try {
      const identity = await page.request.get('/api/v1/auth/session-identity');
      try {
        receipts.identity = await identity.json();
        expect(identity.status()).toBe(200);
        expect(receipts.identity).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
      } finally { await identity.dispose(); }

      // A fresh tenant owns exactly these three records. Seed before mounting any
      // feed so an older in-flight list cannot refill the cache after invalidation.
      // No feature_type or incident source admits execution for these decisions.
      for (let index = 0; index < 3; index++) {
        ids.push(await seedFeedItem(page, {
          event_source: 'Operations',
          context_payload: { description: `Mobile review ${index + 1}` },
          proposed_action: { action_type: 'proposal', summary: 'Review this recorded draft only.' },
        }, owner.tenantId, { requestOrigin: base.origin }));
      }
      page.on('request', request => {
        if (request.method() === 'PUT' && ids.some(id => new URL(request.url()).pathname === `/api/v1/agent-feed/${id}`)) writes.push(request.url());
      });
      const loaded = page.waitForResponse(response => new URL(response.url()).origin === base.origin
        && new URL(response.url()).pathname === '/api/v1/agent-feed' && response.request().method() === 'GET');
      await page.goto('/unified-feed.html');
      await expect(page).toHaveURL(new URL('/unified-feed', baseURL).href);
      const list = await loaded;
      expect(list.status()).toBe(200);
      const body = await list.json();
      receipts.initialFeed = body;
      expect(body.items.map((item: { id: string }) => item.id).sort()).toEqual([...ids].sort());
      expect(body.items.every((item: { tenant_id: string }) => item.tenant_id === owner.tenantId)).toBe(true);
      await expect(page.getByTestId('agent-feed')).toBeVisible();
      const cards = page.getByTestId('agent-feed-card');
      await expect(cards).toHaveCount(3);
      expect(await page.evaluate(() => document.body.scrollWidth)).toBe(375);

      // Preserve the mobile touch contract for every rendered card action.
      for (const button of await cards.getByRole('button').all()) {
        const box = await button.boundingBox();
        expect(box).not.toBeNull();
        expect(box!.height).toBeGreaterThanOrEqual(44);
        expect(box!.width).toBeGreaterThanOrEqual(44);
      }
      for (const [index, id] of ids.entries()) {
        const card = cards.filter({ hasText: `Mobile review ${index + 1}` });
        const committed = page.waitForResponse(response => new URL(response.url()).origin === base.origin
          && new URL(response.url()).pathname === `/api/v1/agent-feed/${id}` && response.request().method() === 'PUT');
        await card.getByRole('button', { name: 'Record approval', exact: true }).click();
        const [response] = await Promise.all([
          committed,
          expect(cards).toHaveCount(2 - index, { timeout: 2000 }),
        ]);
        const receipt = await response.json();
        (receipts.decisions as unknown[]).push({ status: response.status(), body: receipt });
        expect(response.status()).toBe(200);
        expect(receipt).toMatchObject({ id, tenant_id: owner.tenantId, lifecycle_state: 'APPROVED', decision_recorded: true, dispatch: { status: 'NOT_REQUESTED', job_id: null } });
        expect(await e2eDbQuery('SELECT lifecycle_state FROM agent_feed_items WHERE id=$1 AND tenant_id=$2', [id, owner.tenantId])).toEqual([{ lifecycle_state: 'APPROVED' }]);
        expect(await e2eDbQuery('SELECT decision_state, dispatch_status, job_id FROM agent_feed_decisions WHERE action_id=$1 AND tenant_id=$2', [id, owner.tenantId])).toEqual([{ decision_state: 'APPROVED', dispatch_status: 'NOT_REQUESTED', job_id: null }]);
      }
      await expect(page.getByTestId('triage-feed-empty')).toBeVisible();
      await expect(page.getByRole('status', { name: 'Decision status' }).filter({ hasText: 'Approval recorded. Execution or delivery is not verified by this decision.' })).toHaveCount(3);
      expect(writes).toHaveLength(3);
      for (const id of ids) expect(writes.filter(url => new URL(url).pathname === `/api/v1/agent-feed/${id}`)).toHaveLength(1);
      receipts.queuedJobs = await e2eDbQuery('SELECT count(*)::int AS count FROM ohc_job_queue WHERE tenant_id=$1', [owner.tenantId]);
      expect(receipts.queuedJobs).toEqual([{ count: 0 }]);
    } finally {
      try {
        await testInfo.attach('mobile-feed-recorded-decisions', { body: JSON.stringify({ ...receipts, writes }), contentType: 'application/json' });
      } finally {
        await e2eDbTransaction(async query => {
          await query('DELETE FROM agent_feed_decisions WHERE tenant_id=$1 AND action_id=ANY($2::text[])', [owner.tenantId, ids]);
          await query('DELETE FROM agent_feed_items WHERE tenant_id=$1 AND id=ANY($2::text[])', [owner.tenantId, ids]);
        });
      }
    }
  });
});
