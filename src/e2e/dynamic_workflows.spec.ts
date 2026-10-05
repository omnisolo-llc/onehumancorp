import { request as playwrightRequest, type APIResponse, type Response } from '@playwright/test';
import { test, expect } from './fixtures';
import { authenticateRequest } from './authenticate';
import { e2eDbQuery } from './db_utils';
import { seedDashboardAuditOwner } from './support/dashboard_audit_fixture';

test.describe('Dynamic Workflows Orhestrator', () => {
  test('should render the form and handle a basic generation flow', async ({ anonymousPage: page, request, baseURL }, testInfo) => {
    if (!baseURL) throw new Error('The isolated local app URL is required');
    const origin = new URL(baseURL).origin;
    const owner = await seedDashboardAuditOwner(baseURL);
    const prompt = 'Test dynamic workflow prompt';
    const receipts: Record<string, unknown>[] = [];
    const submitted: unknown[] = [];
    const startedAt = Date.now();
    let planId: string | undefined;
    const capture = async (response: APIResponse | Response, label: string) => {
      const receipt: Record<string, unknown> = { label, atMs: Date.now() - startedAt, url: response.url(), status: response.status() };
      receipts.push(receipt);
      const body = await response.text();
      receipt.body = body;
      return body;
    };
    const jobs = () => e2eDbQuery(
      "SELECT count(*)::int AS count FROM sub_agent_queue WHERE tenant_id=$1 AND payload::jsonb->>'workflow_id'=$2",
      [owner.tenantId, planId],
    );
    page.on('request', request => {
      if (request.url() === `${origin}/api/v1/dynamic-workflows` && request.method() === 'POST') submitted.push(request.postDataJSON());
    });
    try {
      await authenticateRequest(page.request, { username: owner.email, password: owner.password, organizationId: owner.tenantId }, origin);
      const identity = await page.request.get('/api/v1/auth/session-identity');
      try {
        expect(identity.status()).toBe(200);
        expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
      } finally { await identity.dispose(); }
      await page.goto('/dynamic-workflows');
      await expect(page.getByRole('heading', { name: 'Dynamic Workflows Orchestrator', exact: true })).toBeVisible();
      await page.getByRole('textbox').fill(prompt);
      const [created] = await Promise.all([
        page.waitForResponse(response => response.url() === `${origin}/api/v1/dynamic-workflows` && response.request().method() === 'POST'),
        page.getByRole('button', { name: 'Generate Workflow', exact: true }).click(),
      ]);
      const raw = await capture(created, 'actual-browser-create');
      expect(created.status(), raw).toBe(200);
      const body = JSON.parse(raw);
      expect(body).toMatchObject({ plan: { tenant_id: owner.tenantId, prompt, status: 'awaiting_confirmation', requires_confirmation: true }, enqueued_jobs: 0 });
      expect(body.plan.id).toMatch(/^dwf-[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/);
      expect(body.plan.tasks.length).toBeGreaterThan(0);
      planId = body.plan.id;
      await expect(page.getByRole('heading', { name: 'Workflow Status: awaiting_confirmation' })).toBeVisible();
      await expect(page.getByText('Plan saved. No work has been queued.', { exact: true })).toBeVisible();
      await expect(page.getByRole('button', { name: 'Approve & Queue Workflow', exact: true })).toBeEnabled();
      await expect(page.locator('main pre')).toContainText(planId!);
      expect(await jobs()).toEqual([{ count: 0 }]);

      // Refresh reads the actual owner plan. Generation does not authorize any
      // queue dispatch or provider work; no owner confirmation is clicked here.
      const [loaded] = await Promise.all([
        page.waitForResponse(response => response.url() === `${origin}/api/v1/dynamic-workflows/${planId}` && response.request().method() === 'GET'),
        page.getByRole('button', { name: 'Refresh', exact: true }).click(),
      ]);
      const loadedRaw = await capture(loaded, 'actual-browser-owner-readback');
      expect(loaded.status(), loadedRaw).toBe(200);
      expect(JSON.parse(loadedRaw)).toEqual(body.plan);
      await expect(page.getByRole('button', { name: 'Approve & Queue Workflow', exact: true })).toBeEnabled();

      const otherIdentity = await request.get('/api/v1/auth/session-identity');
      try {
        expect(otherIdentity.status()).toBe(200);
        expect((await otherIdentity.json()).tenantId).not.toBe(owner.tenantId);
      } finally { await otherIdentity.dispose(); }
      for (const method of ['GET', 'POST']) {
        const path = `/api/v1/dynamic-workflows/${planId}${method === 'POST' ? '/confirm' : ''}`;
        const denied = await request.fetch(path, { method, headers: { origin, 'sec-fetch-site': 'same-origin' }, maxRedirects: 0 });
        try {
          const deniedRaw = await capture(denied, `foreign-${method}`);
          expect(denied.status(), deniedRaw).toBe(404);
          expect(JSON.parse(deniedRaw)).toEqual({ error: 'workflow not found' });
        } finally { await denied.dispose(); }
      }
      const anonymous = await playwrightRequest.newContext({ baseURL, storageState: { cookies: [], origins: [] } });
      try {
        for (const method of ['GET', 'POST']) {
          const denied = await anonymous.fetch(`/api/v1/dynamic-workflows/${planId}${method === 'POST' ? '/confirm' : ''}`, { method, headers: { origin, 'sec-fetch-site': 'same-origin' }, maxRedirects: 0 });
          try { await capture(denied, `anonymous-${method}`); expect(denied.status()).toBe(401); }
          finally { await denied.dispose(); }
        }
      } finally { await anonymous.dispose(); }
      const afterDenied = await page.request.get(`/api/v1/dynamic-workflows/${planId}`);
      try {
        const afterRaw = await capture(afterDenied, 'owner-after-denied-confirmations');
        expect(afterDenied.status(), afterRaw).toBe(200);
        expect(JSON.parse(afterRaw)).toEqual(body.plan);
      } finally { await afterDenied.dispose(); }
      expect(await jobs()).toEqual([{ count: 0 }]);
      expect(submitted).toEqual([{ prompt }]);
    } finally {
      if (planId) {
        try { receipts.push({ label: 'finally-queue-sql', tenantId: owner.tenantId, planId, rows: await jobs() }); }
        catch (error) { receipts.push({ label: 'finally-queue-sql', error: String(error) }); }
      }
      await testInfo.attach('dynamic-workflow-real-receipts', { body: Buffer.from(JSON.stringify({ tenantId: owner.tenantId, planId, submitted, receipts }, null, 2)), contentType: 'application/json' });
    }
  });

  test('shows actual validation failure without inventing a generated workflow', async ({ page }, testInfo) => {
    await page.goto('/dynamic-workflows');
    await page.getByRole('textbox').fill('   ');
    const [response] = await Promise.all([
      page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/dynamic-workflows' && response.request().method() === 'POST'),
      page.getByRole('button', { name: 'Generate Workflow', exact: true }).click(),
    ]);
    const raw = await response.text();
    await testInfo.attach('dynamic-workflow-validation-response', { body: Buffer.from(JSON.stringify({ status: response.status(), body: raw })), contentType: 'application/json' });
    expect(response.status(), raw).toBe(400);
    expect(JSON.parse(raw)).toEqual({ error: 'prompt is required' });
    await expect(page.getByRole('alert').filter({ hasText: 'prompt is required' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Generate Workflow', exact: true })).toBeEnabled();
    await expect(page.getByRole('heading', { name: /Workflow Status:/ })).toHaveCount(0);
  });
});
