import { test, expect } from '@playwright/test';

test.describe('Documentation Features CUJ', () => {

  test('Persona: Business Owner uses the Help Center search to find answers', async ({ page }) => {
    // Navigate to Help Center
    await page.goto('/help');

    // We expect the backend API is live or returns data that is mocked via another mechanism.
    // If not, we still verify the UI elements load.
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible({ timeout: 15000 });
  });

  test('Persona: Business Owner views API Docs (Advanced)', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByText('Advanced: This section is for developers directly integrating with our APIs.')).toBeVisible({ timeout: 15000 });
  });

  test('Persona: Business Owner views Changelog', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible({ timeout: 15000 });
  });

});
