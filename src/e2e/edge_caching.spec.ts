import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('edge_caching_invalidation', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);

  const tenantId = adminUser.tenantId;

  // 1. Create a product via API
  // Create product by manipulating UI
  await page.goto('/products/new');
  await page.fill('input[name="name"]', 'Edge Cache Test Cake');
  await page.fill('input[name="price"]', '19.99');
  await page.click('button:has-text("Save")');
  await page.waitForTimeout(2000);

  const productRes = await request.get('/api/v1/catalog/products');
  const productDataArray = await productRes.json();
  const productData = productDataArray.items ? productDataArray.items[0] : productDataArray[0];

  expect(productData).toBeDefined();
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
  // Update inventory by manipulating UI
  await page.goto(`/products/${productId}`);
  await page.waitForTimeout(1000);
  await page.fill('input[name="stock"]', '9');
  await page.click('button:has-text("Save")');
  await page.waitForTimeout(2000);


  // Wait for the operations agent and invalidator to process the event
  await page.waitForTimeout(1000);

  // 5. Next request should be a MISS because it was invalidated
  storeRes = await request.get(url);
  expect(storeRes.ok()).toBeTruthy();
  cacheStatus = storeRes.headers()['x-cache'];
  expect(cacheStatus).toBe('MISS');
});
