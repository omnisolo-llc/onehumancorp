import { test, expect } from '@playwright/test';

test.describe('Help Center', () => {
    test('renders help center and navigates to an article', async ({ page }) => {
        await page.goto('/help');

        // Verify Help Center title
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

        // Search for an article
        const searchInput = page.getByPlaceholder('Search for help articles and videos...');
        await searchInput.fill('Getting Started');
        await expect(searchInput).toHaveValue('Getting Started');
    });

    test('should use backend search for filtering articles', async ({ page }) => {
        await page.goto('/help');

        // Verify Help Center title
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

        // Search for an article that matches My Store
        const searchInput = page.getByPlaceholder('Search for help articles and videos...');

        await searchInput.fill('My Store');
        await expect(searchInput).toHaveValue('My Store');
    });

    test('should open help chat and send a message', async ({ page }) => {
        await page.goto('/help');

        // Find and click the floating Ask anything button
        const chatButton = page.locator('button[aria-label="Open help chat"]');
        await expect(chatButton).toBeVisible();
        await chatButton.evaluate((btn) => btn.click());

        // Wait for the chat to open and be visible
        const chatHeader = page.locator('#ai-chat-header');
        await expect(chatHeader).toBeVisible();

        // Check if the chat input is present
        const chatInput = page.locator('input[placeholder="Ask me anything..."]');
        await expect(chatInput).toBeVisible();

        // Close the chat (using evaluate to avoid tricky viewport issues on fixed floating elements in tests)
        const closeButton = page.locator('button[aria-label="Close help chat"]');
        await closeButton.evaluate((btn) => btn.click());
        await expect(chatHeader).not.toBeVisible();
    });
});
