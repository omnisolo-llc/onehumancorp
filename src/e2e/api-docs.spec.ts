import { test, expect } from './fixtures';

test.describe('API Docs Page', () => {
  test('should display API Docs', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByText('API Reference for advanced users integrating with OneHumanCorp.')).toBeVisible();
    await expect(page.getByText('Local Backend Server')).toBeVisible();
  });
});
