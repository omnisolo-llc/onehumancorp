import { test, expect } from '../fixtures';
import { v4 as uuidv4 } from 'uuid';

test.describe('POS Offline Sync', () => {
  test('offline to online sync flow deduplicates correctly via /api/v1/pos/sync', async ({ page, request, memberPage }) => {
    // 1. Log in via UI using memberPage fixture
    await memberPage.goto('/dashboard');
    await expect(memberPage.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });

    // Let's create a product via the UI
    await memberPage.click('text=Products & Offers');
    await memberPage.click('text=Add Product');

    const productName = `Offline Sync Item ${uuidv4().substring(0, 5)}`;
    await memberPage.fill('input[name="title"]', productName);
    await memberPage.fill('input[name="price"]', '15.00');

    // We need to enable inventory tracking
    await memberPage.click('text=Track Inventory');
    await memberPage.fill('input[name="inventory_count"]', '10');

    await memberPage.click('button:has-text("Save")');
    await expect(memberPage.locator(`text=${productName}`).first()).toBeVisible({ timeout: 10000 });

    // Since we created it, we can fetch it via the API using the page's cookies.
    const productsRes = await memberPage.request.get('/api/v1/catalog/products');
    const products = await productsRes.json();
    const product = products.products.find((p: any) => p.name === productName || p.title === productName);
    expect(product).toBeDefined();

    const productId = product.id;
    const transactionId = uuidv4();
    const clientMutationId = uuidv4();

    const syncPayload = {
      mutations: [
        {
          transaction_id: transactionId,
          timestamp: new Date().toISOString(),
          product_id: productId,
          quantity_deducted: 2,
          amount: 3000, // $30 for 2
          payment_method: 'card',
          currency: 'usd',
          mutation_type: 'offline_sale',
          client_mutation_id: clientMutationId
        }
      ]
    };

    // Simulate the frontend sending the sync request
    const syncRes = await memberPage.request.post('/api/v1/pos/sync', {
      data: syncPayload
    });

    expect(syncRes.status()).toBe(200);
    const syncResJson = await syncRes.json();
    expect(syncResJson.success).toBe(true);

    // 4. Verify inventory updates correctly on the dashboard
    // Reload the products page
    await memberPage.goto('/dashboard');
    await memberPage.click('text=Products & Offers');

    // Wait for the background job to process
    await memberPage.waitForTimeout(3000);

    await memberPage.click(`text=${productName}`);

    // We should see 8 in stock (10 - 2)
    await expect(memberPage.locator('input[name="inventory_count"]')).toHaveValue('8');

    // 5. Test idempotency
    const syncResDuplicate = await memberPage.request.post('/api/v1/pos/sync', {
      data: syncPayload
    });

    expect(syncResDuplicate.status()).toBe(200);

    await memberPage.waitForTimeout(3000);

    await memberPage.goto('/dashboard');
    await memberPage.click('text=Products & Offers');
    await memberPage.click(`text=${productName}`);

    // Inventory should still be 8, not 6
    await expect(memberPage.locator('input[name="inventory_count"]')).toHaveValue('8');
  });
});
