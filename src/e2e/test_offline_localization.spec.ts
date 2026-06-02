import { test, expect } from '@playwright/test';

test.describe('Offline Localization CUJ', () => {
    test('Fatima can switch the language to Arabic while offline', async ({ page }) => {
        await page.goto('/checkout');

        await page.context().setOffline(true);

        const langSelect = page.locator('select').nth(0);
        await langSelect.selectOption('ar');

        await expect(page.getByText('الدفع')).toBeVisible();
        await expect(page.getByText('ادفع الآن')).toBeVisible();
        await expect(page.getByText('إلغاء')).toBeVisible();

        await page.context().setOffline(false);
    });
});
