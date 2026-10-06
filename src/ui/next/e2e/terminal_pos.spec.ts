import { test, expect } from '../../../e2e/fixtures';

test.describe('Terminal POS - Mobile First & Inventory Sync', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to POS terminal path
    await page.goto('/pos/terminal');

    // Unlock the terminal
    const pins = ['1', '2', '3', '4'];
    for (const p of pins) {
      await page.getByRole('button', { name: p, exact: true }).click();
    }

    // Clock in
    await page.getByRole('button', { name: 'Clock In' }).click();
    await expect(page.getByRole('heading', { name: 'Clocked In', exact: true })).toBeVisible();
    await expect(page.getByText('Offline queue ready for this session.', { exact: true })).toBeVisible();
  });

  test('does not invent a reader or charge when Terminal credentials are unavailable', async ({ page }) => {
    // Wait for the UI to be ready
    await expect(page.getByRole('button', { name: 'New Order' })).toBeVisible();

    // The native runner strips Stripe credentials. The real BFF must reject
    // the missing verified tenant connection without returning a reader secret.
    const token = await page.request.post('/api/v1/payments/terminal/token', {
      headers: { origin: new URL(page.url()).origin, 'sec-fetch-site': 'same-origin' },
    });
    expect(token.status()).toBe(503);
    expect(token.headers()['cache-control']).toBe('private, no-store');
    expect(await token.json()).toEqual({
      success: false, status: 'rejected', error: 'A verified tenant payment connection is required.',
    });
    await token.dispose();
    await expect(page.getByRole('button', { name: 'Discover Readers' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect', exact: true })).toHaveCount(0);
    await expect(page.getByRole('button', { name: /Charge \$/ })).toHaveCount(0);
    await expect(page.getByText('Payment successful!')).toHaveCount(0);
  });

  test('handles offline cash sales without requiring a simulated card reader', async ({ page, context }) => {
    // Wait for the UI to be ready
    await expect(page.getByRole('button', { name: 'Discover Readers' })).toBeVisible();

    const terminal = page.locator('#pos-keypad');
    await terminal.getByRole('button', { name: 'Back', exact: true }).click();
    await terminal.getByRole('button', { name: 'Cash', exact: true }).click();
    // Wait for the real same-owner identity and local queue reads to settle.
    // An online sync revalidation must not be interrupted by this fixture.
    await expect(page.getByText('Offline queue ready for this session.', { exact: true })).toBeVisible();

    // Go offline
    await context.setOffline(true);

    // Check that offline indicator appears
    await expect(page.getByText('Offline - Changes will sync later')).toBeVisible({ timeout: 5000 });

    await terminal.getByRole('button', { name: 'Record Offline Cash Sale 50.00' }).click();

    // Check offline processing text
    await expect(page.getByRole('heading', { name: 'Sale queued offline' })).toBeVisible();
    await expect(page.getByText('The $50.00 sale is saved on this device and still needs to sync.')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Payment Successful!' })).toHaveCount(0);
    await expect(page.getByText(/Offline Mode - \d+ Pending/)).toBeVisible();

    // Go online
    await context.setOffline(false);

    // It should go back to online indicator
    await expect(page.getByText('Online', { exact: true })).toBeVisible({ timeout: 5000 });
  });

});
