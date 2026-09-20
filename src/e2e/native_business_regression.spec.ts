import type { Page } from '@playwright/test';
import { test, expect } from './fixtures';

type ProposalEnvelope = {
  proposal: {
    id: string; status: string; project_scope: string; total_amount_cents: number;
    required_deposit_cents: number; milestones: unknown[]; checkout_url: string | null;
  };
  line_items: unknown[];
};
type InvoiceRecord = {
  id: string; client_name: string; total_amount_cents: number; due_date: number;
  created_at: number; amount_paid_cents: number; stripe_payment_link: string;
  stripe_invoice_id: string; status: string;
};
type UsageSummary = {
  account: { limit_micros: number };
  payment_collection_enabled: boolean;
  customer_direct_inference_is_not_rebilled: boolean;
};

/** Real browser -> authenticated Next proxy -> Rust -> isolated PostgreSQL.
 * No route interception, fake provider receipt, production key or paid model call.
 * These checks prove internal persistence/authorization, not live-provider delivery.
 */
async function api<T = Record<string, unknown>>(page: Page, path: string, method = 'GET', body?: unknown): Promise<{ status: number; value: T }> {
  const result = await page.evaluate(async ({ path, method, body }) => {
    const response = await fetch(path, {
      method,
      headers: body === undefined ? undefined : { 'content-type': 'application/json' },
      body: body === undefined ? undefined : JSON.stringify(body),
      cache: 'no-store',
    });
    const text = await response.text();
    let value: unknown = null;
    try { value = JSON.parse(text); } catch { /* Preserve status for non-JSON errors. */ }
    return { status: response.status, value };
  }, { path, method, body });
  // The test's assertions verify the declared response shape. A malformed
  // success fails those assertions; do not substitute a fabricated default.
  return { status: result.status, value: result.value as T };
}

const item = (description: string, unit_price_cents: number, quantity = 1, is_optional = false) =>
  ({ description, unit_price_cents, quantity, is_optional });

