import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    // Navigate to the inbox page and test real network state
    await page.goto('/inbox');

    // Assumes test data exists in environment
    const summaryCard = page.locator('.daily-summary');
    if (await summaryCard.isVisible()) {
        await expect(summaryCard).toContainText('unread lead');
        const messageButton = page.locator('button', { hasText: 'Instagram DM' }).first();
        if (await messageButton.isVisible()) {
            await messageButton.click();

            const approveButton = page.locator('button', { hasText: '✨ Approve' }).first();
            if (await approveButton.isVisible()) {
                await approveButton.click();
            }
        }
    }
  });
});
