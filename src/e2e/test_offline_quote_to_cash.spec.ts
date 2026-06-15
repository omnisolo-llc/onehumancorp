import { test, expect } from './fixtures';

test.describe('Autonomous Offline-First Agentic Quote-to-Cash Engine', () => {
  test('Owner can create quote and take deposit offline, and it syncs online', async ({ page, context }) => {
    // 1. App opened in airplane mode
    await context.setOffline(true);
    await page.goto('/quote-intake');

    // Simulate the offline environment trigger
    await page.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    // Ensure the offline indicator is visible
    await expect(page.locator('text=Saved Offline')).toBeVisible();

    // 2. Input job details (mock voice)
    const voiceBtn = page.getByRole('button', { name: 'Tell OHC about the job' });
    await expect(voiceBtn).toBeVisible();
    await voiceBtn.click();

    // 3. Generate quote locally (the button simulates taking voice and creating quote draft)
    const quoteCard = page.getByTestId('quote-draft-card');
    await expect(quoteCard).toBeVisible({ timeout: 10000 });
    await expect(quoteCard).toContainText('$150.00');

    // 4. Collect Deposit (offline tap-to-pay)
    const collectBtn = page.getByTestId('collect-deposit-btn');
    await expect(collectBtn).toBeVisible();
    await collectBtn.click();

    // 5. Store locally
    await expect(page.getByTestId('payment-saved-offline-msg')).toBeVisible({ timeout: 5000 });

    // Assert the transaction was written to IndexedDB
    const queuedActions = await page.evaluate(async () => {
       const actions: any[] = [];
       const db = await new Promise<IDBDatabase>((resolve, reject) => {
         const req = window.indexedDB.open("OHC_Offline_Queue", 1);
         req.onsuccess = (e: any) => resolve(e.target.result);
         req.onerror = () => reject(req.error);
       });

       return new Promise<any[]>((resolve, reject) => {
         const tx = db.transaction(["actions"], "readonly");
         const store = tx.objectStore("actions");
         const request = store.getAll();
         request.onsuccess = () => resolve(request.result);
         request.onerror = () => reject(request.error);
       });
    });

    // Ensure it's in the queue
    expect(queuedActions.some(action => action.type === 'offline_quote_deposit')).toBeTruthy();

    // 6. Network restored -> Sync to backend
    await context.setOffline(false);
    await page.evaluate(() => {
      window.dispatchEvent(new Event('online'));
    });

    // Verify it syncs successfully. The sync manager should remove the item from the IndexedDB queue
    await page.waitForFunction(async () => {
      const db = await new Promise<IDBDatabase>((resolve, reject) => {
         const req = window.indexedDB.open("OHC_Offline_Queue", 1);
         req.onsuccess = (e: any) => resolve(e.target.result);
         req.onerror = () => reject(req.error);
       });

       const actions = await new Promise<any[]>((resolve, reject) => {
         const tx = db.transaction(["actions"], "readonly");
         const store = tx.objectStore("actions");
         const request = store.getAll();
         request.onsuccess = () => resolve(request.result);
         request.onerror = () => reject(request.error);
       });

       return actions.filter(a => a.type === 'offline_quote_deposit').length === 0;
    }, { timeout: 15000 });

    // Ensure the queue was cleared successfully
    const afterSyncActions = await page.evaluate(async () => {
      const db = await new Promise<IDBDatabase>((resolve, reject) => {
         const req = window.indexedDB.open("OHC_Offline_Queue", 1);
         req.onsuccess = (e: any) => resolve(e.target.result);
         req.onerror = () => reject(req.error);
       });

       return new Promise<any[]>((resolve, reject) => {
         const tx = db.transaction(["actions"], "readonly");
         const store = tx.objectStore("actions");
         const request = store.getAll();
         request.onsuccess = () => resolve(request.result);
         request.onerror = () => reject(request.error);
       });
    });

    expect(afterSyncActions.filter(a => a.type === 'offline_quote_deposit').length).toBe(0);
  });
});
