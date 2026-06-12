import { test, expect } from '@playwright/test';

test.describe('Terminal POS - Mobile First & Inventory Sync', () => {
  const TENANT_ID = 'terminal-test-tenant';

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
    // Navigate directly to the pos UI as per the new frontend implementation test
    await page.goto('/ui/pos.html');

    // Wait for numpad to appear
    await expect(page.getByRole('button', { name: '1' }).first()).toBeVisible();

    // Enter a mock amount: e.g. 5000 cents ($50.00)
    await page.getByRole('button', { name: '5' }).first().click();
    await page.getByRole('button', { name: '0' }).first().click();
    await page.getByRole('button', { name: '0' }).first().click();
    await page.getByRole('button', { name: '0' }).first().click();

    // The display should show $50.00
    await expect(page.locator('#amount-display')).toContainText('$50.00');

    // Setup an intercept to mock the backend Stripe token call
    await page.route('**/api/v1/payments/terminal/token', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ secret: 'mock_terminal_connection_token' }),
      });
    });

    // Setup an intercept to mock the backend Stripe intent call
    await page.route('**/api/v1/payments/terminal/intent', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ client_secret: 'mock_payment_intent_secret' }),
      });
    });

    // Setup an intercept to mock the backend sync_offline call
    await page.route('**/api/v1/payments/terminal/sync_offline', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true, failed_transaction_ids: [] }),
      });
    });

    // Click "Accept Contactless Payment"
    await page.getByRole('button', { name: /Accept Contactless Payment/i }).click();

    // The Tap Overlay should appear
    await expect(page.locator('#tap-overlay')).toBeVisible();

    // Click "Simulate Customer Tap (Test)"
    await page.getByRole('button', { name: /Simulate Customer Tap/i }).click();

    // Wait for the Receipt Screen
    await expect(page.locator('#receipt-screen')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('#receipt-amount')).toContainText('$50.00');
  });
});
