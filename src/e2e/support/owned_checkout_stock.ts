import { randomUUID } from 'node:crypto';
import { withOwnedBrowserContexts } from '../../../scripts/playwright/owned-contexts.mjs';
import { expect, type Browser, type BrowserContextOptions, type Page, type Response } from '@playwright/test';
import { authenticateRequest } from '../authenticate';
import { e2eDbQuery } from '../db_utils';
import { createGrowthOwner } from '../growth_owner';
import { E2E_ADMIN_USER } from '../identities';

export const CASH_COMMIT_PATH = '/api/v1/payments/terminal/commit';
export const CHECKOUT_SESSION_PATH = '/api/v1/billing/create-checkout-session';

export type OwnedStock = {
  tenantId: string;
  productId: string;
  title: string;
  amountCents: number;
};

export type OwnedCheckoutActors = { stock: OwnedStock; first: Page; second: Page; firstUserId: string; secondUserId: string };

export async function withOwnedCheckoutActors<Result>(
  browser: Browser, origin: string, contextOptions: BrowserContextOptions,
  use: (actors: OwnedCheckoutActors) => Promise<Result>,
): Promise<Result> {
  const options = { ...contextOptions, baseURL: origin, storageState: { cookies: [], origins: [] }, serviceWorkers: 'block' as const };
  return withOwnedBrowserContexts(browser, options, async ([firstContext, secondContext]) => {
    const first = await firstContext.newPage();
    const second = await secondContext.newPage();
    const owner = await createGrowthOwner(first, origin);
    const secondUserId = `e2e-checkout-staff-${randomUUID()}`;
    const secondEmail = `${secondUserId}@example.test`;
    const rows = await e2eDbQuery(
      `WITH actor AS (
         INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, created_at, updated_at)
         SELECT $1, $2, $2, password_hash, ARRAY['ADMIN'], true, tenant_id, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
         FROM users WHERE id = $3 AND tenant_id = $4 AND active = true
         RETURNING id, tenant_id
       ) INSERT INTO identity_user_roles (user_id, role_name, tenant_id, position)
         SELECT id, 'ADMIN', tenant_id, 0 FROM actor RETURNING user_id, tenant_id`,
      [secondUserId, secondEmail, owner.userId, owner.tenantId],
    );
    expect(rows).toEqual([{ user_id: secondUserId, tenant_id: owner.tenantId }]);
    await authenticateRequest(second.request, {
      username: secondEmail, password: E2E_ADMIN_USER.password, organizationId: owner.tenantId,
    }, origin);
    const stock: OwnedStock = {
      tenantId: owner.tenantId, productId: randomUUID(), title: `Owned last unit ${randomUUID()}`, amountCents: 1999,
    };
    const products = await e2eDbQuery(
      `INSERT INTO products (id, tenant_id, title, type, price, price_cents, inventory_count, available_quantity, locked_quantity)
       VALUES ($1, $2, $3, 'physical', $4::bigint::numeric / 100, $4, 1, 1, 0) RETURNING id`,
      [stock.productId, stock.tenantId, stock.title, stock.amountCents],
    );
    expect(products).toEqual([{ id: stock.productId }]);
    return use({ stock, first, second, firstUserId: owner.userId, secondUserId });
  });
}

export function waitForCheckoutPost(page: Page, path: string): Promise<Response> {
  const origin = new URL(page.url()).origin;
  return page.waitForResponse(response => {
    const url = new URL(response.url());
    return url.origin === origin && url.pathname === path && response.request().method() === 'POST';
  });
}

