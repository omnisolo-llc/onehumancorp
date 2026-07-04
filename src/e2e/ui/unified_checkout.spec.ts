import { test, expect } from '../fixtures';
import { pool } from '../global-setup';

test.describe('Unified Mobile Tap-to-Pay & Hybrid Checkout Architecture', () => {
  test('POS terminal UI displays correct options and creates unified checkout session', async ({ page }) => {
    const adminPage = page;
    const tenantId = 'e2e-tenant';
    const productId = 'prod_pos_unified_test';

    // Seed product
    await pool.query(
      `INSERT INTO products (id, tenant_id, title, price_cents, inventory_count)
       VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET inventory_count = 10`,
      [productId, tenantId, 'Test Unified POS Product', 1500, 10]
    );

    // Seed a staff user
    const staffId = 'staff_' + Date.now();
    await pool.query(
      `INSERT INTO users (id, tenant_id, email, full_name, role)
       VALUES ($1, $2, $3, $4, $5) ON CONFLICT (email) DO NOTHING`,
      [staffId, tenantId, staffId + '@example.com', 'Test Staff', 'STAFF']
    );

    await adminPage.goto('/pos/terminal');

    // Wait for the terminal to load
    await adminPage.waitForSelector('text=Terminal Locked');

    // Type pin (assume PIN is '1234' for simplicity if the UI has 1234 enabled)
    // The previous component had a simple pin unlock logic where it triggers setLocked(false) after pin length > 3
    const pinDigits = ['1', '2', '3', '4'];
    for (const digit of pinDigits) {
      const btn = adminPage.locator(`button:has-text("${digit}")`);
      if (await btn.count() > 0) {
        await btn.click();
      }
    }

    // Select product
    const productButton = adminPage.locator(`button:has-text("Test Unified POS Product")`);
    await productButton.waitFor({ state: 'visible' });
    await productButton.click();

    // Verify Cart updates and shows Collect Payment
    const collectBtn = adminPage.locator('button:has-text("Collect Payment")');
    await collectBtn.waitFor({ state: 'visible' });
    await collectBtn.click();

    // Wait for bottom sheet payment method to open
    const tapToPayBtn = adminPage.locator('button:has-text("Tap to Pay (Phone)")');
    await tapToPayBtn.waitFor({ state: 'visible' });

    // Verify options are visible
    await expect(adminPage.locator('button:has-text("Send Payment Link")')).toBeVisible();
    await expect(adminPage.locator('button:has-text("Cash")')).toBeVisible();

    // Click Tap to Pay
    await tapToPayBtn.click();

    // Wait for Tap to Pay Active screen
    await expect(adminPage.locator('h2:has-text("Tap to Pay Active")')).toBeVisible();

    // Click Confirm & Tap
    const confirmBtn = adminPage.locator('button:has-text("Confirm & Tap")');
    await confirmBtn.waitFor({ state: 'visible' });
    await confirmBtn.click();

    // Wait for payment successful text to appear
    await expect(adminPage.locator('h2:has-text("Payment Successful!")')).toBeVisible({ timeout: 10000 });

    // Verify the new success message
    await expect(adminPage.locator('text=Receipt sent. Inventory updated.')).toBeVisible();

    // Verify database has the checkout session
    const res = await pool.query(`SELECT * FROM checkout_sessions WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 1`, [tenantId]);
    expect(res.rows.length).toBe(1);
    expect(res.rows[0].type).toBe('IN_PERSON');
    expect(res.rows[0].status).toBe('PENDING'); // or active depending on your logic
  });
});
