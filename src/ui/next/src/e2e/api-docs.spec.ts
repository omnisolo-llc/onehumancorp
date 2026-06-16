import { test, expect } from '@playwright/test';

test.describe('API Documentation', () => {
  test('should load the Swagger UI on the api-docs page', async ({ page }) => {
    // Navigate to the API Docs page
    await page.goto('/api-docs');

    // Check for the "Advanced" warning banner
    await expect(page.getByText('This section is for developers directly integrating')).toBeAttached({ timeout: 15000 });

    // Check for the Swagger UI container and some basic swagger elements
    await expect(page.locator('.swagger-ui')).toBeAttached({ timeout: 10000 });
  });
});
