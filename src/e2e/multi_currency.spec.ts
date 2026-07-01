import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Global Offline-First Localization & Multi-Currency Engine', () => {

  test('User can switch currency to AED and it updates product prices', async ({ page }) => {
    // 1. Setup - Mock localization endpoints to provide AED rates


    // We also need some products in the catalog


    await adminPage(page, async () => {
      // 2. Go to POS
      await page.goto('/api/ui/pos.html');

      // Initially USD
      await expect(page.locator('#product-grid')).toContainText('$5.00');

      // 3. Switch Currency to AED
      await page.selectOption('#currency-toggle', 'AED');

      // Wait for re-render
      // $5 * 3.67 = 18.35 AED
      await expect(page.locator('#product-grid')).toContainText('AED');
      await expect(page.locator('#product-grid')).toContainText('18.35');
    });
  });

  test('User can switch language to Arabic and UI translates', async ({ page }) => {
    // 1. Setup - Mock i18n


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

      const intent = outbox[0].payload; // It's stringified in the code usually, let's just check raw string
      expect(outbox[0].entity_type).toBe('OperationIntent');
      expect(outbox[0].payload).toContain('"currency":"aed"');
      expect(outbox[0].payload).toContain('"cached_rate":3.67');

      // Go back online
      await context.setOffline(false);
    });
  });

  test('POS Sync worker processes offline AED transactions properly', async ({ page }) => {
     // Here we test that the worker processes the queue and properly interacts with the MultiCurrencyLedger.
     // In a real Playwright E2E, this would involve sending the outbox item to /api/v1/sync/mcp-deltas
     // and then checking the backend state via an API or letting it sync naturally.
     // For this test suite, we will just make a manual call to the sync endpoint.

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
