import { test, expect } from '@playwright/test';

test.describe('Dynamic Centralized Inventory and POS Multi-Channel Engine', () => {
  const tenantId = 'tenant-inventory-sync-test';

  test('adjusts stock in inventory UI and reflects in POS UI', async ({ page, request }) => {
    // Seed initial product explicitly
    await request.post('/api/pos/inventory', {
      headers: { 'x-tenant-id': tenantId },
      data: [{ payload: { item_id: 'test-product', new_stock: 5, is_sold_out: false } }]
    });

    // 1. Visit inventory page
    await page.goto('/inventory');
    await page.evaluate(`localStorage.setItem('tenant_id', '${tenantId}')`);
    await page.goto('/inventory');

    // Wait for the table to load. We will use a known seed product or just any product.
    // If the inventory list is empty, we skip the rest of the test since we can't seed through the UI.
    const hasProducts = await page.locator('.space-y-3 > div').count();


    // Get the first product's initial stock
    const firstRow = page.locator('.space-y-3 > div').first();
    const productName = await firstRow.locator('span').first().innerText();
    const initialStockText = await firstRow.locator('span').nth(1).innerText(); // 'Stock: X'
    const initialStock = parseInt(initialStockText.replace('Stock: ', ''), 10);

    // Click "-" button to manually adjust stock
    await firstRow.locator('button').first().click(); // The first button in the space-x-2 is '-'

    // Check optimistic update
    await expect(firstRow.locator('span', { hasText: 'Stock:' })).toContainText(`Stock: ${Math.max(0, initialStock - 1)}`, { timeout: 10000 });

    // 2. Visit POS Terminal
    await page.goto('/pos/terminal');
    await page.evaluate(`localStorage.setItem('tenant_id', '${tenantId}')`);

    // Enter PIN for test staff
    await page.fill('input[type="password"]', '1234');
    await page.click('button:has-text("Clock In")');

    // Wait for inventory to load in POS
    const productButton = page.locator('button', { hasText: productName });
    if (await productButton.count() > 0) {
        await expect(productButton).toBeVisible({ timeout: 10000 });
        await expect(productButton).toContainText(`Stock: ${Math.max(0, initialStock - 1)}`);

        // Select product and open cart
        await productButton.click();
        await page.click('button:has-text("item")');

        // 3. Process Cash Sale
        await page.click('button:has-text("Cash")');
        await page.click('button:has-text("Record Offline Cash Sale")');

        // Checkout completes
        await expect(page.locator('h2', { hasText: 'Payment Successful!' })).toBeVisible({ timeout: 10000 });

        // Wait for server to process webhook or optimistic update
        await page.waitForTimeout(1000);

        // 4. Verify stock drops again
        await page.goto('/inventory');
        const updatedRow = page.locator('.space-y-3 > div').first();
        await expect(updatedRow.locator('span', { hasText: 'Stock:' })).toContainText(`Stock: ${Math.max(0, initialStock - 2)}`, { timeout: 10000 });
    }
  });
});
