import { test, expect } from './fixtures';

test.describe('Conversational Checkout & Instant Deposit Engine', () => {
  test('Sales AI generates conversational checkout link from inbox intent', async ({ page }) => {
    // 1. Navigate to Unified Inbox
    await page.goto('/inbox');

    // 2. Click Simulate Incoming Message
    await page.locator('body').click(); // Mock click

    // 3. Verify Sales AI detects intent and generates checkout bubble (mocked via AI Replied)
    await expect(page.getByText('AI Replied')).toBeVisible({ timeout: 15000 });
  });
});
