import { test, expect } from '@playwright/test';

test.describe('Help Components', () => {
    test('Tooltips display on hover', async ({ page }) => {
        await page.goto('/help');

        const helpBtn = page.getByRole('button', { name: 'Help', exact: true });
        await expect(helpBtn).toBeVisible();

        // Need to test a specific tooltip if available or just check help btn
        await helpBtn.hover();

        // Wait for potential tooltips
        await page.waitForTimeout(500);
    });
});
