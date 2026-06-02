import { test, expect } from '@playwright/test';

test.describe('Real Business Owner CUJ: Financial Ledger', () => {
  // Use a simulated tenant for tests
  const testTenant = 'test-tenant-' + Date.now();

  test.beforeEach(async ({ page }) => {
    // Navigate to the app root which handles auto-login in test environment
    await page.goto('/');

    // In our test environment, we might need to click "Sign in with Demo Account"
    // or wait for auto-login to complete.
    try {
        const demoBtn = page.getByRole('button', { name: /Sign in with Demo Account/i });
        if (await demoBtn.isVisible()) {
            await demoBtn.click();
        }
    } catch(e) {}

    // Wait until dashboard is visible
    await page.waitForURL('**/dashboard**');
  });

  test('Owner can view total balance and recent transactions on the financials page', async ({ page }) => {
    // 1. From the dashboard, the owner clicks on the "Financials" link
    const financialsLink = page.getByRole('link', { name: /Financials/i });
    await expect(financialsLink).toBeVisible();
    await financialsLink.click();

    // 2. The owner is navigated to the financials page
    await page.waitForURL('**/financials**');

    // 3. The owner sees the "Financials" title
    const heading = page.getByRole('heading', { name: 'Financials', level: 1 });
    await expect(heading).toBeVisible();

    // 4. The owner sees their total balance
    const totalBalanceCard = page.locator('text=Total Balance').locator('..');
    await expect(totalBalanceCard).toBeVisible();
    // Verify it contains a dollar amount
    await expect(totalBalanceCard).toContainText(/\$[0-9,]+\.[0-9]{2}/);

    // 5. The owner sees pending deposits
    const pendingDepositsCard = page.locator('text=Pending Deposits').locator('..');
    await expect(pendingDepositsCard).toBeVisible();
    await expect(pendingDepositsCard).toContainText(/\$[0-9,]+\.[0-9]{2}/);

    // 6. The owner sees the recent activity list
    const recentActivityHeader = page.getByRole('heading', { name: 'Recent Activity', level: 2 });
    await expect(recentActivityHeader).toBeVisible();

    // 7. Verify there is at least one transaction in the list
    const transactions = page.locator('li').filter({ hasText: /Payment Received|Withdrawal/i });
    await expect(transactions.first()).toBeVisible();

    // 8. Verify the transaction has an amount
    await expect(transactions.first()).toContainText(/\$[0-9,]+\.[0-9]{2}/);
  });
});
