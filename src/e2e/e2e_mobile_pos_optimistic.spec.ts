import { test, expect } from './fixtures';

test.describe('Mobile POS Optimistic Inventory Sync', () => {
  test('optimistically updates inventory count on checkout without full reload', async ({ page }) => {
    // Set viewport to mobile (375px minimum)
    await page.setViewportSize({ width: 375, height: 667 });

    // Seed test product with specific inventory
    await page.goto('/api/staff');
    await page.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Carlos', role: 'Manager', pin_hash: '1234' }]));

        // Mock a catalog item in local storage as a fallback in case network isn't used
        const catalog = [{
            id: 'prod_optimistic_test',
            title: 'Optimistic Cake',
            price_cents: 1500,
            inventory_count: 5
        }];
        localStorage.setItem('ohc_catalog_default', JSON.stringify(catalog));
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

    // Verify product shows initial inventory count "5 in stock"
    const productButton = page.locator('button:has-text("Optimistic Cake")').filter({ hasText: 'Optimistic Cake' });
    await expect(productButton).toBeVisible();
    await expect(productButton).toContainText('Stock: 5');

    // Click the product to select it
    await productButton.click();

    // Verify charge button appears
    const chargeBtn = page.locator('button:has-text("Charge")', { hasText: /Collect Payment|Charge/ });
    await expect(chargeBtn).toBeVisible({ timeout: 15000 });

    // Mock network to ensure offline handling kicks in or simulate successful tap
    // (In pos.html, going offline triggers the mock tap-to-pay flow)
    await page.context().setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Verify offline pill appears
    await expect(page.locator('text=Offline - Cash & Saved Cards Only')).toBeVisible({ timeout: 5000 });

    // Click charge (simulates tap to pay when offline)
    await chargeBtn.click();

    // Wait for the receipt screen
    await expect(page.locator('text=Offline Quick Charge Saved.')).toBeVisible({ timeout: 15000 });

    // Check if the inventory updated optimistically to 4 (without page reload)
    // The inventory is on the POS view which might be hidden by receipt.
    // Wait, posView.style.display = 'none'; happens in pos.html. Let's look at local storage.
    const finalCatalogStr = await page.evaluate(() => localStorage.getItem('ohc_catalog_default'));
    const finalCatalog = JSON.parse(finalCatalogStr || '[]');
    const product = finalCatalog.find((p: any) => p.id === 'prod_optimistic_test');

    expect(product).toBeDefined();
    expect(product.inventory_count).toBe(4);

    // Restore network
    await page.context().setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Verify offline pill disappears (or changes to syncing state before disappearing)
    // Here we just verify it doesn't say offline cash only anymore eventually.
    await expect(page.locator('text=Offline - Cash & Saved Cards Only')).toBeHidden({ timeout: 10000 });
  });
});
