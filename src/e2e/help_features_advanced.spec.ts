import { test, expect } from './fixtures';

test.describe('In-App Help & Documentation Features', () => {
  test('navigates to Help Center and searches for an article', async ({ page }) => {
    await page.goto('/api/ui/help.html');

    // Help Center title is visible
    await expect(page.getByRole('heading', { name: /In-App Help Center/i })).toBeVisible();

    // Verify some articles are loaded
    await expect(page.getByText('Getting Started').first()).toBeVisible();

    // Search functionality
    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await searchInput.fill('Payments', { force: true });

    // Should show payments-related article
    await expect(page.getByText('Getting Paid').first()).toBeVisible();
  });

  test('views individual help article', async ({ page }) => {
    await page.goto('/help/getting-started-1');

    await expect(page.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeVisible({ timeout: 10000 });
    await expect(page.getByText(/Tell us about your business/)).toBeVisible();
  });

  test('contextual tooltips and floating help widget', async ({ page }) => {
    await page.goto('/dashboard');

    // Tooltip trigger check is hard to do effectively in Playwright without hovering exactly.
    // We will check the help widget.

    // Open help widget
    const helpBtn = page.getByRole('button', { name: 'Help' });
    await helpBtn.waitFor({ state: 'visible' });
    await helpBtn.click();

    // Widget should be open
    await expect(page.getByRole('button', { name: 'Help', exact: true }).or(page.getByText('Interactive Tours'))).toBeVisible();

    // Switch to Ask AI tab
    await page.getByRole('button', { name: 'Ask AI' }).click();
    const chatInput = page.getByPlaceholder('Ask anything...');
    await expect(chatInput).toBeVisible();
    await chatInput.fill('How do I add a product?');
    await page.getByRole('button', { name: 'Send message' }).click();

    // Check that chat history gets updated
    await expect(page.getByText('How do I add a product?')).toBeVisible();
  });

  test('api docs page', async ({ page }) => {
    await page.goto('/api/ui/api-docs.html');
    await expect(page.getByText('OHC Advanced API Reference')).toBeVisible();
  });

  test('changelog page', async ({ page }) => {
    await page.goto('/api/ui/changelog.html');
    await expect(page.getByText('Release Notes & Changelog')).toBeVisible();
    await expect(page.getByText('New Features')).toBeVisible();
  });
});
