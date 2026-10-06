import { test, expect, loadSupplyPage } from './support/supply_fixture';

test('inventory navigation restores recorded supply after leaving and returning', async ({ page, supplyOwner }) => {
  await loadSupplyPage(page, supplyOwner);
  await page.goto('/dashboard');
  await page.goBack();
  // A back/forward-cache restore may correctly retire the old identity lease.
  await page.getByRole('button', { name: 'Reload supply records' }).click();
  await page.getByRole('button', { name: 'Reload inventory' }).click();
  await expect(page.getByRole('heading', { name: 'Raw Materials', exact: true })).toBeVisible();
  await expect(page.getByText(supplyOwner.vendor.name, { exact: true })).toBeVisible();
  await expect(page.getByTestId(`stock-count-${supplyOwner.product.id}`)).toHaveText('3');
  await page.goForward();
  await expect(page.getByText('Operations Map', { exact: true })).toBeVisible();
  await expect(page.getByText('Vendors', { exact: true }).locator('..')).toContainText('1');
});
