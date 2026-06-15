import { test, expect } from './fixtures';

test.describe('Help Center & Walkthroughs', () => {
  test('Owner can search help, read an article, and use AI chat', async ({ memberPage }) => {
    // 1. Log in as an operator (already done by the memberPage fixture)
    // 2. Navigate to Help Center page
    await memberPage.goto('/help');

    // Verify Help Center title
    await expect(memberPage.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // 3. Search for "Store"
    const searchInput = memberPage.getByPlaceholder('Search for help articles and videos...');
    await searchInput.fill('Store');

    // Wait for the debounce
    await memberPage.waitForTimeout(500);

    // 4. Click the "Getting Started" article
    await memberPage.getByRole('link', { name: /Getting Started/i }).click();

    // 5. Verify the rendered markdown text is visible on the article page
    await expect(memberPage.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeVisible();
    await expect(memberPage.getByText('Welcome to One Human Corp! This is a simple app')).toBeVisible();

    // 6. Navigate back to /help and click the floating AI Help Chat
    await memberPage.goto('/help');

    // Test the floating HelpChat component
    const askAIBtn = memberPage.getByRole('button', { name: 'Ask AI Support Agent' });

    // We will type a garbage search to reveal the "Ask AI Support Agent" button
    await searchInput.fill('xyz123nonsense');
    await memberPage.waitForTimeout(500);

    await expect(askAIBtn).toBeVisible();
    await askAIBtn.click();

    // Verify chat window opens
    const chatInput = memberPage.getByPlaceholder('Ask me anything...');
    await expect(chatInput).toBeVisible();
  });
});
