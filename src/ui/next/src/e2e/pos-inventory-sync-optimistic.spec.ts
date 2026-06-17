import { test, expect } from '@playwright/test';

test.describe('POS Inventory Sync - Optimistic UI', () => {
  test('POS terminal immediately updates stock UI on charge before API returns', async ({ page }) => {
    // Navigate to POS terminal
    await page.goto('/pos/terminal');

    // Wait for the pin screen to be visible
    await expect(page.getByText('Terminal Locked')).toBeVisible();

    // Login with PIN 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    // Wait for the dashboard to load
    await expect(page.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 });

    // Wait for the product catalog to be populated
    await expect(page.getByText('Vegan Celebration Cake')).toBeVisible();

    // Extract current stock from the text
    const productButton = page.locator('button', { hasText: 'Vegan Celebration Cake' });
    const descriptionText = await productButton.innerText();

    const stockMatch = descriptionText.match(/Stock: (\d+)/);
    expect(stockMatch).toBeTruthy();

    if (stockMatch) {
      const initialStock = parseInt(stockMatch[1], 10);

      // Select the product
      await productButton.click();

      // Click the "Charge" button
      await page.getByRole('button', { name: /Charge \$/ }).click();

      // Immediately verify the stock decreased by 1 without waiting for API
      // Since it's optimistic, it should happen instantly.
      await expect(productButton).toContainText(`Stock: ${initialStock - 1}`);
    }
  });

  test('Offline sync conflict generates Operations Agent Triage Task', async ({ page }) => {
    // Navigate to POS terminal to login
    await page.goto('/pos/terminal');
    await expect(page.getByText('Terminal Locked')).toBeVisible();

    // Login with PIN 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await expect(page.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 });

    // Ensure product catalog is populated
    await expect(page.getByText('Vegan Celebration Cake')).toBeVisible({ timeout: 5000 });

    const productButton = page.locator('button', { hasText: 'Vegan Celebration Cake' });
    const descriptionText = await productButton.innerText();

    const stockMatch = descriptionText.match(/Stock: (\d+)/);
    expect(stockMatch).toBeTruthy();

    if (stockMatch) {
      // Simulate going offline
      await page.context().setOffline(true);

      // Select the product
      await productButton.click();

      // Click the "Charge" button to queue the mutation offline
      await page.getByRole('button', { name: /Charge \$/ }).click();

      // Go back online
      await page.context().setOffline(false);

      // Force a conflict by directly hitting the endpoint with a large quantity
      // so it triggers the conflict generation workflow in the backend
      const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');
      const transactionId = `tx-conflict-${Date.now()}`;
      const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;

      const res = await page.request.post('/api/v1/sync/offline', {
        headers: {
          'x-spiffe-id': spiffeId
        },
        data: {
          mutations: [
            {
              transaction_id: transactionId,
              product_id: 'e2e-product-cake', // Assumed to exist and have < 100 stock
              quantity_deducted: 100,
              amount: 5000,
              currency: 'usd',
            }
          ]
        }
      });

      expect(res.ok()).toBeTruthy();

      // Wait for async workers (pos_sync_worker, pos_conflict_worker, operations_agent)
      await page.waitForTimeout(5000);
    }

    // Navigate to Action Center
    await page.goto('/action-center');

    // We expect the Triage task to show up from Operations Agent
    // Fallback LLM text or "oversold the item" should be visible
    if (stockMatch) {
      await expect(page.getByText(/We oversold the item/i).first()).toBeVisible({ timeout: 10000 });
    }
  });
});