export async function prepareCashCart(page: Page, stock: OwnedStock, userId: string) {
  await page.goto('/pos/terminal');
  await expect(page.locator('#pos-keypad')).toBeVisible();
  const authentication = waitForCheckoutPost(page, '/api/v1/pos/auth');
  const inventory = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/pos/inventory' && response.request().method() === 'GET');
  for (const digit of ['1', '2', '3', '4']) await page.locator('#pos-keypad').getByRole('button', { name: digit, exact: true }).click();
  const authenticationResponse = await authentication;
  expect(authenticationResponse.status()).toBe(200);
  expect(await authenticationResponse.json()).toMatchObject({
    success: true, staff: { id: userId, tenant_id: stock.tenantId, role: 'ADMIN' },
  });
  const inventoryResponse = await inventory;
  expect(inventoryResponse.status()).toBe(200);
  expect((await inventoryResponse.json()).inventory).toEqual(expect.arrayContaining([
    expect.objectContaining({ id: stock.productId, name: stock.title, price_cents: stock.amountCents, stock: 1 }),
  ]));
  await expect(page.getByText('Offline queue ready for this session.', { exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Clock In', exact: true }).click();
  await expect(page.getByRole('heading', { name: 'Clocked In', exact: true })).toBeVisible();
  await page.getByRole('button', { name: new RegExp(stock.title) }).click();
  await page.getByRole('button', { name: '1 item Charge $19.99', exact: true }).click();
  await expect(page.getByRole('heading', { name: 'Current Order', exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Back', exact: true }).click();
  await page.getByRole('button', { name: 'Cash', exact: true }).click();
  await expect(page.locator('#cash-btn-offline')).toBeEnabled();
}

export async function prepareOnlineCart(page: Page, stock: OwnedStock) {
  const catalog = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/catalog/products' && response.request().method() === 'GET');
  await page.goto(`/checkout?product_id=${stock.productId}`);
  const response = await catalog;
  expect(response.status()).toBe(200);
  expect(await response.json()).toEqual(expect.arrayContaining([
    expect.objectContaining({ id: stock.productId, title: stock.title, price_cents: stock.amountCents }),
  ]));
  await expect(page.getByRole('heading', { name: stock.title, exact: true })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Pay', exact: true })).toBeEnabled();
}

export async function assertCashRejection(response: Response, page: Page) {
  expect(response.status()).toBe(409);
  expect(await response.json()).toMatchObject({ success: false, status: 'rejected' });
  await expect(page.getByText('Status: Cash sale rejected. Review the cart and available inventory before trying again.', { exact: true })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Payment Successful!', exact: true })).toHaveCount(0);
}

export async function assertCashReceipt(response: Response, page: Page, stock: OwnedStock) {
  expect(response.status()).toBe(200);
  const submitted = response.request().postDataJSON();
  expect(submitted).toMatchObject({ tenant_id: stock.tenantId, amount_cents: stock.amountCents,
    items: [{ product_id: stock.productId, quantity: 1, amount_cents: stock.amountCents }] });
  expect(submitted.operation_id).toMatch(/^[a-f0-9-]{36}$/);
  const body = await response.json();
  expect(body).toMatchObject({ success: true, status: 'completed', receipt: {
    operation_id: submitted.operation_id, tenant_id: stock.tenantId, amount_cents: stock.amountCents,
    customer_id: null, items: [{ product_id: stock.productId, quantity: 1, amount_cents: stock.amountCents, lock_id: '' }],
  } });
  expect(body.receipt.order_id).toMatch(/^[a-f0-9-]{36}$/);
  await expect(page.getByRole('heading', { name: 'Payment Successful!', exact: true })).toBeVisible();
  const readback = await page.request.get(`${CASH_COMMIT_PATH}/${submitted.operation_id}`);
  expect(readback.status()).toBe(200);
  expect(readback.headers()['cache-control']).toBe('private, no-store');
  expect(await readback.json()).toEqual(body);
  return body.receipt;
}

export async function assertPersistedStockOutcome(stock: OwnedStock, cashOrderId: string | null) {
  const products = await e2eDbQuery(
    'SELECT inventory_count, available_quantity, locked_quantity FROM products WHERE id=$1 AND tenant_id=$2',
    [stock.productId, stock.tenantId],
  );
  expect(products).toEqual([cashOrderId
    ? { inventory_count: 0, available_quantity: 0, locked_quantity: 0 }
    : { inventory_count: 1, available_quantity: 0, locked_quantity: 1 }]);
  const orders = await e2eDbQuery(
    `SELECT o.id, o.status, (o.total_amount * 100)::bigint::text AS amount_cents,
            i.product_id, i.quantity, (i.price * 100)::bigint::text AS unit_cents,
            r.operation_id, r.amount_cents::text AS receipt_cents
     FROM orders o JOIN order_items i ON i.order_id=o.id AND i.tenant_id=o.tenant_id
     LEFT JOIN terminal_cash_receipts r ON r.order_id=o.id AND r.tenant_id=o.tenant_id
     WHERE o.tenant_id=$1 AND i.product_id=$2`, [stock.tenantId, stock.productId],
  );
  if (cashOrderId) {
    expect(orders).toEqual([expect.objectContaining({ id: cashOrderId, status: 'completed', amount_cents: String(stock.amountCents),
      product_id: stock.productId, quantity: 1, unit_cents: String(stock.amountCents), receipt_cents: String(stock.amountCents),
      operation_id: expect.stringMatching(/^[a-f0-9-]{36}$/) })]);
  } else {
    expect(orders).toEqual([]);
  }
  const receipts = await e2eDbQuery('SELECT order_id FROM terminal_cash_receipts WHERE tenant_id=$1', [stock.tenantId]);
  expect(receipts).toEqual(cashOrderId ? [{ order_id: cashOrderId }] : []);
}
