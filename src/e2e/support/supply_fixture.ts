import { randomUUID } from 'node:crypto';
import type { Page } from '@playwright/test';
import { test as base, expect } from '../fixtures';
import { createGrowthOwner } from '../growth_owner';
import { e2eDbTransaction } from '../db_utils';
import { requireLoopbackUrl } from './recorded_invitation';

async function createSupplyOwner(page: Page, baseURL: string | undefined) {
  if (!baseURL) throw new Error('Supply tests require the native runner app URL');
  requireLoopbackUrl(baseURL);
  const owner = await createGrowthOwner(page, baseURL);
  const suffix = randomUUID();
  const product = { id: randomUUID(), name: `Owned product ${suffix}`, stock: 3 };
  const materials = [
    { id: randomUUID(), name: `Owned flour ${suffix}`, current_quantity: 2, reorder_threshold: 5 },
    { id: randomUUID(), name: `Owned packaging ${suffix}`, current_quantity: 12, reorder_threshold: 4 },
  ];
  const vendor = { id: randomUUID(), name: `Owned mill ${suffix}`, contact_info: 'orders@example.test' };
  const foreignTenant = `e2e-supply-foreign-${suffix}`;
  const foreignName = `Private foreign supply ${suffix}`;
  await e2eDbTransaction(async query => {
    await query("INSERT INTO tenants(id,name) VALUES($1,'Foreign supply fixture')", [foreignTenant]);
    for (const [tenant, name] of [[owner.tenantId, vendor.name], [foreignTenant, foreignName]]) {
      await query('INSERT INTO vendors(id,tenant_id,name,contact_info) VALUES($1,$2,$3,$4)', [tenant === owner.tenantId ? vendor.id : randomUUID(), tenant, name, vendor.contact_info]);
    }
    for (const material of materials) {
      await query('INSERT INTO raw_materials(id,tenant_id,name,current_quantity,reorder_threshold) VALUES($1,$2,$3,$4,$5)', [material.id, owner.tenantId, material.name, material.current_quantity, material.reorder_threshold]);
    }
    await query('INSERT INTO raw_materials(id,tenant_id,name,current_quantity,reorder_threshold) VALUES($1,$2,$3,900,1)', [randomUUID(), foreignTenant, foreignName]);
    await query(`INSERT INTO products(id,tenant_id,title,type,price,price_cents,inventory_count,available_quantity,locked_quantity)
      VALUES($1,$2,$3,'physical',1,100,3,3,0)`, [product.id, owner.tenantId, product.name]);
  });
  return { ...owner, origin: new URL(baseURL).origin, product, materials, vendor, foreignTenant, foreignName };
}
export type SupplyOwner = Awaited<ReturnType<typeof createSupplyOwner>>;
export const test = base.extend<{ supplyOwner: SupplyOwner }>({
  supplyOwner: async ({ page, baseURL }, use) => { await use(await createSupplyOwner(page, baseURL)); },
});
export { expect };

export function captureSupply(page: Page, owner: SupplyOwner) {
  return page.waitForResponse(response => new URL(response.url()).origin === owner.origin
    && new URL(response.url()).pathname === '/api/v1/ui/supply' && response.request().method() === 'GET').then(response => {
    expect(response.request().headers()['x-ohc-expected-user']).toBe(owner.userId);
    expect(response.request().headers()['x-ohc-expected-tenant']).toBe(owner.tenantId);
    expect(response.headers()['cache-control']).toContain('no-store');
    return response;
  });
}
export async function loadSupplyPage(page: Page, owner: SupplyOwner) {
  const read = captureSupply(page, owner);
  await page.goto('/inventory');
  const response = await read;
  expect(response.status()).toBe(200);
  expect(await response.json()).toEqual({ vendors: [owner.vendor], raw_materials: owner.materials, bom_items: [] });
  await expect(page.getByRole('heading', { name: 'Inventory', exact: true })).toBeVisible();
  await expect(page.getByRole('region', { name: 'Supply records', exact: true }).getByRole('button', { name: 'Reload supply records' })).toBeEnabled();
  await expect(page.getByText(owner.vendor.name, { exact: true })).toBeVisible();
  await expect(page.getByText(owner.foreignName, { exact: true })).toHaveCount(0);
  await expect(page.getByTestId(`stock-count-${owner.product.id}`)).toHaveText('3');
}
