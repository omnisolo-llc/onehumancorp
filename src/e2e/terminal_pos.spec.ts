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
    // Wait for the UI to be ready
    await expect(page.getByRole('button', { name: 'New Order' })).toBeVisible();

    // Click New Order
    await page.getByRole('button', { name: 'New Order' }).click();

    // Check loading/processing state
    await expect(page.getByRole('status')).toBeVisible({ timeout: 10000 });

    // Sometimes it might transition to Payment Completed fast, wait for it
    await expect(page.getByRole('status')).toContainText('Payment Completed', { timeout: 15000 });

    // Go to dashboard feed to check for restock action card
    await page.goto(`/dashboard`);
    await expect(page.getByRole('button', { name: 'Agent Approvals' })).toBeVisible();
    await page.getByRole('button', { name: 'Agent Approvals' }).click();

    // Because the low stock alert is triggered on backend, we can just assert the card will appear
    const approveRestockBtn = page.getByTestId('approve-restock');
    await expect(approveRestockBtn).toBeVisible({ timeout: 10000 });
  });
});
