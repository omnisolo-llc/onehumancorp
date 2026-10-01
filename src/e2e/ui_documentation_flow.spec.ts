import { test, expect } from './fixtures';

test.describe('Documentation Features Flow', () => {
  test('User can navigate the Help Center and view an article', async ({ page }) => {
    await page.goto('/help');
    await expect(page.getByTestId('help-center-title')).toHaveText('In-App Help Center');

    const article = page.getByRole('link').filter({
      has: page.getByRole('heading', { name: 'Getting Started with Your Store', exact: true }),
    });
    await expect(article).toHaveAttribute('href', '/help/getting-started-1');
    await article.click();

    await expect(page).toHaveURL(/\/help\/getting-started-1$/);
    await expect(page.getByRole('heading', { name: 'Getting Started with Your Store', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: '1. Tell us about your business', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: '2. Add your first product', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: '3. Start accepting payments', exact: true })).toBeVisible();
    await page.getByRole('button', { name: 'Back to Help Center', exact: true }).click();
    await expect(page).toHaveURL(/\/help$/);
    await expect(article).toBeVisible();
  });

  test('User can search the Help Center and get no results', async ({ page }) => {
    await page.goto('/help');
    const article = page.getByRole('link').filter({
      has: page.getByRole('heading', { name: 'Getting Started with Your Store', exact: true }),
    });
    await expect(article).toBeVisible();

    const query = 'NonexistentQuery1234';
    const searchResponsePromise = page.waitForResponse(response => {
      const url = new URL(response.url());
      return url.pathname === '/api/v1/help/search' && url.searchParams.get('q') === query;
    });
    const search = page.getByTestId('help-search-input');
    await search.fill(query);
    const response = await searchResponsePromise;
    expect(response.status()).toBe(200);
    expect(await response.json()).toEqual([]);
    await expect(page.getByText(/No results found matching/)).toContainText(query);
    await expect(article).not.toBeVisible();

    await search.clear();
    await expect(article).toBeVisible();
    await expect(page.getByText(/No results found matching/)).not.toBeVisible();
  });

  test('User can open the AI Help Chat widget', async ({ page }) => {
    await page.goto('/help');
    await page.getByRole('button', { name: 'Open help chat', exact: true }).click();
    const widget = page.locator('#ai-chat-interface');
    await expect(widget.getByRole('heading', { name: 'Help Center', exact: true })).toBeVisible();
    await widget.getByRole('button', { name: 'Ask AI (Ask anything)', exact: true }).click();

    const input = widget.getByPlaceholder('Ask anything...', { exact: true });
    const send = widget.getByRole('button', { name: 'Send message', exact: true });
    await expect(send).toBeDisabled();
    const question = 'How do I add a product?';
    await input.fill(question);
    await expect(send).toBeEnabled();

    // The mounted authenticated chat API answers from Help Center knowledge.
    // Require the real response and its displayed article link, not a fake bot reply.
    const replyPromise = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/chat'
      && response.request().method() === 'POST');
    await send.click();
    const response = await replyPromise;
    expect(response.request().postDataJSON()).toEqual({ message: question });
    expect(response.status()).toBe(200);
    const answer = await response.json();
    expect(typeof answer.reply).toBe('string');
    expect(answer.reply.length).toBeGreaterThan(0);
    expect(answer.link.url).toMatch(/^\/help\//);
    await expect(widget.getByText(question, { exact: true })).toBeVisible();
    await expect(widget.getByText(answer.reply, { exact: true })).toBeVisible();
    await expect(widget.getByRole('link', { name: answer.link.title, exact: true })).toHaveAttribute('href', answer.link.url);
    await expect(input).toHaveValue('');
    await expect(send).toBeDisabled();

    await widget.getByRole('button', { name: 'Clear chat', exact: true }).click();
    await expect(widget.getByText(question, { exact: true })).not.toBeVisible();
    await expect(widget.getByText(answer.reply, { exact: true })).not.toBeVisible();
    await widget.getByRole('button', { name: 'Close Help Widget', exact: true }).click();
    await expect(widget).not.toBeVisible();
  });

  test('User can access the Changelog', async ({ page }) => {
    const changelogResponsePromise = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/changelog');
    await page.goto('/changelog');
    const response = await changelogResponsePromise;
    expect(response.status()).toBe(200);
    const sections = await response.json();
    expect(sections.length).toBeGreaterThan(0);
    await expect(page.getByTestId('changelog-title')).toHaveText('Changelog Updates');
    await expect(page.getByRole('heading', { name: sections[0].version, exact: true })).toBeVisible();
  });

  test('Advanced User can access API Documentation', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByTestId('api-docs-title')).toContainText('Advanced:');
    await expect(page.getByTestId('api-docs-title')).toContainText('Not required for normal use.');
    const swagger = page.locator('.swagger-ui');
    await expect(swagger.getByRole('heading', { name: /^API Documentation \(for Advanced Users\)/ })).toBeVisible();
    await expect(swagger.locator('.opblock-summary-path').filter({ hasText: /^\/api\/v1\/help$/ })).toBeVisible();
    await expect(swagger.locator('.opblock-summary-path').filter({ hasText: /^\/api\/v1\/tooltips$/ })).toBeVisible();
  });
});
