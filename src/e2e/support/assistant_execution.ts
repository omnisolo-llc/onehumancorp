import { expect, type Page } from '@playwright/test';
import { e2eDbTransaction } from '../db_utils';

export async function assistantDatabaseState(tenantId: string) {
  return e2eDbTransaction(async query => {
    await query("SELECT set_config('app.current_tenant',$1,true)", [tenantId]);
    const accounts = await query(`SELECT limit_micros::text, spent_micros::text, reserved_micros::text
      FROM ohc_usage_accounts WHERE tenant_id=$1`, [tenantId]);
    const usage = await query(`SELECT event_id,state,reserved_micros::text,charged_micros::text,
      provider_cost_micros::text,scope_json::jsonb AS scope,receipt_json::jsonb AS receipt
      FROM ohc_usage_records WHERE tenant_id=$1 ORDER BY event_id`, [tenantId]);
    const receipts = await query(`SELECT id,actor_id,request_id,phase,output FROM tenant_workflow_receipts
      WHERE tenant_id=$1 ORDER BY id`, [tenantId]);
    const tasks = await query(`SELECT id,actor_id,root_request_id,current_receipt_id,title FROM assistant_execution_tasks
      WHERE tenant_id=$1 ORDER BY id`, [tenantId]);
    const attempts = await query(`SELECT receipt_id,task_id,actor_id,source_receipt_id FROM assistant_execution_attempts
      WHERE tenant_id=$1 ORDER BY receipt_id`, [tenantId]);
    const claims = await query(`SELECT provider,provider_request_id,event_id FROM ohc_usage_receipts
      WHERE tenant_id=$1 ORDER BY event_id`, [tenantId]);
    return { accounts, usage, receipts, tasks, attempts, claims };
  });
}

export async function assertUnavailableAssistantComposer(page: Page, owner: { userId: string; tenantId: string }, prompt: string) {
  const identity = await page.request.get('/api/v1/auth/session-identity');
  expect(identity.status()).toBe(200);
  expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
  const policy = await page.request.get('/api/v1/agents/execution-policy');
  expect(policy.status()).toBe(200);
  expect(await policy.json()).toEqual({ available: false, policy: null, mode: 'text_analysis', workspace_access: false, tools: [] });
  const empty = { accounts: [], usage: [], receipts: [], tasks: [], attempts: [], claims: [] };
  expect(await assistantDatabaseState(owner.tenantId)).toEqual(empty);
  await expect(page.getByText('Text execution is unavailable. Existing receipts can still be read.', { exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'New Task', exact: true }).click();
  await page.getByLabel('Task prompt', { exact: true }).fill(prompt);
  await expect(page.getByRole('button', { name: 'Start Task', exact: true })).toBeDisabled();
  await expect(page.getByLabel('Work directory', { exact: true })).toBeDisabled();
  await expect(page.getByRole('combobox', { name: 'Output format', exact: true }).getByRole('option')).toHaveText(['Text']);
  // Check the authenticated server boundary too: a caller cannot turn the
  // truthful disabled control into a receipt by posting directly.
  const rejected = await page.evaluate(async ({ owner, prompt }) => {
    const response = await fetch('/api/v1/assistant/tasks', { method: 'POST', headers: {
      'content-type': 'application/json', 'Idempotency-Key': crypto.randomUUID(),
      'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId,
    }, body: JSON.stringify({ prompt, workspace: 'Unavailable runtime check', mode: 'Ask', model: 'Auto',
      provider: 'Auto', outputFormat: 'Text', permissionProfile: 'Guarded', constraints: '', workDirectory: '' }) });
    return { status: response.status, body: await response.json() };
  }, { owner, prompt });
  expect(rejected.status).toBe(503);
  expect(rejected.body).toHaveProperty('error');
  expect(rejected.body).not.toHaveProperty('task');
  expect(rejected.body).not.toHaveProperty('receipt');
  expect(await assistantDatabaseState(owner.tenantId)).toEqual(empty);
  await page.reload();
  await expect(page.getByText('Text execution is unavailable. Existing receipts can still be read.', { exact: true })).toBeVisible();
  await expect(page.getByRole('heading', { name: prompt, exact: true })).toHaveCount(0);
  expect(await assistantDatabaseState(owner.tenantId)).toEqual(empty);
}
