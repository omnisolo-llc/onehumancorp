import assert from 'node:assert/strict';
import { expect, type ElementHandle, type Page } from '@playwright/test';
import { e2eDbQuery } from '../db_utils';
import { applicationAlertTexts } from './application_alerts';
import type { ClickEffects } from './ui_click_audit';

/** The canonical owned audit seed has two products; discover every stock control. */
export async function waitForInventoryAuditReady(page: Page) {
  await expect(page.getByRole('button', { name: 'Reload inventory', exact: true })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Reload supply records', exact: true })).toBeEnabled();
  await expect(page.locator('[data-testid^="product-row-"]')).toHaveCount(2);
  for (const name of ['Decrease stock', 'Increase stock']) {
    const controls = page.getByRole('button', { name, exact: true });
    await expect(controls).toHaveCount(2);
    for (const control of await controls.all()) await expect(control).toBeEnabled();
  }
  await expect.poll(() => page.getByRole('alert').evaluateAll(applicationAlertTexts)).toEqual([]);
}

/** A stock request alone is not a saved adjustment; never retire its case early. */
export async function observeInventoryAuditClick(
  page: Page,
  target: ElementHandle<HTMLElement | SVGElement>,
  owner: { userId: string; tenantId: string },
  observe: () => Promise<ClickEffects>,
): Promise<ClickEffects> {
  const label = await target.getAttribute('aria-label');
  if (label !== 'Increase stock' && label !== 'Decrease stock') return observe();
  const itemId = await target.evaluate(element => element.closest('[data-testid^="product-row-"]')?.getAttribute('data-testid')?.slice('product-row-'.length));
  assert.ok(itemId, 'A stock control must belong to an actual product row');
  const count = page.getByTestId(`stock-count-${itemId}`);
  const before = Number(await count.textContent());
  assert.ok(Number.isSafeInteger(before) && before >= 0);
  const delta = label === 'Increase stock' ? 1 : -1;
  assert.ok(before + delta >= 0 && before + delta <= 2147483647, 'The discovered stock control must allow its actual adjustment');
  const origin = new URL(page.url()).origin;
  const posted = page.waitForRequest(request => new URL(request.url()).origin === origin
    && new URL(request.url()).pathname === '/api/v1/ui/inventory' && request.method() === 'POST');
  void posted.catch(() => undefined);
  const effect = await observe();
  const request = await posted;
  assert.equal(request.headers()['x-ohc-expected-user'], owner.userId);
  assert.equal(request.headers()['x-ohc-expected-tenant'], owner.tenantId);
  const mutations = request.postDataJSON();
  assert.ok(Array.isArray(mutations) && mutations.length === 1);
  const mutation = mutations[0];
  assert.equal(typeof mutation.id, 'string');
  assert.match(mutation.id, /^[a-zA-Z0-9_-]{1,128}$/);
  assert.deepEqual(mutation.payload, { item_id: itemId, quantity_change: delta, expected_version: mutation.payload.expected_version });
  assert.match(mutation.payload.expected_version, /^[a-f0-9]{64}$/);
  // The mounted UI reaches this state only after parsing its matching committed
  // POST receipt. A timeout/unknown outcome fails this case; it is never retried.
  await page.getByText('Stock adjustment saved.', { exact: true }).waitFor({ state: 'visible' });
  assert.equal(Number(await count.textContent()), before + delta);
  const saved = await page.request.get(`/api/v1/ui/inventory?adjustment_id=${encodeURIComponent(mutation.id)}`, {
    headers: { 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId },
  });
  const body = await (async () => {
    try {
      assert.equal(saved.status(), 200);
      return await saved.json();
    } finally { await saved.dispose(); }
  })();
  assert.equal(body.success, true);
  assert.ok(body.error == null);
  assert.ok(Array.isArray(body.outcomes) && body.outcomes.length === 1);
  const receipt = body.outcomes[0];
  assert.deepEqual(receipt, { id: mutation.id, item_id: itemId, quantity_change: delta,
    previous_version: mutation.payload.expected_version, inventory_version: receipt.inventory_version,
    status: 'acknowledged', stock: before + delta });
  assert.match(receipt.inventory_version, /^[a-f0-9]{64}$/);
  const rows = await e2eDbQuery('SELECT receipt_json FROM inventory_adjustment_receipts WHERE tenant_id=$1 AND client_mutation_id=$2 AND item_id=$3', [owner.tenantId, mutation.id, itemId]);
  assert.equal(rows.length, 1);
  assert.deepEqual(JSON.parse(rows[0].receipt_json), receipt);
  return effect;
}
