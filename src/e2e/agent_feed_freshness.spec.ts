import type { APIResponse, Response } from '@playwright/test';
import { expect, test } from './fixtures';
import { authenticateRequest } from './authenticate';
import { e2eDbQuery } from './db_utils';
import { seedDashboardAuditOwner } from './support/dashboard_audit_fixture';

for (const mobile of [false, true]) {
  test(`agent feed ${mobile ? 'mobile' : 'standard'} reads committed state after a warmed snapshot`, async ({ anonymousPage: page, baseURL }, testInfo) => {
    if (!baseURL) throw new Error('The isolated local app URL is required');
    const owner = await seedDashboardAuditOwner(baseURL);
    const origin = new URL(baseURL).origin;
    const itemId = `${owner.namespace}-e2e-feed-inbox-quote-1`;
    // Reuse the exact same URL and cache key throughout each variant.
    const feedUrl = new URL(mobile
      ? '/api/v1/agent-feed?limit=20&offset=0&mobile_optimized=true'
      : '/api/v1/agent-feed', origin).href;
    const startedAt = Date.now();
    const receipts: Record<string, unknown>[] = [];
    const states: Record<string, unknown>[] = [];
    const readState = () => e2eDbQuery(
      'SELECT id, tenant_id, lifecycle_state FROM agent_feed_items WHERE tenant_id=$1 AND id=$2',
      [owner.tenantId, itemId],
    );
    const captureResponse = async (response: APIResponse | Response, label: string) => {
      const receipt: Record<string, unknown> = {
        label, atMs: Date.now() - startedAt, url: response.url(), status: response.status(),
        cacheControl: response.headers()['cache-control'],
      };
      receipts.push(receipt);
      const text = await response.text();
      receipt.body = text;
      expect(response.status()).toBe(200);
      const body = JSON.parse(text);
      expect(Array.isArray(body.items)).toBe(true);
      const matches = body.items.filter((item: { id: string }) => item.id === itemId);
      expect(matches).toHaveLength(1);
      const item = matches[0];
      expect(item).toMatchObject({ id: itemId, event_source: 'Sales' });
      if (mobile) {
        expect(Object.keys(item).sort()).toEqual(['created_at', 'event_source', 'id', 'lifecycle_state']);
      } else {
        expect(item).toMatchObject({
          tenant_id: owner.tenantId,
          context_payload: { description: 'Vegan pastry box quote approval' },
          proposed_action: { inbox_message_id: `${owner.namespace}-e2e-inbox-msg-1` },
        });
      }
      return item;
    };
    const readFeed = async (label: string) => {
      const response = await page.request.get(feedUrl, { maxRedirects: 0 });
      try { return await captureResponse(response, label); }
      finally { await response.dispose(); }
    };

    try {
      await authenticateRequest(page.request, {
        username: owner.email, password: owner.password, organizationId: owner.tenantId,
      }, origin);
      const identity = await page.request.get('/api/v1/auth/session-identity');
      try {
        expect(identity.status()).toBe(200);
        expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
      } finally { await identity.dispose(); }

      expect((await readFeed('warm-read')).lifecycle_state).toBe('PENDING_APPROVAL');
      expect((await readFeed('repeat-warm-read')).lifecycle_state).toBe('PENDING_APPROVAL');
      const card = page.getByTestId('agent-feed-card').filter({
        has: page.getByText('Vegan pastry box quote approval', { exact: true }),
      });
      if (!mobile) {
        await page.goto('/unified-feed');
        await expect(card).toBeVisible();
        await expect(card.getByTestId('feed-approve-btn')).toBeEnabled();
      }

      // This isolated fixture transition represents another committed writer.
      // It deliberately bypasses API cache invalidation; it is not a replay of
      // the intermittent real-PUT race or a claim that any work was dispatched.
      const committed = await e2eDbQuery(
        "UPDATE agent_feed_items SET lifecycle_state='APPROVED', updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$1 AND id=$2 AND lifecycle_state='PENDING_APPROVAL' RETURNING id, tenant_id, lifecycle_state",
        [owner.tenantId, itemId],
      );
      states.push({ label: 'committed-fixture-transition', atMs: Date.now() - startedAt, rows: committed });
      const expected = [{ id: itemId, tenant_id: owner.tenantId, lifecycle_state: 'APPROVED' }];
      expect(committed).toEqual(expected);
      const persisted = await readState();
      states.push({ label: 'read-after-commit', atMs: Date.now() - startedAt, rows: persisted });
      expect(persisted).toEqual(expected);

      const current = await readFeed('read-after-commit');
      // Keep both API and browser evidence when the API returns a stale row.
      expect.soft(current.lifecycle_state).toBe('APPROVED');
      if (!mobile) {
        const reloaded = page.waitForResponse(response => response.url() === feedUrl && response.request().method() === 'GET');
        await page.reload();
        const browserItem = await captureResponse(await reloaded, 'browser-reload');
        expect.soft(browserItem.lifecycle_state).toBe('APPROVED');
        await expect(page.getByRole('heading', { name: 'Today', exact: true })).toBeVisible();
        await expect(card).toHaveCount(0);
      }
    } finally {
      try {
        states.push({ label: 'finally', atMs: Date.now() - startedAt, rows: await readState() });
      } catch (error) {
        states.push({ label: 'finally', captureError: String(error) });
      }
      await testInfo.attach('agent-feed-committed-state-receipts', {
        body: Buffer.from(JSON.stringify({ itemId, tenantId: owner.tenantId, mobile, feedUrl, startedAt, receipts, states }, null, 2)),
        contentType: 'application/json',
      });
    }
  });
}
