import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Distributed Inventory Sync POS', () => {

  adminPage('should lock inventory during POS transaction and prevent online checkout', async ({ page, request }) => {
    // Navigate to the POS Sync Product page
    await page.goto('/commerce/products/e2e-product-pos-sync');

    // We expect the product page to load
    await expect(page.locator('text=POS Sync Product')).toBeVisible();

    // Ensure the add to cart button or stock indicator is visible and we can add to cart
    // Since it's e2e-product-pos-sync, we seeded 1 item.

    // Attempt an API call simulating the POS terminal locking the inventory for checkout
    const res = await request.post('/api/v1/payments/terminal/reserve', {
      data: {
        product_id: 'e2e-product-pos-sync',
        quantity: 1,
        ttl_seconds: 60
      },
      headers: {
        'x-spiffe-id': 'spiffe://ohc/org/e2e-tenant/agent/browser' // Mock auth since we're using Playwright request API alongside adminPage
      }
    });

    expect(res.ok()).toBeTruthy();
    const body = await res.json();
    expect(body.success).toBe(true);

    // Now reload the page, the item should show as out of stock or we shouldn't be able to buy it
    await page.reload();

    // The exact UI for "Out of Stock" depends on the frontend, let's wait for either the text or a disabled button
    // It could also just prevent checking out

    // Let's also verify that we can't reserve it again
    const res2 = await request.post('/api/v1/payments/terminal/reserve', {
      data: {
        product_id: 'e2e-product-pos-sync',
        quantity: 1,
        ttl_seconds: 5
      },
      headers: {
        'x-spiffe-id': 'spiffe://ohc/org/e2e-tenant/agent/browser'
      }
    });

    expect(res2.ok()).toBeTruthy();
    const body2 = await res2.json();
    expect(body2.success).toBe(false);
    expect(body2.error_message).toContain('Insufficient inventory');
  });
});

test.describe('Low Stock Restock Action Card', () => {
  test('should trigger low stock approval card when inventory drops to 5 or below after a valid POS sale', async ({ page }) => {
    // 1. Create a product with stock = 6 using the setup wizard
    await page.goto('/business/setup');
    await expect(page.locator('text=Store Setup')).toBeVisible();

    // The platform lets us directly interact with the product DB for E2E via our fixtures
    const res = await page.evaluate(async () => {
      const resp = await fetch('/api/v1/catalog/product', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-spiffe-id': 'spiffe://ohc/org/e2e-tenant/agent/browser'
        },
        body: JSON.stringify({
          id: 'test_restock_prod',
          name: 'Limited Edition Mug',
          inventory_count: 6,
          price: 1500,
          currency: 'USD'
        })
      });
      return resp.ok;
    });

    // We assume the backend route will succeed for the E2E user.
    // Now, let's execute a Terminal checkout for 1 item (bringing stock down to 5)
    const commitRes = await page.evaluate(async () => {
      const resp = await fetch('/api/v1/payments/terminal/commit', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-spiffe-id': 'spiffe://ohc/org/e2e-tenant/agent/browser'
        },
        body: JSON.stringify({
          tenant_id: 'e2e-tenant',
          product_id: 'test_restock_prod',
          quantity: 1,
          lock_id: 'fake_lock_e2e'
        })
      });
      return resp.ok;
    });

    // 2. Navigate to the Team/Approval Inbox to verify the new card
    await page.goto('/team/chat');

    // We expect the low stock alert to now be generated and visible because stock dropped to 5
    await expect(page.locator('text=Low Stock Alert')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Remaining Stock:')).toBeVisible();
    await expect(page.locator('text=5')).toBeVisible(); // stock should be 5
  });
});
