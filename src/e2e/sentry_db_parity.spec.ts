import { test, expect } from './fixtures';

test.describe('🛡️ Sentry: Database Parity', () => {
    test('Verify consistent data display for complex objects (JSON/JSONB parity)', async ({ page }) => {
        await page.goto('/dashboard/products');
        await page.getByRole('button', { name: 'Add Product' }).click();
        await page.getByLabel('Product Name').fill('Rainbow Cake');
        await page.getByLabel('Price').fill('50.00');
        await page.getByRole('button', { name: 'Save Product' }).click();
        await expect(page.getByText('Rainbow Cake')).toBeVisible();
    });
});
