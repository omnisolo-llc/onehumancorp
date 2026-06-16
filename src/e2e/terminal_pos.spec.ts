import { test, expect } from '@playwright/test';
import { e2eDbQuery } from './db_utils';

test.describe('Terminal POS - Mobile First & Inventory Sync', () => {
  const TENANT_ID = 'terminal-test-tenant';
  const PRODUCT_ID = 'prod-terminal-e2e-1';

  test.beforeAll(async () => {
    // Seed tenant and product
    await e2eDbQuery(`INSERT INTO tenants (id, name) VALUES ($1, 'Terminal Test Tenant') ON CONFLICT DO NOTHING`, [TENANT_ID]);
    await e2eDbQuery(`
      INSERT INTO products (id, tenant_id, title, name, inventory_count, available_quantity, price_cents, currency)
      VALUES ($1, $2, 'Terminal Test Product', 'Terminal Test Product', 6, 6, 1999, 'usd')
      ON CONFLICT (id, tenant_id) DO UPDATE SET inventory_count = 6, available_quantity = 6
    `, [PRODUCT_ID, TENANT_ID]);
    await e2eDbQuery(`
      INSERT INTO users (id, tenant_id, email, role, current_pin)
      VALUES ('user-terminal-1', $1, 'terminal@ohc.local', 'owner', '1234')
      ON CONFLICT DO NOTHING
    `, [TENANT_ID]);
  });

  test.beforeEach(async ({ page }) => {
    // Navigate to POS terminal path
    await page.goto(`/pos/terminal`);

    // Unlock the terminal
    const pins = ['1', '2', '3', '4'];
    for (const p of pins) {
      await page.getByRole('button', { name: p, exact: true }).click();
    }

    // Clock in
    await page.getByRole('button', { name: 'Clock In' }).click();
  });

  test('Processes tap-to-pay and reserves inventory', async ({ page }) => {
    // Wait for products to load and click on Terminal Test Product
    await expect(page.getByText('Terminal Test Product')).toBeVisible({ timeout: 10000 });
    await page.getByText('Terminal Test Product').click();

    // Discover readers
    await expect(page.getByRole('button', { name: 'Discover Readers' })).toBeVisible();
    await page.getByRole('button', { name: 'Discover Readers' }).click();

    // Connect to a simulated reader
    await expect(page.getByRole('button', { name: 'Connect' })).toBeVisible({ timeout: 10000 });
    await page.getByRole('button', { name: 'Connect' }).first().click();

    // Wait for and click the "Collect Payment" button
    await expect(page.getByRole('button', { name: /Collect Payment/ })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Collect Payment/ }).click();

    // Check loading/processing state
    await expect(page.getByText('Status: Processing payment...')).toBeVisible({ timeout: 15000 });

    // Payment successful
    await expect(page.getByText('Status: Payment successful!')).toBeVisible({ timeout: 15000 });

    // Go to dashboard feed to check for restock action card
    await page.goto(`/dashboard`);

    // Because the low stock alert is triggered on backend, we can just assert the card will appear
    const approveRestockBtn = page.getByTestId('approve-restock');
    await expect(approveRestockBtn).toBeVisible({ timeout: 10000 });
  });
});
