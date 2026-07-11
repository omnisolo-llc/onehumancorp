import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('API Docs Page', () => {
  test('loads correctly and displays Swagger UI with correct styling', async ({ page }) => {
    // Intercept the API call to return a mock spec
    await page.route('/api/api-docs-spec', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          openapi: '3.0.0',
          info: { title: 'Test API', version: '1.0.0' },
          paths: {}
        }),
      });
    });

    await page.goto("/api/ui/api-docs.html");

    // Wait for the warning banner to appear
    await expect(page.locator('[data-testid="api-docs-title"]')).toBeVisible();

    // Verify the Swagger UI container renders
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 15000 });

    // Verify the title from the mock spec is displayed
    await expect(page.getByText('Test API')).toBeVisible();

    // Verify the glassmorphism styling is present and NOT overridden by solid white
    // Playwright evaluates the computed style
    const swaggerUiBg = await page.locator('.swagger-ui').evaluate((el) => {
        return window.getComputedStyle(el).backgroundColor;
    });

    // It should either be rgba(0, 0, 0, 0) or transparent, not rgb(255, 255, 255)
    expect(swaggerUiBg).not.toBe('rgb(255, 255, 255)');
  });
});
