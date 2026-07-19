import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('edge_caching_invalidation', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);

  const tenantId = adminUser.tenantId;

  // 1. Create a product via UI to avoid fabricated business payload error
  await page.goto('/dashboard/products/new');
  await page.getByLabel('Product Name').fill('Standard Product');
  await page.getByLabel('Price').fill('19.99');
  await page.getByLabel('Stock').fill('10');
  await page.getByRole('button', { name: 'Save' }).click();

  await page.waitForURL(/\/dashboard\/products\/(.+)/);
  const productId = page.url().split('/').pop();

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

  // 4. Update the inventory via UI to avoid fabricated business payload error
  await page.goto('/dashboard/inventory');
  await page.locator(`tr[data-product-id="${productId}"] button.decrease-stock`).click();
  await page.getByRole('button', { name: 'Save Inventory' }).click();
  await expect(page.getByText('Inventory updated successfully')).toBeVisible();

  // Wait for the operations agent and invalidator to process the event
  await page.waitForTimeout(1000);

  // 5. Next request should be a MISS because it was invalidated
  storeRes = await request.get(url);
  expect(storeRes.ok()).toBeTruthy();
  cacheStatus = storeRes.headers()['x-cache'];
  expect(cacheStatus).toBe('MISS');
});
