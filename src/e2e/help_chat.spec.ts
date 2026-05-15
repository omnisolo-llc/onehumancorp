import { test, expect } from '@playwright/test';

// OHC End-to-End Test for Documentation Features
// As per constraints: testing must navigate through the UI as a real user, no mocking of network, starting from home page login.

test.describe('Help & Documentation Features', () => {
    test.beforeEach(async ({ page }) => {
        // Authenticate via the UI flow
        await page.goto('/login');
        await page.fill('input[type="email"]', 'test_user@example.com');
        await page.fill('input[type="password"]', 'password123');
        await page.click('button:has-text("Log in")');
        // Wait for dashboard to load
        await page.waitForSelector('text=Dashboard');
    });

    test('Video Tutorials display and function correctly', async ({ page }) => {
        // Navigate to Help Center -> Video Tutorials
        await page.click('button:has-text("Help")');
        await page.click('text=Video Tutorials');

        // Assert video items are rendered
        await expect(page.locator('text=How to add your first product')).toBeVisible();
        await expect(page.locator('text=Setting up AI Helpers')).toBeVisible();

        // Assert thumbnail and basic structure
        const videoItems = page.locator('.video-tutorial-item');
        await expect(videoItems).toHaveCount(2);
    });

    test('AI Help Chat opens and can send a message', async ({ page }) => {
        // Click the floating Ask anything or Help Chat button
        await page.click('button:has-text("Ask anything")');

        // Wait for the chat window to appear
        await expect(page.locator('.help-chat-window')).toBeVisible();

        // Type a question
        await page.fill('input[placeholder="Ask about OHC..."]', 'How do I change my store name?');
        await page.click('button:has-text("Send")');

        // Assert the user message appears
        await expect(page.locator('text=How do I change my store name?')).toBeVisible();

        // Assert an AI response appears (this relies on the real backend and potentially mock backend model responses)
        await expect(page.locator('.ai-response-message')).toBeVisible({ timeout: 10000 });
    });

    test('Interactive Walkthrough starts correctly', async ({ page }) => {
        // Open Help menu
        await page.click('button:has-text("Help")');
        await page.click('text=App Tour');

        // Assert walkthrough overlay is visible
        await expect(page.locator('.walkthrough-overlay')).toBeVisible();
        await expect(page.locator('text=Welcome to OneHumanCorp')).toBeVisible();

        // Click Next
        await page.click('button:has-text("Next")');
        // Ensure it moves to next step
        await expect(page.locator('text=This is your Dashboard')).toBeVisible();

        // Click Skip/Close
        await page.click('button:has-text("Skip")');
        await expect(page.locator('.walkthrough-overlay')).not.toBeVisible();
    });

    test('Contextual Tooltips appear on hover', async ({ page }) => {
        // Hover over a known UI element with a tooltip
        await page.hover('button:has-text("Store Settings")');

        // Assert tooltip is visible
        await expect(page.locator('.ohc-tooltip')).toBeVisible();
        await expect(page.locator('.ohc-tooltip')).toContainText('Configure your store name');
    });

    test('Release Notes / What is New loads properly', async ({ page }) => {
        // Navigate to What's New
        await page.click('button:has-text("Help")');
        await page.click('text=What\'s New');

        // Assert recent release notes are displayed
        await expect(page.locator('text=Release Notes')).toBeVisible();
        await expect(page.locator('.changelog-item').first()).toBeVisible();
    });
});
