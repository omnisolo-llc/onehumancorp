import { randomUUID } from 'node:crypto';
import { expect, type Page } from '@playwright/test';
import { e2eDbQuery } from '../db_utils';

// Each flow owns its customer and quote. Mutations go through the authenticated
// application API; no shared seeded quote is edited by parallel browser tests.
export async function createOwnerQuote(
  page: Page,
  tenantId: string,
  options: { description: string; serviceId?: string; priceCents?: number },
) {
  if (!process.env.E2E_POSTGRES_CONTAINER?.startsWith('ohc-e2e-pg-')) {
    throw new Error('Quote fixtures require the native runner isolated test database.');
  }
  const customerId = randomUUID();
  await e2eDbQuery(
    'INSERT INTO customers (id, tenant_id, name, email) VALUES ($1, $2, $3, $4)',
    [customerId, tenantId, 'Quote flow customer', `quote-${customerId}@example.test`],
  );
  const response = await page.request.post('/api/v1/quotes', {
    headers: { Origin: new URL(page.url()).origin, 'sec-fetch-site': 'same-origin' },
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
