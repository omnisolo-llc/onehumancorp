import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory E2E', () => {
    test('should allow interaction with multiple components', async ({ page }) => {
        await page.goto('/dashboard');
        await page.goto('/inventory');

        await expect(page.getByRole('heading', { name: 'Inventory' })).toBeVisible();
        await expect(page.getByText('Vendors', { exact: true }).first()).toBeVisible();
    });
});
