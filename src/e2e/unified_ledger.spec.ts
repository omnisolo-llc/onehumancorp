import { test, expect } from '@playwright/test';

test.describe('Unified Ledger & Multi-Currency Settlement Engine', () => {

    test('Ledger balance and statements are visible on the dashboard', async ({ page }) => {
        await page.goto('/login');
        await page.fill('input[name="email"]', 'test@example.com');
        await page.fill('input[name="password"]', 'password123');
        await page.click('button[type="submit"]');
        await page.waitForURL('/dashboard');

        // Check if the new Financials card is displayed
        const financialsCard = page.locator('text=Financials');
        await expect(financialsCard).toBeVisible();
        await expect(page.locator('text=Total Balance')).toBeVisible();

        // Check statement drill-down
        await page.click('text=Recent Activity');
        await expect(page.locator('text=Ledger Statement')).toBeVisible();
    });

    test('Agent Accountant can answer balance queries', async ({ page }) => {
        await page.goto('/agent/chat');
        await page.fill('textarea', 'What is my current ledger balance?');
        await page.click('button[aria-label="Send message"]');

        await expect(page.locator('text=1500.00')).toBeVisible();
        await expect(page.locator('text=USD')).toBeVisible();
    });
});
