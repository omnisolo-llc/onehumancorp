import { test, expect } from '@playwright/test';

test.describe('Local-First Unified Inbox', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the unified inbox page
    await page.goto('/inbox');
  });

  test('should render the Local-First Unified Inbox page without errors', async ({ page }) => {
    // Check if the title is visible
    const title = page.locator('text=Unified Inbox');
    await expect(title).toBeVisible();

    // The sub-title should also be visible
    const subtitle = page.locator('text=Database-backed customer conversations and generated drafts.');
    await expect(subtitle).toBeVisible();

    // Assert that the page loads the local-first empty state or messages successfully
    // We expect either "Loading inbox messages from the database..." or "No inbox message rows found for this tenant." or actual rows.
    const emptyState = page.locator('text=No inbox message rows found for this tenant.').or(page.locator('text=Loading inbox messages from the database...'));
    const messageRows = page.locator('.app-list-item');

    // Either we have an empty state OR we have message rows
    await expect(emptyState.or(messageRows.first())).toBeVisible({ timeout: 10000 });
  });
});
