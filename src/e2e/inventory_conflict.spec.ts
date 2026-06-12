import { test, expect } from '@playwright/test';

test.describe('Distributed POS Sync Architecture', () => {

  test('Persona: Priya prevents double-booking via POS Terminal Redlock', async ({ page, context }) => {
    const tenantId = 'e2e-tenant';
    const productId = `product-${Date.now()}`;
    const productName = `E2E Test Mug ${Date.now()}`;

    // 1. Setup isolated data context via backend API exactly as previously done to maintain hermeticity
    const createRes = await page.request.post('/api/v1/catalog/product', {
        data: {
          id: productId,
          name: productName,
          inventory_count: 1,
          price_cents: 1500,
          currency: 'USD'
        },
        headers: {
          'Content-Type': 'application/json',
          'x-spiffe-id': 'spiffe://ohc/org/e2e-tenant/agent/browser'
        }
    });
    expect(createRes.ok()).toBe(true);

    // Setup online customer context
    const customerPage = await context.newPage();

    // 2. Priya logs in and goes to POS terminal
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button[type="submit"]');

    await page.goto('/pos/terminal');

    // Wait for PIN terminal
    await expect(page.locator('text=Terminal Locked')).toBeVisible();
    await page.fill('input[type="password"]', '1234');
    await page.click('button:has-text("Unlock")');

    // Wait for inventory to load
    await expect(page.locator('text=Product Catalog')).toBeVisible();

    // Select the product we just created
    await page.locator(`button:has-text("${productName}")`).first().click();

    // Wait for StripeTerminalClient to say "Reserving inventory..." or show Available Rewards
    await expect(page.locator('text=Available Rewards').or(page.locator('text=Reserving inventory'))).toBeVisible();

    // The Operations Agent implicitly coordinates the lock. When the online customer tries to buy the same item:
    // Simulate online customer attempting to checkout the same item
    await customerPage.goto(`/checkout?product=${encodeURIComponent(productName)}&tier=starter`);

    // We expect the customer page to show the checkout failure message
    // "Item just sold out" or "Checkout is temporarily unavailable"
    await expect(customerPage.locator('text=Item just sold out').or(customerPage.locator('text=Checkout is temporarily unavailable'))).toBeVisible({ timeout: 15000 });

    // The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"
    await page.goto('/team/chat');
    // We expect the low stock alert to now be generated and visible because stock dropped
    await expect(page.locator('text=sold out. Would you like to draft a restock order?')).toBeVisible({ timeout: 15000 });
  });
});
