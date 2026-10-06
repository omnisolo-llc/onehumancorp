import { randomUUID } from 'node:crypto';
import { test, expect } from '../fixtures';
import { createGrowthOwner } from '../growth_owner';
import { e2eDbQuery } from '../db_utils';
import { requireLoopbackUrl } from '../support/recorded_invitation';

test.describe('Centralized Inventory Management', () => {
  test('Owner can view centralized inventory and manually adjust stock', async ({ page, baseURL, adminUser, loginAs }) => {
    requireLoopbackUrl(baseURL!);
    const owner = await createGrowthOwner(page, baseURL);
    const productId = randomUUID();
    const title = `Owned inventory ${productId}`;
    expect(await e2eDbQuery(
      `INSERT INTO products (id, tenant_id, title, type, price, price_cents, inventory_count, available_quantity, locked_quantity)
       VALUES ($1, $2, $3, 'physical', 10, 1000, 12, 12, 0) RETURNING id`,
      [productId, owner.tenantId, title],
    )).toEqual([{ id: productId }]);
    await loginAs(page, { ...adminUser, email: owner.email, organizationId: owner.tenantId });
    await page.goto('/inventory');
    await expect(page.getByRole('heading', { name: 'Inventory', exact: true })).toBeVisible();
    await expect(page.getByText('Recorded inventory for your business', { exact: true })).toBeVisible();
    const row = page.getByTestId(`product-row-${productId}`);
    await expect(row.getByRole('heading', { name: title, exact: true })).toBeVisible();
    const stock = row.getByTestId(`stock-count-${productId}`);
    await expect(stock).toHaveText('12');
    const origin = new URL(page.url()).origin;
    const endpoint = '/api/v1/ui/inventory';
    const operationIds: string[] = [];

    for (const [quantityChange, expectedStock, control] of [[-1, 11, 'Decrease stock'], [1, 12, 'Increase stock']] as const) {
      const responsePromise = page.waitForResponse(response => {
        const url = new URL(response.url());
        return url.origin === origin && url.pathname === endpoint && response.request().method() === 'POST';
      });
      await row.getByRole('button', { name: control, exact: true }).click();
      const response = await responsePromise;
      expect(response.status()).toBe(200);
      const requests = response.request().postDataJSON();
      expect(requests).toHaveLength(1);
      const request = requests[0];
      expect(request.id).toMatch(/^[a-f0-9-]{36}$/);
      expect(operationIds).not.toContain(request.id);
      operationIds.push(request.id);
      expect(request.payload).toEqual({ item_id: productId, quantity_change: quantityChange, expected_version: expect.stringMatching(/^[a-f0-9]{64}$/) });
      const body = await response.json();
      expect(body).toMatchObject({ success: true, status: 'ok' });
      expect(body.outcomes).toEqual([{
        id: request.id, item_id: productId, quantity_change: quantityChange,
        previous_version: request.payload.expected_version, status: 'acknowledged',
        inventory_version: expect.stringMatching(/^[a-f0-9]{64}$/), stock: expectedStock,
      }]);
      await expect(page.getByRole('status').filter({ hasText: /^Stock adjustment saved\.$/ })).toHaveText('Stock adjustment saved.');
      await expect(stock).toHaveText(String(expectedStock));

      const readback = await page.request.get(`${endpoint}?adjustment_id=${encodeURIComponent(request.id)}`);
      expect(readback.status()).toBe(200);
      expect(await readback.json()).toEqual({ success: true, outcomes: body.outcomes });
      await readback.dispose();
      expect(await e2eDbQuery(
        'SELECT receipt_json::jsonb AS receipt FROM inventory_adjustment_receipts WHERE tenant_id=$1 AND client_mutation_id=$2 AND item_id=$3',
        [owner.tenantId, request.id, productId],
      )).toEqual([{ receipt: body.outcomes[0] }]);
      expect(await e2eDbQuery(
        'SELECT inventory_count, available_quantity, locked_quantity FROM products WHERE tenant_id=$1 AND id=$2',
        [owner.tenantId, productId],
      )).toEqual([{ inventory_count: expectedStock, available_quantity: expectedStock, locked_quantity: 0 }]);
      await page.reload();
      await expect(stock).toHaveText(String(expectedStock));
    }
    expect(await e2eDbQuery(
      'SELECT client_mutation_id FROM inventory_adjustment_receipts WHERE tenant_id=$1 AND item_id=$2 ORDER BY client_mutation_id',
      [owner.tenantId, productId],
    )).toEqual(operationIds.sort().map(client_mutation_id => ({ client_mutation_id })));
  });
});
