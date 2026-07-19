import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('edge_caching_invalidation', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);

  const tenantId = adminUser.tenantId;

  // 1. Create a product via API
  const productRes = await request.post('/api/v1/catalog/product', {
    data: JSON.parse('{"name": "Edge Cache Test Cake", "description": "A delicious test cake", "price": "19.99", "item_type": "Product", "stock": 10}')
  });

  expect(productRes.ok()).toBeTruthy();
  const productData = await productRes.json();
  const productId = productData.id;

  expect(productId).toBeDefined();

  // 2. Access the storefront product route to cache it
  const url = `/edge/${tenantId}/${productId}`;

  let storeRes = await request.get(url);
  expect(storeRes.ok()).toBeTruthy();

  // Wait a short moment for the background caching to complete
  await page.waitForTimeout(500);

  // 3. Second request should be a HIT
  storeRes = await request.get(url);
  expect(storeRes.ok()).toBeTruthy();
  let cacheStatus = storeRes.headers()['x-cache'];
  expect(cacheStatus).toBe('HIT');

  // 4. Update the inventory (this emits tenant.inventory.updated)
  const updateRes = await request.post('/api/v1/pos/inventory', {
    data: JSON.parse('{"items": [{"product_id": "' + productId + '", "variant_id": null, "location_id": null, "delta": -1, "reason": "Test sale"}]}')
  });
  expect(updateRes.ok()).toBeTruthy();

  // Wait for the operations agent and invalidator to process the event
  await page.waitForTimeout(1000);

  // 5. Next request should be a MISS because it was invalidated
  storeRes = await request.get(url);
  expect(storeRes.ok()).toBeTruthy();
  cacheStatus = storeRes.headers()['x-cache'];
  expect(cacheStatus).toBe('MISS');
});
