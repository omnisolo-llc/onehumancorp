import { test, expect } from '@playwright/test';

test.describe('Unified Inventory Ledger CUJ', () => {
  test('Priya views, adjusts, and sells a product in POS mode', async ({ page }) => {
    // We are simulating the CUJ from the issue description:
    // "1. The business owner (Priya) opens the mobile UI (375px)."
    await page.setViewportSize({ width: 375, height: 667 });

    const tenantId = 'priya-boutique-tenant';
    const productId = 'priya-summer-dress-medium';

    // Log in to get token (using default test auth flow)
    await page.goto('/login');
    await page.getByPlaceholder('Email address').fill('admin@ohc.local');
    await page.getByPlaceholder('Password').fill('admin');
    await page.getByRole('button', { name: 'Sign In' }).click();
    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });

    const response = await page.request.post('/api/v1/auth/login', {
        data: {
            email: 'admin@ohc.local',
            password: 'admin'
        }
    });
    expect(response.ok()).toBeTruthy();
    const { token } = await response.json();

    // Set up the product in the backend with 5 stock
    const createProductRes = await page.request.post('/api/v1/catalog/products', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            id: productId,
            title: 'Summer Dress - Medium',
            inventory_count: 5,
            price_cents: 8000
        }
    });
    expect(createProductRes.ok()).toBeTruthy();

    // 2. Priya navigates to the Inventory section and sees a list of products.
    await page.goto('/inventory');
    await page.evaluate((tenant) => {
        localStorage.setItem('tenant_id', tenant);
    }, tenantId);

    // We need to reload so tenant_id is picked up by client-side fetch, or the API call picks up 'priya-boutique-tenant'
    await page.goto('/inventory');

    await expect(page.locator('text=Variants & Products')).toBeVisible({ timeout: 10000 });

    // Wait for product to load
    await expect(page.getByText('Summer Dress - Medium').first()).toBeVisible({ timeout: 15000 });

    // We expect initial stock to be 5
    const row = page.locator('tr').filter({ hasText: 'Summer Dress - Medium' });
    await expect(row.locator('td').nth(1)).toHaveText('5');

    // 3. Priya manually adjusts the stock of "Summer Dress - Medium" from 5 to 4.
    await row.getByRole('button', { name: '-' }).click();

    // It should optimistically update
    await expect(row.locator('td').nth(1)).toHaveText('4');

    // 4. Priya opens a "POS" simulator view, processes an in-person sale for the dress, and the stock goes from 4 to 3.
    const posSimulator = page.locator('.app-panel').filter({ hasText: 'POS Simulator' });
    const simulatorRow = posSimulator.locator('div').filter({ hasText: 'Summer Dress - Medium' });
    await simulatorRow.getByRole('button', { name: 'Sell In-Person (POS)' }).click();

    // 5. The UI updates instantly.
    // Optimistic update should make it 3
    await expect(row.locator('td').nth(1)).toHaveText('3');

    // Wait for successful pos simulate message
    await expect(page.getByText('Sale completed successfully.')).toBeVisible({ timeout: 15000 });

    // Reload page to verify backend persisted changes via Centralized Inventory Ledger
    await page.goto('/inventory');
    const rowReloaded = page.locator('tr').filter({ hasText: 'Summer Dress - Medium' });
    await expect(rowReloaded.locator('td').nth(1)).toHaveText('3', { timeout: 15000 });
  });
});
