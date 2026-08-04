import { test, expect } from '@playwright/test';

test.describe('Edge Ledger Sync Protocol', () => {
    test('should load the terminal interface without crashing', async ({ page }) => {
        // UI test to avoid fabricated payloads
        await page.goto('/login');
        await page.getByLabel('Email or username').fill('test@example.com');
        await page.getByLabel('Password').fill('password123');
        await page.getByLabel(/Organization/).fill('e2e-tenant');
        await Promise.all([
          page.waitForURL('**/dashboard'),
          page.getByRole('button', { name: 'Log in' }).click(),
        ]);

        await page.goto('/terminal');
        const heading = page.locator('h1, h2').first();
        await expect(heading).toBeVisible();
    });
});
