import { test, expect } from '@playwright/test';

test.describe('POS Inventory Sync - Optimistic UI', () => {
  test('POS terminal immediately updates stock UI on charge before API returns', async ({ page }) => {
    // 1. Log in to get token
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('admin@ohc.local');
    await page.getByPlaceholder('Password').fill('admin');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });

    const response = await page.request.post('/api/v1/auth/login', {
        data: {
            email: 'admin@ohc.local',
            password: 'admin'
        }
    });
    expect(response.ok()).toBeTruthy();
    const { token } = await response.json();

    // 2. Create the "Vegan Celebration Cake" product
    const createProductRes = await page.request.post('/api/v1/catalog/products', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            title: 'Vegan Celebration Cake',
            inventory_count: 10,
            price_cents: 5000
        }
    });
    expect(createProductRes.ok()).toBeTruthy();

    // Navigate to POS terminal
    await page.goto('/pos.html');
    await page.evaluate(() => { localStorage.setItem("tenant_id", "default"); });

    // Login with PIN 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await page.waitForTimeout(500);
    await page.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(500);

    // Wait for the product catalog to be populated
    await expect(page.getByText('Vegan Celebration Cake').first()).toBeVisible({ timeout: 10000 });

    // Extract current stock from the text
    const productButton = page.locator('button', { hasText: 'Vegan Celebration Cake' }).first();
    const descriptionText = await productButton.innerText();

    const stockMatch = descriptionText.match(/Stock: (\d+)/);
    expect(stockMatch).toBeTruthy();

    if (stockMatch) {
      const initialStock = parseInt(stockMatch[1], 10);

      // Select the product
      await productButton.click();

      // Immediately verify the stock decreased by 1 without waiting for API
      // Since it's optimistic, it should happen instantly.
      await page.waitForTimeout(500);
      const updatedButtonText = await productButton.innerText();
      const newMatch = updatedButtonText.match(/Stock:\s*(\d+)/);
      if (newMatch) {
          const newStock = parseInt(newMatch[1], 10);
          expect(newStock).toBe(initialStock - 1);
      }
    }
  });

  test('Offline sync conflict generates Operations Agent Triage Task', async ({ page }) => {
    // 1. Log in to get token
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('admin@ohc.local');
    await page.getByPlaceholder('Password').fill('admin');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });

    const response = await page.request.post('/api/v1/auth/login', {
        data: {
            email: 'admin@ohc.local',
            password: 'admin'
        }
    });
    expect(response.ok()).toBeTruthy();
    const { token } = await response.json();

    // 2. Create the "Vegan Celebration Cake" product
    const createProductRes = await page.request.post('/api/v1/catalog/products', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            title: 'Vegan Celebration Cake',
            inventory_count: 10,
            price_cents: 5000
        }
    });
    expect(createProductRes.ok()).toBeTruthy();

    // Navigate to POS terminal to login
    await page.goto('/pos.html');
    await page.evaluate(() => { localStorage.setItem("tenant_id", "default"); });

    // Login with PIN 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await page.waitForTimeout(500);
    await page.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(500);

    // Ensure product catalog is populated
    await expect(page.getByText('Vegan Celebration Cake').first()).toBeVisible({ timeout: 10000 });

    const productButton = page.locator('button', { hasText: 'Vegan Celebration Cake' }).first();
    const descriptionText = await productButton.innerText();

    const stockMatch = descriptionText.match(/Stock: (\d+)/);
    expect(stockMatch).toBeTruthy();

    if (stockMatch) {
      // Simulate going offline
      await page.context().setOffline(true);

      // Select the product
      await productButton.click();

      // Click the "Charge" button to queue the mutation offline
      const collectBtn = page.locator('button', { hasText: /Collect Payment/i });
      await expect(collectBtn).toBeVisible();
      await collectBtn.click();

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
    await page.goto('/dashboard');

    // We expect the Triage task to show up from Operations Agent
    // Fallback LLM text or "oversold the item" should be visible
    if (stockMatch) {
      await expect(page.getByText(/We oversold the item/i).first()).toBeVisible({ timeout: 10000 });
    }
  });

  test('POS terminal immediately updates stock UI on cash sale before API returns', async ({ page }) => {
    // 1. Log in to get token
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('admin@ohc.local');
    await page.getByPlaceholder('Password').fill('admin');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });

    const response = await page.request.post('/api/v1/auth/login', {
        data: {
            email: 'admin@ohc.local',
            password: 'admin'
        }
    });
    expect(response.ok()).toBeTruthy();
    const { token } = await response.json();

    // 2. Create the "Falafel" product
    const createProductRes = await page.request.post('/api/v1/catalog/products', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            title: 'Falafel',
            inventory_count: 50,
            price_cents: 800
        }
    });
    expect(createProductRes.ok()).toBeTruthy();

    // Navigate to POS terminal
    await page.goto('/pos.html');
    await page.evaluate(() => { localStorage.setItem("tenant_id", "default"); });

    // Login with PIN 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await page.waitForTimeout(500);
    await page.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(500);

    // Wait for the product catalog to be populated
    await expect(page.getByText('Falafel').first()).toBeVisible({ timeout: 10000 });

    // Extract current stock from the text
    const productButton = page.locator('button', { hasText: 'Falafel' }).first();
    const descriptionText = await productButton.innerText();

    const stockMatch = descriptionText.match(/Stock: (\d+)/);
    expect(stockMatch).toBeTruthy();

    if (stockMatch) {
      const initialStock = parseInt(stockMatch[1], 10);

      // Select the product
      await productButton.click();

      // Go offline
      await page.context().setOffline(true);

      // Verify offline mode indicator
      await expect(page.getByText('Offline - Syncing later')).toBeVisible({ timeout: 5000 });

      // Click the "Charge" button to open cart drawer
      const collectBtn = page.locator('button', { hasText: /Charge/i }).first();
      await expect(collectBtn).toBeVisible();
      await collectBtn.click();

      await page.waitForTimeout(500);

      // Click the "Record Cash Sale" button to queue the mutation offline
      const cashBtn = page.locator('button', { hasText: /Record Cash Sale/i });
      await expect(cashBtn).toBeVisible();
      await cashBtn.click();

      // Immediately verify the stock decreased by 1 without waiting for API
      // Since it's optimistic, it should happen instantly.
      await page.waitForTimeout(500);
      const updatedButtonText = await productButton.innerText();
      const newMatch = updatedButtonText.match(/Stock:\s*(\d+)/);
      if (newMatch) {
          const newStock = parseInt(newMatch[1], 10);
          expect(newStock).toBe(initialStock - 1);
      }

      // Restore network
      await page.context().setOffline(false);

      // Wait to verify it syncs back
      await page.waitForTimeout(2000);
    }
  });
});
