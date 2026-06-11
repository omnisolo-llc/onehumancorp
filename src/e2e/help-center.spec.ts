import { test, expect, e2eTest } from './fixtures';

test.describe('Help Center', () => {
  e2eTest('Persona: Business Owner uses help center and chat', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard?test_chat=true');

    // Wait for page to load fully
    await page.waitForLoadState('networkidle');

    // Check if HelpChat component is accessible
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();
    await chatButton.click();
    await expect(page.locator('text=Ask AI Help').first()).toBeVisible();

    const input = page.locator('input[placeholder="Ask me anything..."]');
    await input.fill('How do I accept credit cards?');
    await page.locator('button[aria-label="Send message"]').click();

    await expect(page.locator('text=How do I accept credit cards?').first()).toBeVisible();
    await expect(page.locator('text=I am your AI Help Agent!').first()).toBeVisible();
    await expect(page.locator('text=Read the full article').first()).toBeVisible();

    await page.locator('button[aria-label="Close help chat"]').click();

    // Go to /help
    await page.goto('/help');
    await expect(page.locator('text=Help Center').first()).toBeVisible();
    await expect(page.locator('text=Getting Started').first()).toBeVisible();
    await expect(page.locator('text=My Store').first()).toBeVisible();

    await page.click('text=Getting Started');
    await expect(page).toHaveURL(/.*\/help\/getting-started/);
    await expect(page.locator('text=Getting Started with Your Store').first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Step 1: Tell us about your business' })).toBeVisible();

    await page.goto('/help');

    await page.fill('input[placeholder="Search for help articles and videos..."]', 'paid');
    await expect(page.locator('text=Getting Paid').first()).toBeVisible();
  });

  e2eTest('Persona: Business Owner views the Changelog', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.locator('text=Release Notes & Changelog').first()).toBeVisible();
    await expect(page.locator('text=Version 1.1 (Latest)').first()).toBeVisible();
    await expect(page.locator('text=New Features').first()).toBeVisible();
  });

  e2eTest('Persona: Developer views the API documentation', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.locator('text=Advanced:').first()).toBeVisible();
    await expect(page.locator('text=OHC Advanced API Reference').first()).toBeVisible();
  });

  e2eTest('Persona: Business Owner interacts with a Tooltip', async ({ page }) => {
    await page.goto('/dashboard');
    const kairosLink = page.locator('a[href="/kairos"]');
    await expect(kairosLink).toBeVisible();
    await kairosLink.hover();
    await expect(page.locator('text=Click here to see what your AI helpers are working on and how they plan.').first()).toBeVisible();
  });

  e2eTest('Persona: Business Owner navigates to KAIROS page', async ({ page }) => {
     await page.goto('/kairos');
     // Ensure page loaded
     await expect(page.getByRole('heading', { name: 'Kairos' })).toBeVisible();
  });
});

  e2eTest('Persona: Business Owner uses new help center empty state and dashboard walkthrough', async ({ page }) => {
    // Test Help Center functionality
    await page.goto('/help');
    await expect(page.locator('text=Help Center').first()).toBeVisible();

    // Search for something that does not exist
    await page.fill('input[placeholder="Search for help articles and videos..."]', 'Nonexistent search query that yields nothing');

    // Should show the empty state and the "Ask AI" button
    await expect(page.locator('text=No results found matching').first()).toBeVisible();
    const askAiBtn = page.locator('button:has-text("Ask AI Support Agent")');
    await expect(askAiBtn).toBeVisible();

    // Hover over the button to trigger the tooltip
    await askAiBtn.hover();
    // Wait for tooltip to appear
    await expect(page.locator('.ohc-tooltip')).toBeVisible();
    // Assert tooltip text is fetched from backend (or fallback)
    await expect(page.locator('.ohc-tooltip')).toHaveText('Open AI Help Chat to get answers instantly.');

    // Test Walkthrough and Tooltip in Dashboard
    await page.goto('/dashboard');
    await expect(page.locator('text=Welcome back').first()).toBeVisible();

    const walkthroughBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkthroughBtn).toBeVisible();

    // Hover to trigger tooltip
    await walkthroughBtn.hover();
    await expect(page.locator('.ohc-tooltip')).toBeVisible();
    await expect(page.locator('.ohc-tooltip')).toHaveText('Start an interactive guide to learn how to use your dashboard.');

    // Click walkthrough to verify it starts
    await walkthroughBtn.click();
    const walkthroughBubble = page.locator('.ohc-walkthrough-bubble');
    await expect(walkthroughBubble).toBeVisible();
    await expect(page.locator('.ohc-walkthrough-title').first()).toBeVisible();

    // Close walkthrough
    await page.locator('.ohc-walkthrough-close').click();
    await expect(walkthroughBubble).not.toBeVisible();
  });
