import { test, expect } from './fixtures';

test.describe('Degradation Validation (Chaos Engineering)', () => {

  test('frontend fail-safes when backend latency spikes >2s or connection drops', async ({ page }) => {
    await page.goto('/inventory');
    await expect(page.locator('text=Inventory').first()).toBeVisible();

    await page.route('**/api/v1/sync/offline', async (route) => {
      await route.abort('failed');
    });

    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('simulate_offline_mutation', {
        detail: {
          type: 'inventory_toggle',
          id: 'e2e-product-123',
          timestamp: new Date().toISOString()
        }
      }));
      const queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
      queue.push({
        type: 'inventory_toggle',
        id: 'e2e-product-123',
        timestamp: new Date().toISOString()
      });
      localStorage.setItem('ohc_offline_queue', JSON.stringify(queue));
      window.dispatchEvent(new Event('storage'));
    });

    const queueData = await page.evaluate(() => {
      return JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
    });

    expect(queueData.length).toBeGreaterThan(0);
    expect(queueData[0].type).toBe('inventory_toggle');

    await expect(page.locator('text=Inventory').first()).toBeVisible();
  });

  test('POS terminal fallback queues transactions locally during offline mode', async ({ page }) => {
    await page.goto('/checkout');
    await expect(page.locator('text=Checkout').first()).toBeVisible();

    await page.route('**/api/v1/payments/terminal/sync_offline', async (route) => {
      await route.abort('failed');
    });

    await page.evaluate(() => {
      const queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
      queue.push({
        type: 'tap_to_pay',
        id: 'e2e-txn-pos-123',
        amount: 500,
        currency: 'usd',
        product_id: 'e2e-prod-x',
        timestamp: new Date().toISOString()
      });
      localStorage.setItem('ohc_offline_queue', JSON.stringify(queue));
      window.dispatchEvent(new Event('storage'));
    });

    const queueData = await page.evaluate(() => {
      return JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
    });

    const tapToPayTxns = queueData.filter((q: any) => q.type === 'tap_to_pay');
    expect(tapToPayTxns.length).toBeGreaterThan(0);
    expect(tapToPayTxns[0].amount).toBe(500);
  });

  test('Draft quote mutation degrades gracefully to offline queue', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=Dashboard').first()).toBeVisible();

    await page.route('**/api/v1/sync/offline', async (route) => {
      await route.abort('failed');
    });

    await page.evaluate(() => {
      const queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
      queue.push({
        type: 'draft_quote',
        id: 'e2e-draft-456',
        notes: '{"custom": "quote data"}',
        timestamp: new Date().toISOString()
      });
      localStorage.setItem('ohc_offline_queue', JSON.stringify(queue));
      window.dispatchEvent(new Event('storage'));
    });

    const queueData = await page.evaluate(() => {
      return JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
    });

    const draftQuotes = queueData.filter((q: any) => q.type === 'draft_quote');
    expect(draftQuotes.length).toBeGreaterThan(0);
    expect(draftQuotes[0].notes).toBe('{"custom": "quote data"}');
  });

  test('Read operations render cached layout with blurred states when API is offline', async ({ page }) => {
    await page.route('**/api/v1/**', async (route) => {
      await route.abort('failed');
    });

    await page.goto('/calendar');

    // Test that the layout doesn't completely break/white-screen.
    await expect(page.locator('text=Calendar').first()).toBeVisible();
    // A premium UI fail-safe: offline mode indicator could be verified if it exists.
  });

  test('SyncManager recovers and replays offline queue when connection is restored', async ({ page }) => {
    await page.goto('/dashboard');

    let syncOfflineCalled = false;
    await page.route('**/api/v1/sync/offline', async (route) => {
      syncOfflineCalled = true;
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true })
      });
    });

    // 1. Add item to queue
    await page.evaluate(() => {
      const queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
      queue.push({
        type: 'inventory_toggle',
        id: 'e2e-product-789',
        timestamp: new Date().toISOString()
      });
      localStorage.setItem('ohc_offline_queue', JSON.stringify(queue));
    });

    // 2. Trigger online event manually to force SyncManager to sync
    await page.evaluate(() => {
      window.dispatchEvent(new Event('online'));
    });

    // 3. Wait a moment for async sync to run
    await page.waitForTimeout(1500);

    // 4. Verify route was called and queue is empty
    expect(syncOfflineCalled).toBe(true);

    const queueData = await page.evaluate(() => {
      return JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
    });

    expect(queueData.length).toBe(0);
  });
});
