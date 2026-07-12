import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';
import { loginAs } from './utils/auth';

test.describe('POS Tap-to-Pay Offline Resilience (Issue #33945)', () => {
  const adminUser = {
    email: 'admin@example.com',
    password: 'password',
    tenant_id: 'default',
    role: 'owner'
  };

  test('offline transaction queueing and tap-to-pay workflow', async ({ page, context }) => {
    test.setTimeout(60000);

    // Setup local storage state before navigating to POS terminal
    await page.goto('/api/staff');
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{
        id: 'staff_1',
        name: 'Fatima',
        role: 'Manager',
        pin_hash: '1234'
      }]));
      localStorage.setItem('ohc_offline_events', JSON.stringify([]));
      // Start with an empty queue
      localStorage.setItem('ohc_offline_queue', JSON.stringify([]));
    });

    await page.setViewportSize({ width: 375, height: 812 });

    await loginAs(page, adminUser);

    // Navigate to the terminal page
    await page.goto('/pos/terminal');

    // Wait for the UI to load
    await page.waitForTimeout(2000);

    // Handle pin entry lock screen if it exists
    const isLocked = await page.locator('h1', { hasText: 'Terminal Locked' }).isVisible();
    if (isLocked) {
      await page.mouse.click(10, 10);
      await page.waitForTimeout(500);

      await page.waitForSelector('button:has-text("1")');
      await page.getByRole('button', { name: '1', exact: true }).click();
      await page.getByRole('button', { name: '2', exact: true }).click();
      await page.getByRole('button', { name: '3', exact: true }).click();
      await page.getByRole('button', { name: '4', exact: true }).click();
      await expect(page.locator('h1', { hasText: 'Fatima' })).toBeVisible({ timeout: 10000 });
    }

    // Go offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Verify offline banner shows up
    await expect(page.locator('text=Offline Mode')).toBeVisible({ timeout: 5000 });

    // Verify products exist or add quick charge
    await expect(page.getByRole('button', { name: 'Quick Charge $50' })).toBeVisible();
    await page.getByRole('button', { name: 'Quick Charge $50' }).click();

    // Select Tap to Pay
    await expect(page.getByRole('button', { name: 'Tap to Pay' })).toBeVisible();
    await page.getByRole('button', { name: 'Tap to Pay' }).click();

    // Wait for the Confirm & Tap button and click it to process offline
    await expect(page.getByRole('button', { name: /Confirm & Tap/ })).toBeVisible();
    await page.getByRole('button', { name: /Confirm & Tap/ }).click();

    // Verify it was queued locally (Success screen appears)
    await expect(page.locator('h2', { hasText: 'Payment Successful!' })).toBeVisible({ timeout: 15000 });

    // We can also verify that the transaction is in localStorage actions queue
    // Note: IndexedDB is used by offlineQueue.ts. But the E2E flow above checks if we can use our indexedDB offlineQueue logic.
    // Wait for DB to be updated
    await page.waitForTimeout(1000);

    const queueLength = await page.evaluate(async () => {
       const db = await new Promise((resolve, reject) => {
         const req = window.indexedDB.open("OHC_Offline_Queue", 1);
         req.onsuccess = () => resolve(req.result);
         req.onerror = () => reject(req.error);
       });
       return new Promise((resolve, reject) => {
         const tx = (db as IDBDatabase).transaction("actions", "readonly");
         const store = tx.objectStore("actions");
         const req = store.count();
         req.onsuccess = () => resolve(req.result);
         req.onerror = () => reject(req.error);
       });
    });
    expect(queueLength).toBeGreaterThan(0);

    // Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Wait for the sync to complete and verify the queue is empty
    await expect(async () => {
      const qLen = await page.evaluate(async () => {
         const db = await new Promise((resolve, reject) => {
           const req = window.indexedDB.open("OHC_Offline_Queue", 1);
           req.onsuccess = () => resolve(req.result);
           req.onerror = () => reject(req.error);
         });
         return new Promise((resolve, reject) => {
           const tx = (db as IDBDatabase).transaction("actions", "readonly");
           const store = tx.objectStore("actions");
           const req = store.count();
           req.onsuccess = () => resolve(req.result);
           req.onerror = () => reject(req.error);
         });
      });
      expect(qLen).toBe(0);
    }).toPass({ timeout: 15000 });
  });
});
