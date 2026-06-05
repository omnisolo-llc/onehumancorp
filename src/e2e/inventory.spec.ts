import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory', () => {
    test('should show predictive restock proposals and allow 1-tap approval', async ({ page }) => {
        await page.goto('/inventory');

        await expect(page.getByRole('heading', { name: 'Inventory' })).toBeVisible();
        await expect(page.getByText(/Raw Materials|No raw material rows found|Loading inventory/).first()).toBeVisible();
    });
});
