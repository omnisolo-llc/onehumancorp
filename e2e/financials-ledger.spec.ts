import { test, expect } from '@playwright/test';

test.describe('Double-Entry Ledger Financials Dashboard', () => {
  test('Non-technical business owner should be able to view their ledger balance', async ({ page }) => {
    // 1. Owner navigates to the Financials Dashboard
    await page.goto('http://localhost:3000/financials');

    // 2. Expect the page title to be visible
    await expect(page.locator('h1')).toHaveText('Financials');

    // 3. Expect the "Total Balance" section to eventually load and display the balance correctly
    const balanceLocator = page.locator('[data-testid="total-balance"]');
    await expect(balanceLocator).toContainText('$150.00', { timeout: 5000 });

    // 4. Expect recent activity to be displayed
    await expect(page.locator('text=Recent Activity')).toBeVisible();
    await expect(page.locator('text=Deposit')).toBeVisible();
    await expect(page.locator('text=+$150.00')).toBeVisible();
  });
});
