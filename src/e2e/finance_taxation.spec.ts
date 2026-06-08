import { test, expect } from '@playwright/test';
import * as path from 'path';

test.describe('Universal Embedded Finance & AI Taxation Ledger', () => {

    test('Dashboard displays Financial Health and Advisory cards', async ({ page }) => {
        const fileUrl = `file://${path.resolve('src/ui/tauri/src/ui/dashboard.html')}`;
        await page.goto(fileUrl);

        // Wait to make sure page renders completely
        await page.waitForLoadState('domcontentloaded');

        // Check if the Financial Health card is displayed
        const financialsCard = page.locator('text=Financial Health');
        await expect(financialsCard).toBeVisible({ timeout: 10000 });

        // Check for specific stats
        await expect(page.locator('text=Total Revenue')).toBeVisible();
        await expect(page.locator('text=Estimated Taxes Saved')).toBeVisible();
        await expect(page.locator('text=Available Cash')).toBeVisible();

        // Check if the Advisory Card is displayed
        const advisoryCard = page.locator('text=Finance Advisory');
        await expect(advisoryCard).toBeVisible();
        await expect(page.locator('text=You have collected $500 in sales tax this month. Move to tax savings?')).toBeVisible();

        // Interact with the Advisory Card
        const approveBtn = page.locator('button#approve-advisory-btn');
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();

        // Verify button text changes to Approved!
        await expect(page.locator('text=Approved!')).toBeVisible();

        // Verify the card disappears after a timeout
        await page.waitForTimeout(2000); // Wait longer for the 1500ms timeout in the script
        await expect(page.locator('text=You have collected $500 in sales tax this month. Move to tax savings?')).toBeHidden();
    });
});
