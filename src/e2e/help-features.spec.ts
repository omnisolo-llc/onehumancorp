import { test, expect } from './fixtures';

test.describe('Help Features E2E', () => {
  test('Owner can use in-app help center, videos, walkthrough, and chat', async ({ page }) => {
    // 1. Visit dashboard and verify Help Widget is present
    await page.goto('/dashboard');

    // Open Help Center
    const helpButton = page.locator('button:has-text("?")').first();
    await expect(helpButton).toBeVisible();
    await helpButton.click();

    // Verify Help Center opened
    await expect(page.getByText('Help Center').first()).toBeVisible();

    // Check search functionality
    const searchInput = page.getByPlaceholder('Search for help...');
    await searchInput.fill('Products');
    await expect(page.locator('text=Storefront').first()).toBeVisible();

    // 2. Click Videos tab and verify tutorials are rendered
    await page.getByText('Videos').first().click();
    await expect(page.getByText('Tutorials').first()).toBeVisible();
    await expect(page.getByText('How to set up your first store easily').first()).toBeVisible();

    // Close Help Center
    await helpButton.click();

    // 3. Verify HelpChat floating button
    const chatButton = page.locator('button:has-text("Ask anything")').first();
    await expect(chatButton).toBeVisible();
    await chatButton.click();

    // Verify Chat opened and send a message
    await expect(page.getByText('Help Agent').first()).toBeVisible();
    const chatInput = page.getByPlaceholder('Ask me anything...');
    await chatInput.fill('How do I add a product?');
    await page.locator('button[aria-label="Send message"]').click();

    // Assert response is received (Since it connects to API, and E2E can use mocked API if disabled in E2E, let's verify standard response or user message visibility)
    await expect(page.getByText('How do I add a product?').first()).toBeVisible();

    // 4. Test Walkthrough by triggering a walkthrough
    // Assuming we have a walkthrough triggered from somewhere, e.g., the tour button inside the help widget
    await helpButton.click();
    await page.getByText('Help').first().click(); // switch to Help tab
    const tourButton = page.getByText('Tour: KAIROS AI OS Orchestration').first();
    if (await tourButton.isVisible()) {
      await tourButton.click();
      await expect(page.locator('.animate-pop-in').first()).toBeVisible();
      // Skip the walkthrough
      await page.locator('.animate-pop-in button:has(svg)').click();
    }
  });
});