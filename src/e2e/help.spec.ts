import { test, expect } from './fixtures';

test.describe('Help Center Widget', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
  });

  test('should display help widget button', async ({ page }) => {
    const helpButton = page.locator('button[aria-label="Help"]');
    await expect(helpButton).toBeVisible();
  });

  test('should open help widget on click', async ({ page }) => {
    const helpButton = page.locator('button[aria-label="Help"]');
    await helpButton.click();

    await expect(page.locator('h3', { hasText: 'Help Center' })).toBeVisible();
  });
});

test.describe('Help Chat', () => {
  test('should display floating chat button', async ({ page }) => {
    await page.goto('/dashboard');
    const chatButton = page.locator('button', { hasText: 'Ask anything' });
    await expect(chatButton).toBeVisible();
  });
});

test.describe('API Documentation', () => {
  test('should load api docs page', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.locator('text=Advanced API Reference').first()).toBeVisible();
  });
});

test.describe('Changelog', () => {
  test('should load changelog page', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
  });
});
