import { test, expect } from './fixtures';
import { pool } from './global-setup';

test.describe('POS Catalog Checkout Flow', () => {
  test('POS terminal handles catalog items cart and checkout', async ({ browser }) => {
    const tenantId = 'e2e-tenant';
    const productId = 'prod_pos_catalog_1';

    // Seed product
    await pool.query(
      `INSERT INTO products (id, tenant_id, title, price_cents, inventory_count)
       VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET inventory_count = 10`,
      [productId, tenantId, 'Catalog Product', 1999, 10]
    );

    const adminPage = await browser.newPage();
    await adminPage.goto('/pos.html');

    // Switch to Catalog Tab
    await adminPage.locator('text=Catalog').click();

    // Ensure product is visible
    await expect(adminPage.locator('text=Catalog Product')).toBeVisible({ timeout: 10000 });

    // Click on product to add to cart
    await adminPage.locator('text=Catalog Product').click();

    // Ensure cart has item
    await expect(adminPage.locator('.cart-item-name', { hasText: 'Catalog Product' })).toBeVisible();
    await expect(adminPage.locator('#cart-total')).toHaveText('$19.99');

    // Click Charge
    await adminPage.locator('#checkout-btn').click();

    // Wait for the tap overlay and click the simulate button
    await expect(adminPage.locator('#tap-overlay')).toBeVisible();
    await adminPage.locator('#simulate-tap-btn').click();

    // Verify success receipt
    await expect(adminPage.locator('.receipt-text', { hasText: 'Payment Successful' })).toBeVisible();

    // Verify inventory deduction
    const res = await pool.query('SELECT available_quantity FROM products WHERE id = $1', [productId]);
    expect(res.rows[0].available_quantity).toBe(9);
  });
});
