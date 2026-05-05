import { test, expect } from '@playwright/test';

test.describe('Documentation Features', () => {

  test.beforeEach(async ({ page }) => {
    // Start from the home page and login
    await page.goto('/');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');
  });

  test('should display contextual tooltips on hover', async ({ page }) => {
    await page.goto('/dashboard');
    // Hover over a known UI element with a tooltip
    await page.hover('text=Analytics');
    // Verify tooltip visibility
    await expect(page.locator('.tooltip')).toBeVisible();
    await expect(page.locator('.tooltip')).toContainText('View your business metrics');
  });

  test('should open Help Center and verify sections', async ({ page }) => {
    await page.goto('/dashboard');
    await page.click('button:has-text("?")'); // The help button

    const helpCenter = page.locator('.help-center-modal');
    await expect(helpCenter).toBeVisible();

    await expect(helpCenter.locator('text=Getting Started')).toBeVisible();
    await expect(helpCenter.locator('text=My Store')).toBeVisible();
    await expect(helpCenter.locator('text=Payments')).toBeVisible();
    await expect(helpCenter.locator('text=AI Agents')).toBeVisible();
  });

  test('should verify AI-Powered Help Chat functionality', async ({ page }) => {
    await page.goto('/dashboard');
    await page.click('button:has-text("Ask anything")');

    const chatModal = page.locator('.help-chat-modal');
    await expect(chatModal).toBeVisible();

    await chatModal.locator('input[type="text"]').fill('How do I set up payments?');
    await chatModal.locator('button:has-text("Send")').click();

    // Verify AI response simulation
    await expect(chatModal.locator('.ai-message').first()).toBeVisible();
  });

  test('should verify Interactive Walkthroughs trigger', async ({ page }) => {
    await page.goto('/dashboard');
    await page.click('button:has-text("Start Tour")');

    const walkthroughBubble = page.locator('.walkthrough-bubble');
    await expect(walkthroughBubble).toBeVisible();
    await expect(walkthroughBubble).toContainText('Welcome to your OHC dashboard!');
  });

  test('should display embedded Video Tutorials', async ({ page }) => {
    await page.goto('/help/tutorials');

    const videoPlayer = page.locator('video').first();
    await expect(videoPlayer).toBeVisible();
  });

  test('should verify API Documentation visibility for advanced settings', async ({ page }) => {
    await page.goto('/settings');
    await page.click('button:has-text("Advanced Options")');
    await page.click('text=API Reference');

    await expect(page.locator('text=OpenAPI Specification')).toBeVisible();
  });

});
