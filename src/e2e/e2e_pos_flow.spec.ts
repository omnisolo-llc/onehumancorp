import { test, expect } from '@playwright/test';

test.describe('In-Person Payment (POS) Flow', () => {

  test.beforeEach(async ({ page }) => {
    await page.goto('/api/staff');
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{
        id: 'staff_1',
        name: 'Carlos',
        role: 'Manager',
        pin_hash: '1234'
      }]));
      localStorage.setItem('ohc_offline_events', JSON.stringify([]));
    });

    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('omni_user@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();

    // Navigate to the POS terminal page
    await page.goto('/pos.html');

    // Wait for the UI to load and auto-fetch the staff data
    await page.waitForTimeout(2000);

    await expect(page.locator('h1', { hasText: 'Terminal Locked' })).toBeVisible({ timeout: 25000 });

    // Click inside the body to ensure interaction context
    await page.mouse.click(10, 10);
    await page.waitForTimeout(1000);

    // Enter PIN: 1234
    await page.waitForSelector('button:has-text("1")');
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    // Verify unlocked and shows staff name
    await expect(page.locator('h1', { hasText: 'Carlos' })).toBeVisible({ timeout: 10000 });
  });

  test('should complete a tap-to-pay transaction offline and sync', async ({ page, context }) => {
    // Set network to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Expect the offline pill to show
    await expect(page.locator('text=Offline - Cash & Saved Cards Only')).toBeVisible();

    // Trigger New Order
    await page.getByRole('button', { name: 'Quick Charge $50' }).click();
    await expect(page.locator('text=Offline Quick Charge Saved.')).toBeVisible();

    // Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Wait for background sync to trigger and clear events
    await expect(async () => {
      const db = await page.evaluate(() => {
        return new Promise((resolve, reject) => {
          const request = indexedDB.open('ohc_offline_queue', 1);
          request.onsuccess = (e) => {
            const db = e.target.result;
            const tx = db.transaction('mutations', 'readonly');
            const store = tx.objectStore('mutations');
            const getReq = store.getAll();
            getReq.onsuccess = () => resolve(getReq.result);
            getReq.onerror = () => reject(getReq.error);
          };
          request.onerror = () => reject(request.error);
        });
      });
      expect((db as any[]).length).toBe(0);
    }).toPass({ timeout: 15000 });
  });

  test('should process multiple items in a single offline transaction and sync', async ({ page, context }) => {
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Add multiple items via Quick Charge
    await page.getByRole('button', { name: 'Quick Charge $50' }).click();
    await expect(page.locator('text=Offline Quick Charge Saved.')).toBeVisible();

    // Go back to main menu
    await page.getByRole('button', { name: 'New Sale' }).click();

    await page.getByRole('button', { name: 'Quick Charge $50' }).click();
    await expect(page.locator('text=Offline Quick Charge Saved.')).toBeVisible();

    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    await expect(async () => {
      const db = await page.evaluate(() => {
        return new Promise((resolve, reject) => {
          const request = indexedDB.open('ohc_offline_queue', 1);
          request.onsuccess = (e) => {
            const db = e.target.result;
            const tx = db.transaction('mutations', 'readonly');
            const store = tx.objectStore('mutations');
            const getReq = store.getAll();
            getReq.onsuccess = () => resolve(getReq.result);
            getReq.onerror = () => reject(getReq.error);
          };
          request.onerror = () => reject(request.error);
        });
      });
      expect((db as any[]).length).toBe(0);
    }).toPass({ timeout: 15000 });
  });

  test('should clear the offline queue UI when sync succeeds', async ({ page, context }) => {
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    await page.getByRole('button', { name: 'Quick Charge $50' }).click();

    // Check that pending sync UI is visible
    await expect(page.locator('text=1 Items Pending Sync')).toBeVisible();

    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // UI should say Syncing... and then disappear
    await expect(page.locator('text=Syncing...')).toBeVisible({ timeout: 5000 });

    // UI should hide completely
    await expect(page.locator('text=Syncing...')).toBeHidden({ timeout: 15000 });
    await expect(page.locator('text=1 Items Pending Sync')).toBeHidden({ timeout: 15000 });
  });

  test('should perform clock-in while offline and sync when online', async ({ page, context }) => {
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Perform an offline clock in
    await page.getByRole('button', { name: 'Clock In' }).click();
    await expect(page.locator('h2', { hasText: 'Clocked In' })).toBeVisible();

    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    await expect(async () => {
      const remainingEvents = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_events') || '[]'));
      expect(remainingEvents.length).toBe(0);
    }).toPass({ timeout: 15000 });
  });

  test('should handle online mode correctly without queuing', async ({ page, context }) => {
    // Ensure online
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Offline pill should not be visible
    await expect(page.locator('text=Offline - Cash & Saved Cards Only')).toBeHidden();

    // Trigger New Order
    await page.getByRole('button', { name: 'Quick Charge $50' }).click();

    // Wait for the tap overlay to appear
    await expect(page.locator('text=Tap, swipe, or insert')).toBeVisible();

    // Simulate Tap
    await page.getByRole('button', { name: 'Simulate Customer Tap (Test)' }).click();

    // Should say Payment Successful because it's online
    await expect(page.locator('text=Payment Successful')).toBeVisible({ timeout: 15000 });
  });
});
