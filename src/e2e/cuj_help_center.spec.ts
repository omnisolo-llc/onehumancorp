import { test, expect } from './fixtures';

test.describe('Help Center CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to a page that contains the Help Widget floating button.
    await page.goto('/');
  });

  test('should open help widget and see the help center link', async ({ page }) => {
    // Find the Help button (the one that opens the widget)
    // Based on `help.tsx`, the button has an aria-label "Help" or has the tooltip text
    const helpButton = page.locator('button[aria-label="Help"]');
    await helpButton.click();

    // Verify the Help widget opens by checking for tabs or text
    await expect(page.locator('#help-widget-container')).toBeVisible();

    // Switch to Articles tab if not selected (or verify it's the default)
    // The tab buttons have text 'Articles'
    const articlesTab = page.locator('button', { hasText: 'Help' }).first();
    await articlesTab.click();

    // Ensure the link to full Help Center is visible
    const fullHelpLink = page.locator('a', { hasText: 'Go to full Help Center →' });
    await expect(fullHelpLink).toBeVisible();
    await expect(fullHelpLink).toHaveAttribute('href', '/help-center');
  });

  test('should navigate to full help center and see articles', async ({ page }) => {
    await page.goto('/help-center');

    // Check if the Help Center page title is there
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // Check if articles are loaded (there should be at least one Link wrapper)
    // We expect some articles from /api/help to be rendered
    const articleLink = page.locator('a[href^="/help-center/"]').first();
    await expect(articleLink).toBeVisible();

    // Get the title of the first article
    const articleTitle = await articleLink.locator('h2').innerText();
    expect(articleTitle).toBeTruthy();
  });

  test('should view a specific help article', async ({ page }) => {
    // Go directly to the getting-started article
    await page.goto('/help-center/getting-started');

    // Wait for the article title to appear (it's fetched dynamically)
    await expect(page.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeVisible();

    // Check back button works
    const backButton = page.locator('button', { hasText: 'Back to Help Center' });
    await backButton.click();

    // We should be back on the Help Center page
    await expect(page.url()).toContain('/help-center');
  });

  test('should display floating AI chat and be interactive', async ({ page }) => {
    await page.goto('/');

    // The floating chat button has "Ask anything" text when hovered, and an aria-label "Open help chat"
    const openChatBtn = page.locator('button[aria-label="Open help chat"]');
    await openChatBtn.click();

    // The chat interface should open
    await expect(page.locator('h3:has-text("Help Agent")')).toBeVisible();

    // Should have initial agent message
    await expect(page.locator('text=Need help setting up your store')).toBeVisible();

    // Close chat
    const closeChatBtn = page.locator('button[aria-label="Close help chat"]');
    await closeChatBtn.click();

    // Chat should be hidden (the button returns to "Open help chat")
    await expect(page.locator('button[aria-label="Open help chat"]')).toBeVisible();
  });

  test('should trigger tooltip when hovering over help button', async ({ page }) => {
    await page.goto('/');
    const helpButton = page.locator('button[aria-label="Help"]');
    await expect(helpButton).toBeVisible();

    // Hover the button
    await helpButton.hover();

    // Wait for tooltip to appear
    const tooltipText = page.locator('text=Need help? Click here to access our Help Center').first();
    await expect(tooltipText).toBeVisible();
  });
});