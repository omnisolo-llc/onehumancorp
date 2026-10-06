import { test, expect, E2E_ADMIN_USER } from './fixtures';
import { createGrowthOwner } from './growth_owner';
import { authenticateRequest } from './authenticate';
import { e2eDbTransaction } from './db_utils';
import { assistantDatabaseState } from './support/assistant_execution';
import { startConfiguredAssistantFixture, ASSISTANT_PROMPT, ASSISTANT_OUTPUT, ASSISTANT_MAXIMUM, assistantRequest } from '../../scripts/checkout-browser-fixture.mjs';
import { withOwnedBrowserContexts } from '../../scripts/playwright/owned-contexts.mjs';

// The normal runner deliberately has no model configured. This one owned app
// uses the real signed session, BFF, native adapter and canonical PostgreSQL
// receipts, with a measured loopback provider and a positive bounded allowance.
test('owned text runtime reserves, executes and persists exactly one measured assistant response', async ({ page, baseURL, browser, contextOptions }, testInfo) => {
  test.setTimeout(150000);
  const owner = await createGrowthOwner(page, baseURL);
  const empty = { accounts: [], usage: [], receipts: [], tasks: [], attempts: [], claims: [] };
  expect(await assistantDatabaseState(owner.tenantId)).toEqual(empty);
  await e2eDbTransaction(async query => {
    await query("SELECT set_config('app.current_tenant',$1,true)", [owner.tenantId]);
    expect(await query(`INSERT INTO ohc_usage_accounts(tenant_id,limit_micros) SELECT id,20000 FROM tenants
      WHERE id=$1 RETURNING tenant_id,limit_micros::text`, [owner.tenantId]))
      .toEqual([{ tenant_id: owner.tenantId, limit_micros: '20000' }]);
  });
  const runtime = await startConfiguredAssistantFixture({ tenantId: owner.tenantId });
  try {
    await withOwnedBrowserContexts(browser, { ...contextOptions, baseURL: runtime.origin, proxy: runtime.proxy,
      storageState: { cookies: [], origins: [] }, serviceWorkers: 'block' }, async ([context, reopened]) => {
      for (const current of [context, reopened]) expect(await current.storageState({ indexedDB: true })).toEqual({ cookies: [], origins: [] });
      const signIn = async (current: typeof context) => {
        await authenticateRequest(current.request, { username: owner.email, password: E2E_ADMIN_USER.password,
          organizationId: owner.tenantId }, runtime.origin);
        const identity = await current.request.get('/api/v1/auth/session-identity');
        expect(identity.status()).toBe(200);
        expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
      };
      await signIn(context);
      const actor = await context.newPage();
      const policyRead = actor.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/agents/execution-policy');
      await actor.goto('/assistant');
      const policy = await policyRead;
      expect(policy.status()).toBe(200);
      expect(await policy.json()).toMatchObject({ available: true, mode: 'text_analysis', workspace_access: false, tools: [],
        policy: { provider: 'openai-compatible', model: 'owned-browser-model', max_output_tokens: 128 } });
      await expect(actor.getByText('One text response using the configured model. No file, coding, browsing, or delegation tools.', { exact: true })).toBeVisible();
      await actor.getByRole('button', { name: 'New Task', exact: true }).click();
      await actor.getByLabel('Task prompt', { exact: true }).fill(ASSISTANT_PROMPT);
      const submitted = actor.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/assistant/tasks' && response.request().method() === 'POST');
      await expect(actor.getByRole('button', { name: 'Start Task', exact: true })).toBeEnabled();
      await actor.getByRole('button', { name: 'Start Task', exact: true }).click();
      const response = await submitted;
      expect(response.status()).toBe(202);
      expect(response.headers()['cache-control']).toContain('no-store');
      expect(response.request().headers()['x-ohc-expected-user']).toBe(owner.userId);
      expect(response.request().headers()['x-ohc-expected-tenant']).toBe(owner.tenantId);
      const { task } = await response.json();
      const requestId = response.request().headers()['idempotency-key'];
      expect(requestId).toMatch(/^[a-f0-9]{8}(?:-[a-f0-9]{4}){3}-[a-f0-9]{12}$/);
      expect(task).toMatchObject({ title: ASSISTANT_PROMPT, output: null, execution: {
        id: task.id, request_id: requestId, tenant_id: owner.tenantId, actor_id: owner.userId, output: null,
      } });
      await runtime.waitForRequest();
      expect(runtime.evidence().requests).toEqual([{ method: 'POST', path: '/v1/chat/completions', body: assistantRequest(), status: null }]);
      const held = await assistantDatabaseState(owner.tenantId);
      expect(held.accounts).toEqual([{ limit_micros: '20000', spent_micros: '0', reserved_micros: String(ASSISTANT_MAXIMUM) }]);
      expect(held.usage).toEqual([{ event_id: task.id, state: 'in_flight', reserved_micros: String(ASSISTANT_MAXIMUM),
        charged_micros: null, provider_cost_micros: null, receipt: null, scope: {
          tenant_id: owner.tenantId, task_id: task.id, attempt_id: '1', provider: 'openai-compatible', model: 'owned-browser-model', payer: 'managed_api',
          rate_card: { revision: 'owned-browser-v1', input_micros_per_million: 1000000, output_micros_per_million: 2000000, cached_input_micros_per_million: 1000000 },
        } }]);
      expect(held.receipts).toEqual([{ id: task.id, actor_id: owner.userId, request_id: requestId, phase: 'dispatching', output: null }]);
      expect(held.tasks).toEqual([{ id: task.id, actor_id: owner.userId, root_request_id: requestId, current_receipt_id: task.id, title: ASSISTANT_PROMPT }]);
      expect(held.attempts).toEqual([{ receipt_id: task.id, task_id: task.id, actor_id: owner.userId, source_receipt_id: null }]);
      expect(held.claims).toEqual([]);
      expect(ASSISTANT_MAXIMUM).toBeGreaterThan(0);
      runtime.release();
      await expect(actor.getByRole('article', { name: 'Text response', exact: true })).toHaveText(ASSISTANT_OUTPUT);
      await expect(actor.getByText('Status: completed', { exact: true })).toBeVisible();
      const settled = await assistantDatabaseState(owner.tenantId);
      expect(settled.accounts).toEqual([{ limit_micros: '20000', spent_micros: '160', reserved_micros: '0' }]);
      expect(settled.usage).toEqual([{ ...held.usage[0], state: 'settled', charged_micros: '160', provider_cost_micros: '160',
        receipt: { provider_request_id: runtime.receiptId, counts: { input: 100, output: 30, cached_input: 0 } } }]);
      expect(settled.receipts).toEqual([{ ...held.receipts[0], phase: 'completed', output: ASSISTANT_OUTPUT }]);
      expect(settled.tasks).toEqual(held.tasks); expect(settled.attempts).toEqual(held.attempts);
      expect(settled.claims).toEqual([{ provider: 'openai-compatible', provider_request_id: runtime.receiptId, event_id: task.id }]);
      // Authenticated replay of the exact UI request must return the saved task,
      // without calling the owned provider or consuming a second allowance.
      const replay = await actor.evaluate(async ({ owner, requestId, body }) => {
        const response = await fetch('/api/v1/assistant/tasks', { method: 'POST', headers: {
          'content-type': 'application/json', 'Idempotency-Key': requestId,
          'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId,
        }, body: JSON.stringify(body) });
        return { status: response.status, body: await response.json() };
      }, { owner, requestId, body: response.request().postDataJSON() });
      expect(replay.status).toBe(202);
      expect(replay.body.task).toMatchObject({ id: task.id, status: 'completed', output: ASSISTANT_OUTPUT });
      await signIn(reopened);
      const fresh = await reopened.newPage();
      await fresh.goto('/assistant');
      await fresh.getByRole('button', { name: 'Results', exact: true }).click();
      await expect(fresh.getByRole('article', { name: 'Text response', exact: true })).toHaveText(ASSISTANT_OUTPUT);
      await expect(fresh.getByText(`Execution receipt: ${task.id}`, { exact: true })).toBeVisible();
      await fresh.reload();
      await fresh.getByRole('button', { name: 'Results', exact: true }).click();
      await expect(fresh.getByRole('article', { name: 'Text response', exact: true })).toHaveText(ASSISTANT_OUTPUT);
      expect(await assistantDatabaseState(owner.tenantId)).toEqual(settled);
      expect(runtime.evidence().requests).toEqual([{ method: 'POST', path: '/v1/chat/completions', body: assistantRequest(), status: 200 }]);
    });
  } finally {
    await runtime.close();
    await testInfo.attach('owned-assistant-runtime-evidence', { contentType: 'application/json', body: JSON.stringify(runtime.evidence(), null, 2) });
  }
});
