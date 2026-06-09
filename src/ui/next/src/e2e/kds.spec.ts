import { test, expect } from '@playwright/test';

test.describe('KDS Offline & Multilingual', () => {
  test.describe.configure({ mode: 'serial' });

  test.beforeEach(async ({ page, context, request }) => {
    // Clear cookies and state
    await context.clearCookies();
    await page.goto('/pos/kds');
    await page.evaluate(() => localStorage.clear());

    // Reset backend state before each test
    await request.delete('/api/pos/orders');
    await request.delete('/api/pos/inventory');

    // Reload to ensure fresh state with clean local storage
    await page.reload();

    // Wait for mock data to load after reload
    await expect(page.locator('text=Active Orders')).toBeVisible({ timeout: 10000 });
  });

  test('KDS Order Sync & Multilingual Display', async ({ page }) => {

    await expect(page.locator('text=#1 - Ahmed')).toBeVisible();
    await expect(page.getByText('Chicken Over Rice', { exact: true })).toBeVisible();

    // Toggle language
    await page.getByTestId('lang-toggle').click();

    // Check Arabic translations
    await expect(page.locator('text=الطلبات النشطة')).toBeVisible();
    await expect(page.locator('text=دجاج فوق الرز')).toBeVisible();

    // Check RTL
    const dir = await page.locator('div[dir="rtl"]').count();
    expect(dir).toBeGreaterThan(0);
  });

  test('KDS Offline Actions & Background Sync', async ({ page, context }) => {
    await expect(page.locator('text=#1 - Ahmed')).toBeVisible();
    // Verify initial state is "Received" before attempting to click "Prepare"
    await expect(page.getByTestId('btn-prepare-1')).toBeVisible({ timeout: 5000 });

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
    await page.getByTestId('toggle-soldout-inv_1').click();
    await expect(page.getByTestId('toggle-soldout-inv_1')).toHaveText('Sold Out');

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


  test('KDS Full Lifecycle of Order Status', async ({ page }) => {
    await expect(page.locator('text=#1 - Ahmed')).toBeVisible();
    await expect(page.getByTestId('btn-prepare-1')).toBeVisible({ timeout: 5000 });

    // Transition: Received -> Preparing
    await page.getByTestId('btn-prepare-1').click();
    await expect(page.getByTestId('btn-ready-1')).toBeVisible();

    // Transition: Preparing -> Ready
    await page.getByTestId('btn-ready-1').click();

    // Status should update to Ready and the button disabled
    const readyBtn = page.getByRole('button', { name: 'Ready' });
    await expect(readyBtn).toBeDisabled();
  });

  test('KDS Multiple Offline Actions Aggregate properly', async ({ page, context }) => {
    // Set network to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(page.locator('text=Offline Mode')).toBeVisible();

    // Toggle inventory multiple times
    await page.getByTestId('toggle-soldout-inv_1').click();
    await expect(page.getByTestId('toggle-soldout-inv_1')).toHaveText('Sold Out');

    // Toggle again
    await page.getByTestId('toggle-soldout-inv_1').click();
    await expect(page.getByTestId('toggle-soldout-inv_1')).toHaveText('Available');

    // Toggle again
    await page.getByTestId('toggle-soldout-inv_1').click();
    await expect(page.getByTestId('toggle-soldout-inv_1')).toHaveText('Sold Out');

    // Verify localStorage queued events has all 3 toggles
    const events = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_kds_events') || '[]'));
    expect(events.length).toBe(3);
    expect(events[0].type).toBe('TOGGLE_SOLD_OUT');
    expect(events[0].payload.is_sold_out).toBe(true);
    expect(events[1].payload.is_sold_out).toBe(false);
    expect(events[2].payload.is_sold_out).toBe(true);
  });

  test('KDS Offline Queue Persists Across Reloads', async ({ page, context }) => {
    // Set network to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(page.locator('text=Offline Mode')).toBeVisible();

    // Action 1: Update order status
    await page.getByTestId('btn-prepare-1').click();
    await expect(page.getByTestId('btn-ready-1')).toBeVisible();

    // Check local storage event
    let events = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_kds_events') || '[]'));
    expect(events.length).toBe(1);

    // Reload page while offline
    await page.reload();
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(page.locator('text=Offline Mode')).toBeVisible();

    // Event should still be in queue
    events = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_kds_events') || '[]'));
    expect(events.length).toBe(1);
    expect(events[0].type).toBe('UPDATE_ORDER_STATUS');
    expect(events[0].payload.status).toBe('Preparing');

    // Important: we need to manually sync when restoring network in a fresh reload in tests
    // that rely on network events to be dispatched.
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Wait for background sync to trigger (interval is 5s) and clear events
    await expect(async () => {
      const remainingEvents = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_kds_events') || '[]'));
      expect(remainingEvents.length).toBe(0);
    }).toPass({ timeout: 10000 });
  });
});
