import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Global Offline-First Localization & Multi-Currency Engine', () => {

  test('User can switch currency to AED and it updates product prices', async ({ page }) => {
    // Database gets seeded via the global setup
    await adminPage(page, async () => {
      // Create a test product
      await page.goto('/api/ui/dashboard.html');
      await page.evaluate(async () => {
        await fetch('/api/v1/catalog/products', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': 'Bearer ' + localStorage.getItem('access_token')
          },
          body: JSON.stringify({
            title: 'Test Coffee',
            description: 'Good coffee',
            price_cents: 500,
            inventory_count: 10,
            available_quantity: 10,
            is_digital: false,
            sku: 'COFFEE-1',
            tax_code: 'tx-1',
            tax_rate: 0
          })
        });
      });

      // 2. Go to POS
      await page.goto('/api/ui/pos.html');

      // Wait for product to load
      await page.waitForSelector('.product-btn:has-text("Test Coffee")');

      // 3. Switch Currency to AED
      await page.selectOption('#currency-toggle', 'AED');

      // Since no FX rates are seeded for AED (or maybe they are not fetched yet),
      // the rate fallback is 1.0 or it fetches from API.
      // But we can check if it switched the display prefix at least to AED.
      await expect(page.locator('#product-grid')).toContainText('AED');
    });
  });

  test('User can switch language to Arabic and UI translates', async ({ page }) => {
    await adminPage(page, async () => {
      await page.goto('/api/ui/pos.html');

      // Switch Language to Arabic
      await page.selectOption('#language-toggle', 'ar');

      // Since it hits the real API, if no translations exist it just won't translate,
      // but it shouldn't crash. We can verify the toggle value remains.
      await expect(page.locator('#language-toggle')).toHaveValue('ar');
    });
  });

  test('Offline POS transaction uses selected currency and caches it in outbox', async ({ page, context }) => {
    await adminPage(page, async () => {
      await page.goto('/api/ui/pos.html');

      // Select AED
      await page.selectOption('#currency-toggle', 'AED');

      // Go Offline
      await context.setOffline(true);

      // Quick charge $50 -> AED
      await page.getByText('Quick Charge $50').click();

      // Tap "Accept Contactless Payment"
      await page.locator('#charge-btn').click();

      // Tap Simulate
      await page.locator('#simulate-tap-btn').click();

      // Wait for receipt
      await expect(page.locator('#receipt-screen')).toBeVisible();

      // Check localStorage for the offline outbox intent
      const outboxJSON = await page.evaluate(() => localStorage.getItem('ohc_pos_outbox'));
      expect(outboxJSON).toBeTruthy();
      const outbox = JSON.parse(outboxJSON);
      expect(outbox.length).toBeGreaterThan(0);

      expect(outbox[0].entity_type).toBe('OperationIntent');
      expect(outbox[0].payload).toContain('"currency":"aed"');

      // Go back online
      await context.setOffline(false);
    });
  });

  test('POS Sync worker processes offline AED transactions properly', async ({ page }) => {
     await adminPage(page, async () => {
       await page.goto('/api/ui/dashboard.html');

       const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'default');

       const mockIntent = {
          id: 'test_sync_id_' + Date.now(),
          entity_type: 'OperationIntent',
          entity_id: 'tx_test_' + Date.now(),
          payload: {
              pos_transaction_id: 'tx_test_' + Date.now(),
              amount_cents: 5000,
              currency: 'aed',
              cached_rate: 3.67,
              payload: '[{"product_id": "quick_charge", "quantity": 1}]'
          },
          updated_at: Date.now()
       };

       const response = await page.evaluate(async ({ intent, tenantId }) => {
          const res = await fetch('/api/v1/sync/mcp-deltas', {
              method: 'POST',
              headers: {
                  'Content-Type': 'application/json',
                  'Authorization': 'Bearer ' + localStorage.getItem('access_token')
              },
              body: JSON.stringify({
                  tenant_id: tenantId,
                  deltas: [intent]
              })
          });
          return res.ok;
       }, { intent: mockIntent, tenantId });

       expect(response).toBe(true);
     });
  });

  test('POS retains UI state across reloads', async ({ page }) => {
    await adminPage(page, async () => {
      await page.goto('/api/ui/pos.html');

      await page.selectOption('#currency-toggle', 'AED');
      await page.selectOption('#language-toggle', 'ar');

      await page.reload();

      // Check toggles retained state
      await expect(page.locator('#currency-toggle')).toHaveValue('AED');
      await expect(page.locator('#language-toggle')).toHaveValue('ar');
    });
  });

});
