import { test, expect } from './fixtures';

test.describe('Help Center and Contextual Help (Tauri UI)', () => {

  test('Persona: Business Owner views the Changelog', async ({ page }) => {
    await page.goto('/api/ui/changelog.html');
    await expect(page.locator('text=Release Notes & Changelog').first()).toBeVisible();
    await expect(page.locator('text=Version 1.0 (Latest)').first()).toBeVisible();
    await expect(page.locator('text=New Features').first()).toBeVisible();
  });

  test('Persona: Developer views the API documentation', async ({ page }) => {
    await page.goto('/api/ui/api-docs.html');
    await expect(page.locator('text=Advanced:').first()).toBeVisible();
    await expect(page.locator('text=OHC Advanced API Reference').first()).toBeVisible();
  });

});
