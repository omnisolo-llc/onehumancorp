import { test, expect } from '../../../../e2e/fixtures';

test.describe('API Documentation', () => {
  test('should load the Swagger UI on the api-docs page', async ({ page }) => {
    // Navigate to the API Docs page using next.js app routing
    await page.goto('/api-docs');

    // Wait for the specific data test id to be attached to the dom
    await page.waitForSelector('[data-testid="api-docs-title"]', { state: 'attached' });

    // Ensure the title block exists and is attached
    await expect(page.getByTestId('api-docs-title')).toBeAttached({ timeout: 15000 });

    // Check for the Swagger UI container and some basic swagger elements
    // We expect the swagger-ui container to eventually be attached when data finishes fetching
    await expect(page.locator('.swagger-ui')).toBeAttached({ timeout: 15000 });
  });
});
