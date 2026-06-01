import { test, expect } from '@playwright/test';

test.describe('Documentation Pages', () => {
  test('Help Center page loads correctly', async ({ page }) => {
    await page.goto('/help');
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
  });

  test('Changelog page loads correctly', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: /Release Notes/i })).toBeVisible();
  });

  test('API Docs page loads correctly', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByRole('heading', { name: 'API Reference' })).toBeVisible();
  });
});
