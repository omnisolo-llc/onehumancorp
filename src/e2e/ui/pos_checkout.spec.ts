import { expect } from '@playwright/test';
import { test } from '../fixtures';

test.describe('POS Checkout - Centralized Inventory', () => {
  test('Prevents double booking with Redis lock when checkout occurs concurrently', async ({ page, context }) => {
    // Navigate to the POS terminal page
    await page.goto('/pos/terminal');
    // Ensure 375px mobile responsiveness
    await page.setViewportSize({ width: 375, height: 667 });

    await expect(page.locator('h1', { hasText: 'Terminal Locked' })).toBeVisible({ timeout: 25000 });

    // Unlock POS
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    // Clock in
    const clockInBtn = page.getByRole('button', { name: 'Clock In' });
    await expect(clockInBtn).toBeVisible({ timeout: 10000 });
    await clockInBtn.click();

    // Wait for the UI to be ready
    await expect(page.locator('text=Vegan Celebration Cake')).toBeVisible();

    const discoverBtn = page.locator('text=Discover Readers');
    if (await discoverBtn.isVisible()) {
        await discoverBtn.click();
        await page.locator('text=Connect').first().click();
    }

    await page.locator('text=Vegan Celebration Cake').click();
    await expect(page.locator('text=Collect Payment $39.99')).toBeVisible({ timeout: 10000 });

    // Simulate an online customer checking out at the exact same moment.
    // We will do this by triggering the lock from the same test context via API, rather than mocking!
    const res = await context.request.post('/api/v1/payments/terminal/reserve', {
        data: {
            tenant_id: 'e2e-tenant',
            product_id: 'e2e-product-cake',
            quantity: 1,
            ttl_seconds: 15
        }
    });

    expect(res.ok()).toBeTruthy();
    const reserveData = await res.json();
    // Verify the lock was acquired
    expect(reserveData.success).toBeTruthy();
    const lockId = reserveData.lock_id;

    // Simulate clicking charge while the item is already locked by the online customer above
    await page.locator('text=Collect Payment $39.99').click();

    // Expect out of stock or failure message
    await expect(page.locator('text=Reservation failed: Item is currently being checked out by another customer')).toBeVisible({ timeout: 10000 });

    // Release the lock
    await context.request.post('/api/v1/payments/terminal/commit', {
      data: {
        tenant_id: 'e2e-tenant',
        product_id: 'e2e-product-cake',
        quantity: 1,
        lock_id: lockId,
        amount_cents: 3999
      }
    });
  });
});
