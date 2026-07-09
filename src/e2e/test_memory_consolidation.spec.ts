import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Memory Consolidation E2E', () => {
  test('Agent remembers custom preference over time', async ({ page }) => {
    // 1. Log in
    await page.goto('/');

    // 2. Chat with agent about preference
    // Wait for the unified agent feed
    await page.waitForSelector('text=Unified Agent Feed', { state: 'visible' });

    // Send a message to set a memory context
    await page.fill('input[placeholder="Message..."]', 'My favorite cake is chocolate');
    await page.click('button:has-text("Send")');

    // Wait for agent to process and respond
    // Since this uses the builtin agent, we'll verify it appears
    await expect(page.locator('text=chocolate').first()).toBeVisible();

    // We expect the system to run consolidation in the background.
    // In a real e2e, we would reload and ask again
    await page.reload();

    await page.waitForSelector('text=Unified Agent Feed', { state: 'visible' });
    await page.fill('input[placeholder="Message..."]', 'What is my favorite cake?');
    await page.click('button:has-text("Send")');

    // Eventually the agent answers based on consolidated memory
    // Wait for some response showing "chocolate"
    await expect(page.locator('.agent-message:has-text("chocolate")').first()).toBeVisible({ timeout: 15000 });
  });
});
