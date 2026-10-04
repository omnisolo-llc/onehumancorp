import { randomUUID } from 'node:crypto';
import { expect, type APIRequestContext, type Page } from '@playwright/test';
import { e2eDbQuery } from '../db_utils';
import { authenticateRequest, type E2ECredentials } from '../authenticate';

type QuoteOptions = { description: string; serviceId?: string; priceCents?: number };

function requireIsolatedQuoteDatabase() {
  if (!process.env.E2E_POSTGRES_CONTAINER?.startsWith('ohc-e2e-pg-')) {
    throw new Error('Quote fixtures require the native runner isolated test database.');
  }
}

// Each flow owns its customer and quote. Mutations go through the authenticated
// application API; no shared seeded quote is edited by parallel browser tests.
export async function createOwnerQuote(
  page: Page,
  tenantId: string,
  options: QuoteOptions,
) {
  return persistOwnerQuote(page.request, tenantId, options, new URL(page.url()).origin, '/api/v1/quotes');
}

async function persistOwnerQuote(request: APIRequestContext, tenantId: string, options: QuoteOptions, origin: string, destination: string) {
  requireIsolatedQuoteDatabase();
  const customerId = randomUUID();
  await e2eDbQuery(
    'INSERT INTO customers (id, tenant_id, name, email) VALUES ($1, $2, $3, $4)',
    [customerId, tenantId, 'Quote flow customer', `quote-${customerId}@example.test`],
  );
  const response = await request.post(destination, {
    headers: { Origin: origin, 'sec-fetch-site': 'same-origin' },
    data: {
      customer_id: customerId,
      service_id: options.serviceId,
      required_deposit_cents: 1000,
      line_items: [{
        description: options.description,
        unit_price_cents: options.priceCents ?? 5000,
        quantity: 1,
        is_optional: false,
      }],
    },
  });
  expect(response.status(), await response.text()).toBe(201);
  const { id } = await response.json();
  expect(id).toMatch(/^[0-9a-f-]{36}$/);
  return { quoteId: id as string, customerId };
}


// API-only setup is explicit: configured loopback origin, native database proof,
// actual login, and a matching current-session readback before creating data.
// It never manufactures a document URL or relaxes the ordinary Page helper.
export async function createOwnerQuoteFromRequest(
  request: APIRequestContext,
  baseURL: string,
  credentials: E2ECredentials & { organizationId: string },
  options: QuoteOptions,
) {
  requireIsolatedQuoteDatabase();
  const base = new URL(baseURL);
  if (!['http:', 'https:'].includes(base.protocol) || !['127.0.0.1', 'localhost', '[::1]'].includes(base.hostname)
      || base.username || base.password || base.pathname !== '/' || base.search || base.hash) {
    throw new Error('API quote fixture requires the configured isolated local origin');
  }
  const authenticated = await authenticateRequest(request, credentials, base.origin);
  if (!authenticated || typeof authenticated !== 'object' || !('id' in authenticated)
      || typeof authenticated.id !== 'string' || !authenticated.id || !('organizationId' in authenticated)
      || authenticated.organizationId !== credentials.organizationId) throw new Error('Quote fixture login owner identity was not verified');
  const identityUrl = `${base.origin}/api/v1/auth/session-identity`;
  const response = await request.get(identityUrl);
  if (response.status() !== 200 || response.url() !== identityUrl) throw new Error('Quote fixture current owner identity was not verified');
  const identity = await response.json();
  if (identity?.userId !== authenticated.id || identity?.tenantId !== credentials.organizationId) throw new Error('Quote fixture reached a different current owner identity');
  return persistOwnerQuote(request, credentials.organizationId, options, base.origin, `${base.origin}/api/v1/quotes`);
}
