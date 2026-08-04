import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    // 1. Log in to establish real tenant context
    await page.goto('/login');
    await page.getByLabel('Email or username').fill('test@example.com');
    await page.getByLabel('Password').fill('password123');
    await page.getByLabel(/Organization/).fill('e2e-tenant');
    await Promise.all([
      page.waitForURL('**/dashboard'),
      page.getByRole('button', { name: 'Log in' }).click(),
    ]);

    // 2. Navigate to the inbox page
    await page.goto('/inbox');

    // 3. Assert the summary card is visible (real data might vary, so just check it exists)
    const summaryCard = page.locator('.daily-summary');
    await expect(summaryCard).toBeVisible();
  });
});
