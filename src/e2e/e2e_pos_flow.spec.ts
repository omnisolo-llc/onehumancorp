import { test, expect } from './fixtures';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should complete a tap-to-pay transaction successfully', async ({ page }) => {
    await page.goto('/pos/terminal');

    // Setup staff
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{
        id: 'staff_1',
        name: 'Carlos',
        role: 'Manager',
        pin_hash: '1234'
      }]));
    });

    // Reload to apply localStorage
    await page.goto('/pos/terminal');
    await expect(page.locator('h1', { hasText: 'Terminal Locked' })).toBeVisible();

    // Login with PIN
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await expect(page.locator('h1', { hasText: 'Carlos' })).toBeVisible();

    // Start a new order
    await page.getByRole('button', { name: 'New Order' }).click();

    // Verify StripeTerminalClient renders and wait for connection to a reader
    await expect(page.locator('h2', { hasText: 'Stripe Terminal' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Discover Readers' })).toBeVisible();

    // The underlying terminal logic connects and lets us charge
    await page.getByRole('button', { name: 'Discover Readers' }).click();
    await page.getByRole('button', { name: 'Connect' }).first().click();

    // Execute charge
    await page.getByRole('button', { name: 'Charge $50.00' }).click();
    await expect(page.locator('p', { hasText: 'Status: Payment successful!' })).toBeVisible({ timeout: 15000 });
  });

  test('should fail to charge and show out of stock when item is locked by online checkout', async ({ page, request }) => {
    await page.goto('/pos/terminal');

    // Setup staff
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{
        id: 'staff_1',
        name: 'Carlos',
        role: 'Manager',
        pin_hash: '1234'
      }]));
    });

    // Reload to apply localStorage
    await page.goto('/pos/terminal');
    await expect(page.locator('h1', { hasText: 'Terminal Locked' })).toBeVisible();

    // Login with PIN
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await expect(page.locator('h1', { hasText: 'Carlos' })).toBeVisible();

    // Start a new order (which grabs an item like inv_1 from the inventory API)
    await page.getByRole('button', { name: 'New Order' }).click();

    // We will simulate a race condition where the item gets locked.
    // By creating a conversational checkout intent for the same product, it locks it for 15m.
    await request.post('/api/v1/booking/conversational_checkout', {
      data: {
        tenant_id: 'default_tenant', // Assuming default_tenant or we can fetch what the app uses
        customer_id: 'cust_123',
        amount_cents: 5000,
        product_id: 'inv_1' // Match the inventory default available item
      }
    });

    // Verify StripeTerminalClient renders and wait for connection to a reader
    await expect(page.locator('h2', { hasText: 'Stripe Terminal' })).toBeVisible();
    await page.getByRole('button', { name: 'Discover Readers' }).click();
    await page.getByRole('button', { name: 'Connect' }).first().click();

    // Execute charge
    await page.getByRole('button', { name: 'Charge $50.00' }).click();

    // Verify lock fails
    await expect(page.locator('p', { hasText: 'Status: Item sold out online just now.' })).toBeVisible({ timeout: 15000 });
  });
});
