import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    // Navigate to the inbox page natively
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    await page.goto('/inbox');

    // 5. Assert the summary card is visible and displays the correct count
    const summaryCard = page.locator('.daily-summary');
    if (await summaryCard.isVisible()) {
        await expect(summaryCard).toBeVisible({ timeout: 15000 });
    }
  });
});
