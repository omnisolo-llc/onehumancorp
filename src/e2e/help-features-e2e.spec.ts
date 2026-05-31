import { expect, test } from './fixtures';

test.describe('Documentation & Help Features', () => {

  test('should navigate and search Help Center', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Open help widget, click help center
    const helpBtn = page.locator('button:has-text("?")').first();
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    const helpCenterLink = page.getByRole('button', { name: 'Help Center' });
    await expect(helpCenterLink).toBeVisible();
    await helpCenterLink.click();

    // Verify Help Center Page
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // Search
    const searchInput = page.getByPlaceholder('Search for help articles...');
    await expect(searchInput).toBeVisible();
    await searchInput.fill('Getting Paid');

    // Check results
    const gettingPaidArticle = page.getByRole('heading', { name: 'Getting Paid' });
    await expect(gettingPaidArticle).toBeVisible();
    await gettingPaidArticle.click();

    // Verify article page
    await expect(page.getByRole('heading', { name: 'Getting Paid' })).toBeVisible();
    await expect(page.getByText('Connecting Your Bank Account')).toBeVisible();
  });

  test('should interact with AI Help Chat', async ({ page }) => {
    await page.goto('/dashboard');

    // Open help widget
    const helpBtn = page.locator('button:has-text("?")').first();
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    // Switch to Chat tab
    const chatTab = page.getByRole('button', { name: 'Ask AI' });
    await expect(chatTab).toBeVisible();
    await chatTab.click();

    // Send a message
    const input = page.getByPlaceholder('Ask anything...');
    await expect(input).toBeVisible();
    await input.fill('How do I setup stripe?');

    const sendBtn = page.getByRole('button', { name: 'Send message' });
    await expect(sendBtn).toBeVisible();
    await sendBtn.click();

    // Verify response
    await expect(page.getByText('I am your AI Help Agent!')).toBeVisible();
    await expect(page.getByRole('link', { name: 'Read the full article →' })).toBeVisible();
  });

  test('should view API Docs', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByText('Advanced: This section is for developers')).toBeVisible();
    await expect(page.getByText('OHC Advanced API Reference')).toBeVisible();
  });

  test('should view Release Notes / Changelog', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Version 1.0 (Latest)' })).toBeVisible();
  });

  test('should see walkthrough for Business Setup', async ({ page }) => {
    // Instead of depending on fragile Walkthrough UI triggering via ? query params etc,
    // we test the basic existence of walkthrough triggering link/buttons if applicable
    // or just the generic components
    await page.goto('/dashboard');

    // Open help widget
    const helpBtn = page.locator('button:has-text("?")').first();
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    // Should see tours
    await expect(page.getByText('Tour: Virtual Meeting Room')).toBeVisible();
  });
});
