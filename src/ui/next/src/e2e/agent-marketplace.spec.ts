import { test, expect } from '../../../../e2e/marketplace_fixtures';

test.describe('Agent Marketplace', () => {
  test('loads the real catalogue and distinguishes a matching search from no results', async ({ page, baseURL, marketplaceOwner }) => {
    await page.goto('/agent-marketplace');
    await expect(page.getByRole('main').getByRole('heading', { name: 'Agent Marketplace', level: 1 })).toBeVisible();
    const search = page.getByPlaceholder('Search for agents...');
    const reading = page.waitForResponse(response => {
      const url = new URL(response.url());
      return url.origin === new URL(baseURL!).origin && url.pathname === '/api/v1/agents/definitions' && url.searchParams.get('q') === 'Senior Rust Developer' && response.request().method() === 'GET';
    }).then(async response => ({ status: response.status(), body: await response.json() }));
    await search.fill('Senior Rust Developer');
    const receipt = await reading;
    expect(receipt.status).toBe(200);
    expect(receipt.body.definitions).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'Senior Rust Developer', source: 'first_party', version: 1 })]));
    expect(receipt.body.installations).toEqual([]);
    await expect(page.getByRole('article', { name: 'Senior Rust Developer' })).toBeVisible();
    await expect(page.getByRole('article', { name: 'Technical Writer' })).toHaveCount(0);
    await search.fill(`No definition ${marketplaceOwner.userId}`);
    await expect(page.getByText('No agents found.', { exact: true })).toBeVisible();
    await expect(page.getByRole('article')).toHaveCount(0);
    await expect(page.getByRole('button', { name: 'Retry marketplace' })).toHaveCount(0);
  });

  test('navigates to full-field review and persists the explicitly published definition', async ({ page, baseURL, marketplaceOwner }) => {
    await page.goto('/agent-marketplace');
    await page.getByRole('link', { name: 'Publish New Agent', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Publish New Agent', level: 1 })).toBeVisible();
    const name = `Secondary catalogue test ${marketplaceOwner.userId}`;
    const description = 'Synthetic isolated test content for public-field review.';
    const role = 'Draft Reviewer';
    const prompt = 'Review synthetic text. No execution or external access is granted.';
    await page.getByLabel('Agent Name', { exact: true }).fill(name);
    await page.getByLabel('Description', { exact: true }).fill(description);
    await page.getByLabel('Role', { exact: true }).fill(role);
    await page.getByLabel('System Prompt', { exact: true }).fill(prompt);
    await page.getByRole('button', { name: 'Review Publication', exact: true }).click();
    const review = page.getByRole('region', { name: 'Public agent definition review' });
    await expect(review).toContainText('other authenticated users of this OHC marketplace');
    for (const value of [name, description, role, prompt]) await expect(review.getByText(value, { exact: true })).toBeVisible();
    const saving = page.waitForResponse(response => new URL(response.url()).origin === new URL(baseURL!).origin && new URL(response.url()).pathname === '/api/v1/agents/definitions' && response.request().method() === 'POST')
      .then(async response => ({ status: response.status(), body: await response.json() }));
    await review.getByRole('button', { name: 'Publish Publicly', exact: true }).click();
    const receipt = await saving;
    expect(receipt.status).toBe(200);
    expect(receipt.body).toMatchObject({ success: true, status: 'published', user_id: marketplaceOwner.userId, organization_id: marketplaceOwner.tenantId, definition: { name, description, role, system_prompt: prompt, visibility: 'public', source: 'community', version: 1 } });
    expect(receipt.body.definition.digest).toMatch(/^[0-9a-f]{64}$/);
    await expect(page).toHaveURL(/\/agent-marketplace$/);
    await page.reload();
    await page.getByPlaceholder('Search for agents...').fill(name);
    await expect(page.getByRole('article', { name })).toContainText(description);
    const recovered = await page.request.get(`/api/v1/agents/definitions/operations/${encodeURIComponent(receipt.body.request_id)}`);
    expect(recovered.status()).toBe(200);
    expect(await recovered.json()).toMatchObject({ ...receipt.body, replayed: true });
  });
});
