import { test, expect } from '@playwright/test';

test.describe('Mobile POS Basic Verification', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should unlock and show products', async ({ page }) => {
    await page.goto('/pos/terminal');

    // Inject mock data
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([
        { id: 'staff_1', name: 'Priya', role: 'Owner', pin_hash: '1234', tenant_id: 'tenant_1' }
      ]));
      localStorage.setItem('ohc_offline_products', JSON.stringify([
        { id: 'prod_1', title: 'Silk Dress', price_cents: 12000, inventory_count: 5 }
      ]));
    });

    await page.reload();

    // Enter PIN
    await page.get_by_role('button', { name: '1', exact: true }).click();
    await page.get_by_role('button', { name: '2', exact: true }).click();
    await page.get_by_role('button', { name: '3', exact: true }).click();
    await page.get_by_role('button', { name: '4', exact: true }).click();

    // Check if Priya is visible
    await expect(page.getByText('Priya')).toBeVisible();
    await expect(page.getByText('Silk Dress')).toBeVisible();

    // Add to cart
    await page.getByText('Silk Dress').click();
    await expect(page.getByText('View Cart')).toBeVisible();

    // Checkout
    await page.getByText('View Cart').click();
    await expect(page.getByText('Checkout')).toBeVisible();
    await expect(page.getByText('$120.00')).toBeVisible();

    await page.getByText('Complete Sale').click();
    await expect(page.getByText('Sale saved offline.')).toBeVisible();
  });
});
