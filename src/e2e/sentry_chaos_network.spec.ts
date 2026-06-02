import { test, expect } from './fixtures';

test.describe('🛡️ Sentry: Chaos Network Resilience', () => {
    test('Verify UI remains functional during simulated network jitter', async ({ adminPage: page }) => {
        await page.goto('/dashboard/inventory');
        await page.route('**/api/**', async route => {
            const delay = Math.floor(Math.random() * 500);
            await new Promise(resolve => setTimeout(resolve, delay));
            await route.continue();
        });
        const productRow = page.locator('.inventory-item').first();
        await productRow.getByRole('button', { name: 'Edit' }).click();
        await page.getByLabel('Inventory Count').fill('100');
        await page.getByRole('button', { name: 'Update' }).click();
        await expect(page.getByText('Inventory updated successfully')).toBeVisible();
    });
});
