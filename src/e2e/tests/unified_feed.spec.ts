import { test, expect } from '../fixtures';
import type { Response } from '@playwright/test';
import { authenticateRequest } from '../authenticate';
import { e2eDbQuery } from '../db_utils';
import { seedDashboardAuditOwner } from '../support/dashboard_audit_fixture';

test.describe('Unified Feed E2E', () => {
  test('Complete flow: view feed and approve an item', async ({ anonymousPage: page, baseURL }, testInfo) => {
    if (!baseURL) throw new Error('The isolated local app URL is required');
    // Seed the owned tenant before its first read so shared decisions and cached
    // feed lists cannot remove this case's approval controls.
    const owner = await seedDashboardAuditOwner(baseURL);
    const origin = new URL(baseURL).origin;
    const itemId = `${owner.namespace}-e2e-feed-inbox-quote-1`;
    const remainingId = `${owner.namespace}-e2e-feed-reschedule`;
    const readState = (id: string) => e2eDbQuery(
      'SELECT lifecycle_state FROM agent_feed_items WHERE id=$1 AND tenant_id=$2',
      [id, owner.tenantId],
    );
    const startedAt = Date.now();
    let phase = 'setup';
    const responses: Record<string, unknown>[] = [];
    const states: Record<string, unknown>[] = [];
    const submittedDecisions: unknown[] = [];
    const pendingBodies: Promise<void>[] = [];
    const observeResponse = (response: Response) => {
      const request = response.request();
      const url = new URL(response.url());
      if (url.origin !== origin || !(
        request.method() === 'GET' && url.pathname === '/api/v1/agent-feed'
        || request.method() === 'PUT' && url.pathname === `/api/v1/agent-feed/${itemId}`
      )) return;
      const headers = response.headers();
      const receipt: Record<string, unknown> = {
        sequence: responses.length + 1, phase, observedAtMs: Date.now() - startedAt,
        method: request.method(), path: url.pathname + url.search, status: response.status(),
        fromServiceWorker: response.fromServiceWorker(),
        cacheControl: headers['cache-control'], age: headers.age, xCache: headers['x-cache'],
        ...(request.method() === 'PUT' ? { requestBody: request.postData() } : {}),
      };
      responses.push(receipt);
      // Observe actual owned-tenant bodies without routing, delaying, or replacing responses.
      pendingBodies.push((async () => {
        try {
          receipt.body = await response.text();
          receipt.finishedError = await response.finished();
        } catch (error) {
          receipt.captureError = String(error);
        } finally {
          receipt.completedAtMs = Date.now() - startedAt;
          receipt.timing = request.timing();
        }
      })());
    };
    const captureState = async (label: string) => {
      const receipt: Record<string, unknown> = { label, phase, startedAtMs: Date.now() - startedAt };
      states.push(receipt);
      try {
        receipt.rows = await e2eDbQuery(
          'SELECT id, tenant_id, lifecycle_state, updated_at FROM agent_feed_items WHERE tenant_id=$1 AND id=ANY($2::text[]) ORDER BY id',
          [owner.tenantId, [itemId, remainingId]],
        );
        receipt.marker = await page.evaluate(key => localStorage.getItem(key),
          `omnisolo_feed_decision_v1:${encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId]))}:${itemId}`);
      } catch (error) {
        receipt.captureError = String(error);
      } finally {
        receipt.completedAtMs = Date.now() - startedAt;
      }
    };
    page.on('response', observeResponse);
    try {
      expect(await readState(itemId)).toEqual([{ lifecycle_state: 'PENDING_APPROVAL' }]);
      expect(await readState(remainingId)).toEqual([{ lifecycle_state: 'PENDING_APPROVAL' }]);

      await authenticateRequest(page.request, {
        username: owner.email, password: owner.password, organizationId: owner.tenantId,
      }, origin);
      const identity = await page.request.get('/api/v1/auth/session-identity');
      try {
        expect(identity.status()).toBe(200);
        expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
      } finally { await identity.dispose(); }
      await page.context().addInitScript(tenant => {
        localStorage.setItem('tenant_id', tenant);
        localStorage.setItem('tenant', tenant);
        localStorage.setItem('business_display_name', tenant);
      }, owner.tenantId);

      phase = 'dashboard';
      await page.goto('/dashboard');
      await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
      phase = 'unified-feed';
      await page.goto('/unified-feed');
      await expect(page.getByRole('heading', { name: 'Today', exact: true })).toBeVisible();

      const card = page.getByTestId('agent-feed-card').filter({
        has: page.getByText('Vegan pastry box quote approval', { exact: true }),
      });
      const remainingCard = page.getByTestId('agent-feed-card').filter({
        has: page.getByText('Reschedule delivery appointment for Order #1042', { exact: true }),
      });
      await expect(card).toHaveCount(1);
      await expect(remainingCard).toBeVisible();
      const approveBtn = card.getByTestId('feed-approve-btn');
      await expect(approveBtn).toBeVisible();
      await expect(approveBtn).toBeEnabled();

      const decisionUrl = new URL(`/api/v1/agent-feed/${itemId}`, origin).href;
      page.on('request', request => {
        if (request.url() === decisionUrl && request.method() === 'PUT') {
          submittedDecisions.push(request.postDataJSON());
        }
      });
      phase = 'approval';
      const [decision] = await Promise.all([
        page.waitForResponse(response => response.url() === decisionUrl && response.request().method() === 'PUT'),
        approveBtn.click(),
      ]);
      expect(decision.status()).toBe(200);
      expect(await decision.json()).toMatchObject({
        id: itemId, tenant_id: owner.tenantId, lifecycle_state: 'APPROVED',
      });
      expect(await readState(itemId)).toEqual([{ lifecycle_state: 'APPROVED' }]);
      await expect(page.getByRole('status', { name: 'Decision status' })).toHaveText(
        'Approval recorded. Execution or delivery is not verified by this decision.',
      );

      // Other pending approvals must survive; a live global .first() locator would
      // retarget to one of them after this specific acknowledged card is removed.
      await expect(card).toHaveCount(0);
      await expect(remainingCard.getByTestId('feed-approve-btn')).toBeEnabled();
      expect(await readState(remainingId)).toEqual([{ lifecycle_state: 'PENDING_APPROVAL' }]);
      phase = 'reload';
      await page.reload();
      await captureState('immediately-after-reload');
      await expect(page.getByRole('heading', { name: 'Today', exact: true })).toBeVisible();
      await expect(remainingCard.getByTestId('feed-approve-btn')).toBeEnabled();
      await expect(card).toHaveCount(0);
      expect(await readState(itemId)).toEqual([{ lifecycle_state: 'APPROVED' }]);
      expect(await readState(remainingId)).toEqual([{ lifecycle_state: 'PENDING_APPROVAL' }]);
      expect(submittedDecisions).toEqual([{ state: 'APPROVED' }]);
    } finally {
      await captureState('finally');
      page.off('response', observeResponse);
      await Promise.all(pendingBodies);
      await testInfo.attach('unified-feed-actual-response-and-sql-diagnostics', {
        body: Buffer.from(JSON.stringify({
          itemId, remainingId, userId: owner.userId, tenantId: owner.tenantId,
          startedAt, responses, states, submittedDecisions,
        }, null, 2)),
        contentType: 'application/json',
      });
    }
  });
});
