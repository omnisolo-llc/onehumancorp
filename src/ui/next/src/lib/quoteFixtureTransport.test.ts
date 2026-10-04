import { afterEach, expect, it, vi } from 'vitest';
import type { APIRequestContext, Page } from '@playwright/test';
import { isTrustedMutationOrigin } from './auth/origin';
vi.mock('../../../../e2e/db_utils', () => ({ e2eDbQuery: vi.fn(async () => []) }));
import { createOwnerQuote, createOwnerQuoteFromRequest } from '../../../../e2e/playwright/quote_fixture';

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


const credentials = { username: 'audit@example.test', password: 'fixture-password', organizationId: 'owned-tenant' };
function requestFixture(identity: unknown = { userId: 'owned-user', tenantId: 'owned-tenant' }) {
  const origin = 'http://127.0.0.1:18789';
  const post = vi.fn(async (url: string, options: { headers: Record<string, string> }) => {
    expect(isTrustedMutationOrigin(new Headers(options.headers), origin)).toBe(true);
    if (url.endsWith('/auth/login')) return { ok: () => true, status: () => 200, json: async () => ({ user: { id: 'owned-user', organizationId: 'owned-tenant' } }) };
    return { status: () => 201, text: async () => '', json: async () => ({ id: '11111111-1111-4111-8111-111111111111' }) };
  });
  const get = vi.fn(async () => ({ status: () => 200, url: (): string => `${origin}/api/v1/auth/session-identity`, json: async () => identity }));
  return { origin, request: { post, get } as unknown as APIRequestContext, post, get };
}

it('creates an API-only quote only after real login and matching current-owner readback at the configured local origin', async () => {
  vi.stubEnv('E2E_POSTGRES_CONTAINER', 'ohc-e2e-pg-isolated-fixture');
  const f = requestFixture();
  const result = await createOwnerQuoteFromRequest(f.request, f.origin, credentials, { description: 'Actual request-context service' });
  expect(result.quoteId).toBe('11111111-1111-4111-8111-111111111111');
  expect(f.post.mock.calls.map(([url]) => url)).toEqual(['/api/v1/auth/login', `${f.origin}/api/v1/quotes`]);
  expect(f.get).toHaveBeenCalledExactlyOnceWith(`${f.origin}/api/v1/auth/session-identity`);
  expect(f.post.mock.invocationCallOrder[0]).toBeLessThan(f.get.mock.invocationCallOrder[0]);
  expect(f.get.mock.invocationCallOrder[0]).toBeLessThan(f.post.mock.invocationCallOrder[1]);
});

it.each(['https://outside.test', 'http://user:pass@127.0.0.1:18789', 'about:blank', 'http://127.0.0.1:18789/other', 'http://127.0.0.1:18789?token=bad'])('rejects API fixture origin %s before login or mutation', async origin => {
  vi.stubEnv('E2E_POSTGRES_CONTAINER', 'ohc-e2e-pg-isolated-fixture');
  const f = requestFixture();
  await expect(createOwnerQuoteFromRequest(f.request, origin, credentials, { description: 'Service' })).rejects.toThrow(/origin|local|destination/i);
  expect(f.post).not.toHaveBeenCalled(); expect(f.get).not.toHaveBeenCalled();
});

it.each([{ userId: 'foreign-user', tenantId: 'owned-tenant' }, { userId: 'owned-user', tenantId: 'foreign-tenant' }, null])('rejects mismatched current-session identity before quote creation: %j', async identity => {
  vi.stubEnv('E2E_POSTGRES_CONTAINER', 'ohc-e2e-pg-isolated-fixture');
  const f = requestFixture(identity);
  await expect(createOwnerQuoteFromRequest(f.request, f.origin, credentials, { description: 'Service' })).rejects.toThrow(/identity|owner/i);
  expect(f.post.mock.calls.map(([url]) => url)).toEqual(['/api/v1/auth/login']);
});

it('requires the native isolated database before request-only login or mutation', async () => {
  vi.stubEnv('E2E_POSTGRES_CONTAINER', '');
  const f = requestFixture();
  await expect(createOwnerQuoteFromRequest(f.request, f.origin, credentials, { description: 'Service' })).rejects.toThrow(/isolated test database/i);
  expect(f.post).not.toHaveBeenCalled(); expect(f.get).not.toHaveBeenCalled();
});

it('rejects a redirected identity read instead of creating a quote for an unverified origin', async () => {
  vi.stubEnv('E2E_POSTGRES_CONTAINER', 'ohc-e2e-pg-isolated-fixture');
  const f = requestFixture();
  f.get.mockResolvedValueOnce({ status: () => 200, url: () => 'https://outside.test/api/v1/auth/session-identity', json: async () => ({ userId: 'owned-user', tenantId: 'owned-tenant' }) });
  await expect(createOwnerQuoteFromRequest(f.request, f.origin, credentials, { description: 'Service' })).rejects.toThrow(/identity|owner/i);
  expect(f.post.mock.calls.map(([url]) => url)).toEqual(['/api/v1/auth/login']);
});
