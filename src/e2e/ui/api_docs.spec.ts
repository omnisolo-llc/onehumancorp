import { test, expect } from '../fixtures';

test.describe('API Documentation', () => {
  test('should render Swagger UI', async ({ page }) => {
    // Navigate to the API Docs page
    await page.goto('/api/ui/api-docs.html');
    await page.waitForLoadState('networkidle');

    // Swagger UI typically renders a div with class "swagger-ui", but we are loading from unpkg,
    // so it might fail. Instead we check if the container #swagger-ui exists.
    const swaggerUIContainer = page.locator('#swagger-ui').first();
    await expect(swaggerUIContainer).toBeVisible({ timeout: 15000 });

    // Check for the title inside Swagger UI
    const apiTitle = page.getByText('OHC Advanced API Reference');
    await expect(apiTitle).toBeVisible();
  });
});
