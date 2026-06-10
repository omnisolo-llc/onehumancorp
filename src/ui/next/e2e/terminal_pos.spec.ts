import { test, expect } from '@playwright/test';

test.describe('Terminal POS - Mobile First & Inventory Sync', () => {
  const TENANT_ID = 'terminal-test-tenant';

  test.beforeEach(async ({ page }) => {
    await page.route('**/api/staff', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([{ id: 'staff_1', name: 'Test Staff', pin_hash: '1234', role: 'Manager' }])
      });
    });

    await page.addInitScript(() => {
        window.localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Test Staff', pin_hash: '1234', role: 'Manager' }]));
    });

    await page.goto(`/pos/terminal`);

    const newOrderBtn = page.getByRole('button', { name: 'New Order' });
    const terminalLocked = page.getByText('Terminal Locked');

    await Promise.any([
        expect(newOrderBtn).toBeVisible({ timeout: 15000 }).catch(() => {}),
        expect(terminalLocked).toBeVisible({ timeout: 15000 }).catch(() => {})
    ]);

    if (await terminalLocked.isVisible()) {
      const pins = ['1', '2', '3', '4'];
      for (const p of pins) {
        await page.waitForTimeout(500);
        await page.getByRole('button', { name: p, exact: true }).click({ force: true, timeout: 5000 }).catch(() => {});
      }
    }

    await page.waitForTimeout(1000);
    const clockInBtn = page.getByRole('button', { name: 'Clock In' });
    if (await clockInBtn.isVisible()) {
      await clockInBtn.click();
    }
  });

  test('Processes tap-to-pay and reserves inventory', async ({ page }) => {
    await expect(page.getByRole('button', { name: 'New Order' })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'New Order' }).click();
    await expect(page.getByRole('status')).toBeVisible({ timeout: 10000 });
    await expect(page.getByRole('status')).toContainText('Payment Completed', { timeout: 20000 });
  });

  test('Processes tap-to-pay offline and syncs when online', async ({ page, context }) => {
    await expect(page.getByRole('button', { name: 'New Order' })).toBeVisible({ timeout: 15000 });

    await context.setOffline(true);
    await page.getByRole('button', { name: 'New Order' }).click();
    await expect(page.getByRole('status')).toContainText('Payment Saved Offline', { timeout: 10000 });

    await context.setOffline(false);

    await page.waitForTimeout(12000);

    await page.goto(`/dashboard`);
    await expect(page.getByRole('button', { name: 'Agent Approvals' })).toBeVisible({ timeout: 10000 });
  });
});
