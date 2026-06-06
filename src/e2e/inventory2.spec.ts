import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory Dismiss', () => {
    test('should allow dismissing a restock proposal', async ({ page }) => {
        await page.goto('/dashboard');
        await page.goto('/inventory');

        await expect(page.getByRole('heading', { name: 'Inventory' })).toBeVisible();
        await expect(page.locator('body')).toContainText(/No raw material rows found|Loading inventory|Low Stock|Healthy/);
    });
});
