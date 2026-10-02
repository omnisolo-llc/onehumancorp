import { afterEach, expect, it, vi } from 'vitest';
import type { Page } from '@playwright/test';
import { isTrustedMutationOrigin } from './auth/origin';
vi.mock('../../../../e2e/db_utils', () => ({ e2eDbQuery: vi.fn(async () => []) }));
import { createOwnerQuote } from '../../../../e2e/playwright/quote_fixture';

afterEach(() => vi.unstubAllEnvs());

it('sends the real authenticated quote fixture with the same-origin mutation metadata', async () => {
  vi.stubEnv('E2E_POSTGRES_CONTAINER', 'ohc-e2e-pg-isolated-fixture');
  const quoteId = '11111111-1111-4111-8111-111111111111';
  const post = vi.fn(async (_url: string, options: { headers: Record<string, string>; data: unknown }) => {
    const trusted = isTrustedMutationOrigin(new Headers(options.headers), 'http://127.0.0.1:18789');
    return { status: () => trusted ? 201 : 403, text: async () => trusted ? '' : '{"error":"forbidden"}', json: async () => ({ id: quoteId }) };
  });
  const page = { url: () => 'http://127.0.0.1:18789/dashboard', request: { post } } as unknown as Page;
  const created = await createOwnerQuote(page, 'owned-tenant', { description: 'Actual reviewed service', priceCents: 1234 });
  expect(created.quoteId).toBe(quoteId);
  expect(post).toHaveBeenCalledWith('/api/v1/quotes', expect.objectContaining({ data: expect.objectContaining({
    line_items: [expect.objectContaining({ description: 'Actual reviewed service', unit_price_cents: 1234 })],
  }) }));
});

it('cannot turn an opaque document origin into an accepted quote mutation', async () => {
  vi.stubEnv('E2E_POSTGRES_CONTAINER', 'ohc-e2e-pg-isolated-fixture');
  const post = vi.fn(async (_url: string, options: { headers: Record<string, string> }) => {
    const trusted = isTrustedMutationOrigin(new Headers(options.headers), 'http://127.0.0.1:18789');
    return { status: () => trusted ? 201 : 403, text: async () => '{"error":"forbidden"}' };
  });
  const page = { url: () => 'about:blank', request: { post } } as unknown as Page;
  await expect(createOwnerQuote(page, 'owned-tenant', { description: 'Service' })).rejects.toThrow();
});
