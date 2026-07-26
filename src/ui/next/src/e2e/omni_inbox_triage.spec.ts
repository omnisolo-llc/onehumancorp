import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary', async ({ page }) => {
    await page.goto('/inbox');
    const summaryCard = page.locator('.daily-summary');
    await expect(summaryCard).toBeVisible();
  });
});
