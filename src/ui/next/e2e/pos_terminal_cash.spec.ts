import { test, expect } from '@playwright/test';

test.describe('POS Terminal - Cash Ledger & Shift Lifecycle', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to homepage first as per protocol
    await page.goto('/');

    // Navigate to terminal
    await page.goto('/pos/terminal');

    // Unlock the terminal
    // The UI shows 1-9 then 0 in the middle
    const pins = ['1', '2', '3', '4'];
    for (const p of pins) {
      await page.getByRole('button', { name: p, exact: true }).click();
    }
  });

  test('Maya completes a full shift with cash transactions and reconciliation', async ({ page }) => {
    // 1. Start Shift with Opening Balance
    await expect(page.getByRole('heading', { name: 'Start Shift' })).toBeVisible();
    await page.locator('#opening-balance-input').fill('100');
    await page.getByRole('button', { name: 'Open Terminal' }).click();

    // Verify Clocked In
    await expect(page.getByText('Clocked In')).toBeVisible();

    // 2. Perform a Cash Sale
    await page.getByRole('button', { name: 'Cash $50' }).click();
    await expect(page.getByText('Cash payment recorded.')).toBeVisible();

    // 3. Perform a Drawer Drop
    await page.getByRole('button', { name: 'Drawer Ops' }).click();
    await page.getByPlaceholder('0.00').fill('20');
    await page.getByPlaceholder('e.g. Mid-day drop').fill('Lunch run');
    await page.getByRole('button', { name: 'Cash Drop' }).click();
    await expect(page.getByText('DROP recorded.')).toBeVisible();

    // 4. End Shift & Reconcile
    await page.getByRole('button', { name: 'Clock Out' }).click();
    await expect(page.getByRole('heading', { name: 'End Shift' })).toBeVisible();

    // Expected: 100 (open) + 50 (sale) - 20 (drop) = 130
    // The summary shows expected cash
    await expect(page.getByText('$130.00')).toBeVisible();

    // Enter Actual Balance
    // In End Shift modal, it has placeholder 0.00
    await page.getByPlaceholder('0.00').fill('130');
    await expect(page.getByText('Drawer is balanced.')).toBeVisible();

    // Close Terminal
    await page.getByRole('button', { name: 'Close Terminal' }).click();
    await expect(page.getByText('Terminal Locked')).toBeVisible();
  });
});
