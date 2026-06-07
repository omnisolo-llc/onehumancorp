import { test, expect } from './fixtures';

test.describe('Offline POS Conflict Resolution', () => {
  test('should detect oversell, trigger Operations agent task, and notify user', async ({ page }) => {

    await page.goto('/dashboard');

    // We will just directly call the /api/v1/sync/offline with a massive deduction to trigger a conflict.
    const res = await page.evaluate(async () => {
      const resp = await fetch('/api/v1/sync/offline', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-spiffe-id': 'spiffe://ohc/org/e2e/agent/browser' // E2E mock
        },
        body: JSON.stringify({
          mutations: [
            {
              transaction_id: 'tx-conflict-test-' + Date.now(),
              product_id: 'e2e-product-cake',
              quantity_deducted: 9999, // guaranteed oversell
              amount: 3999,
              currency: 'USD'
            }
          ]
        })
      });
      return resp.ok;
    });

    expect(res).toBe(true);

    // AI tasks and notifications show up in Kairos orchestration / tasks table
    await page.goto('/kairos');

    // Wait for the task to be processed and appear
    await expect(page.locator('text=Heads up! A pop-up sale overlapped')).toBeVisible({ timeout: 15000 });
  });
});
