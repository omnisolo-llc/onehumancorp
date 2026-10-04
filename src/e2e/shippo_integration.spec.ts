import { createHmac, randomUUID } from 'node:crypto';
import type { Page } from '@playwright/test';
import { test, expect, E2E_ADMIN_USER } from './fixtures';
import { authenticateRequest } from './authenticate';
import { e2eDbQuery, e2eDbTransaction } from './db_utils';
import { verifiedShippoBrowserEnvironment } from '../../scripts/shippo-browser-fixture.mjs';

type ShippingFixture = Awaited<ReturnType<typeof prepareOrder>>;

async function providerEvidence(path: string) {
  const provider = verifiedShippoBrowserEnvironment();
  const url = new URL(path, provider.baseURL);
  // Do not follow a response or configuration to a live provider or payment site.
  expect(url.origin).toBe(provider.baseURL);
  const response = await fetch(url, {
    redirect: 'error', signal: AbortSignal.timeout(5000),
    headers: { authorization: `ShippoToken ${provider.token}` },
  });
  expect(response.status).toBe(200);
  return response.json();
}

async function prepareOrder(page: Page, baseURL: string | undefined) {
  if (!baseURL) throw new Error('The local application URL is required');
  const origin = new URL(baseURL);
  expect(origin.protocol).toBe('http:');
  expect(origin.hostname).toBe('127.0.0.1');
  const provider = verifiedShippoBrowserEnvironment();
  expect(await providerEvidence('/__fixture/health')).toEqual({
    runId: provider.runId, tenantId: provider.tenantId, accountNamespace: provider.accountNamespace,
  });
  expect(process.env.E2E_POSTGRES_CONTAINER).toBe(`ohc-e2e-pg-${provider.runId}`);
  expect(E2E_ADMIN_USER.organizationId).toBe(provider.tenantId);
  const user = await authenticateRequest(page.request, {
    username: E2E_ADMIN_USER.email, password: E2E_ADMIN_USER.password, organizationId: provider.tenantId,
  }, origin.origin);
  // The web login returns the canonical sealed-session shape, not the backend
  // wire user (organization_id). Verify both the response and its issued cookie.
  expect(user).toMatchObject({ id: 'e2e-admin-user', username: E2E_ADMIN_USER.email, organizationId: provider.tenantId, roles: expect.arrayContaining(['ADMIN']) });
  const identityResponse = await page.request.get(new URL('/api/v1/auth/session-identity', origin).href);
  expect(identityResponse.status()).toBe(200);
  const identity = await identityResponse.json();
  expect(identity).toEqual({ userId: 'e2e-admin-user', tenantId: provider.tenantId, expiresAt: expect.any(Number) });
  expect(Number.isFinite(identity.expiresAt)).toBe(true);
  expect(identity.expiresAt).toBeGreaterThan(Date.now());

  // Both parallel tests and every retry own fresh rows. Never reset a shared
  // order, reuse its paid lifecycle, or rely on the retired production fallback.
  const suffix = randomUUID();
  const orderId = `shipping-order-${suffix}`, customerId = `shipping-customer-${suffix}`;
  const customerName = `Shipping customer ${suffix}`;
  await e2eDbTransaction(async query => {
    await query('INSERT INTO customers(id,tenant_id,name) VALUES($1,$2,$3)', [customerId, provider.tenantId, customerName]);
    await query("INSERT INTO orders(id,tenant_id,customer_id,total_amount,status) VALUES($1,$2,$3,42.50,'pending')", [orderId, provider.tenantId, customerId]);
  });
  const receivedOrders = page.waitForResponse(response => response.url() === `${origin.origin}/api/v1/ui/orders` && response.request().method() === 'GET');
  await page.goto(`/orders/${orderId}`);
  const orderResponse = await receivedOrders;
  expect(orderResponse.status()).toBe(200);
  expect(await orderResponse.json()).toEqual(expect.arrayContaining([{ id: orderId, customer_name: customerName, total_amount: 42.5, status: 'pending', created_at: expect.any(String) }]));
  await expect(page.getByRole('heading', { name: `Order ${orderId}`, exact: true })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Fulfillment', exact: true })).toBeVisible();
  await expect(page.getByText(customerName, { exact: true })).toBeVisible();
  await expect(page.getByText('Powered by Shippo', { exact: true })).toBeVisible();
  return { ...provider, origin: origin.origin, orderId, customerId, customerName };
}

async function purchaseAndVerifyLabel(page: Page, fixture: ShippingFixture) {
  await page.getByRole('spinbutton', { name: 'Package weight in ounces' }).fill('20');
  await page.getByRole('textbox', { name: 'Package dimensions' }).fill('12x10x8');
  const receivedRates = page.waitForResponse(response => response.url() === `${fixture.origin}/api/v1/shipping/rates` && response.request().method() === 'POST');
  await page.getByRole('button', { name: 'Get Shipping Rates', exact: true }).click();
  const ratesResponse = await receivedRates;
  expect(ratesResponse.status()).toBe(200);
  expect(ratesResponse.request().postDataJSON()).toEqual({ orderId: fixture.orderId, weight: '20', dimensions: '12x10x8' });
  const rates = await ratesResponse.json();
  expect(rates.rates).toEqual([{ id: expect.stringMatching(/^rate_[a-f0-9]{32}\.[a-f0-9]{64}$/), carrier: 'USPS', service: 'Priority Mail', amount: '8.25', days: 2 }]);
  const signedRate = rates.rates[0].id as string;
  const [rateId, signature] = signedRate.split('.');
  expect(signature).toBe(createHmac('sha256', fixture.token).update(`${fixture.tenantId}\0${fixture.orderId}\0${rateId}`).digest('hex'));
  await expect(page.getByRole('alert')).toHaveCount(0);
  await expect(page.getByRole('heading', { name: 'Select a Service', exact: true })).toBeVisible();
  const radio = page.getByRole('radio', { name: /USPS Priority Mail/ });
  await radio.check();
  await expect(radio).toHaveValue(signedRate);

  const purchased = page.waitForResponse(response => response.url() === `${fixture.origin}/api/v1/shipping/label` && response.request().method() === 'POST');
  await page.getByRole('button', { name: 'Buy Label', exact: true }).click();
  const purchaseResponse = await purchased;
  expect(purchaseResponse.status()).toBe(200);
  expect(purchaseResponse.request().postDataJSON()).toEqual({ orderId: fixture.orderId, rateId: signedRate });
  const label = await purchaseResponse.json();
  expect(label).toEqual({ success: true, labelUrl: expect.any(String), trackingNumber: expect.any(String), carrier: 'usps', transactionId: expect.stringMatching(/^transaction_[a-f0-9]{32}$/), test: true });
  const evidence = await providerEvidence(`/__fixture/receipts/${label.transactionId}`);
  expect(evidence.shipmentRequest).toEqual({
    address_from: JSON.parse(process.env.SHIPPO_ADDRESS_FROM_JSON!),
    address_to: JSON.parse(process.env.SHIPPO_ADDRESS_TO_JSON!),
    parcels: [{ length: '12', width: '10', height: '8', distance_unit: 'in', weight: '20', mass_unit: 'oz' }], async: false,
  });
  expect(evidence.response).toEqual({
    status: 'SUCCESS', object_id: label.transactionId, test: true, rate: rateId, metadata: evidence.purchaseRequest.metadata,
    label_url: label.labelUrl, tracking_number: label.trackingNumber, tracking_carrier: label.carrier,
  });
  expect(evidence.purchaseCount).toBe(1);
  expect(label.labelUrl).toBe(`https://app.goshippo.com/labels/${label.transactionId}.pdf`);
  await expect(page.getByText('Label Purchased Successfully', { exact: true })).toBeVisible();
  await expect(page.getByText('Label created', { exact: true })).toBeVisible();
  await expect(page.getByText(label.trackingNumber, { exact: true })).toBeVisible();
  // Verify the actual print/open action and receipt URL without navigating to
  // Shippo. The fixture is provider-boundary proof, not a live carrier/PDF claim.
  const print = page.getByRole('link', { name: 'Open Shipping Label', exact: true });
  await expect(print).toBeVisible();
  await expect(print).toHaveAttribute('href', evidence.response.label_url);
  await expect(print).toHaveAttribute('target', '_blank');
  await expect(print).toHaveAttribute('rel', 'noopener noreferrer');
  await expect(page.getByText('Shipped', { exact: true })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Buy Label', exact: true })).toBeDisabled();

  const rows = await e2eDbQuery(`SELECT o.status AS order_status, o.customer_id, c.name AS customer_name,
      d.status AS delivery_status, d.provider, d.provider_delivery_id,
      b.provider_object_id, b.account_namespace, b.label_url, b.tracking_number, b.carrier, b.is_test,
      b.last_event_at_ms, b.last_event_digest,
      p.id AS purchase_id, p.actor_id, p.rate_id, p.status AS purchase_status, p.receipt_json
    FROM orders o JOIN customers c ON c.id=o.customer_id AND c.tenant_id=o.tenant_id
    JOIN delivery_tasks d ON d.order_id=o.id AND d.organization_id=o.tenant_id
    JOIN delivery_provider_bindings b ON b.delivery_task_id=d.id AND b.organization_id=o.tenant_id
    JOIN shipping_purchase_intents p ON p.order_id=o.id AND p.organization_id=o.tenant_id
    WHERE o.tenant_id=$1 AND o.id=$2`, [fixture.tenantId, fixture.orderId]);
  expect(rows).toHaveLength(1);
  const row = rows[0];
  expect(row).toEqual({
    order_status: 'pending', customer_id: fixture.customerId, customer_name: fixture.customerName,
    delivery_status: 'LABEL_CREATED', provider: 'shippo', provider_delivery_id: label.trackingNumber,
    provider_object_id: label.transactionId, account_namespace: fixture.accountNamespace,
    label_url: label.labelUrl, tracking_number: label.trackingNumber, carrier: 'usps', is_test: true,
    last_event_at_ms: null, last_event_digest: null,
    purchase_id: expect.any(String), actor_id: 'e2e-admin-user', rate_id: rateId, purchase_status: 'recorded', receipt_json: expect.any(String),
  });
  expect(JSON.parse(row.receipt_json)).toEqual(label);
  expect(evidence.purchaseRequest).toEqual({ rate: rateId, metadata: `ohc_shipping_${row.purchase_id}`, async: false, label_file_type: 'PDF' });

  // A repeated authenticated UI request returns the durable receipt, not a
  // second provider purchase. This is a real application call, not interception.
  const duplicate = await page.evaluate(async body => {
    const response = await fetch('/api/v1/shipping/label', {
      method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(body),
    });
    return { status: response.status, body: await response.json() };
  }, { orderId: fixture.orderId, rateId: signedRate });
  expect(duplicate).toEqual({ status: 200, body: label });
  expect((await providerEvidence(`/__fixture/receipts/${label.transactionId}`)).purchaseCount).toBe(1);
}

test('User can purchase a shipping label and receive the exact print link for an owned order', async ({ page, baseURL }) => {
  const fixture = await prepareOrder(page, baseURL);
  await purchaseAndVerifyLabel(page, fixture);
});

test('User encounters invalid parcel dimensions and corrects them before purchasing a label', async ({ page, baseURL }) => {
  const fixture = await prepareOrder(page, baseURL);
  await page.getByRole('spinbutton', { name: 'Package weight in ounces' }).fill('20');
  await page.getByRole('textbox', { name: 'Package dimensions' }).fill('0x10x8');
  const rejected = page.waitForResponse(response => response.url() === `${fixture.origin}/api/v1/shipping/rates` && response.request().method() === 'POST');
  await page.getByRole('button', { name: 'Get Shipping Rates', exact: true }).click();
  const response = await rejected;
  expect(response.status()).toBe(400);
  expect(await response.json()).toEqual({ error: 'dimensions must contain three positive numbers' });
  await expect(page.getByRole('alert')).toHaveText('Shipping rates are unavailable.');
  await expect(page.getByRole('radio')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Buy Label', exact: true })).toHaveCount(0);
  expect(await e2eDbQuery('SELECT id FROM shipping_purchase_intents WHERE organization_id=$1 AND order_id=$2', [fixture.tenantId, fixture.orderId])).toEqual([]);
  await purchaseAndVerifyLabel(page, fixture);
});
