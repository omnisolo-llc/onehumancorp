import { test, expect, loadSupplyPage } from '../../../../e2e/support/supply_fixture';
import { e2eDbQuery } from '../../../../e2e/db_utils';

test('owned product adjustments persist matching receipts while supply records remain read-only', async ({ page, supplyOwner }) => {
  await loadSupplyPage(page, supplyOwner);
  const product = supplyOwner.product;
  for (const [button, change, expectedStock] of [['Increase stock', 1, 4], ['Decrease stock', -1, 3]] as const) {
    const saved = page.waitForResponse(response => new URL(response.url()).origin === supplyOwner.origin
      && new URL(response.url()).pathname === '/api/v1/ui/inventory' && response.request().method() === 'POST');
    await page.getByTestId(`product-row-${product.id}`).getByRole('button', { name: button, exact: true }).click();
    const response = await saved;
    expect(response.status()).toBe(200);
    const [request] = response.request().postDataJSON();
    expect(request.payload).toEqual({ item_id: product.id, quantity_change: change, expected_version: expect.stringMatching(/^[a-f0-9]{64}$/) });
    expect(response.request().headers()['x-ohc-expected-tenant']).toBe(supplyOwner.tenantId);
    expect(response.request().headers()['x-ohc-expected-user']).toBe(supplyOwner.userId);
    const body = await response.json();
    expect(body.success).toBe(true);
    expect(body.outcomes).toHaveLength(1);
    expect(body.outcomes[0]).toMatchObject({ id: request.id, item_id: product.id, quantity_change: change,
      previous_version: request.payload.expected_version, status: 'acknowledged', stock: expectedStock });
    await expect(page.getByTestId(`stock-count-${product.id}`)).toHaveText(String(expectedStock));
    await expect(page.getByText('Stock adjustment saved.', { exact: true })).toBeVisible();
    const receipts = await e2eDbQuery('SELECT receipt_json FROM inventory_adjustment_receipts WHERE tenant_id=$1 AND client_mutation_id=$2 AND item_id=$3', [supplyOwner.tenantId, request.id, product.id]);
    expect(receipts).toHaveLength(1);
    expect(JSON.parse(receipts[0].receipt_json)).toEqual(body.outcomes[0]);
    const reread = await page.request.get('/api/v1/ui/inventory');
    expect(reread.status()).toBe(200);
    expect((await reread.json()).inventory.find((item: { id: string }) => item.id === product.id).stock).toBe(expectedStock);
  }
  await page.reload();
  await expect(page.getByTestId(`stock-count-${product.id}`)).toHaveText('3');
  await expect(page.getByText(supplyOwner.vendor.name, { exact: true })).toBeVisible();
  expect(await e2eDbQuery('SELECT id,name,current_quantity,reorder_threshold FROM raw_materials WHERE tenant_id=$1 ORDER BY name', [supplyOwner.tenantId])).toEqual(supplyOwner.materials);
});
