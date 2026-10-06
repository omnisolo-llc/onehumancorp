import { test, expect } from '../fixtures';

test.describe('Documentation Features CUJ', () => {
  test('User can access the maintained Help Center', async ({ page }) => {
    await page.goto('/help');
    await expect(page.getByTestId('help-center-title')).toHaveText('In-App Help Center');
    await expect(page.getByTestId('help-search-input')).toBeVisible();
  });

  test('User can open and interact with the Help Chat widget through its visible launcher', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Open help chat', exact: true }).click();
    const widget = page.locator('#ai-chat-interface');
    await expect(widget).toBeVisible();
    await expect(widget.getByRole('button', { name: 'Help', exact: true })).toBeVisible();
    await widget.getByRole('button', { name: 'Ask AI (Ask anything)', exact: true }).click();
    await expect(widget.getByPlaceholder('Ask anything...', { exact: true })).toBeVisible();
    await expect(widget.getByRole('button', { name: 'Send message', exact: true })).toBeDisabled();
    await widget.getByRole('button', { name: 'Close Help Widget', exact: true }).click();
    await expect(widget).toBeHidden();
    await expect(page.getByRole('button', { name: 'Open help chat', exact: true })).toBeFocused();
  });

  test('User can launch a real store walkthrough and dismiss it after navigation', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Open help chat', exact: true }).click();
    const widget = page.locator('#ai-chat-interface');
    const responsePromise = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/walkthrough/store-setup');
    await widget.getByRole('button', { name: 'Tour: Set up your store', exact: true }).click();
    const response = await responsePromise;
    expect(response.status()).toBe(200);
    const steps = await response.json() as Array<{ title: string; content: string }>;
    expect(steps.length).toBeGreaterThan(0);
    await expect(page).toHaveURL(/\/storefront-builder$/);
    const bubble = page.locator('#walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText(steps[0].title);
    await expect(bubble).toContainText(steps[0].content);
    await bubble.getByRole('button', { name: 'Close walkthrough', exact: true }).click();
    await expect(bubble).toBeHidden();
  });

  test('User can view recorded Release Notes and Changelog', async ({ page }) => {
    const responsePromise = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/changelog');
    await page.goto('/changelog');
    const response = await responsePromise;
    expect(response.status()).toBe(200);
    const changes = await response.json() as Array<{ version: string }>;
    expect(changes.length).toBeGreaterThan(0);
    await expect(page.getByTestId('changelog-title')).toHaveText('Changelog Updates');
    await expect(page.getByRole('heading', { name: changes[0].version, exact: true })).toBeVisible();
  });

  test('User can read the actual API reference and its advanced-user tooltip', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByTestId('api-docs-title')).toContainText('Not required for normal use.');
    const tooltipTarget = page.locator('#api-docs-tooltip');
    await tooltipTarget.hover();
    const tooltip = page.getByRole('tooltip');
    await expect(tooltip).toHaveCount(1);
    await expect(tooltip).toHaveText('Direct API access is only for custom integrations.');
    await expect(page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.swagger-ui .opblock-summary-path').filter({ hasText: /^\/api\/v1\/help$/ })).toBeVisible();
  });

  test('User can open and close the actual video controls at a mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    const responsePromise = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/videos');
    await page.goto('/dashboard');
    const response = await responsePromise;
    expect(response.status()).toBe(200);
    const videos = await response.json() as Array<{ title: string; video_url: string }>;
    expect(videos.length).toBeGreaterThan(0);
    await page.getByRole('button', { name: 'Open help chat', exact: true }).click();
    const widget = page.locator('#ai-chat-interface');
    await widget.getByRole('button', { name: 'Videos', exact: true }).click();
    await expect(widget.getByRole('heading', { name: 'Tutorials', exact: true })).toBeVisible();
    await widget.getByText(videos[0].title, { exact: true }).click();
    const dialog = page.getByRole('dialog').filter({ has: page.getByRole('button', { name: 'Close video', exact: true }) });
    await expect(dialog).toBeVisible();
    await expect(dialog.locator('video')).toHaveAttribute('src', videos[0].video_url);
    await dialog.getByRole('button', { name: 'Close video', exact: true }).click();
    await expect(dialog).toBeHidden();
    await expect(widget).toBeVisible();
  });
});
