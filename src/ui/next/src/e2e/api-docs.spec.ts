import { test, expect } from '@playwright/test';

test.describe('API Documentation', () => {
  test('should load the Swagger UI on the api-docs page', async ({ page }) => {
    // Navigate to the API Docs page
    await page.goto('/api-docs');
    await expect(page.getByText('Advanced:')).toBeVisible({ timeout: 15000 });
  });
});
