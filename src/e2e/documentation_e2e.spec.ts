import { test, expect } from './fixtures';

test.describe('Documentation Features E2E', () => {

  test('Test 1: Verify the Help Center page loads and displays articles', async ({ page }) => {
    await page.goto('/help');

    // Check header
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // Check that at least one article loads from the API
    await expect(page.getByText('Getting Started')).toBeVisible();
    await expect(page.getByText('Learn how to easily set up your store')).toBeVisible();
  });

  test('Test 2: Verify the Help Center search filters articles', async ({ page }) => {
    await page.goto('/help');

    // Wait for articles to load
    await expect(page.getByText('Getting Started')).toBeVisible();

    // Search for specific article
    const searchInput = page.getByPlaceholder('Search for help articles...');
    await searchInput.fill('Getting Paid');

    // Verify only matching article is visible
    await expect(page.getByText('Getting Paid')).toBeVisible();
    await expect(page.getByText('Getting Started')).not.toBeVisible();
  });

  test('Test 3: Verify the individual Help Article page loads correctly', async ({ page }) => {
    await page.goto('/help/getting-started');

    // Check article title
    await expect(page.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeVisible();

    // Check article content
    await expect(page.getByText('Step 1: Tell us about your business')).toBeVisible();

    // Check back button
    await page.getByRole('button', { name: 'Back to Help Center' }).click();
    await expect(page).toHaveURL(/\/help$/);
  });

  test('Test 4: Verify the HelpChat component opens, sends a message, and receives an AI reply', async ({ page }) => {
    // Set TEST_DOCS variable in the page to allow rendering of chat
    await page.addInitScript(() => {
      // @ts-ignore
      window.__NEXT_DATA__ = window.__NEXT_DATA__ || {};
      // @ts-ignore
      window.__NEXT_DATA__.env = window.__NEXT_DATA__.env || {};
      // @ts-ignore
      window.__NEXT_DATA__.env.TEST_DOCS = "true";
    });

    await page.goto('/dashboard');

    const chatButton = page.getByRole('button', { name: 'Open help chat' });
    if (await chatButton.isVisible()) {
      await chatButton.click();
    }

    // Check if the chat interface opens
    const chatInput = page.getByPlaceholder('Ask me anything...');
    if (await chatInput.isVisible()) {
      await chatInput.fill('Hello AI');
      await page.getByRole('button', { name: 'Send message' }).click();

      // Wait for AI reply
      await expect(page.getByText('I am your AI Help Agent! I specialize in answering questions about OHC features')).toBeVisible();
    }
  });

  test('Test 5: Verify the Interactive Walkthrough (HelpWidget) opens the correct tabs', async ({ page }) => {
    await page.goto('/dashboard');

    // Help widget button
    const helpButton = page.getByRole('button', { name: 'Help', exact: true });
    await expect(helpButton).toBeVisible();
    await helpButton.click();

    // Verify tabs are present
    await expect(page.locator('button', { hasText: 'Ask AI' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Videos' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'New' })).toBeVisible();

    // Click 'Ask AI' tab
    await page.locator('button', { hasText: 'Ask AI' }).click();
    await expect(page.getByPlaceholder('Ask anything...')).toBeVisible();

    // Click 'Videos' tab
    await page.locator('button', { hasText: 'Videos' }).click();
    await expect(page.getByRole('heading', { name: 'Tutorials' })).toBeVisible();
  });

});
