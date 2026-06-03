import { test, expect } from '@playwright/test';

test.describe('Help Center', () => {
    test('renders help center and navigates to an article', async ({ page }) => {
        await page.goto('/help');

        // Verify Help Center title
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

        // Search for an article
        const searchInput = page.getByPlaceholder('Search for help articles and videos...');
        await searchInput.fill('Getting Started');

        // Click on the article
        const articleLink = page.locator('a[href="/help/getting-started"]');
        await expect(articleLink).toBeVisible();
        await articleLink.click();

        // Wait for navigation and API load
        await page.waitForURL('/help/getting-started');

        // Verify article content
        await expect(page.locator('h1', { hasText: 'Getting Started with Your Store' })).toBeVisible();
        await expect(page.locator('p', { hasText: 'Welcome to OneHumanCorp!' })).toBeVisible();

        // Click back button
        const backButton = page.locator('button', { hasText: 'Back to Help Center' });
        await backButton.click();

        // Verify back navigation
        await page.waitForURL('/help');
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
    });

    test('opens help chat, sends a message, and closes it', async ({ page }) => {
        // Go to dashboard to test the floating chat across major screens
        // Also add ?test_chat=true because E2E tests disable HelpChat by default in layout
        await page.goto('/dashboard?test_chat=true');

        // Locate the floating button
        const askAiButton = page.locator('button[aria-label="Open help chat"]');
        await expect(askAiButton).toBeVisible();
        await expect(askAiButton).toHaveText(/Ask anything/);

        // Click to open
        await askAiButton.click();

        // Verify chat interface
        await expect(page.locator('h3', { hasText: 'Ask AI Help' })).toBeVisible();
        await expect(page.getByText("Hi! I'm your AI Help Agent.")).toBeVisible();

        // Send a message
        const input = page.locator('input[placeholder="Ask me anything..."]');
        await input.fill('How do I reset my store?');
        await page.locator('button[aria-label="Send message"]').click();

        // User bubble
        await expect(page.getByText('How do I reset my store?')).toBeVisible();

        // Agent response (using mock logic inside component or API)
        await expect(page.getByText(/I am your AI Help Agent!/)).toBeVisible();
        await expect(page.locator('a', { hasText: 'Read the full article →' })).toBeVisible();

        // Close chat
        const closeButton = page.locator('button[aria-label="Close help chat"]');
        await closeButton.click();

        // Verify chat closed
        await expect(page.locator('h3', { hasText: 'Ask AI Help' })).not.toBeVisible();
        await expect(askAiButton).toBeVisible();
    });
});
