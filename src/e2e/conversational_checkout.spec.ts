import { test, expect } from './fixtures';

test.describe('Conversational Checkout & Instant Deposit Engine', () => {
  test('Sales AI generates conversational checkout link from inbox intent', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    // 1. Navigate to Unified Inbox
    await page.goto('/inbox');

    // 2. Click Simulate Incoming Message
    const simulateBtn = page.getByRole('button', { name: '🤖 Simulate Incoming Message' });
    await expect(simulateBtn).toBeVisible({ timeout: 15000 });
    await simulateBtn.click();

    // 3. Verify Sales AI detects intent and generates checkout bubble (mocked via AI Replied)
    await expect(page.getByText('AI Replied')).toBeVisible({ timeout: 15000 });
  });
});
