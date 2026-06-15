import { test, expect } from '@playwright/test';

test.describe('Offline-First Agentic Quote-to-Cash', () => {

  test('generates quote offline, collects deposit, and syncs when online', async ({ page, context }) => {
    // Navigate to the field ops quote-to-cash page
    await page.goto('http://127.0.0.1:3000/field-ops/quote-to-cash');

    // Simulate offline mode
    await context.setOffline(true);
    // Trigger offline event manually in the browser since setOffline sometimes doesn't fire events immediately on page context in Playwright
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Verify offline indicator is visible
    await expect(page.locator('[data-testid="offline-indicator"]')).toBeVisible();

    // Intake input
    await page.locator('[data-testid="intake-textarea"]').fill('We need to fix the water heater in the basement.');
    await page.locator('[data-testid="generate-quote-btn"]').click();

    // Verify local AI draft quote is generated
    await expect(page.locator('[data-testid="draft-quote-card"]')).toBeVisible();
    await expect(page.locator('text=$500.00')).toBeVisible(); // total amount
    await expect(page.locator('text=Collect $100.00 Deposit')).toBeVisible(); // required deposit

    // Simulate collecting deposit (offline)
    await page.locator('[data-testid="collect-deposit-btn"]').click();

    // Verify "Saved Offline" confirmation
    await expect(page.locator('[data-testid="saved-offline-msg"]')).toBeVisible();
    await expect(page.locator('[data-testid="draft-quote-card"]')).not.toBeVisible();

    // Now come back online
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Ensure the offline queue emptied (meaning it synced to the backend successfully)
    // Wait for the background sync to finish.
    await page.waitForTimeout(3000);

    const queueLength = await page.evaluate(async () => {
        const { getActions } = await import('../../ui/next/src/app/utils/offlineQueue.ts').catch(() => ({ getActions: async () => [] }));
        // Note: the above dynamic import path might fail depending on Playwright config.
        // A safer way is checking indexedDB directly.
        return new Promise<number>((resolve) => {
            const req = window.indexedDB.open("OHC_Offline_Queue", 1);
            req.onsuccess = (e: any) => {
                const db = e.target.result;
                if (!db.objectStoreNames.contains("actions")) {
                    resolve(0);
                    return;
                }
                const tx = db.transaction("actions", "readonly");
                const store = tx.objectStore("actions");
                const countReq = store.count();
                countReq.onsuccess = () => resolve(countReq.result);
            };
            req.onerror = () => resolve(0);
        });
    });

    expect(queueLength).toBe(0);
  });
});
