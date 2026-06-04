import { test, expect } from '@playwright/test';


test.beforeEach(async ({ page }) => {
  // Pre-populate offline staff so we can log in
  await page.addInitScript(() => {
    localStorage.setItem('ohc_offline_staff', JSON.stringify([
      { id: '1', name: 'Test User', role: 'Manager', pin_hash: '1234' }
    ]));
  });
});
test.describe('Offline Resilient POS', () => {
  test('should enqueue mutations offline and sync when online', async ({ page, context }) => {
    // Navigate to POS terminal
    await page.goto('/pos/terminal');

    // Unlock terminal
    await page.getByText('1').click();
    await page.getByText('2').click();
    await page.getByText('3').click();
    await page.getByText('4').click();

    await expect(page.getByText('Test User', { exact: false })).toBeVisible(); // Mock user

    // Go offline
    await context.setOffline(true);

    // Click "New Order"
    // Handle the alert
    page.on('dialog', async (dialog) => {
        expect(dialog.message()).toContain('New Order Total:');
        await dialog.accept();
    });

    await page.getByText('New Order').click();

    // We expect the local state to have queued the order, and optimistic UI (offline conversion notice if different currency, though here just the offline mechanism)
    // Unfortunately we can't easily assert the internal Zustand state from Playwright directly,
    // but we can observe the UI effects and then the network requests when coming back online.

    let syncRequestMade = false;
    page.on('request', request => {
      if (request.url().includes('/api/pos/orders') && request.method() === 'POST') {
        syncRequestMade = true;
      }
    });

    // Go back online
    await context.setOffline(false);

    // Wait for the sync to complete (SyncManager triggers every 15s or on 'online' event)
    await page.waitForTimeout(2000); // Give it some time to fire

    expect(syncRequestMade).toBe(true);
  });
});
