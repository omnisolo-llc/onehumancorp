import { test, expect } from './fixtures';

test.describe('Help Center Page', () => {
  test('should load help center and navigate to article', async ({ page }) => {
    await page.goto('/help');

    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    await expect(page.locator('h2:has-text("Getting Started")')).toBeVisible();

    await page.locator('h2:has-text("Getting Started")').click();

    await expect(page.getByRole('heading', { name: 'Getting Started' })).toBeVisible();

    await expect(page.locator('text=Welcome to OneHumanCorp!')).toBeVisible();
  });
});

test.describe('API Documentation', () => {
  test('should load Swagger UI', async ({ page }) => {
    await page.goto('/api-docs');

    await page.waitForLoadState('domcontentloaded');

    await expect(page.locator('.swagger-ui')).toBeVisible();

    await expect(page.locator('text=OHC Advanced API Reference').first()).toBeVisible();

    await expect(page.locator('text=This section is for developers directly integrating with our APIs')).toBeVisible();
  });
});

test.describe('Release Notes and Changelog', () => {
  test('should load changelog page', async ({ page }) => {
    await page.goto('/changelog');

    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Version 1.0 (Latest)' })).toBeVisible();
    await expect(page.locator('text=Interactive AI Store Builder:')).toBeVisible();
  });
});
