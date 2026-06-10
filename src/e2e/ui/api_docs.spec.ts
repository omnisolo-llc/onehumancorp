import { test, expect } from '@playwright/test';

test.describe('API Documentation', () => {
  test('should render Swagger UI', async ({ page }) => {
    // Navigate to the API Docs page
    await page.goto('/api-docs');

    // Wait for the Swagger UI container to become visible
    // Swagger UI typically renders a div with class "swagger-ui"
    const swaggerUIContainer = page.locator('.swagger-ui').first();
    await expect(swaggerUIContainer).toBeVisible({ timeout: 15000 });

    // Check for the title inside Swagger UI
    const apiTitle = page.getByText('OHC Advanced API Reference');
    await expect(apiTitle).toBeVisible();
  });
});
