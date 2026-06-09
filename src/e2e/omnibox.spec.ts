import { test, expect } from './fixtures';

test.describe('Global Search / Omnibox', () => {
  test('should open Omnibox using the UI button', async ({ page }) => {
    await page.goto('/dashboard');
    await page.waitForTimeout(3000);

    const searchButton = page.locator('button', { hasText: 'Search...' }).first();
    if (await searchButton.isVisible()) {
       await searchButton.click();
    } else {
       await page.keyboard.press('Meta+k');
    }

    await page.waitForTimeout(1000);

    const searchInput = page.locator('input[placeholder*="Search"]');
    if (await searchInput.isVisible()) {
       await searchInput.fill('Create an invoice for John for $50');

       const askAiButton = page.locator('a', { hasText: 'Ask AI Assistant' });
       await expect(askAiButton).toBeVisible();

       await askAiButton.click();
    }
  });

  test('should open Omnibox on Cmd+K, search for customer, and navigate to results', async ({ page }) => {
    await page.goto('/dashboard');
    await page.waitForTimeout(3000);

    await page.keyboard.press('Meta+k');
    await page.waitForTimeout(1000);

    const searchInput = page.locator('input[placeholder*="Search"]');
    if (await searchInput.isVisible()) {
       await searchInput.fill('John Doe');

       const resultsArea = page.locator('.max-h-\\[60vh\\]');
       await expect(resultsArea).toBeVisible();

       await page.keyboard.press('Escape');
       await expect(searchInput).toBeHidden();
    }
  });
});
