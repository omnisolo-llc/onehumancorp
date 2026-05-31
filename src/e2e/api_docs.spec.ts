import { test, expect } from './fixtures';

test.describe('API Documentation', () => {
  test('should display API docs for advanced users', async ({ page }) => {
    await page.goto('/api-docs');

    // Check for advanced warning
    await expect(page.getByText('Advanced:')).toBeVisible();

    // Check if Swagger UI loaded (at least the top level container)
    // Playwright locator will wait for it
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 10000 });
  });
});
