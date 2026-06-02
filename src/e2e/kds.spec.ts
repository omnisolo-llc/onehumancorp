import { test, expect } from '@playwright/test';

test.describe('KDS Offline & Multilingual', () => {

  test.beforeEach(async ({ request, page }) => {
    await request.delete('http://localhost:3000/api/pos/orders');
    await request.delete('http://localhost:3000/api/pos/inventory');
    await page.goto('http://localhost:3000/pos/kds');
    await page.evaluate(() => localStorage.clear());
  });

  test('KDS Order Sync & Multilingual Display', async ({ page }) => {
    await page.goto('http://localhost:3000/pos/kds');

    // Wait for mock data to load
    await expect(page.locator('text=Active Orders')).toBeVisible();
    await expect(page.locator('text=#1 - Ahmed')).toBeVisible();
    await expect(page.locator('text=Chicken Over Rice').first()).toBeVisible();

    // Toggle language
    await page.getByTestId('lang-toggle').click();

    // Check Arabic translations
    await expect(page.locator('text=الطلبات النشطة')).toBeVisible();
    await expect(page.locator('text=دجاج فوق الرز').first()).toBeVisible();

    // Check RTL
    const dir = await page.locator('div[dir="rtl"]').count();
    expect(dir).toBeGreaterThan(0);
  });

  test('KDS Offline Actions & Background Sync', async ({ page, context }) => {
    await page.goto('http://localhost:3000/pos/kds');
    await expect(page.locator('text=#1 - Ahmed')).toBeVisible();

    // Set network to offline
    await context.setOffline(true);
    // Simulate offline event directly in browser to trigger state updates reliably
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Wait for UI to reflect offline state
    await expect(page.locator('text=Offline Mode')).toBeVisible();

    // Perform optimistic action 1: Update order status
    await page.getByTestId('btn-prepare-1').click();
    await expect(page.getByTestId('btn-ready-1')).toBeVisible();

    // Perform optimistic action 2: Toggle sold out
    // wait for initial state
    await expect(page.locator('data-testid=toggle-soldout-inv_1').first()).toHaveText('Available');
    await page.locator('data-testid=toggle-soldout-inv_1').first().click();
    await expect(page.locator('data-testid=toggle-soldout-inv_1').first()).toHaveText('Sold Out');

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
