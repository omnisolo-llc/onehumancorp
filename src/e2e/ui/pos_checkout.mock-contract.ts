import { test, expect } from '../fixtures';

test.describe('POS Checkout - Centralized Inventory', () => {
  test('Shows out of stock message when lock fails', async ({ page }) => {
    // We mock the /api/v1/payments/terminal/reserve to simulate Redis lock failure
    await page.route('/api/v1/payments/terminal/reserve', async route => {
      const json = { success: false, error_message: 'Item is currently being checked out by another customer.' };
      await route.fulfill({ json });
    });

    // Set viewport to mobile (375px minimum)
    await page.setViewportSize({ width: 375, height: 667 });

    // Seed test product with specific inventory
    await page.goto('/api/v1/staff');
    await page.evaluate(() => {
        localStorage.setItem('omnisolo_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Carlos', role: 'Manager', pin_hash: '1234' }]));

        const catalog = [{
            id: 'prod_lock_fail_test',
            title: 'Red Dress',
            price_cents: 10000,
            inventory_count: 1
        }];
        localStorage.setItem('omnisolo_catalog_default', JSON.stringify(catalog));
    });

    // Navigate to POS terminal
    await page.goto('/pos.html');

    // Unlock terminal
    await expect(page.locator('text=Terminal Locked')).toBeVisible({ timeout: 15000 });

    // Tap pin
    await page.waitForSelector('button:has-text("1")');
    for (let i = 1; i <= 4; i++) {
        await page.getByRole('button', { name: i.toString(), exact: true }).click();
    }

    // Clock in
    await page.getByRole('button', { name: 'Clock In' }).click();

    // Verify product shows initial inventory count "1 in stock"
    const productButton = page.locator('button:has-text("Red Dress")').filter({ hasText: 'Red Dress' });
    await expect(productButton).toBeVisible();
    await expect(productButton).toContainText('Stock: 1');

    // Click the product to select it
    await productButton.click();

    // Verify charge button appears
    const chargeBtn = page.locator('button:has-text("Charge")', { hasText: /Collect Payment|Charge/ });
    await expect(chargeBtn).toBeVisible({ timeout: 15000 });

    // Click charge (this will attempt reservation)
    await chargeBtn.click();

    // We'd look for the error message
    await expect(page.locator('text=Oops! Item just sold out.')).toBeVisible({ timeout: 15000 });
  });
});
