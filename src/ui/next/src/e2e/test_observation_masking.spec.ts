import { test, expect } from '../../../../e2e/fixtures';
import { createGrowthOwner } from '../../../../e2e/growth_owner';
import { e2eDbQuery } from '../../../../e2e/db_utils';

test('text-only Assistant rejects unused masking settings and persists its real name', async ({ page, baseURL }) => {
  const owner = await createGrowthOwner(page, baseURL);
  const key = `assistant.agent_name:${owner.tenantId}`;
  const savedRows = () => e2eDbQuery('SELECT key,value,updated_by FROM application_settings WHERE key=$1', [key]);
  const identity = await page.request.get('/api/v1/auth/session-identity');
  expect(identity.status()).toBe(200);
  expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
  await page.goto('/assistant?panel=system');
  await expect(page.getByRole('heading', { name: 'System', exact: true })).toBeVisible();
  await expect(page.getByText('This text-only Assistant has no tool observations to mask. Tenant masking controls are unavailable. Built-in agent masking is configured separately.', { exact: true })).toBeVisible();
  await expect(page.getByRole('checkbox', { name: 'Observation Masking Toggle', exact: true })).toBeDisabled();
  await expect(page.getByRole('checkbox', { name: 'Observation Masking Toggle', exact: true })).not.toBeChecked();
  await expect(page.getByRole('button', { name: 'Save UI Settings', exact: true })).toBeDisabled();
  const uiWrites: string[] = [];
  page.on('request', request => { if (request.method() === 'PATCH' && new URL(request.url()).pathname === '/api/v1/assistant/settings') uiWrites.push(request.postData() ?? ''); });
  expect(await savedRows()).toEqual([]);
  // Real authenticated direct callers must also receive rejection, including a
  // mixed write that might otherwise silently save only the supported field.
  for (const payload of [{ observationMasking: true }, { observationMasking: false },
    { observationMasking: null }, { agentName: 'Must not be saved', observationMasking: true }]) {
    const response = await page.request.patch('/api/v1/assistant/settings', { headers: {
      origin: new URL(baseURL!).origin, 'sec-fetch-site': 'same-origin',
      'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId,
    }, data: payload });
    expect(response.status()).toBe(400);
    expect(await response.text()).not.toContain('"settings"');
    expect(await savedRows()).toEqual([]);
    const read = await page.request.get('/api/v1/assistant/settings');
    expect(read.status()).toBe(200); expect(await read.json()).toEqual({ settings: {} });
  }
  expect(uiWrites).toEqual([]);
  const name = `Owned assistant ${owner.userId.slice(-12)}`;
  await page.getByRole('textbox', { name: 'Assistant name', exact: true }).fill(name);
  const write = page.waitForEvent('requestfinished', { predicate: request => request.method() === 'PATCH'
    && new URL(request.url()).pathname === '/api/v1/assistant/settings' });
  await page.getByRole('button', { name: 'Save Name', exact: true }).click();
  const response = await (await write).response();
  expect(response).not.toBeNull();
  expect(response!.status()).toBe(200);
  expect(response!.request().postDataJSON()).toEqual({ agentName: name });
  expect(await response!.json()).toEqual({ settings: { agentName: name } });
  await expect(page.getByRole('status').filter({ hasText: /^Settings saved$/ })).toBeVisible();
  expect(await savedRows()).toEqual([{ key, value: name, updated_by: owner.userId }]);
  expect(uiWrites).toEqual([JSON.stringify({ agentName: name })]);
  await page.reload();
  await expect(page.getByRole('heading', { name: `${name} Assistant`, exact: true })).toBeVisible();
  const read = await page.request.get('/api/v1/assistant/settings');
  expect(read.status()).toBe(200); expect(await read.json()).toEqual({ settings: { agentName: name } });
  expect(await savedRows()).toEqual([{ key, value: name, updated_by: owner.userId }]);
  await expect(page.getByRole('button', { name: 'Save UI Settings', exact: true })).toBeDisabled();
});
