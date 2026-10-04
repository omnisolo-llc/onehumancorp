import { test, expect } from '../fixtures';
import { e2eDbQuery } from '../db_utils';
import { createOwnerQuote } from './quote_fixture';
import { requireLoopbackUrl } from '../support/recorded_invitation';

const routes = [
  { name: 'quote route', path: (id: string) => `/quote/${id}` },
  { name: 'proposal query route', path: (id: string) => `/proposals/customer-view?id=${id}` },
];

test.beforeEach(({ baseURL }) => {
  requireLoopbackUrl(baseURL ?? '');
  // The native runner drops payment/provider credentials and owns the database.
  expect(process.env.E2E_POSTGRES_CONTAINER).toMatch(/^ohc-e2e-pg-/);
  expect(process.env.STRIPE_API_KEY).toBeUndefined();
});

for (const route of routes) {
  test.describe(`Maintained customer acceptance: ${route.name}`, () => {
    test('accepts reviewed terms, reads the committed invoice and reloads the same unpaid receipt', async ({ page, loginAs, adminUser }) => {
      await loginAs(page, adminUser);
      const { quoteId, customerId } = await createOwnerQuote(page, adminUser.organizationId, {
        description: 'Customer reviewed service', priceCents: 5123,
      });
      const endpoint = `/api/v1/quotes/${quoteId}`;
      const loaded = page.waitForResponse(response => new URL(response.url()).pathname === endpoint && response.request().method() === 'GET');
      await page.goto(route.path(quoteId));
      const reviewed = await (await loaded).json();
      expect(reviewed.quote.updated_at).toEqual(expect.any(String));
      await expect(page.getByText('Customer reviewed service x1', { exact: true })).toBeVisible();
      await expect(page.getByText('$51.23', { exact: true })).toHaveCount(2);
      await expect(page.getByText('$10.00', { exact: true })).toBeVisible();
      const origin = new URL(page.url()).origin;
      const mutation = page.waitForResponse(response => new URL(response.url()).origin === origin
        && new URL(response.url()).pathname === `${endpoint}/accept` && response.request().method() === 'POST');
      await page.getByRole('button', { name: 'Accept quote', exact: true }).click();
      const response = await mutation;
      expect(response.status(), await response.text()).toBe(200);
      expect(response.request().postDataJSON()).toEqual({ expected_updated_at: reviewed.quote.updated_at });
      const receipt = await response.json();
      expect(receipt).toMatchObject({ success: true, status: 'accepted', quote_id: quoteId,
        invoice_status: 'Draft', payment_status: 'unverified', checkout_status: 'not_configured', stripe_payment_link: '' });
      expect(receipt.invoice_id).toMatch(/^[0-9a-f-]{36}$/);
      await expect(page.getByRole('heading', { name: 'Quote accepted', exact: true })).toBeVisible();
      await expect(page.getByText(`Invoice: ${receipt.invoice_id}`, { exact: true })).toBeVisible();
      await expect(page.getByText('Payment status: unverified', { exact: true })).toBeVisible();
      await expect(page.getByText(/Checkout is not configured/)).toBeVisible();
      await expect(page.getByRole('link', { name: 'Continue to payment' })).toHaveCount(0);
      await expect(page.getByText(/business has been notified|continue scheduling/i)).toHaveCount(0);
      await expect(page).toHaveURL(`${origin}${route.path(quoteId)}`);

      const stored = await page.request.get(endpoint);
      expect(stored.status()).toBe(200);
      expect(await stored.json()).toMatchObject({ quote: { id: quoteId, customer_id: customerId,
        status: 'ACCEPTED', total_amount_cents: 5123, required_deposit_cents: 1000 }, acceptance: receipt });
      const invoices = await e2eDbQuery(
        `SELECT i.id, i.customer_id, i.total_amount_cents, i.payment_status, i.status,
                q.status AS quote_status, q.acceptance_receipt->>'invoice_id' AS receipt_invoice
         FROM invoices i JOIN quotes q ON q.id::text=i.quote_id::text AND q.tenant_id=i.tenant_id
         WHERE i.quote_id::text=$1 AND i.tenant_id=$2`, [quoteId, adminUser.organizationId],
      );
      expect(invoices).toHaveLength(1);
      expect(invoices[0]).toMatchObject({ id: receipt.invoice_id, customer_id: customerId,
        total_amount_cents: 5123, payment_status: 'unverified', status: 'Draft', quote_status: 'ACCEPTED', receipt_invoice: receipt.invoice_id });
      const lines = await e2eDbQuery('SELECT description, quantity, unit_price_cents, amount_cents FROM invoice_line_items WHERE invoice_id::text=$1 AND tenant_id=$2',
        [receipt.invoice_id, adminUser.organizationId]);
      expect(lines).toHaveLength(1);
      expect(lines[0].description).toBe('Customer reviewed service');
      expect(Number(lines[0].unit_price_cents)).toBe(5123);
      expect(Number(lines[0].amount_cents)).toBe(5123);
      expect(lines[0].quantity).toBe(1);

      // Receipt replay has no new checkout effect and cannot make another invoice.
      const replay = await page.request.post(`${endpoint}/accept`, {
        headers: { Origin: origin, 'sec-fetch-site': 'same-origin' }, data: {},
      });
      expect(replay.status(), await replay.text()).toBe(200);
      expect(await replay.json()).toEqual(receipt);
      await page.reload();
      await expect(page.getByRole('heading', { name: 'Quote accepted', exact: true })).toBeVisible();
      await expect(page.getByText(`Invoice: ${receipt.invoice_id}`, { exact: true })).toBeVisible();
      await expect(page.getByRole('button', { name: 'Accept quote', exact: true })).toHaveCount(0);
      const [count] = await e2eDbQuery('SELECT COUNT(*)::int AS count FROM invoices WHERE quote_id::text=$1 AND tenant_id=$2', [quoteId, adminUser.organizationId]);
      expect(count.count).toBe(1);
    });

    test('rejects missing and stale review versions, then requires deliberate refreshed review', async ({ page, loginAs, adminUser }) => {
      await loginAs(page, adminUser);
      const { quoteId } = await createOwnerQuote(page, adminUser.organizationId, { description: 'Versioned customer service' });
      const endpoint = `/api/v1/quotes/${quoteId}`;
      const origin = new URL(page.url()).origin;
      const headers = { Origin: origin, 'sec-fetch-site': 'same-origin' };
      const missing = await page.request.post(`${endpoint}/accept`, { headers, data: {} });
      expect(missing.status(), await missing.text()).toBe(409);
      expect(await missing.json()).toMatchObject({ success: false, reason: 'reviewed_quote_version_required' });
      const loaded = page.waitForResponse(response => new URL(response.url()).pathname === endpoint && response.request().method() === 'GET');
      await page.goto(route.path(quoteId));
      const original = await (await loaded).json();
      await expect(page.getByRole('button', { name: 'Accept quote', exact: true })).toBeEnabled();
      // A real second owner edit changes the terms while the browser still shows
      // the earlier reviewed version; no network response is substituted.
      const edited = await page.request.put(endpoint, { headers, data: {
        expected_updated_at: original.quote.updated_at, total_amount_cents: 4321, required_deposit_cents: 700,
        line_items: [{ description: 'Updated customer service', unit_price_cents: 4321, quantity: 1, is_optional: false }],
      } });
      expect(edited.status(), await edited.text()).toBe(200);
      const editReceipt = await edited.json();
      const rejected = page.waitForResponse(response => new URL(response.url()).pathname === `${endpoint}/accept` && response.request().method() === 'POST');
      await page.getByRole('button', { name: 'Accept quote', exact: true }).click();
      const rejection = await rejected;
      expect(rejection.status(), await rejection.text()).toBe(409);
      expect(rejection.request().postDataJSON()).toEqual({ expected_updated_at: original.quote.updated_at });
      await expect(page.getByRole('region', { name: 'Quote details' }).getByRole('alert')).toContainText('could not be confirmed');
      await expect(page.getByRole('button', { name: 'Accept quote', exact: true })).toBeDisabled();
      const [before] = await e2eDbQuery('SELECT COUNT(*)::int AS count FROM invoices WHERE quote_id::text=$1 AND tenant_id=$2', [quoteId, adminUser.organizationId]);
      expect(before.count).toBe(0);
      await page.getByRole('button', { name: 'Refresh quote', exact: true }).click();
      await expect(page.getByText('Updated customer service x1', { exact: true })).toBeVisible();
      await expect(page.getByText('$43.21', { exact: true })).toHaveCount(2);
      const accepted = page.waitForResponse(response => new URL(response.url()).pathname === `${endpoint}/accept` && response.request().method() === 'POST');
      await page.getByRole('button', { name: 'Accept quote', exact: true }).click();
      const response = await accepted;
      expect(response.status(), await response.text()).toBe(200);
      expect(response.request().postDataJSON()).toEqual({ expected_updated_at: editReceipt.updated_at });
      await expect(page.getByRole('heading', { name: 'Quote accepted', exact: true })).toBeVisible();
      const invoices = await e2eDbQuery('SELECT total_amount_cents, payment_status FROM invoices WHERE quote_id::text=$1 AND tenant_id=$2', [quoteId, adminUser.organizationId]);
      expect(invoices).toEqual([{ total_amount_cents: 4321, payment_status: 'unverified' }]);
    });

    test('keeps a foreign owned quote inaccessible on read and accept', async ({ page, loginAs, adminUser, unlimitedAdminUser }) => {
      await loginAs(page, adminUser);
      const { quoteId } = await createOwnerQuote(page, adminUser.organizationId, { description: 'Private customer terms' });
      const endpoint = `/api/v1/quotes/${quoteId}`;
      const own = await page.request.get(endpoint);
      expect(own.status()).toBe(200);
      const version = (await own.json()).quote.updated_at;
      await loginAs(page, unlimitedAdminUser);
      const read = page.waitForResponse(response => new URL(response.url()).pathname === endpoint && response.request().method() === 'GET');
      await page.goto(route.path(quoteId));
      expect((await read).status()).toBe(404);
      await expect(page.getByRole('region', { name: 'Quote details' }).getByRole('alert')).toContainText('Quote not found');
      await expect(page.getByText(/Private customer terms/)).toHaveCount(0);
      await expect(page.getByRole('button', { name: 'Accept quote', exact: true })).toHaveCount(0);
      const denied = await page.request.post(`${endpoint}/accept`, {
        headers: { Origin: new URL(page.url()).origin, 'sec-fetch-site': 'same-origin' },
        data: { expected_updated_at: version },
      });
      expect(denied.status(), await denied.text()).toBe(404);
      const [unchanged] = await e2eDbQuery('SELECT status, acceptance_receipt FROM quotes WHERE id::text=$1 AND tenant_id=$2', [quoteId, adminUser.organizationId]);
      expect(unchanged).toEqual({ status: 'DRAFT', acceptance_receipt: null });
      const [count] = await e2eDbQuery('SELECT COUNT(*)::int AS count FROM invoices WHERE quote_id::text=$1', [quoteId]);
      expect(count.count).toBe(0);
    });
  });
}
