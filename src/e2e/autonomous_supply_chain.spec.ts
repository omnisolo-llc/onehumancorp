import { test, expect, loadSupplyPage } from './support/supply_fixture';
import { e2eDbQuery } from './db_utils';

// These are recorded supply readbacks. They do not prove autonomous forecasting,
// purchase-order approval, supplier dispatch, or physical receipt of materials.
test.describe('Recorded supply and vendor readback', () => {
  test('offers actual material and vendor sections alongside the maintained ledger', async ({ page, supplyOwner }) => {
    await loadSupplyPage(page, supplyOwner);
    for (const name of ['Products & Variants', 'Raw Materials', 'Vendors']) {
      await expect(page.getByRole('heading', { name, exact: true })).toBeVisible();
    }
  });

  test('displays only the authenticated business vendor and its persisted contact information', async ({ page, supplyOwner }) => {
    await loadSupplyPage(page, supplyOwner);
    const row = page.getByTestId(`vendor-row-${supplyOwner.vendor.id}`);
    await expect(row).toContainText(supplyOwner.vendor.contact_info);
    expect(await e2eDbQuery('SELECT id,name,contact_info FROM vendors WHERE tenant_id=$1 ORDER BY name', [supplyOwner.tenantId])).toEqual([supplyOwner.vendor]);
    const forged = await page.request.get(`/api/v1/ui/supply?tenant_id=${encodeURIComponent(supplyOwner.foreignTenant)}`);
    expect(forged.status()).toBe(200);
    expect((await forged.json()).vendors).toEqual([supplyOwner.vendor]);
  });

  test('shows recorded quantities and threshold-derived low-stock and healthy states', async ({ page, supplyOwner }) => {
    await loadSupplyPage(page, supplyOwner);
    const mobile = await page.request.get('/api/v1/ui/supply?mobile_optimized=true');
    expect(mobile.status()).toBe(200);
    expect((await mobile.json()).raw_materials).toEqual(supplyOwner.materials);
    for (const [index, material] of supplyOwner.materials.entries()) {
      const row = page.getByTestId(`material-row-${material.id}`);
      await expect(row.getByTestId(`material-quantity-${material.id}`)).toHaveText(String(material.current_quantity));
      await expect(row.getByTestId(`material-threshold-${material.id}`)).toHaveText(String(material.reorder_threshold));
      await expect(row.getByText(index === 0 ? 'Low Stock' : 'Healthy', { exact: true })).toBeVisible();
    }
  });

  test('read-only supply views neither offer approval nor create purchase orders', async ({ page, supplyOwner }) => {
    const writes: string[] = [];
    page.on('request', request => { if (['POST', 'PUT', 'PATCH', 'DELETE'].includes(request.method()) && new URL(request.url()).pathname.includes('/supply')) writes.push(request.url()); });
    await loadSupplyPage(page, supplyOwner);
    const region = page.getByRole('region', { name: 'Supply records', exact: true });
    await expect(region.getByRole('button')).toHaveCount(1);
    await expect(region.getByRole('button', { name: /approve|purchase|order/i })).toHaveCount(0);
    expect(writes).toEqual([]);
    expect(await e2eDbQuery('SELECT id FROM purchase_orders WHERE tenant_id=$1', [supplyOwner.tenantId])).toEqual([]);
  });
});
