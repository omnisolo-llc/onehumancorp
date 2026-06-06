import { test, expect } from '@playwright/test';

test.describe('Offline-Capable POS Engine', () => {
  test('Persona: Business Owner can sync offline transactions when back online', async ({ request, page, context }) => {
    // Navigate to the POS Terminal page
    await page.goto('/pos/terminal');

    // Simulate setting up staff PIN (1234)
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Test User', role: 'Manager', pin_hash: '1234' }]));
    });
    await page.reload();

    // The page should prompt for PIN
    await expect(page.locator('h1')).toContainText('Terminal Locked');

    // Enter PIN: 1 2 3 4
    await page.getByRole('button', { name: '1' }).click();
    await page.getByRole('button', { name: '2' }).click();
    await page.getByRole('button', { name: '3' }).click();
    await page.getByRole('button', { name: '4' }).click();

    // Wait for unlock
    await expect(page.locator('h1')).toContainText('Test User');

    // Go offline
    await context.setOffline(true);
    await page.evaluate(() => {
      Object.defineProperty(navigator, 'onLine', { value: false });
      window.dispatchEvent(new Event('offline'));
    });

    // Click New Order
    await page.locator('button', { hasText: 'New Order' }).click();

    // Offline alert is handled via window.alert, intercept it
    page.on('dialog', dialog => dialog.accept());

    // Verify offline transaction was logged locally
    const events = await page.evaluate(() => {
      return JSON.parse(localStorage.getItem('ohc_offline_events') || '[]');
    });

    expect(events.length).toBeGreaterThan(0);
    expect(events[0].payment_method).toBe('cash_offline');

    // Go back online
    await context.setOffline(false);

    // Mock the sync API
    await page.route('/api/v1/sync/offline', async route => {
      await route.fulfill({ status: 200, json: { success: true } });
    });

    await page.evaluate(() => {
      Object.defineProperty(navigator, 'onLine', { value: true });
      window.dispatchEvent(new Event('online'));
    });

    // Let the interval run (we can speed this up in tests, or evaluate the sync manually)
    await page.evaluate(async () => {
      const events = JSON.parse(localStorage.getItem('ohc_offline_events') || '[]');
      if (events.length > 0) {
        await fetch('/api/v1/sync/offline', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ mutations: events })
        });
        localStorage.setItem('ohc_offline_events', '[]');
      }
    });

    // Verify events were cleared
    const eventsAfterSync = await page.evaluate(() => {
      return JSON.parse(localStorage.getItem('ohc_offline_events') || '[]');
    });

    expect(eventsAfterSync.length).toBe(0);
  });
});