// Each case owns its records; only one case changes the spending limit. Do not
// skip independent financial/isolation checks when another case fails.
test.describe('Native business correctness and truthful completion', () => {
  test.describe.configure({ mode: 'default' });

  test('visible login opens the owner connection controls at phone width', async ({ anonymousPage, adminUser }) => {
    const page = anonymousPage;
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/login?next=%2Fintegrations');
    await page.getByLabel('Email or username').fill(adminUser.email);
    await page.getByLabel('Password', { exact: true }).fill(adminUser.password);
    await page.getByLabel(/Organization/).fill(adminUser.organizationId);
    await page.getByRole('button', { name: 'Log in', exact: true }).click();
    await expect(page).toHaveURL(/\/integrations/);
    await expect(page.getByRole('heading', { name: 'Verified business connections' })).toBeVisible();
    await page.getByRole('button', { name: 'Configure OpenAI API', exact: true }).click();
    await expect(page.getByLabel('OpenAI API API key', { exact: true })).toHaveAttribute('type', 'password');
    await expect(page.getByText(/subscription.*not.*API/i)).toBeVisible();
  });

  test('unpriced intake persists the actual inquiry and never invents price or scope', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const inquiry = `Repair existing product photographs, not a website redesign ${crypto.randomUUID()}`;
    const created = await api<ProposalEnvelope>(page, '/api/v1/proposals/intake', 'POST', {
      inquiry, customer_id: `customer-${crypto.randomUUID()}`,
    });
    expect(created.status).toBe(200);
    const proposal = created.value.proposal;
    expect(proposal.status).toBe('NEEDS_PRICING');
    expect(proposal.project_scope).toBe(inquiry);
    expect(proposal.total_amount_cents).toBe(0);
    expect(proposal.required_deposit_cents).toBe(0);
    expect(proposal.milestones).toEqual([]);
    expect(proposal.checkout_url).toBeNull();
    await page.goto(`/proposals/${proposal.id}`);
    await expect(page.getByText('NEEDS_PRICING', { exact: true })).toBeVisible();
    await expect(page.getByRole('button', { name: /Approve.*Send/ })).toHaveCount(0);
    await page.reload();
    const stored = await api<ProposalEnvelope>(page, `/api/v1/proposals/${proposal.id}`);
    expect(stored.status).toBe(200);
    expect(stored.value.proposal.project_scope).toBe(inquiry);
    expect(stored.value.proposal.total_amount_cents).toBe(0);
  });

  test('priced intake uses checked owner line items and stays private to its tenant', async ({ page, browser, loginAs, adminUser, unlimitedAdminUser }) => {
    await loginAs(page, adminUser);
    const created = await api<ProposalEnvelope>(page, '/api/v1/proposals/intake', 'POST', {
      inquiry: 'Deliver two reviewed product images; the optional add-on is not selected.',
      customer_id: `customer-${crypto.randomUUID()}`,
      line_items: [item('Reviewed image', 4500, 2), item('Optional banner', 2000, 1, true)],
      required_deposit_cents: 1000,
    });
    expect(created.status).toBe(200);
    const proposal = created.value.proposal;
    expect(proposal.status).toBe('DRAFT');
    expect(proposal.total_amount_cents).toBe(9000);
    expect(proposal.required_deposit_cents).toBe(1000);
    expect(created.value.line_items).toHaveLength(2);
    await page.goto(`/proposals/${proposal.id}`);
    await expect(page.getByText('Reviewed image (x2)', { exact: true })).toBeVisible();
    await page.reload();
    const saved = await api<ProposalEnvelope>(page, `/api/v1/proposals/${proposal.id}`);
    expect(saved.status).toBe(200);
    expect(saved.value.proposal.total_amount_cents).toBe(9000);
    expect(saved.value.line_items).toHaveLength(2);

    const other = await browser.newPage({ baseURL: new URL(page.url()).origin, storageState: { cookies: [], origins: [] } });
    try {
      await loginAs(other, unlimitedAdminUser);
      expect((await api(other, `/api/v1/proposals/${proposal.id}`)).status).toBe(404);
    } finally { await other.close(); }
  });

  test('invalid financial inputs fail before a proposal is persisted', async ({ page, loginAs, adminUser, memberPage }) => {
    await loginAs(page, adminUser);
    for (const change of [
      { line_items: [item('Invalid amount', -1)], required_deposit_cents: 0 },
      { line_items: [item('Invalid quantity', 100, 0)], required_deposit_cents: 0 },
      { line_items: [item('Valid item', 100)], required_deposit_cents: 101 },
    ]) {
      const response = await api(page, '/api/v1/proposals/intake', 'POST', {
        inquiry: 'Only the authorized supplied amount may be used.',
        customer_id: `invalid-${crypto.randomUUID()}`, ...change,
      });
      expect(response.status).toBe(422);
      expect(response.value.proposal).toBeUndefined();
    }
    const member = await api(memberPage, '/api/v1/proposals/intake', 'POST', {
      inquiry: 'A member cannot authorize pricing.', customer_id: `member-${crypto.randomUUID()}`,
      line_items: [item('Restricted pricing', 100)],
    });
    expect(member.status).toBe(403);
  });

  test('an invoice draft persists correct money without a fabricated checkout session', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const name = `Native invoice ${crypto.randomUUID()}`;
    const created = await api<InvoiceRecord>(page, '/api/v1/invoices', 'POST', {
      client_id: `invoice-client-${crypto.randomUUID()}`, client_name: name,
      due_date: Math.floor(Date.now() / 1000) + 86400, currency: 'USD',
      line_items: [{ description: 'Reviewed item', quantity: 2, unit_price: 12.34 }],
    });
    expect(created.status).toBe(200);
    expect(created.value.total_amount_cents).toBe(2468);
    expect(created.value.due_date).toBeGreaterThan(0);
    expect(created.value.created_at).toBeGreaterThan(0);
    expect(created.value.stripe_payment_link).toBe('');
    expect(created.value.stripe_invoice_id).toBe('');
    expect(created.value.status).toBe('draft');
    await page.reload();
    const listed = await api<{ invoices: InvoiceRecord[] }>(page, '/api/v1/invoices');
    expect(listed.status).toBe(200);
    const stored = listed.value.invoices.find((invoice: { id: string }) => invoice.id === created.value.id);
    expect(stored.client_name).toBe(name);
    expect(stored.total_amount_cents).toBe(2468);
    expect(stored.amount_paid_cents).toBe(0);
    expect(stored.stripe_payment_link).toBe('');
    expect(stored.due_date).toBe(created.value.due_date);
    expect(stored.created_at).toBeGreaterThan(0);
    const voided = await api<InvoiceRecord>(page, `/api/v1/invoices/${created.value.id}/status`, 'PUT', { status: 'void' });
    expect(voided.status).toBe(200);
    expect(voided.value.due_date).toBe(created.value.due_date);
    expect(voided.value.status).toBe('void');
    const fabricated = await api(page, `/api/v1/invoices/${created.value.id}/status`, 'PUT', { status: 'paid' });
    expect(fabricated.status).toBe(409);
  });

  test('spending authorization is durable, owner-only and not payment credit', async ({ page, loginAs, adminUser, memberPage }) => {
    await loginAs(page, adminUser);
    const updated = await api<{ is_payment_credit: boolean; spending_authorization: { limit_micros: number } }>(page, '/api/v1/billing/usage/spending-limit', 'PUT', { limit_micros: 1_000_000 });
    expect(updated.status).toBe(200);
    expect(updated.value.is_payment_credit).toBe(false);
    expect(updated.value.spending_authorization.limit_micros).toBe(1_000_000);
    await page.reload();
    const summary = await api<UsageSummary>(page, '/api/v1/billing/usage');
    expect(summary.status).toBe(200);
    expect(summary.value.account.limit_micros).toBe(1_000_000);
    expect(summary.value.payment_collection_enabled).toBe(false);
    expect(summary.value.customer_direct_inference_is_not_rebilled).toBe(true);
    expect((await api(memberPage, '/api/v1/billing/usage/spending-limit', 'PUT', { limit_micros: 2_000_000 })).status).toBe(403);
    expect((await api(page, '/api/v1/billing/usage/spending-limit', 'PUT', { limit_micros: -1 })).status).toBe(400);
    expect((await api<UsageSummary>(page, '/api/v1/billing/usage')).value.account.limit_micros).toBe(1_000_000);
  });

  test('unconfigured payment adapters never manufacture checkout success', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const unavailable = await api(page, '/api/v1/checkout/mercadopago', 'POST', { amount_cents: 1234 });
    expect(unavailable.status).toBe(501);
    expect(unavailable.value.success).toBe(false);
    expect(unavailable.value.checkout_url).toBeUndefined();
    const invalidWalkup = await api(page, '/api/v1/ui/walkup', 'POST', { message: '   ' });
    expect(invalidWalkup.status).toBe(400);
    expect(invalidWalkup.value.success).toBe(false);
  });
});
