import { test, expect, captureSupply, loadSupplyPage } from './support/supply_fixture';
import { e2eDbQuery } from './db_utils';

test('a malformed owned database quantity becomes unavailable and recovers after data repair', async ({ page, supplyOwner }) => {
  await loadSupplyPage(page, supplyOwner);
  const material = supplyOwner.materials[0];
  expect(await e2eDbQuery('UPDATE raw_materials SET current_quantity=NULL WHERE id=$1 AND tenant_id=$2 RETURNING id', [material.id, supplyOwner.tenantId])).toEqual([{ id: material.id }]);
  const failed = captureSupply(page, supplyOwner);
  await page.getByRole('button', { name: 'Reload supply records' }).click();
  const response = await failed;
  expect(response.status()).toBe(500);
  const region = page.getByRole('region', { name: 'Supply records', exact: true });
  await expect(region.getByRole('alert')).toHaveText('Supply records are unavailable. Reload to try again.');
  await expect(region.getByTestId(`material-quantity-${material.id}`)).toHaveCount(0);
  await expect(region.getByText(/No .* recorded/)).toHaveCount(0);
  await expect(page.getByTestId(`stock-count-${supplyOwner.product.id}`)).toHaveText('3');
  // The browser has already rendered this exact failing GET. Chromium may not
  // retain an unread fetch error body; prove the envelope with an authenticated
  // read while the same owned row is still malformed, without repairing it.
  const rawQuantity = () => e2eDbQuery('SELECT current_quantity FROM raw_materials WHERE id=$1 AND tenant_id=$2', [material.id, supplyOwner.tenantId]);
  expect(await rawQuantity()).toEqual([{ current_quantity: null }]);
  const unavailable = await page.request.get('/api/v1/ui/supply', {
    headers: { 'x-ohc-expected-user': supplyOwner.userId, 'x-ohc-expected-tenant': supplyOwner.tenantId },
  });
  try {
    expect(unavailable.status()).toBe(500);
    expect(await unavailable.json()).toEqual({ error: 'supply_unavailable', success: false });
  } finally { await unavailable.dispose(); }
  expect(await rawQuantity()).toEqual([{ current_quantity: null }]);
  // The unified feed keeps independent dashboard data while marking supply unknown.
  const dashboard = await page.request.get('/api/v1/ui/dashboard/unified-feed');
  try {
    expect(dashboard.status()).toBe(200);
    expect((await dashboard.json()).supply).toEqual({ error: 'supply_unavailable', success: false });
  } finally { await dashboard.dispose(); }
  await page.goto('/dashboard');
  await expect(page.getByText('Vendors', { exact: true }).locator('..')).toContainText('Unavailable');
  await expect(page.getByText('Operations Map', { exact: true })).toBeVisible();
  await e2eDbQuery('UPDATE raw_materials SET current_quantity=2 WHERE id=$1 AND tenant_id=$2', [material.id, supplyOwner.tenantId]);
  await loadSupplyPage(page, supplyOwner);
});
