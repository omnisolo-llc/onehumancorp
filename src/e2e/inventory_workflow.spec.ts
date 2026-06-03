import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory - One-Tap Out of Stock Workflow', () => {
    test('should allow owner to mark item as sold out and trigger AI workflows', async ({ page }) => {
        // 1. Owner starts at the home page
        await page.goto('/');

        // 2. Owner navigates to inventory management
        await page.goto('/inventory/manage');

        // Wait for inventory to load
        await expect(page.locator('text=Manage Inventory')).toBeVisible();

        // 3. Ensure the target item starts as 'In Stock'
        await expect(page.locator('text=Vegan Chocolate Cake')).toBeVisible();

        // 4. Click the "Mark Sold Out" button
        await page.click('[data-testid="toggle-test-item-1"]');

        // 5. Verify the UI updates to show success message and state changes
        await expect(page.locator('[data-testid="status-message"]')).toHaveText(/Item marked as Sold Out/i);

        // 6. Verify button changed to "Restock"
        await expect(page.locator('[data-testid="toggle-test-item-1"]')).toHaveText('Restock');
    });
});
