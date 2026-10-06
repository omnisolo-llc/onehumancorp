import { test, expect, loadSupplyPage } from './support/supply_fixture';
import { createGrowthOwner } from './growth_owner';

test('switching to another owned account cannot retain the previous business supply or stock', async ({ page, baseURL, supplyOwner }) => {
  await loadSupplyPage(page, supplyOwner);
  await createGrowthOwner(page, baseURL);
  // Account-switch notification is the application identity-retirement event;
  // authentication above went through the real server, not injected cookies.
  await page.evaluate(() => window.dispatchEvent(new Event('omnisolo_auth_changed')));
  await expect(page.getByTestId(`stock-count-${supplyOwner.product.id}`)).toHaveCount(0);
  await expect(page.getByText(supplyOwner.vendor.name, { exact: true })).toHaveCount(0);
  await page.getByRole('button', { name: 'Reload supply records' }).click();
  await expect(page.getByText('No vendors recorded for this business.', { exact: true })).toBeVisible();
  await expect(page.getByText('No raw materials recorded for this business.', { exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Reload inventory' }).click();
  await expect(page.getByText('No products found in the inventory ledger.', { exact: true })).toBeVisible();
  await expect(page.getByText(supplyOwner.materials[0].name, { exact: true })).toHaveCount(0);
});
