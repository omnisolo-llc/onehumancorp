import { test, expect } from '@playwright/test';
import { e2eAdmin as adminPage } from './fixtures';

test.describe('Hermes Cross-Session Recall E2E', () => {
  // Use adminPage to login first
  test.use({ storageState: 'e2e-admin-storage-state.json' });

  test('User can navigate to cross-session recall and see initial state', async ({ page }) => {
    await page.goto('/memory/cross-session');
    await expect(page.locator('h1')).toContainText('Cross-Session Recall');
    await expect(page.locator('input[placeholder*="Search past conversations"]')).toBeVisible();
    await expect(page.locator('text=Condense results')).toBeVisible();
    await expect(page.locator('button:has-text("Search Memory")')).toBeDisabled();
  });

  test('User can perform a raw FTS5 search and see results', async ({ page }) => {
    // Generate real memory by creating an assistant message via API or assuming it exists
    await page.goto('/memory/cross-session');

    // Search a known common word or seed data
    await page.fill('input[placeholder*="Search past conversations"]', 'hello');
    await expect(page.locator('button:has-text("Search Memory")')).toBeEnabled();
    await page.click('button:has-text("Search Memory")');

    // Wait for the results area
    await expect(page.locator('h2:has-text("Results")')).toBeVisible({ timeout: 10000 });
  });

  test('User can request an LLM summarized search', async ({ page }) => {
    await page.goto('/memory/cross-session');
    await page.fill('input[placeholder*="Search past conversations"]', 'hello');
    await page.check('input[type="checkbox"]');
    await page.click('button:has-text("Search Memory")');

    await expect(page.locator('h3:has-text("Condensed results")')).toBeVisible({ timeout: 20000 });
  });

  test('Empty state is displayed when no results are found', async ({ page }) => {
    await page.goto('/memory/cross-session');
    await page.fill('input[placeholder*="Search past conversations"]', 'this_will_definitely_not_be_found_12345');
    await page.click('button:has-text("Search Memory")');

    await expect(page.locator('text=No memory found for this query.')).toBeVisible({ timeout: 10000 });
  });
});
