import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('edge_caching_invalidation', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);

  const tenantId = adminUser.tenantId;

  // Use existing seeded product to avoid fake API payload
  await page.goto('/dashboard/catalog');
  await page.locator('text="Vegan Celebration Cake"').click();
  const pageUrl = page.url();
  const match = pageUrl.match(/\/dashboard\/catalog\/([a-zA-Z0-9-]+)/);
  expect(match).toBeTruthy();
  const productId = match![1];

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

  // 4. Update the inventory (this emits tenant.inventory.updated) via UI
  await page.goto('/dashboard/inventory');
  await page.locator('text="Vegan Celebration Cake"').click();
  await page.getByLabel('Adjustment').fill('-1');
  await page.getByRole('button', { name: 'Save Adjustment' }).click();

  // Wait for the operations agent and invalidator to process the event
  await page.waitForTimeout(1000);

  // 5. Next request should be a MISS because it was invalidated
  storeRes = await request.get(url);
  expect(storeRes.ok()).toBeTruthy();
  cacheStatus = storeRes.headers()['x-cache'];
  expect(cacheStatus).toBe('MISS');
});
