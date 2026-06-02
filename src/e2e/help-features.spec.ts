import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('help-features');
import { test, expect } from './fixtures';

test.describe('Documentation & Help Center CUJ', () => {
  test('Persona: Business Owner uses help center and chat', async ({ page }) => {
    // Navigate to login
    await page.goto('/login');

    // Fill in login credentials and submit
    await page.getByPlaceholder('Email or Username').fill('maya@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();

    // Wait for dashboard to load fully
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
    await page.waitForLoadState('networkidle');

    // Check if HelpChat component is accessible via the Ask AI button
    const askAiButton = page.getByRole('button', { name: '✨ Ask anything' });
    await expect(askAiButton).toBeVisible();
    await askAiButton.click();

    // Verify AI Help Chat floating widget is visible
    const chatWidget = page.locator('#ai-chat-widget');
    await expect(chatWidget).toBeVisible();

    // Fill in a question
    const input = page.locator('#ai-chat-input');
    await input.fill('How do I accept credit cards?');
    await page.getByRole('button', { name: 'Send' }).click();

    // Verify chat message displays
    await expect(page.locator('.chat-msg.user', { hasText: 'How do I accept credit cards?' }).first()).toBeVisible();

    // Close the chat
    await page.locator('#ai-chat-widget').locator('text=✕').click();
    await expect(chatWidget).not.toBeVisible();

    // Go to /help using global help btn
    const helpBtn = page.locator('#global-help-btn');
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    // Verify Help Center screen is visible
    const helpScreen = page.locator('#help-screen');
    await expect(helpScreen).toBeVisible();
    await expect(helpScreen.getByRole('heading', { name: 'Help Center' })).toBeVisible();
    await expect(helpScreen.getByText('Getting Started')).toBeVisible();

    // Search functionality
    const searchInput = page.locator('#help-search-input');
    await searchInput.fill('payments');
    await page.getByRole('button', { name: 'Search' }).click();

    // Should display the article in search results
    const resultsContainer = page.locator('#help-search-results');
    await expect(resultsContainer).toBeVisible();

    // Navigate to /changelog via the nav link
    const changelogNav = page.locator('#nav-changelog');
    await expect(changelogNav).toBeVisible();
    await changelogNav.click();

    const changelogScreen = page.locator('#changelog-screen');
    await expect(changelogScreen).toBeVisible();
    await expect(changelogScreen.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
  });
});
