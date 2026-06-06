import { test, expect } from './fixtures';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should complete a tap-to-pay transaction', async ({ page }) => {
    // 1. Setup offline staff data so we can login with a PIN
    await page.goto('/pos/terminal');
    await page.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_123', name: 'Carlos', role: 'Manager', pin_hash: '1234' }]));
    });
    // Reload to apply local storage
    await page.goto('/pos/terminal');

    // 2. Unlock the terminal (enter 1234 PIN)
    await expect(page.getByText('Terminal Locked')).toBeVisible();
    await page.getByRole('button', { name: '1' }).click();
    await page.getByRole('button', { name: '2' }).click();
    await page.getByRole('button', { name: '3' }).click();
    await page.getByRole('button', { name: '4' }).click();

    // 3. Start a new order
    await expect(page.getByText('Carlos')).toBeVisible();
    await expect(page.getByText('Quick Actions')).toBeVisible();

    // 4. Set up mock responses for Stripe terminal integration
    let tokenCallCount = 0;
    await page.route('/api/v1/payments/terminal/token', async route => {
      tokenCallCount++;
      await route.fulfill({ json: { secret: 'mock_connection_token_123' } });
    });

    let intentCallCount = 0;
    let intentAmount = 0;
    await page.route('/api/v1/payments/terminal/intent', async route => {
      intentCallCount++;
      const data = route.request().postDataJSON();
      intentAmount = data.amount;
      await route.fulfill({ json: { client_secret: 'pi_mock_123_secret_123' } });
    });

    await page.getByRole('button', { name: 'New Order' }).click();

    // 5. Verify Stripe Terminal component mounts
    await expect(page.getByText('Stripe Terminal')).toBeVisible();

    // Check initialized state, which means the token was successfully fetched and SDK loaded.
    // Sometimes it's quick, but wait for 'Discover Readers' to ensure it's ready.
    await expect(page.getByRole('button', { name: 'Discover Readers' })).toBeVisible({ timeout: 10000 });

    // Mock terminal SDK since standard playwright won't be able to connect to real bluetooth reader
    // The test requirements allow checking if 'Discover Readers' successfully connects to our mocks

    // 6. Connect the simulated reader
    await page.getByRole('button', { name: 'Discover Readers' }).click();
    await expect(page.getByText('Discovered 1 readers.')).toBeVisible({ timeout: 10000 });
    await page.getByRole('button', { name: 'Connect' }).click();

    // 7. Verify intent call count on charge
    await expect(page.getByText('Connected to reader: ')).toBeVisible({ timeout: 10000 });
    await page.getByRole('button', { name: 'Charge $50.00' }).click();

    // 8. Assert payment completion
    await expect(page.getByText('Payment successful!')).toBeVisible({ timeout: 10000 });

    // Ensure we made the expected backend calls
    expect(tokenCallCount).toBeGreaterThan(0);
    expect(intentCallCount).toBeGreaterThan(0);
    expect(intentAmount).toBe(5000);
  });
});
