import { test, expect } from './fixtures';

const acceptance = process.env.OMNISOLO_E2E_TEXT_ANALYSIS_ACCEPTANCE === '1';
const prerequisite = 'Release owner must provision the exact-source isolated stack with a real authorized text provider, genuine funded usage policy, and explicit spending approval. No workspace runtime is granted by this gate.';

test('recorded analysis reads real owned history without dispatch when text execution is unconfigured', async ({ page, unlimitedAdminUser, loginAs }) => {
  test.skip(acceptance, 'This contract requires an unconfigured text-analysis service.');
  await loginAs(page, unlimitedAdminUser);
  const identityResponse = await page.request.get('/api/v1/auth/session-identity');
  expect(identityResponse.status()).toBe(200); const identity = await identityResponse.json();
  const mutations: string[] = [], runtime: string[] = [];
  page.on('request', request => {
    const path = new URL(request.url()).pathname;
    if (request.method() !== 'GET' && path.startsWith('/api/v1/agents/')) mutations.push(path);
    if (path === '/api/v1/agents/protocol') runtime.push(path);
  });
  const policy = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/agents/execution-policy');
  const history = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/agents/workflows');
  await page.goto('/agent-protocol');
  const configuration = await policy; expect(configuration.status()).toBe(200);
  expect(await configuration.json()).toMatchObject({ available: false, mode: 'text_analysis', workspace_access: false, tools: [], policy: null });
  const records = await history; expect(records.status()).toBe(200);
  const body = await records.json(); expect(Array.isArray(body.workflows)).toBe(true);
  for (const receipt of body.workflows) { expect(receipt.tenant_id).toBe(unlimitedAdminUser.organizationId); expect(typeof receipt.actor_id).toBe('string'); }
  const panel = page.getByRole('region', { name: 'Recorded text analysis' });
  await expect(panel.getByText('Text analysis is not configured. No task can be submitted.')).toBeVisible();
  await expect(panel.getByRole('button', { name: /^Open analysis / })).toHaveCount(body.workflows.filter((receipt: { actor_id: string }) => receipt.actor_id === identity.userId).length);
  await panel.getByLabel('Analysis name').fill('Review supplied business notes');
  await panel.getByLabel('Analysis task').fill('Analyze only these supplied notes.');
  await expect(panel.getByRole('button', { name: 'Start text analysis' })).toBeDisabled();
  await expect(page.getByText('No tasks found.')).toHaveCount(0);
  expect(mutations).toEqual([]); expect(runtime).toEqual([]);
});

test('explicit text analysis returns actual durable provider output after reload @text-analysis-acceptance', async ({ page, unlimitedAdminUser, loginAs }) => {
  test.skip(!acceptance, prerequisite); test.setTimeout(180_000);
  await loginAs(page, unlimitedAdminUser);
  const identityResponse = await page.request.get('/api/v1/auth/session-identity');
  expect(identityResponse.status()).toBe(200); const identity = await identityResponse.json();
  const calls: string[] = [];
  page.on('request', request => { const path = new URL(request.url()).pathname; if (path === '/api/v1/agents/hire' && request.method() === 'POST' || path === '/api/v1/agents/protocol') calls.push(path); });
  const policyResponse = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/agents/execution-policy');
  await page.goto('/agent-protocol');
  const configuration = await policyResponse; expect(configuration.status()).toBe(200);
  const policy = await configuration.json(); expect(policy).toMatchObject({ available: true, mode: 'text_analysis', workspace_access: false, tools: [] });
  const panel = page.getByRole('region', { name: 'Recorded text analysis' });
  await panel.getByLabel('Analysis name').fill(`Supplied-text analysis ${Date.now()}`);
  await panel.getByLabel('Analysis task').fill('Use only these supplied notes: inventory has 12 blue mugs and 8 red mugs. Summarize the total and suggest one non-binding stock check. Do not use tools, files or external sources.');
  const accepted = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/agents/hire' && response.request().method() === 'POST');
  await panel.getByRole('button', { name: 'Start text analysis' }).click();
  const response = await accepted; expect(response.status()).toBe(201);
  const receipt = await response.json(); expect(receipt.status).toBe('queued'); expect(receipt.workflow_id).toMatch(/^[a-f0-9-]{36}$/);
  await expect(panel.getByLabel('Recorded analysis output')).not.toBeEmpty({ timeout: 120_000 });
  const returned = await panel.getByLabel('Recorded analysis output').innerText();
  expect(returned.trim()).not.toBe('');
  const detail = await page.request.get(`/api/v1/agents/workflows/${receipt.workflow_id}`);
  expect(detail.status()).toBe(200);
  const actual = (await detail.json()).workflow;
  expect(actual.id).toBe(receipt.workflow_id); expect(actual.tenant_id).toBe(unlimitedAdminUser.organizationId); expect(actual.actor_id).toBe(identity.userId);
  expect(actual.provider).toBe(policy.policy.provider); expect(actual.model).toBe(policy.policy.model);
  expect(actual.request_id).toBe(response.request().headers()['idempotency-key']);
  expect(actual.command).toBe('Configured tenant text analysis; no tools or workspace access');
  expect(actual.status).toBe('completed'); expect(actual.output).toBe(returned); expect(actual.error).toBeNull();
  await page.reload();
  await expect(panel.getByLabel('Recorded analysis output')).toHaveText(returned);
  expect(calls).toEqual(['/api/v1/agents/hire']);
});
