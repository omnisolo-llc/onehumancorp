import { expect, test } from './fixtures';

test.describe('Documentation Critical Features (Help Center, Tooltips, Walkthrough, Chat)', () => {
  test('Help Center displays searchable articles and video tutorials', async ({ page }) => {
    // Navigate to the Help Center page
    await page.goto('/help');

    // Verify the Help Center header
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();

    // Verify an article from the API appears
    await expect(page.locator('text=Getting Started')).toBeVisible();

    // Test the search functionality
    const searchInput = page.getByPlaceholder('Search for help articles...');
    await searchInput.fill('Finding Customers');

    // "Getting Started" should disappear
    await expect(page.locator('text=Getting Started')).not.toBeVisible();
    // "Finding Customers" should be visible
    await expect(page.locator('text=Finding Customers')).toBeVisible();
  });

  test('Contextual Tooltip displays correctly on hover', async ({ page }) => {
    // Navigate to a page with a tooltip, like the dashboard
    await page.goto('/dashboard');

    // Ensure the tooltip registry has time to load
    await page.waitForTimeout(1000);

    // Hover over the target element to trigger the tooltip
    // We target the Team Activity header which is wrapped in a WithTooltip
    const teamActivityHeader = page.locator('text=Team Activity');
    await expect(teamActivityHeader).toBeVisible();
    await teamActivityHeader.hover();

    // Verify the tooltip text appears (this uses the floating div with z-[100])
    await expect(page.locator('text=Monitor the real-time actions and tasks being performed by your AI workforce.')).toBeVisible();
  });

  test('Help Chat button and interface are operational', async ({ page }) => {
    // Navigate to any page to see the global HelpChat widget
    await page.goto('/dashboard');

    // Find and click the "Ask anything" floating button
    const askButton = page.locator('button:has-text("Ask anything")');
    await expect(askButton).toBeVisible();
    await askButton.click();

    // Verify the chat interface opens with the welcome message
    await expect(page.locator('text=Always here to help')).toBeVisible();
    await expect(page.locator('text=Need help setting up your store')).toBeVisible();

    // Enter a message and send
    const chatInput = page.getByPlaceholder('Ask me anything...');
    await chatInput.fill('How do I add a product?');
    const sendButton = page.locator('button[aria-label="Send message"]');
    await sendButton.click();

    // Wait for the AI's response (mocked or real)
    await expect(page.locator('text=I am your AI Help Agent!')).toBeVisible();
  });

  test('Release Notes & Changelog is accessible', async ({ page }) => {
    // Navigate to the Changelog
    await page.goto('/changelog');

    // Verify the Changelog page renders
    await expect(page.locator('h1:has-text("Release Notes & Changelog")')).toBeVisible();
    await expect(page.locator('text=New AI Store Builder')).toBeVisible();
  });
});
