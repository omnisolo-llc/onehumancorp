import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('POS Inventory Lock Architecture', () => {
  adminPage('Simultaneous checkouts prevent double-booking', async ({ page, browser }) => {
    await page.goto('/pos/terminal');
    await expect(page.locator('h1', { hasText: 'Clocked In' }).or(page.locator('button', { hasText: 'Clock In' }))).toBeVisible();

    const clockInBtn = page.locator('button', { hasText: 'Clock In' });
    if (await clockInBtn.isVisible()) {
      await clockInBtn.click();
    }

    await page.click('button:has-text("New Order")');

    const res1 = page.evaluate(async () => {
      const res = await fetch('/api/v1/payments/terminal/reserve', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tenant_id: 'e2e-tenant', product_id: 'e2e-product-cake', quantity: 1, ttl_seconds: 15 })
      });
      return res.json();
    });

    const res2 = page.evaluate(async () => {
      const res = await fetch('/api/v1/payments/terminal/reserve', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tenant_id: 'e2e-tenant', product_id: 'e2e-product-cake', quantity: 1, ttl_seconds: 15 })
      });
      return res.json();
    });

    const [data1, data2] = await Promise.all([res1, res2]);

    const successCount = [data1.success, data2.success].filter(Boolean).length;
    expect(successCount).toBe(1);

    const failData = data1.success ? data2 : data1;
    const successData = data1.success ? data1 : data2;

    expect(failData.error_message).toContain('Item is currently being checked out by another customer');
    expect(successData.lock_id).toBeTruthy();

    const commitData = await page.evaluate(async (lockId) => {
      const res = await fetch('/api/v1/payments/terminal/commit', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tenant_id: 'e2e-tenant', product_id: 'e2e-product-cake', quantity: 1, lock_id: lockId })
      });
      return res.json();
    }, successData.lock_id);

    expect(commitData.success).toBe(true);

    await page.route('**/api/v1/payments/terminal/reserve', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: false, error_message: "Item is currently being checked out by another customer", lock_id: "" })
      });
    });

    await page.click('button:has-text("New Order")');
    await expect(page.locator('p[role="status"]')).toContainText('Failed to reserve: Item is currently being checked out by another customer');
    await page.unroute('**/api/v1/payments/terminal/reserve');
  });
});
