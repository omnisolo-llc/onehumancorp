import type { Page, Response } from '@playwright/test';
import { quoteAuditRoutes as sourceQuoteRoutes, quoteAuditRoute } from '../../../scripts/ui-audit-fixture.cjs';
import { E2E_ADMIN_USER } from '../identities';
import { createOwnerQuote } from '../playwright/quote_fixture';
import { parseQuoteDetail } from '../../ui/next/src/app/quotes/[id]/quoteDetail';
import { isQuoteVersion } from '../../ui/next/src/lib/quoteVersion';
import type { AuditNavigationReceipt } from './ui_audit_navigation';

export const quoteAuditRoutes = new Set<string>(sourceQuoteRoutes);
type QuoteAuditRecord = {
  namespace?: string; tenantId: string; quoteId: string; customerId: string;
  description: string; priceCents: number; depositCents: number;
};
type Navigate = (page: Page, route: string) => Promise<AuditNavigationReceipt>;

function verifyDetail(body: unknown, record: QuoteAuditRecord) {
  const detail = parseQuoteDetail(body, record.quoteId);
  const tenant = (body as { quote: { tenant_id?: unknown } }).quote.tenant_id;
  if (tenant !== record.tenantId || detail.customer_id !== record.customerId || detail.status !== 'DRAFT'
      || !isQuoteVersion(detail.updated_at) || detail.acceptance !== null
      || detail.total_amount_cents !== record.priceCents || detail.required_deposit_cents !== record.depositCents
      || detail.line_items.length !== 1 || detail.line_items[0].description !== record.description
      || detail.line_items[0].unit_price_cents !== record.priceCents || detail.line_items[0].quantity !== 1
      || detail.line_items[0].is_optional !== false) throw new Error('Audit quote detail differs from the owned persisted baseline');
}

/** Prove both the actual page read and the canonical authenticated detail GET. */
export async function navigateQuoteAudit(page: Page, baseURL: string, route: string, record: QuoteAuditRecord, navigate: Navigate): Promise<AuditNavigationReceipt> {
  const destination = quoteAuditRoute(route, record.quoteId);
  const origin = new URL(baseURL).origin;
  const detailUrl = `${origin}/api/v1/quotes/${record.quoteId}`;
  const pageReadUrl = route === '/quoting' ? `${origin}/api/v1/quotes?id=${record.quoteId}` : detailUrl;
  const pageRead = page.waitForResponse((response: Response) => response.url() === pageReadUrl && response.request().method() === 'GET');
  const [receipt, response] = await Promise.all([navigate(page, destination), pageRead]);
  const expectedUrl = new URL(destination, origin).href;
  if (receipt.requestedUrl !== expectedUrl || receipt.finalUrl !== expectedUrl || receipt.redirected) throw new Error('Audit quote navigation differs from the source-bound destination');
  if (response.status() !== 200) throw new Error(`Audit quote page read failed: HTTP ${response.status()}`);
  const completionError = await response.finished();
  if (completionError) throw completionError;
  verifyDetail(await response.json(), record);
  const canonical = await page.request.get(detailUrl, { failOnStatusCode: false });
  if (canonical.status() !== 200 || canonical.url() !== detailUrl) throw new Error(`Audit canonical quote read failed: HTTP ${canonical.status()}`);
  verifyDetail(await canonical.json(), record);
  // The HTTP response alone is not rendered control inventory. Await the
  // specific persisted view, never accept its loading/error shell as coverage.
  const heading = route === '/quotes/e2e-id' ? 'Review Estimate' : route === '/quoting' ? 'Project Proposal' : 'Your Quote';
  await page.getByRole('heading', { name: heading, exact: true }).waitFor({ state: 'visible' });
  return { ...receipt, sourceRoute: route, quoteFixture: {
    ...(record.namespace ? { namespace: record.namespace } : {}), tenantId: record.tenantId,
    quoteId: record.quoteId, customerId: record.customerId, detailUrl, method: 'GET', status: 200,
  } };
}

export async function prepareQuoteAudit(page: Page, baseURL: string, route: string, navigate: Navigate) {
  if (!quoteAuditRoutes.has(route)) throw new Error('Unclassified quote audit route');
  await navigate(page, '/dashboard');
  const description = 'UI audit persisted quote';
  const created = await createOwnerQuote(page, E2E_ADMIN_USER.organizationId, { description, priceCents: 15000 });
  return navigateQuoteAudit(page, baseURL, route, { ...created, tenantId: E2E_ADMIN_USER.organizationId, description, priceCents: 15000, depositCents: 1000 }, navigate);
}
