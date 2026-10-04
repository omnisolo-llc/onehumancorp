import { afterEach, expect, it, vi } from 'vitest';
import type { Browser, BrowserContext, Page, Request, Response as BrowserResponse } from '@playwright/test';
import { createDashboardAuditCase } from '../../../../e2e/support/dashboard_audit_fixture';
import { e2eDbTransaction } from '../../../../e2e/db_utils';
import { authenticateRequest } from '../../../../e2e/authenticate';
import { ownedQuoteAuditRecord, quoteAuditRoute } from '../../../../../scripts/ui-audit-fixture.cjs';

vi.mock('../../../../e2e/db_utils', () => ({ e2eDbTransaction: vi.fn() }));
vi.mock('../../../../e2e/authenticate', () => ({ authenticateRequest: vi.fn(async () => undefined) }));
afterEach(() => vi.restoreAllMocks());
const origin = 'http://127.0.0.1:18789';

async function fixture(holdDestination = false) {
  vi.mocked(e2eDbTransaction).mockImplementation(async operation => operation(async (sql, parameters = []) => {
    if (sql.includes('SELECT u.id')) return [{ id: parameters[0], tenant_id: parameters[2] }];
    if (sql.includes('AS tenants')) return [{ tenants: 5, feed: 8, approvals: 2, inbox: 1, products: 2, opportunities: 2, onboarding: 0, quotes: 1, quote_lines: 1 }];
    return [];
  }));
  let now = 0;
  vi.spyOn(Date, 'now').mockImplementation(() => now);
  const listeners = new Map<string, ((event: unknown) => void)[]>();
  const emit = (name: string, event: unknown) => listeners.get(name)?.forEach(listener => listener(event));
  let url = 'about:blank';
  const waiters: { predicate: (response: BrowserResponse) => boolean; resolve: (response: BrowserResponse) => void }[] = [];
  const request = (destination: string) => ({ url: () => destination, method: () => 'GET', resourceType: () => 'fetch', frame: () => ({ url: () => url }) }) as unknown as Request;
  const response = (req: Request, body: unknown, status = 200) => ({ request: () => req, url: () => req.url(), status: () => status,
    headers: () => ({ 'content-type': 'application/json' }), json: async () => body, finished: async () => null }) as unknown as BrowserResponse;
  const detail = () => {
    const quote = ownedQuoteAuditRecord(actor);
    return { quote: { id: quote.quoteId, tenant_id: actor.tenantId, customer_id: quote.customerId, status: 'DRAFT', updated_at: '2026-10-04T00:00:00.123456Z', total_amount_cents: 15000, required_deposit_cents: 5000 },
      line_items: [{ id: 'line-1', quote_id: quote.quoteId, description: quote.description, unit_price_cents: 15000, quantity: 1, is_optional: false }], acceptance: null };
  };
  const strandedStartup = () => {
    // Exact hosted pattern: received API headers without completed body and an
    // RSC request awaiting headers. Neither has emitted a terminal event.
    const api = request(`${origin}/api/v1/agent-feed`);
    emit('request', api); emit('response', response(api, {}, 200));
    emit('request', request(`${origin}/dashboard/campaigns?_rsc=initial`));
  };
  const goto = vi.fn(async (destination: string) => {
    url = destination;
    if (new URL(destination).pathname === '/dashboard') strandedStartup();
    else {
      const quote = ownedQuoteAuditRecord(actor);
      const endpoint = new URL(destination).pathname === '/quoting' ? `${origin}/api/v1/quotes?id=${quote.quoteId}` : `${origin}/api/v1/quotes/${quote.quoteId}`;
      const req = request(endpoint), read = response(req, detail());
      emit('request', req); emit('response', read);
      for (const waiter of waiters) if (waiter.predicate(read)) waiter.resolve(read);
      emit('requestfinished', req);
      if (holdDestination) strandedStartup();
    }
    return { status: () => 200 };
  });
  const context = { on: (name: string, callback: (event: unknown) => void) => listeners.set(name, [...listeners.get(name) ?? [], callback]),
    addInitScript: vi.fn(), newPage: async () => page, close: vi.fn() } as unknown as BrowserContext;
  const page = { context: () => context, goto, url: () => url,
    waitForLoadState: vi.fn(async () => undefined), waitForTimeout: async (delay: number) => { now += delay; },
    waitForFunction: async () => ({ dispose: vi.fn() }), evaluate: vi.fn(),
    waitForResponse: (predicate: (response: BrowserResponse) => boolean) => new Promise<BrowserResponse>(resolve => waiters.push({ predicate, resolve })),
    getByRole: () => ({ waitFor: vi.fn() }), getByText: () => ({ waitFor: vi.fn() }),
    request: { get: async (destination: string) => destination.endsWith('/auth/session-identity')
      ? { status: () => 200, json: async () => ({ userId: actor.userId, tenantId: actor.tenantId }) }
      : response(request(destination), detail()) },
  } as unknown as Page;
  const browser = { newContext: async () => context } as unknown as Browser;
  const owned = await createDashboardAuditCase(browser, origin, { width: 1280, height: 720 });
  const actor = owned.actor;
  return { owned, goto, now: () => now };
}

it.each(['/quotes/e2e-id', '/quote/e2e-id', '/quoting', '/proposals/customer-view'])('navigates directly to owned %s without starting an unrelated dashboard document', async route => {
  const f = await fixture();
  const receipt = await f.owned.navigate(route);
  const record = ownedQuoteAuditRecord(f.owned.actor);
  expect(f.goto.mock.calls.map(([url]) => url)).toEqual([`${origin}${quoteAuditRoute(route, record.quoteId)}`]);
  expect(authenticateRequest).toHaveBeenCalledWith(f.owned.page.request, expect.objectContaining({ organizationId: f.owned.actor.tenantId }), origin);
  expect(receipt.quoteFixture).toMatchObject({ quoteId: record.quoteId, customerId: record.customerId, tenantId: f.owned.actor.tenantId, method: 'GET', status: 200 });
});

it('still rejects all unfinished destination API and RSC requests at the unchanged five-second gate', async () => {
  const f = await fixture(true);
  await expect(f.owned.navigate('/quote/e2e-id')).rejects.toThrow(/baseline reads did not settle.*agent-feed.*awaiting-completion.*dashboard\/campaigns.*awaiting-response/);
  expect(f.now()).toBeGreaterThanOrEqual(5000);
  expect(f.now()).toBeLessThan(5500);
});
