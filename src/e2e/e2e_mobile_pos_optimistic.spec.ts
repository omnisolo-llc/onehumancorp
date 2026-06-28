import { test, expect } from './fixtures';
import { loginAs } from './utils/auth';
import { adminUser } from './utils/test-data';

test.describe('Mobile POS Optimistic Inventory Sync', () => {
  test('optimistically updates inventory count on checkout without full reload', async ({ page }) => {
    // Set viewport to mobile (375px minimum)
    await page.setViewportSize({ width: 375, height: 667 });

    // Login correctly to establish session and tenant context
    await loginAs(page, adminUser);

    // Wait for network/session initialization before writing to localStorage
    await page.waitForLoadState('networkidle');

    // Seed test product with specific inventory into the correct tenant cache
    await page.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Carlos', role: 'Manager', pin_hash: '1234' }]));

        // Use the e2e-tenant key which will be picked up by pos.html
        const catalog = [{
            id: 'e2e-product-pos-sync',
            title: 'Optimistic Cake',
            price_cents: 1500,
            inventory_count: 5
        }];
        localStorage.setItem('ohc_catalog_e2e-tenant', JSON.stringify(catalog));
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
    const productButton = page.locator('button.product-btn').filter({ hasText: 'Optimistic Cake' });
    await expect(productButton).toBeVisible();
    await expect(productButton.locator('.product-btn-inventory')).toHaveText('5 in stock');

    // Click the product to select it
    await productButton.click();

    // Verify charge button appears
    const chargeBtn = page.locator('button.charge-btn', { hasText: /Collect Payment|Charge/ });
    await expect(chargeBtn).toBeVisible({ timeout: 15000 });

    // Mock network to ensure offline handling kicks in or simulate successful tap
    // (In pos.html, going offline triggers the mock tap-to-pay flow)
    await page.context().setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Click charge (simulates tap to pay when offline)
    await chargeBtn.click();

    // Wait for the receipt screen
    await expect(page.locator('text=Offline Quick Charge Saved.')).toBeVisible({ timeout: 15000 });

    // Check if the inventory updated optimistically to 4 (without page reload)
    // The inventory is on the POS view which might be hidden by receipt.
    // Wait, posView.style.display = 'none'; happens in pos.html. Let's look at local storage.
    const finalCatalogStr = await page.evaluate(() => localStorage.getItem('ohc_catalog_e2e-tenant'));
    const finalCatalog = JSON.parse(finalCatalogStr || '[]');
    const product = finalCatalog.find((p: any) => p.id === 'e2e-product-pos-sync');

    expect(product).toBeDefined();
    expect(product.inventory_count).toBe(4);

    // Restore network
    await page.context().setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
  });
});
