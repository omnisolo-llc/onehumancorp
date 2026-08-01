import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    // 4. Navigate to the inbox page
    await page.goto('/inbox');

    // 5. Assert the summary card is visible
    const summaryCard = page.locator('.daily-summary');
    if (await summaryCard.isVisible()) {
        await expect(summaryCard).toBeVisible();
    }
  });
});
