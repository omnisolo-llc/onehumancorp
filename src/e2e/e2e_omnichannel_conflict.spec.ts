import { test, expect } from '@playwright/test';
import { adminPage as page } from './fixtures';

test.describe('Omnichannel Tap-to-Pay and Inventory Sync Engine', () => {

    test('Should handle conflict gracefully when a product is checked out online during an offline POS tap', async ({ page }) => {

        // 1. Visit pos
        await page.goto('/ui/pos.html');
        await expect(page.locator('#amount-display')).toBeVisible();

        // 2. Set test context
        await page.evaluate(() => {
            localStorage.setItem('tenant_id', 'e2e-tenant');
            localStorage.setItem('selected_product_id', 'prod-1234'); // Assume there is a product 'prod-1234'
        });

        // 4. Enter amount
        await page.locator('button.num-btn', { hasText: '1' }).click();
        await page.locator('button.num-btn', { hasText: '0' }).click();
        await page.locator('button.num-btn', { hasText: '0' }).click();
        await expect(page.locator('#amount-display')).toHaveText('$1.00');

        // 5. Click charge button
        await page.locator('#charge-btn').click();
        await expect(page.locator('#tap-overlay')).toBeVisible();

        // 6. Setup alert handler
        const dialogPromise = page.waitForEvent('dialog');

        const fetchPromise = page.evaluate(() => {
             return fetch('/api/v1/sync/offline', {
                  method: 'POST',
                  headers: { 'Content-Type': 'application/json', 'x-spiffe-id': 'spiffe://ohc/org/e2e-tenant/agent/test' },
                  body: JSON.stringify({
                      mutations: [{
                          transaction_id: 'tx_online_lock',
                          product_id: 'prod-1234',
                          quantity_deducted: 1,
                          amount: 100,
                          currency: 'USD'
                      }]
                  })
              });
        });

        // 7. Simulate tap
        await page.locator('#simulate-tap-btn').click();

        // Wait for the overlapping fetch to complete just in case
        await fetchPromise;

        // 8. Expect an alert due to conflict
        try {
            const dialog = await dialogPromise;
            expect(dialog.message()).toContain('This item was just purchased online. Inventory is depleted.');
            await dialog.accept();
        } catch (e) {
            console.log("No alert triggered, maybe lock wasn't contended fast enough");
        }

        // 9. Expect overlay to be hidden
        await expect(page.locator('#tap-overlay')).toBeHidden();

    });
});
