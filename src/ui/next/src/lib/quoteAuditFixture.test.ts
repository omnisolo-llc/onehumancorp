import { expect, it, vi } from 'vitest';
import type { Page } from '@playwright/test';
import { navigateQuoteAudit, prepareQuoteAudit } from '../../../../e2e/support/quote_audit_fixture';
vi.mock('../../../../e2e/playwright/quote_fixture', () => ({ createOwnerQuote: vi.fn() }));
import { createOwnerQuote } from '../../../../e2e/playwright/quote_fixture';

const quoteId = '11111111-1111-4111-8111-111111111111';
const customerId = '22222222-2222-4222-8222-222222222222';
const origin = 'http://127.0.0.1:18789';
const record = { quoteId, customerId, tenantId: 'owned-tenant', description: 'Actual service', priceCents: 15000, depositCents: 5000 };
function fixture(route = '/quotes/e2e-id') {
  const detail = { quote: { id: quoteId, customer_id: customerId, tenant_id: record.tenantId, status: 'DRAFT', total_amount_cents: 15000, required_deposit_cents: 5000, updated_at: '2026-10-04T01:00:00.123456Z' }, line_items: [{ id: 'line-id', quote_id: quoteId, description: record.description, quantity: 1, unit_price_cents: 15000, is_optional: false }], acceptance: null };
  const detailUrl = `${origin}/api/v1/quotes/${quoteId}`;
  const pageReadUrl = route === '/quoting' ? `${origin}/api/v1/quotes?id=${quoteId}` : detailUrl;
  const response = { status: () => 200, json: async () => detail, finished: async () => null, url: () => pageReadUrl, request: () => ({ method: () => 'GET' }) };
  const waitForResponse = vi.fn(async (predicate: (response: unknown) => boolean) => { expect(predicate(response)).toBe(true); expect(predicate({ ...response, url: () => `${pageReadUrl}&wrong=1` })).toBe(false); return response; });
  const get = vi.fn(async () => ({ ...response, url: () => detailUrl }));
  const waitFor = vi.fn();
  const page = { waitForResponse, request: { get }, getByRole: () => ({ waitFor }), getByText: () => ({ waitFor }) } as unknown as Page;
  const navigate = vi.fn(async (_page: Page, destination: string) => ({ requestedUrl: `${origin}${destination}`, finalUrl: `${origin}${destination}`, redirected: false }));
  return { detail, response, page, get, navigate, waitForResponse, waitFor };
}
for (const route of ['/quotes/e2e-id', '/quote/e2e-id', '/quoting', '/proposals/customer-view']) {
  it(`waits for ${route}'s actual page read and verifies canonical owned GET detail before discovery`, async () => {
    const f = fixture(route);
    const receipt = await navigateQuoteAudit(f.page, origin, route, record, f.navigate);
    const destination = route.endsWith('/e2e-id') ? route.replace('e2e-id', quoteId) : `${route}?id=${quoteId}`;
    expect(f.navigate).toHaveBeenCalledWith(f.page, destination);
    expect(f.get).toHaveBeenCalledWith(`${origin}/api/v1/quotes/${quoteId}`, { failOnStatusCode: false });
    expect(receipt.sourceRoute).toBe(route);
    expect(receipt.quoteFixture).toMatchObject({ quoteId, customerId, tenantId: record.tenantId, status: 200, method: 'GET' });
    expect(f.waitFor).toHaveBeenCalled();
  });
}
for (const kind of ['missing', 'foreign tenant', 'foreign customer', 'foreign quote', 'changed terms', 'missing version', 'already mutated', 'foreign line']) {
  it(`refuses a ${kind} quote instead of auditing an empty or incorrect page`, async () => {
    const f = fixture();
    if (kind === 'missing') f.response.status = () => 404;
    else if (kind === 'foreign tenant') f.detail.quote.tenant_id = 'other';
    else if (kind === 'foreign customer') f.detail.quote.customer_id = 'other';
    else if (kind === 'foreign quote') f.detail.quote.id = 'other';
    else if (kind === 'changed terms') f.detail.quote.total_amount_cents = 1;
    else if (kind === 'missing version') f.detail.quote.updated_at = '';
    else if (kind === 'already mutated') f.detail.quote.status = 'ACCEPTED';
    else f.detail.line_items[0].quote_id = 'other';
    await expect(navigateQuoteAudit(f.page, origin, '/quotes/e2e-id', record, f.navigate)).rejects.toThrow();
  });
}
it('global audit starts from a real same-origin authenticated document before API-backed quote creation', async () => {
  const f = fixture('/proposals/customer-view');
  vi.mocked(createOwnerQuote).mockImplementationOnce(async () => {
    expect(f.navigate).toHaveBeenCalledWith(f.page, '/dashboard');
    f.detail.quote.tenant_id = 'e2e-tenant'; f.detail.quote.required_deposit_cents = 1000;
    f.detail.line_items[0].description = 'UI audit persisted quote';
    return { quoteId, customerId };
  });
  const receipt = await prepareQuoteAudit(f.page, origin, '/proposals/customer-view', f.navigate);
  expect(createOwnerQuote).toHaveBeenCalledWith(f.page, 'e2e-tenant', { description: 'UI audit persisted quote', priceCents: 15000 });
  expect(receipt.finalUrl).toBe(`${origin}/proposals/customer-view?id=${quoteId}`);
});

it('the isolated owner quote audits entry and editing controls as separate inventories', async () => {
  const { clickAuditStates, prepareClickAuditState } = await import('../../../../e2e/support/dashboard_audit_fixture');
  expect(clickAuditStates('/quotes/e2e-id')).toEqual(['entry', 'editing']);
  for (const route of ['/quote/e2e-id', '/quoting', '/proposals/customer-view']) expect(clickAuditStates(route)).toEqual(['entry']);
  const click = vi.fn(); const waitFor = vi.fn();
  const getByRole = vi.fn(() => ({ click, waitFor }));
  await prepareClickAuditState({ getByRole } as unknown as Page, '/quotes/e2e-id', 'editing');
  expect(getByRole).toHaveBeenCalledWith('button', { name: 'Edit quote', exact: true });
  expect(getByRole).toHaveBeenCalledWith('button', { name: 'Save Changes', exact: true });
  expect(click).toHaveBeenCalledTimes(1);
  expect(waitFor).toHaveBeenCalledWith({ state: 'visible' });
});
