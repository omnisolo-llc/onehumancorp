import { test, expect } from '@playwright/test';

test.describe('KDS Offline & Multilingual', () => {

  test('KDS Order Sync & Multilingual Display', async ({ page }) => {
    await page.goto('/pos/kds');

    // Wait for data to load
    await expect(page.locator('text=Active Orders')).toBeVisible();
    await expect(page.getByText('Ava Customer')).toBeVisible();
    await expect(page.getByText('Vegan Celebration Cake')).toBeVisible();

    // Toggle language
    await page.getByTestId('lang-toggle').click();

    // Check Arabic translations
    await expect(page.locator('text=الطلبات النشطة')).toBeVisible();
    await expect(page.locator('text=كعكة احتفال نباتية')).toBeVisible();

    // Check RTL
    const dir = await page.locator('div[dir="rtl"]').count();
    expect(dir).toBeGreaterThan(0);
  });

  test('KDS Offline Actions & Background Sync', async ({ page, context }) => {
    await page.goto('/pos/kds');
    await expect(page.getByText('Ava Customer')).toBeVisible();

    // Set network to offline
    await context.setOffline(true);
    // Simulate offline event directly in browser to trigger state updates reliably
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Wait for UI to reflect offline state
    await expect(page.locator('text=Offline Mode')).toBeVisible();

    // The seeded order 'e2e-order-2' is 'pending' and for 'Ben Buyer'
    await expect(page.getByText('Ben Buyer')).toBeVisible();

    // Perform optimistic action 1: Update order status for pending order
    await page.getByTestId('btn-prepare-e2e-order-2').click();
    await expect(page.getByTestId('btn-ready-e2e-order-2')).toBeVisible();

    // Perform optimistic action 2: Toggle sold out
    await page.getByTestId('toggle-soldout-e2e-product-cake').click();
    await expect(page.getByTestId('toggle-soldout-e2e-product-cake')).toHaveText('Sold Out');

    // Verify localStorage queued events
    const events = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_kds_events') || '[]'));
    expect(events.length).toBe(2);
    expect(events[0].type).toBe('UPDATE_ORDER_STATUS');
    expect(events[1].type).toBe('TOGGLE_SOLD_OUT');

    // Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Expect offline badge to disappear
    await expect(page.locator('text=Offline Mode')).toBeHidden();

    // Wait for background sync to trigger (interval is 5s) and clear events
    await expect(async () => {
      const remainingEvents = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_kds_events') || '[]'));
      expect(remainingEvents.length).toBe(0);
    }).toPass({ timeout: 10000 });
  });

});
