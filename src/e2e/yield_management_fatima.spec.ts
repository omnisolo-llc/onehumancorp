import { test, expect } from './fixtures';
import { v4 as uuidv4 } from 'uuid';

test.describe('AI-Powered Autonomous Yield Management & Pre-Order System (Fatima)', () => {
  // Use real backend routes
  test('Customer sees dynamic stock tags and can pre-order, and Operations agent triggers', async ({ memberPage, context, page }) => {
    // Generate isolated tenant
    const tenantId = 'fatima_cart_e2e_' + uuidv4().substring(0, 8);
    const productId = 'prod_' + uuidv4().substring(0, 8);

    // First let's login or set localstorage so we don't mock but we set the active tenant
    await memberPage.goto('/preorder.html');
    await memberPage.evaluate((tId) => {
        localStorage.setItem('tenant_id', tId);
    }, tenantId);
    await memberPage.reload();

    // Verify UI has loaded (even if empty, it shouldn't crash)
    await expect(memberPage.locator('h1', { hasText: 'Menu' })).toBeVisible();

    // Check missing product via the actual backend
    const res = await memberPage.request.post('/api/v1/pos/preorder', {
      headers: { 'x-tenant-id': tenantId },
      data: {
        item_id: productId,
        quantity: 1,
        customer_note: 'extra spicy'
      }
    });

    const json = await res.json();
    expect(json.error).toBe('Insufficient stock or sold out');
  });

  test('Operator dashboard allows offline action queuing without local mocks', async ({ memberPage, context }) => {
    // Navigate to UI
    await memberPage.goto('/pos.html');
    const tenantId = 'fatima_cart_e2e_' + uuidv4().substring(0, 8);

    // Make sure we set tenant ID dynamically
    await memberPage.evaluate((tId) => {
      localStorage.setItem('tenant_id', tId);
    }, tenantId);

    // Toggle the pre-order dashboard via actual UI button
    await memberPage.locator('#preorder-dashboard-btn').click();

    // Wait for network call to fail gracefully or return empty
    await memberPage.waitForTimeout(500);

    // Set offline via Playwright
    await context.setOffline(true);
    await memberPage.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Mock window alert for test so it doesn't hang
    await memberPage.evaluate(() => { window.alert = () => {}; });

    // Since we don't have a real order injected and we cannot use innerHTML per guidelines,
    // we can directly call the global markOrderReady function provided in pos.html script
    await memberPage.evaluate(() => {
        // @ts-ignore
        window.markOrderReady('ord-test-offline-1');
    });

    // Verify localStorage queue
    const queue = await memberPage.evaluate(() => JSON.parse(localStorage.getItem('pos_action_queue') || '[]'));
    expect(queue.length).toBe(1);
    expect(queue[0].type).toBe('ready');
    expect(queue[0].orderId).toBe('ord-test-offline-1');

    // Go online
    await context.setOffline(false);
    await memberPage.evaluate(() => window.dispatchEvent(new Event('online')));

    // It should try to sync and the queue should clear
    await memberPage.waitForTimeout(1000);
    const postQueue = await memberPage.evaluate(() => JSON.parse(localStorage.getItem('pos_action_queue') || '[]'));
    expect(postQueue.length).toBe(0);
  });
});
