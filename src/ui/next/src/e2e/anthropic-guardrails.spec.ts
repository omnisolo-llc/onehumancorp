import { test, expect } from '../../../../e2e/fixtures';
import { createMarketplaceOwner } from '../../../../e2e/marketplace_fixtures';

// These exercise OHC storage and role enforcement. No model or provider tool gate is simulated.
test.describe('Marketplace installation and publication authority', () => {
  test('installs a reviewed inactive definition without dispatching agent work', async ({ page, baseURL }) => {
    const owner = await createMarketplaceOwner(page, baseURL);
    const executionRequests: string[] = [];
    page.on('request', request => {
      if (request.method() !== 'GET' && /^\/api\/v1\/(agents\/hire(?:\/|$)|agents\/workflows(?:\/|$)|agents\/approvals(?:\/|$)|workflows(?:\/|$)|approvals(?:\/|$))/.test(new URL(request.url()).pathname)) executionRequests.push(request.url());
    });
    await page.goto('/agent-marketplace');
    await page.getByPlaceholder('Search for agents...').fill('Technical Writer');
    const card = page.getByRole('article', { name: 'Technical Writer', exact: true });
    await card.getByRole('button', { name: 'Install Agent', exact: true }).click();
    const review = page.getByRole('region', { name: 'Review inactive installation' });
    await expect(review).toContainText('No work starts, and no tools or network permissions are granted');
    await expect(review.locator('pre')).not.toBeEmpty();
    const saving = page.waitForResponse(response => new URL(response.url()).origin === new URL(baseURL!).origin && /^\/api\/v1\/agents\/definitions\/[^/]+\/install$/.test(new URL(response.url()).pathname) && response.request().method() === 'POST')
      .then(async response => ({ status: response.status(), body: await response.json() }));
    await review.getByRole('button', { name: 'Confirm Inactive Installation', exact: true }).click();
    const receipt = await saving;
    expect(receipt.status).toBe(200);
    expect(receipt.body).toMatchObject({ success: true, status: 'installed_inactive', user_id: owner.userId, organization_id: owner.tenantId, installation: { name: 'Technical Writer', status: 'installed_inactive' } });
    await expect(card.getByRole('button', { name: 'Installed', exact: true })).toHaveAttribute('aria-pressed', 'true');
    await page.reload();
    await page.getByPlaceholder('Search for agents...').fill('Technical Writer');
    await expect(page.getByRole('article', { name: 'Technical Writer', exact: true }).getByRole('button', { name: 'Installed', exact: true })).toHaveAttribute('aria-pressed', 'true');
    const saved = await page.request.get('/api/v1/agents/definitions?q=Technical%20Writer');
    expect(saved.status()).toBe(200);
    expect((await saved.json()).installations).toContainEqual(receipt.body.installation);
    expect(executionRequests).toEqual([]);
  });

  test('rejects a member publication and retains the reviewed draft without a saved operation', async ({ memberPage: page, baseURL }) => {
    const identity = await page.request.get('/api/v1/auth/session-identity');
    expect(identity.status()).toBe(200);
    const owner = await identity.json();
    expect(owner.userId).toEqual(expect.any(String)); expect(owner.tenantId).toEqual(expect.any(String));
    const name = `Denied member publication ${crypto.randomUUID()}`;
    await page.goto('/agent-marketplace');
    await page.getByRole('link', { name: 'Publish New Agent', exact: true }).click();
    await page.getByLabel('Agent Name', { exact: true }).fill(name);
    await page.getByLabel('Description', { exact: true }).fill('Synthetic member permission test.');
    await page.getByLabel('Role', { exact: true }).fill('Draft Reviewer');
    await page.getByLabel('System Prompt', { exact: true }).fill('Keep this synthetic definition uncommitted when permission is denied.');
    await page.getByRole('button', { name: 'Review Publication', exact: true }).click();
    const saving = page.waitForResponse(response => new URL(response.url()).origin === new URL(baseURL!).origin && new URL(response.url()).pathname === '/api/v1/agents/definitions' && response.request().method() === 'POST')
      .then(async response => ({ status: response.status(), body: await response.json(), request: response.request().postDataJSON() }));
    await page.getByRole('button', { name: 'Publish Publicly', exact: true }).click();
    const receipt = await saving;
    expect(receipt.status).toBe(403);
    expect(receipt.body).toEqual({ success: false, reason: 'owner_or_admin_required' });
    await expect(page.getByLabel('Agent Name', { exact: true })).toHaveValue(name);
    await expect(page).toHaveURL(/\/agent-marketplace\/publish$/);
    const operation = await page.request.get(`/api/v1/agents/definitions/operations/${encodeURIComponent(receipt.request.request_id)}`);
    expect(operation.status()).toBe(404);
    expect(await operation.json()).toEqual({ success: false, reason: 'operation_not_found' });
    const catalogue = await page.request.get('/api/v1/agents/definitions?' + new URLSearchParams({ q: name }));
    expect(catalogue.status()).toBe(200);
    expect((await catalogue.json()).definitions).toEqual([]);
  });
});
