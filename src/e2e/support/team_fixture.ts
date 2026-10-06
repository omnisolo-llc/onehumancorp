import { randomUUID } from 'node:crypto';
import type { Page } from '@playwright/test';
import { test as base, expect } from '../fixtures';
import { createGrowthOwner } from '../growth_owner';
import { e2eDbQuery, e2eDbTransaction } from '../db_utils';
import { requireLoopbackUrl } from './recorded_invitation';

export type TeamOwner = Awaited<ReturnType<typeof createTeamOwner>>;
export type TeamRequest = {
  id: string; tenant_id: string; department: 'Operations'; description: string;
  status: 'PendingApproval'; action_risk: 'DraftForReview';
  payload: { original_request: string; action: 'semantic_routed_task' };
};

async function createTeamOwner(page: Page, baseURL: string | undefined) {
  if (!baseURL) throw new Error('Team tests require the native runner app URL');
  requireLoopbackUrl(baseURL);
  const owner = await createGrowthOwner(page, baseURL);
  const identity = await page.request.get('/api/v1/auth/session-identity');
  expect(identity.status()).toBe(200);
  expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
  await page.context().addInitScript(tenant => {
    if (location.protocol === 'http:' || location.protocol === 'https:') {
      localStorage.setItem('tenant_id', tenant);
      localStorage.setItem('tenant', tenant);
    }
  }, owner.tenantId);
  const foreignTenant = `e2e-team-foreign-${randomUUID()}`;
  const foreignId = randomUUID();
  // Only pending source data is seeded. Requests under test and their decisions
  // must travel through the real authenticated UI/backend, never synthetic ACKs.
  await e2eDbTransaction(async query => {
    await query("INSERT INTO tenants(id,name) VALUES($1,'Foreign Team fixture')", [foreignTenant]);
    await query(`INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,proposed_action,lifecycle_state)
      VALUES($1,$2,'operations',$3::jsonb,$4::jsonb,'PENDING_APPROVAL')`, [foreignId, foreignTenant,
      JSON.stringify({ description: `Private foreign request ${foreignId}` }),
      JSON.stringify({ original_request: `Foreign operations ${foreignId}`, action: 'semantic_routed_task' })]);
  });
  expect(await e2eDbQuery('SELECT id FROM agent_feed_items WHERE tenant_id=$1', [owner.tenantId])).toEqual([]);
  return { ...owner, origin: new URL(baseURL).origin, foreignTenant, foreignId };
}

export const test = base.extend<{ teamOwner: TeamOwner }>({
  teamOwner: async ({ page, baseURL }, use) => { await use(await createTeamOwner(page, baseURL)); },
});
export { expect };

export function captureTeamResponse(page: Page, owner: TeamOwner, path: string, method = 'GET') {
  return page.waitForResponse(response => new URL(response.url()).origin === owner.origin
    && new URL(response.url()).pathname === path && response.request().method() === method).then(response => {
    expect(response.status()).toBe(200);
    expect(response.request().headers()['x-ohc-expected-user']).toBe(owner.userId);
    expect(response.request().headers()['x-ohc-expected-tenant']).toBe(owner.tenantId);
    return response;
  });
}

export async function readTeamPage(page: Page, owner: TeamOwner, requests: TeamRequest[], navigate: () => Promise<unknown>) {
  const read = captureTeamResponse(page, owner, '/api/v1/agents/approvals');
  await navigate();
  const response = await read;
  expect(response.headers()['cache-control']).toContain('no-store');
  expect(await response.json()).toEqual({ pending_approvals: [...requests].sort((a, b) => a.id.localeCompare(b.id)), next_cursor: null });
  await expect(page.getByRole('button', { name: 'Refresh recorded decisions', exact: true })).toBeEnabled();
  await expect(page.getByRole('alert')).toHaveCount(0);
  await expect(page.getByText(`Private foreign request ${owner.foreignId}`, { exact: true })).toHaveCount(0);
}

export async function assertPendingRequests(page: Page, owner: TeamOwner, requests: TeamRequest[]) {
  const response = await page.request.get('/api/v1/agents/approvals?limit=100');
  expect(response.status()).toBe(200);
  expect(await response.json()).toEqual({ pending_approvals: [...requests].sort((a, b) => a.id.localeCompare(b.id)), next_cursor: null });
  expect(await e2eDbQuery(`SELECT id,tenant_id,event_source,context_payload,proposed_action,lifecycle_state
    FROM agent_feed_items WHERE tenant_id=$1 AND lifecycle_state='PENDING_APPROVAL' ORDER BY id`, [owner.tenantId]))
    .toEqual([...requests].sort((a, b) => a.id.localeCompare(b.id)).map(item => ({ id: item.id, tenant_id: owner.tenantId,
      event_source: 'operations', context_payload: { description: item.description }, proposed_action: item.payload, lifecycle_state: 'PENDING_APPROVAL' })));
}

