import { test, expect } from '@playwright/test';

test.describe('API Documentation', () => {
  test('should load the Swagger UI on the api-docs page', async ({ page }) => {
    // Navigate to the API Docs page
    await page.goto('/api-docs');

    // Check for the "Advanced" warning banner
    await expect(page.getByText('Advanced: This section is for developers')).toBeVisible();

    // Check for the Swagger UI container and some basic swagger elements
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 10000 });

    // Check if the title from our spec is rendered
    await expect(page.getByText('OHC Advanced API Reference')).toBeVisible();

    // Check if at least one of our API paths is rendered
    await expect(page.getByText('/api/orgs/register')).toBeVisible();
  });

  test('should allow interacting with Swagger UI elements', async ({ page }) => {
    // E2E Test 4: API docs interaction
    await page.goto('/api-docs');

    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 10000 });

    // Verify a second endpoint is visible
    await expect(page.getByText('/api/agents/status')).toBeVisible();

    // Click on the first endpoint block to expand it
    const registerEndpoint = page.locator('.opblock-summary-path', { hasText: '/api/orgs/register' }).first();
    await registerEndpoint.click();

    // Check that the expanded section shows parameters/try it out
    await expect(page.locator('.opblock-body').getByText('Parameters', { exact: true })).toBeVisible();
  });
});
