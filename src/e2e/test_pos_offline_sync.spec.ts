import { test, expect } from './fixtures';

test.describe('Offline-Tolerant POS Terminal Checkout', () => {
  test('POS terminal queues transaction when offline and syncs when online', async ({ memberPage, context }) => {
    // Navigate to the POS UI served by Tauri web
    await memberPage.goto('/ui/pos.html');

    // Enter a quick charge amount using UI
    // Ensure display initializes to $0.00
    await expect(memberPage.locator('#amount-display')).toHaveText('$0.00');

    // Tap 5, 0, 0, 0 to create $50.00 charge
    await memberPage.locator('button.num-btn', { hasText: '5' }).first().click();
    await memberPage.locator('button.num-btn', { hasText: '0' }).nth(0).click();
    await memberPage.locator('button.num-btn', { hasText: '0' }).nth(0).click();
    await memberPage.locator('button.num-btn', { hasText: '0' }).nth(0).click();

    await expect(memberPage.locator('#amount-display')).toHaveText('$50.00');

    // Click "Charge"
    await memberPage.locator('#charge-btn').click();

    // Ensure overlay is visible
    await expect(memberPage.locator('#tap-overlay')).toBeVisible();
    await expect(memberPage.locator('#tap-amount-subtitle')).toHaveText('$50.00');

    // Set network to offline BEFORE simulating tap
    await context.setOffline(true);
    await memberPage.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Ensure network indicator says "Working Offline"
    await expect(memberPage.locator('#network-status-text')).toHaveText('Working Offline', { timeout: 5000 });

    // Click Simulate Tap Button
    await memberPage.locator('#simulate-tap-btn').click();

    // Verify it drops back to receipt screen showing offline queue message
    await expect(memberPage.locator('#receipt-screen')).toBeVisible();
    await expect(memberPage.locator('.receipt-text')).toHaveText('Payment saved offline. Will sync when network is restored.');

    // Check IndexedDB
    const queuedTxs = await memberPage.evaluate(async () => {
        return new Promise((resolve) => {
            const request = window.indexedDB.open("OHC_Offline_Queue", 1);
            request.onsuccess = (e: any) => {
                const db = e.target.result;
                const tx = db.transaction("actions", "readonly");
                const store = tx.objectStore("actions");
                const all = store.getAll();
                all.onsuccess = () => resolve(all.result);
            };
            request.onerror = () => resolve([]);
        });
    });

    expect((queuedTxs as any[]).length).toBeGreaterThan(0);
    expect((queuedTxs as any[])[0].amount).toBe(5000);

    // Click new sale to reset view
    await memberPage.getByRole('button', { name: 'New Sale' }).click();

    // Make network online
    await context.setOffline(false);
    await memberPage.evaluate(() => window.dispatchEvent(new Event('online')));

    // Wait for the sync to complete and the local storage to be cleared
    await memberPage.waitForFunction(() => {
        return new Promise((resolve) => {
            const request = window.indexedDB.open("OHC_Offline_Queue", 1);
            request.onsuccess = (e: any) => {
                const db = e.target.result;
                const tx = db.transaction("actions", "readonly");
                const store = tx.objectStore("actions");
                const all = store.getAll();
                all.onsuccess = () => resolve(all.result.length === 0);
            };
            request.onerror = () => resolve(true);
        });
    }, { timeout: 15000 });

    // Verify online sync completion
    await expect(memberPage.locator('#network-status-indicator')).toBeHidden();
  });
});
