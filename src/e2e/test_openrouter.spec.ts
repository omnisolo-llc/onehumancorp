import { test, expect } from './fixtures';
import { integrationStorage } from './support/integration_storage';

// OpenRouter is an enum value, not a mounted chat provider. Keep this real
// authenticated negative boundary separate from the configured response test
// required in docs/development/openrouter-chat-acceptance.md.
test('OpenRouter configuration cannot invent a verified connection or assistant answer', async ({ page, unlimitedAdminUser, loginAs }) => {
  await loginAs(page, unlimitedAdminUser);
  const before = await integrationStorage(unlimitedAdminUser.organizationId, ['openrouter']);
  const loaded = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/assistant/tasks' && response.request().method() === 'GET');
  await page.goto('/assistant');
  const taskList = await loaded;
  expect(taskList.status()).toBe(200);
  const current = await taskList.json();
  expect(current.capabilities.modelProviders).toEqual(['Auto']);
  await page.getByRole('button', { name: 'New Task', exact: true }).click();
  const model = page.getByRole('combobox', { name: 'Model', exact: true });
  await expect(model).toBeVisible();
  await expect(model.getByRole('option')).toHaveText(['Auto']);

  // The production vault does not support OpenRouter. This synthetic credential
  // must be rejected before network verification, storage or model invocation.
  for (let attempt = 0; attempt < 2; attempt += 1) {
    const response = await page.evaluate(async () => {
      const reply = await fetch('/api/v1/integrations/openrouter/connect', {
        method: 'POST', headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ api_token: 'public-local-unverified-openrouter-fixture' }),
      });
      return { status: reply.status, body: await reply.json() };
    });
    expect(response.status).toBe(501);
    expect(response.body).toMatchObject({ success: false, status: 'unavailable', usable: false });
    expect(response.body).not.toHaveProperty('reply');
    expect(response.body).not.toHaveProperty('receipt');
    expect(await integrationStorage(unlimitedAdminUser.organizationId, ['openrouter'])).toEqual(before);
  }
  await expect(model).toHaveValue('Auto');
});
