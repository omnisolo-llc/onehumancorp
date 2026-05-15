import { test, expect } from '@playwright/test';

test.describe('In-App Help & Documentation System', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('should navigate to Help Center and display sections', async ({ page }) => {
    // Click the Help link in the navigation
    await page.locator('nav a:has-text("? Help")').click();

    // Check if Help Center screen is visible
    const helpCenter = page.locator('#help-center-screen');
    await expect(helpCenter).toBeVisible();

    // Check if title is correct
    await expect(helpCenter.locator('h1')).toHaveText('Help Center');

    // Check for categories
    await expect(page.locator('text="Getting Started"')).toBeVisible();
    await expect(page.locator('text="Payments & Billing"')).toBeVisible();
    await expect(page.locator('text="AI Agents"')).toBeVisible();
    await expect(page.locator('text="Video Tutorials"')).toBeVisible();
  });

  test('should open Release Notes', async ({ page }) => {
    // Navigate to Help Center first to avoid display issues
    await page.goto('/help');
    await page.evaluate(() => {
        // @ts-ignore
        window.showScreen('help-center-screen');
    });

    const whatsNewBtn = page.locator('button:has-text("What\'s New?")');
    await whatsNewBtn.click();

    const releaseNotes = page.locator('#release-notes-screen');
    await expect(releaseNotes).toBeVisible();
    await expect(releaseNotes.locator('h1')).toHaveText("What's New");
  });

  test('should open API Documentation', async ({ page }) => {
    // Navigate to Help Center first
    await page.goto('/help');
    await page.evaluate(() => {
        // @ts-ignore
        window.showScreen('help-center-screen');
    });

    const apiDocsBtn = page.locator('button:has-text("Advanced: API Docs")');
    await apiDocsBtn.click();

    const apiDocs = page.locator('#api-docs-screen');
    await expect(apiDocs).toBeVisible();
    await expect(apiDocs.locator('h1')).toHaveText("API Documentation");
  });

  test('should check tooltips on dashboard buttons', async ({ page }) => {
    // Check Inbox tooltip
    const inboxBtn = page.locator('button:has-text("Check Inbox")').first();
    await expect(inboxBtn).toHaveAttribute('data-tooltip', 'View and reply to customer messages');

    // Check My Agents tooltip
    const myAgentsBtn = page.locator('button:has-text("My Agents")').first();
    await expect(myAgentsBtn).toHaveAttribute('data-tooltip', 'Manage your AI workforce');
  });

  test('should open and interact with Help Chat Widget', async ({ page }) => {
    // Widget should be visible
    const widgetBtn = page.locator('#help-chat-widget-btn');
    await expect(widgetBtn).toBeVisible();

    // Click widget to open chat
    await widgetBtn.click();

    // Chat window should be visible
    const chatWindow = page.locator('#help-chat-window');
    await expect(chatWindow).toBeVisible();

    // Type a message
    const chatInput = page.locator('#help-chat-input');
    await chatInput.fill('How do I add a product?');

    // Click send
    const sendBtn = page.locator('#help-chat-input-area button:has-text("Send")');
    await sendBtn.click();

    // User message should appear
    await expect(page.locator('.msg-user')).toHaveText('How do I add a product?');

    // AI response should appear after timeout (waiting slightly)
    await page.waitForTimeout(1100);
    const aiMsg = page.locator('.msg-ai').nth(1); // The first one is the greeting
    await expect(aiMsg).toContainText('Here is an article that might help');
  });

  test('should start Interactive Walkthrough', async ({ page }) => {
    // Expose the function globally if not accessible from Playwright easily
    await page.evaluate(() => {
        // @ts-ignore
        window.startWalkthrough();
    });

    // Walkthrough overlay should be visible
    const overlay = page.locator('#walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('#walkthrough-bubble');
    await expect(bubble).toBeVisible();

    // Should be on Step 1
    await expect(page.locator('#wt-title')).toHaveText('Welcome!');

    // Click Next
    const nextBtn = page.locator('button:has-text("Next")');
    await nextBtn.click();

    // Should be on Step 2 (Quick Actions)
    await expect(page.locator('#wt-title')).toHaveText('Quick Actions');

    // Click Skip to close
    const skipBtn = page.locator('button:has-text("Skip")');
    await skipBtn.click();

    // Overlay should be gone
    await expect(overlay).not.toBeVisible();
  });
});