export async function sendTeamRequest(page: Page, owner: TeamOwner, message: string): Promise<TeamRequest> {
  await expect(page.getByRole('textbox', { name: 'Message your team', exact: true })).toBeEnabled();
  await page.getByRole('textbox', { name: 'Message your team', exact: true }).fill(message);
  const saved = captureTeamResponse(page, owner, '/api/v1/agents/chat', 'POST');
  await page.getByRole('button', { name: 'Send', exact: true }).click();
  const response = await saved;
  expect(response.request().postDataJSON()).toEqual({ message });
  const body = await response.json();
  expect(body).toEqual({ success: true, department_assigned: 'operations', approval: {
    id: expect.stringMatching(/^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/), tenant_id: owner.tenantId,
    department: 'Operations', description: 'Task routed via semantic gateway to Operations',
    status: 'PendingApproval', action_risk: 'DraftForReview', payload: { original_request: message, action: 'semantic_routed_task' },
  } });
  const card = page.getByTestId('action-card').filter({ hasText: message });
  await expect(card).toHaveCount(1);
  await expect(card.getByRole('heading', { name: body.approval.description, exact: true })).toBeVisible();
  await expect(card).toContainText('Owner review required');
  await expect(card.getByRole('button', { name: 'Record approval', exact: true })).toBeEnabled();
  await expect(page.getByRole('status').filter({ hasText: 'Request saved for department review. No execution or delivery is confirmed.' })).toBeVisible();
  await expect(page.getByRole('textbox', { name: 'Message your team', exact: true })).toHaveValue('');
  return body.approval;
}

export const approvalNotice = 'Approval recorded. No dispatch was requested; execution or delivery is not verified.';

export async function recordTeamApproval(page: Page, owner: TeamOwner, request: TeamRequest) {
  const saved = captureTeamResponse(page, owner, `/api/v1/agent-feed/${request.id}`, 'PUT');
  await page.getByRole('button', { name: 'Record approval', exact: true }).click();
  const response = await saved;
  expect(response.request().postDataJSON()).toEqual({ state: 'APPROVED' });
  const body = await response.json();
  expect(body).toMatchObject({ id: request.id, tenant_id: owner.tenantId, lifecycle_state: 'APPROVED', decision_recorded: true,
    event_source: 'operations', proposed_action: request.payload, context_payload: { description: request.description } });
  // A semantic review request has no executable feature_type. Verify the exact
  // no-dispatch receipt, rather than treating approval as completed provider work.
  expect(body.dispatch).toEqual({ status: 'NOT_REQUESTED', job_id: null, attempted_at: null, dispatch_returned_at: null, detail: null });
  await expect(page.getByRole('status').filter({ hasText: approvalNotice })).toBeVisible();
  await assertRecordedApproval(page, owner, request);
  return body;
}

export async function assertRecordedApproval(page: Page, owner: TeamOwner, request: TeamRequest) {
  const read = await page.request.get(`/api/v1/agent-feed/${request.id}/decision`);
  expect(read.status()).toBe(200);
  expect(await read.json()).toMatchObject({ id: request.id, tenant_id: owner.tenantId, lifecycle_state: 'APPROVED', decision_recorded: true,
    proposed_action: request.payload, dispatch: { status: 'NOT_REQUESTED', job_id: null, attempted_at: null, dispatch_returned_at: null, detail: null } });
  expect(await e2eDbQuery(`SELECT f.id,f.tenant_id,f.lifecycle_state,f.proposed_action,d.action_id,d.decision_state,d.actor_id,
    d.dispatch_status,d.job_id,d.attempted_at,d.dispatch_returned_at,d.dispatch_payload
    FROM agent_feed_items f JOIN agent_feed_decisions d ON d.action_id=f.id AND d.tenant_id=f.tenant_id
    WHERE f.id=$1 AND f.tenant_id=$2`, [request.id, owner.tenantId])).toEqual([{
    id: request.id, tenant_id: owner.tenantId, lifecycle_state: 'APPROVED', proposed_action: request.payload,
    action_id: request.id, decision_state: 'APPROVED', actor_id: owner.userId, dispatch_status: 'NOT_REQUESTED',
    job_id: null, attempted_at: null, dispatch_returned_at: null, dispatch_payload: null,
  }]);
  expect(await e2eDbQuery("SELECT id FROM ohc_job_queue WHERE tenant_id=$1 AND job_type='agent_feed_action'", [owner.tenantId])).toEqual([]);
}

export async function assertForeignRequestUnchanged(page: Page, owner: TeamOwner) {
  const foreign = await page.request.get(`/api/v1/agent-feed/${owner.foreignId}/decision`);
  expect(foreign.status()).toBe(404);
  expect(await foreign.json()).toEqual({ error: 'Feed item not found', success: false });
  const write = await page.request.put(`/api/v1/agent-feed/${owner.foreignId}`, {
    headers: { origin: owner.origin, 'sec-fetch-site': 'same-origin',
      'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId },
    data: { state: 'APPROVED' },
  });
  expect(write.status()).toBe(404);
  expect(await write.json()).toEqual({ error: 'Feed item not found', success: false });
  expect(await e2eDbQuery('SELECT id,lifecycle_state FROM agent_feed_items WHERE tenant_id=$1', [owner.foreignTenant]))
    .toEqual([{ id: owner.foreignId, lifecycle_state: 'PENDING_APPROVAL' }]);
  expect(await e2eDbQuery('SELECT action_id FROM agent_feed_decisions WHERE tenant_id=$1', [owner.foreignTenant])).toEqual([]);
}
