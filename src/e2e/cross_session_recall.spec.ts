import { test, expect } from '@playwright/test';
import { e2eAdmin as adminPage } from './fixtures';

test.describe('Hermes Cross-Session Recall E2E', () => {
  test('User can navigate to cross-session recall and see initial state', async ({ page }) => {
    await page.goto('/memory/cross-session');

    // Verify the page title
    await expect(page.locator('h1')).toContainText('Cross-Session Recall');

    // Verify search input exists
    await expect(page.locator('input[placeholder*="Search past conversations"]')).toBeVisible();

    // Verify summarize checkbox exists
    await expect(page.locator('text=LLM Summarize')).toBeVisible();

    // Verify search button is disabled initially
    await expect(page.locator('button:has-text("Search Memory")')).toBeDisabled();
  });

  test('User can perform a raw FTS5 search and see results', async ({ page }) => {
    // We will mock the API response for deterministic testing of the UI
    await page.route('/api/memory/cross-session', async route => {
      const json = { results: ['[Session: 123] ... customer asked for chocolate cake ...', '[Session: 456] ... prefers vegan options ...'] };
      await route.fulfill({ json });
    });

    await page.goto('/memory/cross-session');

    // Type a query
    await page.fill('input[placeholder*="Search past conversations"]', 'cake');

    // Button should now be enabled
    await expect(page.locator('button:has-text("Search Memory")')).toBeEnabled();

    // Submit
    await page.click('button:has-text("Search Memory")');

    // Check results
    await expect(page.locator('h2:has-text("Results")')).toBeVisible();
    await expect(page.locator('text=chocolate cake')).toBeVisible();
    await expect(page.locator('text=prefers vegan options')).toBeVisible();
  });

  test('User can request an LLM summarized search', async ({ page }) => {
    await page.route('/api/memory/cross-session', async route => {
      const request = route.request();
      const postData = JSON.parse(request.postData() || '{}');

      expect(postData.summarize).toBe(true);

      const json = { results: ['The customer frequently asks for chocolate and vegan cakes across multiple sessions.'] };
      await route.fulfill({ json });
    });

    await page.goto('/memory/cross-session');

    // Type query
    await page.fill('input[placeholder*="Search past conversations"]', 'cake preferences');

    // Check "Summarize"
    await page.check('input[type="checkbox"]');

    // Submit
    await page.click('button:has-text("Search Memory")');

    // Verify synthesis view
    await expect(page.locator('h3:has-text("AI Synthesis")')).toBeVisible();
    await expect(page.locator('text=The customer frequently asks')).toBeVisible();
  });

  test('Empty state is displayed when no results are found', async ({ page }) => {
    await page.route('/api/memory/cross-session', async route => {
      await route.fulfill({ json: { results: [] } });
    });

    await page.goto('/memory/cross-session');

    await page.fill('input[placeholder*="Search past conversations"]', 'nonexistent_query_123');
    await page.click('button:has-text("Search Memory")');

    await expect(page.locator('text=No memory found for this query.')).toBeVisible();
  });

  test('Error state is displayed on API failure', async ({ page }) => {
    await page.route('/api/memory/cross-session', async route => {
      await route.fulfill({ status: 500, json: { error: 'Internal Server Error' } });
    });

    await page.goto('/memory/cross-session');

    await page.fill('input[placeholder*="Search past conversations"]', 'trigger error');
    await page.click('button:has-text("Search Memory")');

    await expect(page.locator('text=Internal Server Error')).toBeVisible();
  });
});