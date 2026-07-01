import { test, expect } from './fixtures';

test.describe('Documentation Features Flow', () => {
  test('User can navigate the Help Center and view an article', async ({ page }) => {
    // Navigate directly without mocking, allowing the real backend / fallback APIs to respond.
    await page.goto('/help');

    // Help Center Index
    await expect(page).toHaveURL(/\/help/);

    // Wait until hydration finishes or layout settles before clicking
    await page.waitForLoadState('networkidle');

    // Check title using testid
    await expect(page.locator('[data-testid="help-center-title"]')).toBeVisible();

    // Click on the first article
    const articleLink = page.locator('a[href="/help/getting-started-1"]').first();
    await articleLink.click({ force: true });

    // Help Article Page
    await expect(page).toHaveURL(/\/help\/getting-started-1/, { timeout: 15000 });
  });

  test('User can search the Help Center and get no results', async ({ page }) => {
    await page.goto('/help');
    await page.waitForLoadState('networkidle');

    const searchInput = page.locator('[data-testid="help-search-input"]');
    await searchInput.fill('NonexistentQuery1234');

    // Wait for debounce and search to complete
    await page.waitForTimeout(500);

    // Verify empty state text
    await expect(page.locator('text=No results found matching')).toBeVisible();
    await expect(page.locator('text="NonexistentQuery1234"')).toBeVisible();
  });

  test('User can open the AI Help Chat widget', async ({ page }) => {
    await page.goto('/help');
    await page.waitForLoadState('networkidle');

    // The Ask anything button at the bottom right
    const aiButton = page.locator('button[aria-label="Open help chat"]');
    await expect(aiButton).toBeVisible();

    // Click it to open the chat interface
    await aiButton.click();

    // Wait for the modal/dialog to appear
    const chatModal = page.locator('#ai-chat-interface');
    await expect(chatModal).toBeVisible();

    // Type a message
    const input = page.locator('input[placeholder="Ask anything..."]');
    await input.fill('Hello AI');

    const sendBtn = page.locator('button[aria-label="Send message"]');
    await sendBtn.click();

    await expect(page.locator('text=Hello AI').first()).toBeVisible();
  });

  test('User can access the Changelog', async ({ page }) => {
    await page.goto('/changelog');
    await page.waitForLoadState('networkidle');

    // Verify title
    await expect(page.locator('[data-testid="changelog-title"]')).toBeVisible();
    await expect(page.locator('text=Release Notes & Changelog')).toBeVisible();
  });

  test('Advanced User can access API Documentation', async ({ page }) => {
    await page.goto('/api-docs');
    await page.waitForLoadState('networkidle');

    // Verify the advanced disclaimer
    await expect(page.locator('[data-testid="api-docs-title"]')).toBeVisible();
    await expect(page.locator('text=Advanced:')).toBeVisible();
  });
});
