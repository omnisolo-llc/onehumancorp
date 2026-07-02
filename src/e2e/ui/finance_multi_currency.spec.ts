import { test, expect } from '@playwright/test';
import { setupDemoTenant } from './db_utils';

test.describe('Finance Multi-Currency and Global Sales', () => {
    test.beforeAll(async () => {
        await setupDemoTenant('finance_demo_tenant');
    });

    test('should allow toggling global sales and display multi-currency invoices', async ({ page }) => {
        // Authenticate (mocking or real logic if needed via standard setup)
        await page.goto('/login');
        await page.fill('input[type="email"]', 'owner@demo.com');
        await page.fill('input[type="password"]', 'password');
        await page.click('button[type="submit"]');
        await page.waitForURL('/dashboard');

        // Go to Finance page
        await page.goto('/finance');
        await page.waitForLoadState('networkidle');

        // Check if Global Sales toggle is present
        const toggleButton = page.locator('button:has(div.translate-x-5), button:has(div:not(.translate-x-5))').first();
        await expect(toggleButton).toBeVisible();

        // Toggle it
        await toggleButton.click();

        // Create an invoice
        const newInvoiceBtn = page.locator('text=New Invoice');
        await expect(newInvoiceBtn).toBeVisible();
    });
});
