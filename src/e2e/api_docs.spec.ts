import { test, expect } from './fixtures';

test.describe('API Documentation for Advanced Users', () => {
  test('should display Swagger UI with OHC Advanced API Reference', async ({ page }) => {
    await page.goto('/api-docs');

    // Wait for Swagger UI to load
    await expect(page.locator('.swagger-ui')).toBeVisible();

    // Verify the title exists
    await expect(page.locator('text=OHC Advanced API Reference')).toBeVisible();

    // Verify at least one API endpoint is documented
    await expect(page.locator('.opblock-summary-path').filter({ hasText: '/api/orgs/register' })).toBeVisible();

    // Verify the premium UI wrapper elements
    await expect(page.locator('h1:has-text("OHC API Explorer")')).toBeVisible();
    await expect(page.locator('text=This section is for developers directly integrating with our APIs.')).toBeVisible();
  });
});
