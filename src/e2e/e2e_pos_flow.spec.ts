import { test, expect } from './fixtures';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should complete a tap-to-pay transaction', async ({ page, context }) => {
    // Navigate to the POS Terminal page
    await page.goto('/pos/terminal');

    // Simulate setting up staff PIN (1234)
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Test User', role: 'Manager', pin_hash: '1234' }]));
    });
    await page.reload();

    // The page should prompt for PIN
    await expect(page.locator('h1')).toContainText('Terminal Locked');

    // Enter PIN: 1 2 3 4
    await page.getByRole('button', { name: '1' }).click();
    await page.getByRole('button', { name: '2' }).click();
    await page.getByRole('button', { name: '3' }).click();
    await page.getByRole('button', { name: '4' }).click();

    // Wait for unlock
    await expect(page.locator('h1')).toContainText('Test User');

    // Click New Order
    await page.locator('button', { hasText: 'New Order' }).click();

    // Verify StripeTerminalClient renders
    await expect(page.locator('h2', { hasText: 'Stripe Terminal' })).toBeVisible();

    // Mock API token request
    await page.route('/api/v1/payments/terminal/token', async route => {
      await route.fulfill({ status: 200, json: { token: 'mock_token' } });
    });

    await page.locator('button', { hasText: 'Discover Readers' }).click();
    await expect(page.locator('body')).toContainText(/Discovering readers...|Discovered 1 readers|simulated-reader/i);

    // Click Connect on the simulated reader
    await page.locator('button', { hasText: 'Connect' }).click();
    await expect(page.locator('body')).toContainText(/Connected to reader|simulated-reader/i);

    // Charge button should now be visible
    await expect(page.locator('button', { hasText: 'Charge $50.00' })).toBeVisible();

    // Mock API intent request
    await page.route('/api/v1/payments/terminal/intent', async route => {
      await route.fulfill({ status: 200, json: { intent_id: 'mock_intent_id' } });
    });

    await page.locator('button', { hasText: 'Charge $50.00' }).click();

    await expect(page.locator('body')).toContainText(/Payment successful!/i);
  });
});
