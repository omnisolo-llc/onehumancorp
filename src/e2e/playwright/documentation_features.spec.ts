import { test, expect } from '../fixtures';

test.describe('Documentation Features CUJ', () => {
  test('User can access help center via the main navigation', async ({ page }) => {
    await page.goto('/help');
    await expect(page.locator('h1', { hasText: 'In-App Help Center' })).toBeVisible();
  });

  test('User can open and interact with the AI Help Chat widget', async ({ page }) => {
    await page.goto('/dashboard');
    await page.evaluate(() => {
        const event = new CustomEvent('open-help-chat');
        window.dispatchEvent(event);
    });

    const chatInterface = page.locator('#ai-chat-interface');
    await expect(chatInterface).toBeVisible();
    await expect(chatInterface.locator('text=Articles')).toBeVisible();
    await expect(chatInterface.locator('text=Ask AI')).toBeVisible();
    await chatInterface.locator('button[data-target="tab-chat"]').click();
    const chatInput = page.locator('#ohc-help-chat-input');
    await expect(chatInput).toBeVisible();
  });

  test('User can launch an interactive walkthrough from the help widget', async ({ page }) => {
    await page.goto('/dashboard');
    await page.evaluate(() => {
        window.dispatchEvent(new CustomEvent('open-help-chat'));
    });

    const chatInterface = page.locator('#ai-chat-interface');
    await expect(chatInterface).toBeVisible();
    await chatInterface.locator('button[data-target="tab-tours"]').click();
    await chatInterface.locator('.ohc-tour-card').first().click();
    await expect(page.locator('#walkthrough-bubble')).toBeVisible();
    await page.locator('#wt-close').click();
    await expect(page.locator('#walkthrough-bubble')).not.toBeVisible();
  });

  test('User can view Release Notes and Changelog', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.locator('h1')).toContainText('Release Notes & Changelog');
  });

  test('User can navigate to Advanced API Docs and see tooltip on header', async ({ page }) => {
    // Intercept network request to API spec to avoid breaking due to no backend
    await page.route('/api/v1/api-docs-spec', async (route) => {
      const json = {
        openapi: "3.0.0",
        info: {
          title: "API Documentation (for Advanced Users)",
          version: "1.0.0"
        },
        paths: {}
      };
      await route.fulfill({ json });
    });

    await page.goto('/api-docs');
    await expect(page.locator('h1')).toContainText('OHC Advanced API Reference');

    // Wait for the Swagger UI to be rendered and check for the correct mock title
    await expect(page.locator('.swagger-ui')).toBeVisible();
    await expect(page.locator('.swagger-ui .title')).toContainText('API Documentation (for Advanced Users)');

    const tooltipTarget = page.locator('#api-docs-tooltip');
    await expect(tooltipTarget).toBeVisible();
    await tooltipTarget.hover();
    const globalTooltip = page.locator('.ohc-tooltip');
    await expect(globalTooltip).toHaveClass(/visible/);
    await expect(globalTooltip).toContainText('Direct API access is only for custom integrations.');
  });

  test('User can view mobile-optimized help videos in widget', async ({ page }) => {
    await page.goto('/dashboard');
    await page.setViewportSize({ width: 375, height: 667 });
    await page.evaluate(() => {
        window.dispatchEvent(new CustomEvent('open-help-chat'));
    });

    const chatInterface = page.locator('#ai-chat-interface');
    await expect(chatInterface).toBeVisible();
    await chatInterface.locator('button[data-target="tab-videos"]').click();
    await expect(page.locator('#video-list')).toBeVisible();
  });
});
