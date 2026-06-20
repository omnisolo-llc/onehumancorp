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
    await expect(page.locator('h1', { hasText: 'Manager' }).first()).toBeVisible({ timeout: 5000 });

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
      await page.locator('button', { hasText: 'Charge' }).last().click();

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

    await expect(page.locator('h1', { hasText: 'Manager' }).first()).toBeVisible({ timeout: 5000 });

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
      await page.locator('button', { hasText: 'Charge' }).last().click();

      // Go back online
      await page.context().setOffline(false);

      // Force a conflict by directly hitting the endpoint with a large quantity
      // so it triggers the conflict generation workflow in the backend
      const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');
      const transactionId = `tx-conflict-${Date.now()}`;
      const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;

      const res = await page.request.post('/api/v1/payments/terminal/sync_offline', {
        headers: {
          'x-spiffe-id': spiffeId
        },
        data: {
          session_id: 'e2e-session',
          transactions: [
            {
              id: transactionId,
              client_id: 'e2e-client',
              amount_cents: 5000,
              currency: 'usd',
              payload: JSON.stringify({ mutation: { product_id: 'e2e-product-cake', quantity_deducted: 100 } }),
              timestamp: new Date().toISOString(),
              device_signature: 'mock_secure_enclave_signature_123'
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
      await expect(page.getByText(/Supplier Reorder Draft/i).first()).toBeVisible({ timeout: 10000 });
    }
  });

  test('Simulated payment failure creates Payment Recovery task', async ({ page }) => {
    await page.goto('/pos/terminal');
    await expect(page.getByText('Terminal Locked')).toBeVisible();

    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await expect(page.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');
    const transactionId = `tx-fail-${Date.now()}`;
    const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;

    const res = await page.request.post('/api/v1/payments/terminal/sync_offline', {
      headers: {
        'x-spiffe-id': spiffeId
      },
      data: {
        session_id: 'e2e-session',
        transactions: [
          {
            id: transactionId,
            client_id: 'e2e-client',
            amount_cents: 5000,
            currency: 'usd',
            payload: JSON.stringify({ simulate_payment_failure: true, mutation: { product_id: 'e2e-product-cake', quantity_deducted: 1 } }),
            timestamp: new Date().toISOString(),
            device_signature: 'mock_secure_enclave_signature_123'
          }
        ]
      }
    });

    expect(res.ok()).toBeTruthy();

    await page.waitForTimeout(5000);

    await page.goto('/action-center');
    await expect(page.getByText(/Hi, your card at Fatima's Food Cart couldn't be processed later/i).first()).toBeVisible({ timeout: 10000 });
  });

});
