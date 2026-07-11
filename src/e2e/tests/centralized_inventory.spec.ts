import { test, expect } from '@playwright/test';

test.describe('Centralized Inventory Management', () => {
  test('Owner can view centralized inventory and manually adjust stock', async ({ page }) => {
    // Navigate to the inventory page directly (assuming basic auth setup or test environment bypassing login)
    // Wait for the UI to load
    await page.goto('/inventory');

    // Verify page structure
    await expect(page.locator('text="Inventory"').first()).toBeVisible();
    await expect(page.locator('text="Centralized dynamic inventory tracking"')).toBeVisible();

    // The backend should return the test product created in DB migrations/seeds
    // Wait for row
    const firstRow = page.locator('tr[data-testid^="inventory-row-"]').first();
    await expect(firstRow).toBeVisible({ timeout: 10000 });

    // Extract the variant ID from the data-testid
    const rowId = await firstRow.getAttribute('data-testid');
    const variantId = rowId?.replace('inventory-row-', '');
    expect(variantId).toBeTruthy();

    const stockCell = page.locator(`[data-testid="stock-count-${variantId}"]`);
    const initialStockStr = await stockCell.textContent();
    const initialStock = parseInt(initialStockStr || '0', 10);

    // Click to decrease stock
    const decreaseBtn = page.locator(`[data-testid="adjust-dec-${variantId}"]`);
    await decreaseBtn.click();

    // Verify optimistic update
    await expect(stockCell).toHaveText(String(initialStock - 1));

    // Reload page to verify persistence
    await page.reload();
    const reloadedStockCell = page.locator(`[data-testid="stock-count-${variantId}"]`);
    await expect(reloadedStockCell).toHaveText(String(initialStock - 1));

    // Increase stock back
    const increaseBtn = page.locator(`[data-testid="adjust-inc-${variantId}"]`);
    await increaseBtn.click();
    await expect(reloadedStockCell).toHaveText(String(initialStock));
  });
});
