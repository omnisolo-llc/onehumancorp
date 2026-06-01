import { test, expect } from '@playwright/test';

test.describe('Documentation Features', () => {

  test('Help Center Navigation and Search', async ({ page }) => {
    await page.goto('/help');

    // Check if Help Center page renders
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

    // Verify search works
    const searchInput = page.locator('input[placeholder="Search for help articles..."]');
    await searchInput.fill('Getting Started');

    const gettingStartedArticle = page.locator('h2', { hasText: 'Getting Started' });
    await expect(gettingStartedArticle).toBeVisible();

    // Click into the article
    await gettingStartedArticle.click();

    // Verify article page content
    await expect(page.locator('h1', { hasText: 'Getting Started with Your Store' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Step 1: Tell us about your business' })).toBeVisible();
  });

  test('Contextual Tooltips on Dashboard', async ({ page }) => {
    // We navigate to the dashboard where tooltips are present
    await page.goto('/dashboard');

    // Wait for tooltip data to load via fetch simulation / rendering
    await page.waitForTimeout(1000);

    // Hover over the Team Activity header which is wrapped in a tooltip
    const teamActivityHeader = page.locator('h2', { hasText: 'Team Activity' }).first();
    await expect(teamActivityHeader).toBeVisible();

    await teamActivityHeader.hover();

    // The tooltip should appear
    const tooltipText = page.locator('text=See exactly what your AI helpers are doing right now.');
    await expect(tooltipText).toBeVisible();
  });

  test('Help Chat Widget Interaction', async ({ page }) => {
    await page.goto('/dashboard');

    // The help chat button might be the one with the star icon in the bottom right
    const openChatBtn = page.getByRole('button', { name: 'Open help chat' });
    await expect(openChatBtn).toBeVisible();
    await openChatBtn.click();

    // The chat window should be open
    await expect(page.locator('h3', { hasText: 'Help Agent' })).toBeVisible();

    // Type a message
    const inputField = page.locator('input[placeholder="Ask me anything..."]');
    await inputField.fill('How do I set up Stripe?');

    const sendBtn = page.getByRole('button', { name: 'Send message' });
    await sendBtn.click();

    // Check if user message appears
    await expect(page.locator('text=How do I set up Stripe?')).toBeVisible();
  });

  test('Interactive Walkthrough Trigger from Help Widget', async ({ page }) => {
    await page.goto('/dashboard');

    // Click the main help widget trigger
    const helpWidgetBtn = page.locator('button:has-text("Help & Guides")');
    if (await helpWidgetBtn.isVisible()) {
       await helpWidgetBtn.click();

       // Click on the interactive tour "Accept your first payment"
       const paymentTourBtn = page.locator('text=Tour: Accept your first payment');
       await expect(paymentTourBtn).toBeVisible();
       await paymentTourBtn.click();

       // Verify the URL changed or walkthrough overlay appeared
       // Because Playwright might run too fast, wait for the walkthrough bubble
       const walkthroughBubble = page.locator('text=Connect Stripe');
       await expect(walkthroughBubble).toBeVisible();
    }
  });

});
