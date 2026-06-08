import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('autonomous_inventory_crm_pricing');

test.describe('Autonomous Inventory, CRM, and Pricing Synchronization', () => {
  test('should display predictive restock and price adjustments in the agent feed and allow approval', async ({ page }) => {
    // Navigate to the Team page where the Agent Feed is displayed
    await page.goto('/team');

    // Verify the page loaded
    await expect(page.locator('h1', { hasText: 'Your Team' })).toBeVisible();

    // Check that the Agent Feed section is present
    await expect(page.locator('h2', { hasText: 'Agent Feed' })).toBeVisible();

    // Verify the pending action card appears
    const actionCard = page.getByTestId('agent-action-card');
    await expect(actionCard).toBeVisible();

    // Verify the card content correctly describes the restock and price adjust
    await expect(actionCard).toContainText('Needs Approval');
    await expect(actionCard).toContainText('BusinessAdvisory');
    await expect(actionCard).toContainText('Product e2e-product-out-of-stock is out of stock. Suggest reordering 50 units and adjusting price.');

    // Approve the action
    const approveBtn = page.getByTestId('approve-action-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // The action card should disappear from the pending feed
    await expect(actionCard).not.toBeVisible();

    // Now navigate to products to verify the price was actually updated in the backend
    await page.goto('/products');

    // Wait for product list to load
    await expect(page.getByRole('heading', { name: 'Products & Services' })).toBeVisible();

    // The price should be updated to 46.00 (4600 cents) from the original 40.00
    // "Sold Out Donut" row should have the new price
    const productRow = page.locator('div').filter({ hasText: 'Sold Out Donut' }).last();
    await expect(productRow).toContainText('$46.00');
  });
});
