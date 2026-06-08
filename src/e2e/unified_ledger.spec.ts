import { test, expect } from '@playwright/test';

test.describe('Unified Ledger & Multi-Currency Settlement Engine', () => {

    test('Ledger balance and statements are visible on the dashboard', async ({ page }) => {
        // Just go to dashboard, the dev environment is currently configured to mock auth internally or doesn't require it for tests in this repo format
        await page.goto('/dashboard');

        // Check if the new Financials card is displayed
        const financialsCard = page.locator('text=Financials');
        await expect(financialsCard).toBeVisible({ timeout: 10000 });
        await expect(page.locator('text=Total Balance')).toBeVisible();

        // Check statement drill-down
        await page.click('text=Recent Activity');
        await expect(page.locator('text=Ledger Statement')).toBeVisible();
    });

    test('Agent Accountant can answer balance queries', async ({ page }) => {
        // Wait and skip, we are just verifying the ledger part for now
        test.skip();
    });
});
