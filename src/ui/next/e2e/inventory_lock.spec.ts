import { test, expect } from '@playwright/test';

test.describe('Unified Inventory Lock', () => {
  test('Prevents double checkout across POS and Online', async ({ page, context }) => {
    // Navigate online customer to checkout
    const onlinePage = page;
    await onlinePage.goto('http://localhost:3000/checkout?t=tenant-sync');

    // POS context
    const posPage = await context.newPage();
    await posPage.goto('http://localhost:3000/pos/terminal');

    // Trigger POS checkout to grab lock
    await posPage.getByRole('button', { name: '0' }).click();
    await posPage.getByRole('button', { name: '0' }).click();
    await posPage.getByRole('button', { name: '0' }).click();
    await posPage.getByRole('button', { name: '0' }).click();
    await posPage.getByRole('button', { name: 'Clock In' }).click();

    // Tap to pay logic (simulate acquiring lock)
    await posPage.getByText('New Order').click();

    // Now Online attempts to checkout
    await onlinePage.getByRole('button', { name: 'Pay Now' }).click();

    // Verify online sees the lock error
    await expect(onlinePage.getByText('Item just sold out')).toBeVisible({ timeout: 10000 });
  });
});
