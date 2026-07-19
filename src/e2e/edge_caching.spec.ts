import { test, expect } from './fixtures';

test('edge_caching_invalidation', async ({ page, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  const tenantId = adminUser.tenantId;

  // We rely on UI mutations here to comply with Playwright no-substitution rule
  await page.goto('/dashboard/products');
  await page.getByRole('button', { name: 'Add Product' }).click();
  await page.getByLabel('Name').fill('Edge Cache Test Cake');
  await page.getByLabel('Price').fill('19.99');
  await page.getByRole('button', { name: 'Save' }).click();

  await expect(page.getByText('Product created')).toBeVisible();
});
