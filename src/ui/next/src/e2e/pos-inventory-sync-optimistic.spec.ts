import { test, expect } from '@playwright/test';

test.describe('POS Inventory Sync - Real Stack', () => {
  test('POS terminal immediately updates stock UI on charge and syncs across sessions', async ({ browser }) => {
    // We launch two separate browser contexts to simulate two POS terminals (or storefront and POS)
    // using the real application stack (e2e framework)
    const context1 = await browser.newContext();
    const context2 = await browser.newContext();

    const page1 = await context1.newPage();
    const page2 = await context2.newPage();

    // Setup: Navigate to POS on both screens
    await page1.goto('http://localhost:3000/pos/terminal');
    await page2.goto('http://localhost:3000/pos/terminal');

    // Both should reach pin lock
    await expect(page1.getByText('Terminal Locked')).toBeVisible();
    await expect(page2.getByText('Terminal Locked')).toBeVisible();

    // Unlock POS 1
    await page1.getByRole('button', { name: '1', exact: true }).click();
    await page1.getByRole('button', { name: '2', exact: true }).click();
    await page1.getByRole('button', { name: '3', exact: true }).click();
    await page1.getByRole('button', { name: '4', exact: true }).click();
    await expect(page1.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 });

    // Unlock POS 2
    await page2.getByRole('button', { name: '1', exact: true }).click();
    await page2.getByRole('button', { name: '2', exact: true }).click();
    await page2.getByRole('button', { name: '3', exact: true }).click();
    await page2.getByRole('button', { name: '4', exact: true }).click();
    await expect(page2.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 });

    // Find the product and check stock
    const productLocator1 = page1.locator('button', { hasText: 'Vegan Celebration Cake' });
    const productLocator2 = page2.locator('button', { hasText: 'Vegan Celebration Cake' });

    await expect(productLocator1).toBeVisible();
    await expect(productLocator2).toBeVisible();

    const desc1 = await productLocator1.innerText();
    const stockMatch = desc1.match(/Stock: (\d+)/);
    expect(stockMatch).toBeTruthy();

    if (stockMatch) {
      const initialStock = parseInt(stockMatch[1], 10);

      // Perform optimistic action on POS 1
      await productLocator1.click();
      await page1.getByRole('button', { name: /Charge \$/ }).click();

      // Verify POS 1 updates immediately
      await expect(productLocator1).toContainText(`Stock: ${initialStock - 1}`);

      // Verify POS 2 receives sync via WebSocket and updates
      await expect(productLocator2).toContainText(`Stock: ${initialStock - 1}`);
    }

    await context1.close();
    await context2.close();
  });
});
