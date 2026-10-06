import { test, expect, captureSupply, loadSupplyPage } from './support/supply_fixture';
import { e2eDbQuery } from './db_utils';

test('explicit supply reload displays a newly recorded quantity without forecasting or ordering', async ({ page, supplyOwner }) => {
  await loadSupplyPage(page, supplyOwner);
  const material = supplyOwner.materials[0];
  expect(await e2eDbQuery('UPDATE raw_materials SET current_quantity=8 WHERE id=$1 AND tenant_id=$2 RETURNING current_quantity', [material.id, supplyOwner.tenantId])).toEqual([{ current_quantity: 8 }]);
  const read = captureSupply(page, supplyOwner);
  await page.getByRole('button', { name: 'Reload supply records' }).click();
  const response = await read;
  expect(response.status()).toBe(200);
  expect((await response.json()).raw_materials.find((row: { id: string }) => row.id === material.id).current_quantity).toBe(8);
  await expect(page.getByTestId(`material-quantity-${material.id}`)).toHaveText('8');
  await expect(page.getByTestId(`material-row-${material.id}`).getByText('Healthy', { exact: true })).toBeVisible();
  expect(await e2eDbQuery('SELECT id FROM purchase_orders WHERE tenant_id=$1', [supplyOwner.tenantId])).toEqual([]);
});
