import { readFileSync } from 'node:fs';
import { randomUUID } from 'node:crypto';
import path from 'node:path';
import type { APIRequestContext, APIResponse, Page, TestInfo } from '@playwright/test';
import { computeCacheBustingSearchParam } from '../ui/next/node_modules/next/dist/shared/lib/router/utils/cache-busting-search-param';
import { retiredPageDestination } from '../ui/next/src/lib/auth/retiredPageRoutes';
import { parseAuthRuntimeConfig } from '../ui/next/src/lib/auth/runtimeConfig';
import { parseSessionKeyRing } from '../ui/next/src/lib/auth/sessionKeys';
import { openSession, sealSession } from '../ui/next/src/lib/auth/sessionCodec';
import { sessionCodecContext } from '../ui/next/src/lib/auth/sessionCookie';
import { loadAuthenticatedState } from '../../scripts/playwright/session-state.mjs';
import { test, expect, E2E_ADMIN_USER, E2E_UNLIMITED_ADMIN_USER } from './fixtures';
import { requireLoopbackUrl } from './support/recorded_invitation';
import { publishOwnedStorefront } from './published_storefront_fixture';

// Discover the actual checkout's map so the same suite covers 6, 14 or 16
// aliases on the stacked branches. A changed map syntax must fail discovery,
// not silently remove the retirement boundary checks.
const registry = readFileSync(path.resolve(__dirname, '../ui/next/src/lib/auth/retiredPageRoutes.ts'), 'utf8');
const aliases = Array.from(registry.matchAll(/^\s*\["([^"\n]+)", "([^"\n]+)"\],?\s*$/gm), match => [match[1], match[2]] as const);
if (aliases.length === 0 || new Set(aliases.map(([alias]) => alias)).size !== aliases.length
  || aliases.some(([alias, destination]) => retiredPageDestination(alias) !== destination)
  || (registry.match(/\.html"/g)?.length ?? 0) !== aliases.length) {
  throw new Error('Cannot enumerate every exact retired-page mapping for native boundary acceptance');
}

const query = `?tenant=${E2E_UNLIMITED_ADMIN_USER.organizationId}&campaign=boundary&campaign=again&empty=&unicode=%e2%9c%93+%20&next=%2Forders&next=%2Fdashboard`;
const queryEntries = Array.from(new URLSearchParams(query).entries());
const flightTree = encodeURIComponent(JSON.stringify(['', { children: ['__PAGE__', {}] }, null, null, true]));
type Headers = Record<string, string>;
type Observation = { method: string; path: string; protocol: Headers; status: number; location?: string; nextRedirect?: string; contentType?: string; cacheControl?: string };
type FetchOptions = NonNullable<Parameters<APIRequestContext['fetch']>[1]>;

function nativeOrigin(baseURL: string | undefined): string {
  if (!baseURL) throw new Error('Proxy boundary acceptance requires the native runner base URL');
  requireLoopbackUrl(baseURL);
  if (!process.env.OMNISOLO_E2E_SESSION_STATE_DIR || !process.env.E2E_POSTGRES_CONTAINER?.startsWith('ohc-e2e-pg-')) {
    throw new Error('Proxy boundary acceptance requires native disposable PostgreSQL and cached real sessions');
  }
  return new URL(baseURL).origin;
}

async function restoreOwner(page: Page, baseURL: string | undefined) {
  const origin = nativeOrigin(baseURL);
  const state = await loadAuthenticatedState(process.env.OMNISOLO_E2E_SESSION_STATE_DIR!, origin, E2E_ADMIN_USER);
  await page.context().clearCookies();
  await page.context().addCookies(state.cookies);
  const identity = await page.request.get('/api/v1/auth/session-identity', { maxRedirects: 0 });
  expect(identity.status()).toBe(200);
  const owner = await identity.json();
  expect(owner).toMatchObject({ tenantId: E2E_ADMIN_USER.organizationId, userId: expect.any(String) });
  expect(owner.userId).not.toBe('');
  const session = (await page.context().cookies(origin)).find(cookie => cookie.name === 'omnisolo_session' || cookie.name === '__Host-omnisolo_session');
  expect(session).toBeDefined();
  return { origin, owner, session: session! };
}

async function observed(request: APIRequestContext, observations: Observation[], target: string, options: FetchOptions = {}) {
  const response = await request.fetch(target, { ...options, maxRedirects: 0 });
  const headers = response.headers();
  const protocol = Object.fromEntries(Object.entries(options.headers ?? {}).filter(([name]) => ['rsc', 'purpose', 'next-router-prefetch', 'next-router-state-tree', 'next-router-segment-prefetch', 'next-url', 'next-action', 'x-nextjs-data'].includes(name.toLowerCase())));
  observations.push({ method: options.method ?? 'GET', path: target, protocol, status: response.status(), location: headers.location,
    nextRedirect: headers['x-nextjs-redirect'], contentType: headers['content-type'], cacheControl: headers['cache-control'] });
  return response;
}

async function expiredCookie(session: { name: string; value: string }, origin: string): Promise<string> {
  // Negative-only fixture: preserve the genuinely issued actor/access token and
  // use only this native run's private ring to shorten its sealed lifetime.
  // Sending Cookie explicitly ensures the server, not browser expiry, rejects it.
  const config = parseAuthRuntimeConfig(process.env);
  expect(config.canonicalOrigin).toBe(origin);
  expect(config.cookieName).toBe(session.name);
  const ring = await parseSessionKeyRing(process.env);
  const codec = sessionCodecContext(config);
  const now = Math.floor(Date.now() / 1000);
  const actual = await openSession(session.value, ring, codec, now);
  expect(actual.user.organizationId).toBe(E2E_ADMIN_USER.organizationId);
  const expired = await sealSession({ ...actual, iat: now - 120, exp: now - 60 }, ring, codec, { now: now - 120, backendExpiresAt: actual.exp });
  await expect(openSession(expired, ring, codec, now)).rejects.toThrow('invalid web session');
  return `${session.name}=${expired}`;
}

async function attach(testInfo: TestInfo, observations: Observation[]) {
  // Never attach the server-issued session cookie or an authenticated payload.
  await testInfo.attach('proxy-boundary-responses', { body: Buffer.from(JSON.stringify(observations, null, 2)), contentType: 'application/json' });
}

function noRedirect(response: APIResponse) {
  expect.soft(response.headers().location).toBeUndefined();
  expect.soft(response.headers()['x-nextjs-redirect']).toBeUndefined();
}

function privateResponse(response: APIResponse) {
  expect.soft(response.headers()['cache-control']).toBe('private, no-store');
}

function loginBoundary(response: APIResponse, target: string, origin: string, forceApi = false) {
  const requested = new URL(target, origin);
  privateResponse(response);
  if (forceApi || requested.pathname.startsWith('/api/')) {
    expect.soft(response.status()).toBe(401);
    noRedirect(response);
    return;
  }
  expect.soft(response.status()).toBe(307);
  // The native router marks /_next/data/*.json as x-nextjs-data even when
  // the client omitted that header. The adapter moves its login redirect to
  // x-nextjs-redirect; an absent Location must not hide a missing auth guard.
  const isDataPath = requested.pathname.startsWith('/_next/data/') && requested.pathname.endsWith('.json');
  const location = isDataPath ? response.headers()['x-nextjs-redirect'] : response.headers().location;
  if (isDataPath) expect.soft(response.headers().location).toBeUndefined();
  else expect.soft(response.headers()['x-nextjs-redirect']).toBeUndefined();
  expect.soft(location).toBeDefined();
  if (location === undefined) return;
  const login = new URL(location, origin);
  expect.soft(login.origin).toBe(origin);
  expect.soft(login.pathname).toBe('/login');
  expect.soft(login.hash).toBe('');
  expect.soft(login.searchParams.getAll('next')).toHaveLength(1);
  const returnTo = new URL(login.searchParams.get('next') ?? '', origin);
  expect.soft(returnTo.origin).toBe(origin);
  expect.soft(returnTo.pathname).toBe(requested.pathname);
  expect.soft(Array.from(returnTo.searchParams.entries())).toEqual(Array.from(requested.searchParams.entries()));
}

async function flightPath(target: string, headers: Headers): Promise<string> {
  // Use the installed app framework's cache-key algorithm; an invalid key can
  // trigger a separate render-layer redirect and conceal the alias regression.
  const prefetch = headers['next-router-prefetch'];
  if (prefetch !== undefined && prefetch !== '0' && prefetch !== '1' && prefetch !== '2' && prefetch !== '3') {
    throw new Error('Invalid prefetch variant in boundary test');
  }
  const cacheKey = await computeCacheBustingSearchParam(prefetch as Parameters<typeof computeCacheBustingSearchParam>[0], headers['next-router-segment-prefetch'], headers['next-router-state-tree'], headers['next-url']);
  const url = new URL(target, 'http://localhost');
  url.searchParams.set('_rsc', cacheKey);
  return url.pathname + url.search;
}

async function malformedPublicRsc(request: APIRequestContext, observations: Observation[], target: string, origin: string) {
  // A header-only RSC GET is malformed in pinned Next: require its exact
  // same-endpoint cache-key normalization, never a canonical/foreign redirect.
  const response = await observed(request, observations, target, { headers: { rsc: '1' } });
  expect.soft(response.status()).toBe(307);
  const location = response.headers().location;
  expect.soft(location).toBeDefined();
  expect.soft(response.headers()['x-nextjs-redirect']).toBeUndefined();
  const validPath = await flightPath(target, { rsc: '1' });
  if (location !== undefined) {
    const normalized = new URL(location, origin);
    const expected = new URL(validPath, origin);
    expect.soft(normalized.origin).toBe(origin);
    expect.soft(normalized.pathname).toBe(expected.pathname);
    expect.soft(normalized.hash).toBe('');
    expect.soft(Array.from(normalized.searchParams.entries())).toEqual(Array.from(expected.searchParams.entries()));
  }
  expect.soft(await response.text()).toBe('');
  return validPath;
}

for (const [alias, canonical] of aliases) {
  test.describe(`proxy boundary ${alias}`, () => {
    test('RSC-02 document GET/HEAD preserve ordered queries and existing authentication', async ({ page, anonymousPage, baseURL }, testInfo) => {
      const { origin } = await restoreOwner(page, baseURL);
      const observations: Observation[] = [];
      try {
        for (const method of ['GET', 'HEAD']) {
          const response = await observed(page.request, observations, alias + query, { method });
          expect.soft(response.status()).toBe(307);
          privateResponse(response);
          const location = response.headers().location;
          expect.soft(location).toBeDefined();
          if (location !== undefined) {
            const destination = new URL(location, origin);
            expect.soft(destination.origin).toBe(origin);
            expect.soft(destination.pathname).toBe(canonical);
            expect.soft(destination.hash).toBe('');
            expect.soft(Array.from(destination.searchParams.entries())).toEqual(queryEntries);
          }
          if (method === 'HEAD') expect.soft((await response.body()).length).toBe(0);
          const anonymous = await observed(anonymousPage.request, observations, alias + query, { method });
          // Encoded slash in next is intentionally rejected by safeReturnPath.
          if (alias.startsWith('/api/')) loginBoundary(anonymous, alias, origin);
          else {
            expect.soft(anonymous.status()).toBe(307);
            privateResponse(anonymous);
            const login = new URL(anonymous.headers().location ?? '', origin);
            expect.soft(login.origin).toBe(origin);
            expect.soft(login.pathname).toBe('/login');
            expect.soft(login.searchParams.getAll('next')).toEqual(['/dashboard']);
          }
          if (method === 'HEAD') expect.soft((await anonymous.body()).length).toBe(0);
          const safe = `${alias}?campaign=one&campaign=two&empty=&unicode=%E2%9C%93+%20`;
          loginBoundary(await observed(anonymousPage.request, observations, safe, { method }), safe, origin);
        }
      } finally { await attach(testInfo, observations); }
    });

    test('RSC-03 valid RSC and every prefetch variant stay on the missing alias', async ({ page, baseURL }, testInfo) => {
      await restoreOwner(page, baseURL);
      const observations: Observation[] = [];
      try {
        const variants: Array<{ path: string; headers: Headers }> = [
          { path: await flightPath(alias, { rsc: '1' }), headers: { rsc: '1' } },
          { path: `${alias}?_rsc=opaque`, headers: {} },
          { path: alias, headers: { purpose: 'prefetch' } },
          { path: alias, headers: { 'next-router-prefetch': '1' } },
        ];
        for (const headers of [
          { rsc: '1', 'next-router-state-tree': flightTree, 'next-url': '/dashboard' },
          { rsc: '1', 'next-router-prefetch': '1', 'next-router-state-tree': flightTree, 'next-url': '/dashboard' },
          { rsc: '1', 'next-router-prefetch': '1', 'next-router-segment-prefetch': '/_tree', 'next-url': '/dashboard' },
        ]) variants.push({ path: await flightPath(alias, headers), headers });
        for (const variant of variants) {
          const response = await observed(page.request, observations, variant.path, { headers: variant.headers });
          expect.soft(response.status(), JSON.stringify(variant)).toBe(404);
          noRedirect(response);
        }
      } finally { await attach(testInfo, observations); }
    });

    test('RSC-04 malformed RSC normalization never migrates to a canonical page', async ({ page, baseURL }, testInfo) => {
      const { origin } = await restoreOwner(page, baseURL);
      const observations: Observation[] = [];
      try {
        for (const target of [alias, `${alias}?_rsc=invalid-cache-key`]) {
          const response = await observed(page.request, observations, target, { headers: { rsc: '1' } });
          // Normalization precedes app route handlers too. A dedicated Next
          // 404 render path may instead reject immediately without that hop.
          if (response.status() === 404) {
            // Next skips hash normalization in its dedicated 404 render path;
            // an immediate missing-route result is also a complete rejection.
            expect.soft(response.status()).toBe(404);
            noRedirect(response);
          } else {
            expect.soft(response.status()).toBe(307);
            const location = response.headers().location;
            expect.soft(location).toBeDefined();
            if (location !== undefined) {
              const normalized = new URL(location, origin);
              expect.soft(normalized.origin).toBe(origin);
              expect.soft(normalized.pathname).toBe(alias);
              expect.soft(normalized.hash).toBe('');
              expect.soft(Array.from(normalized.searchParams.entries())).toEqual([['_rsc', '']]);
              expect.soft(await response.text()).toBe('');
              // Follow only the independently constructed valid alias, never a
              // potentially incorrect or foreign Location from a failed check.
              const valid = await observed(page.request, observations, `${alias}?_rsc=`, { headers: { rsc: '1' } });
              expect.soft(valid.status()).toBe(404);
              noRedirect(valid);
            }
          }
        }
      } finally { await attach(testInfo, observations); }
    });

    test('RSC-05 missing, damaged, duplicate and expired sealed sessions cannot open aliases or Flight responses', async ({ page, anonymousPage, baseURL }, testInfo) => {
      const { origin, session } = await restoreOwner(page, baseURL);
      const observations: Observation[] = [];
      try {
        const credentials: Headers[] = [
          {},
          { cookie: `${session.name}=damaged-native-session` },
          { cookie: `${session.name}=${session.value}; ${session.name}=${session.value}` },
          { cookie: await expiredCookie(session, origin) },
        ];
        for (const credential of credentials) {
          for (const variant of [
            { target: alias, headers: {} },
            { target: `${alias}?_rsc=`, headers: { rsc: '1' } },
            { target: alias, headers: { 'next-router-prefetch': '1' } },
            { target: alias, headers: { purpose: 'prefetch' } },
          ]) {
            const response = await observed(anonymousPage.request, observations, variant.target, {
              headers: { ...credential, ...variant.headers, 'x-organization-id': E2E_UNLIMITED_ADMIN_USER.organizationId, 'x-user-id': 'untrusted-browser-user' },
            });
            loginBoundary(response, variant.target, origin);
            if (credential.cookie !== undefined) {
              expect.soft(response.headers()['set-cookie']).toContain(`${session.name}=;`);
              expect.soft(response.headers()['set-cookie']).toContain('Max-Age=0');
            }
          }
        }
      } finally { await attach(testInfo, observations); }
    });

    test('RSC-05 unsafe methods retain origin checks and never become page redirects', async ({ page, anonymousPage, baseURL }, testInfo) => {
      const { origin } = await restoreOwner(page, baseURL);
      const observations: Observation[] = [];
      try {
        for (const method of ['POST', 'PUT', 'PATCH', 'DELETE']) {
          const sameOrigin = { origin, 'sec-fetch-site': 'same-origin' };
          const response = await observed(page.request, observations, alias, { method, headers: sameOrigin });
          expect.soft(response.status()).toBe(404);
          noRedirect(response);
          for (const headers of [{ origin: 'https://foreign.invalid', 'sec-fetch-site': 'cross-site' }, {}, { origin, 'sec-fetch-site': 'cross-site' }]) {
            const rejected = await observed(page.request, observations, alias, { method, headers });
            expect.soft(rejected.status()).toBe(403);
            privateResponse(rejected);
            noRedirect(rejected);
          }
          loginBoundary(await observed(anonymousPage.request, observations, alias, { method, headers: sameOrigin }), alias, origin);
        }
        const actionHeaders = { 'next-action': 'unregistered-boundary-action', origin, 'sec-fetch-site': 'same-origin' };
        loginBoundary(await observed(anonymousPage.request, observations, alias, { method: 'POST', headers: actionHeaders }), alias, origin, true);
        const missingAction = await observed(page.request, observations, alias, { method: 'POST', headers: actionHeaders });
        expect.soft(missingAction.status()).toBe(404);
        noRedirect(missingAction);
        const action = await observed(page.request, observations, alias, { method: 'POST', headers: { ...actionHeaders, origin: 'https://foreign.invalid' } });
        expect.soft(action.status()).toBe(403);
        noRedirect(action);
      } finally { await attach(testInfo, observations); }
    });

    test('RSC-05 OPTIONS retains authentication and framework method handling', async ({ page, anonymousPage, baseURL }, testInfo) => {
      const { origin } = await restoreOwner(page, baseURL);
      const observations: Observation[] = [];
      try {
        loginBoundary(await observed(anonymousPage.request, observations, alias, { method: 'OPTIONS' }), alias, origin);
        const response = await observed(page.request, observations, alias, { method: 'OPTIONS' });
        // Next auto-implements OPTIONS for the real API route's six methods.
        // Missing page aliases use the 404 renderer, which is explicitly exempt
        // from Next's OPTIONS rejection for existing page handlers.
        noRedirect(response);
        if (alias.startsWith('/api/v1/ui/')) {
          expect.soft(response.status()).toBe(204);
          expect.soft(response.headers().allow).toBe('DELETE, GET, HEAD, OPTIONS, PATCH, POST, PUT');
          expect.soft((await response.body()).length).toBe(0);
        } else {
          expect.soft(response.status()).toBe(404);
          expect.soft(response.headers()['content-type']).toContain('text/html');
          expect.soft(await response.text()).toContain('This page could not be found.');
        }
      } finally { await attach(testInfo, observations); }
    });

    test('RSC-09 only exact aliases migrate and ambiguous paths stay rejected', async ({ page, baseURL }, testInfo) => {
      const { origin } = await restoreOwner(page, baseURL);
      const observations: Observation[] = [];
      try {
        for (const target of [alias.toUpperCase(), `${alias}/extra`, `/not-retired${alias}`]) {
          const response = await observed(page.request, observations, target);
          expect.soft(response.status()).toBe(404);
          noRedirect(response);
        }
        for (const target of [alias.replace('.html', '%2ehtml'), `${alias}%2fextra`, `${alias}%252fextra`, `${alias}%zz`]) {
          const response = await observed(page.request, observations, target);
          expect.soft(response.status()).toBe(400);
          noRedirect(response);
        }
        const trailing = await observed(page.request, observations, `${alias}/?campaign=one&campaign=two`);
        expect.soft(trailing.status()).toBe(308);
        const normalized = new URL(trailing.headers().location ?? '', origin);
        expect.soft(normalized.origin).toBe(origin);
        expect.soft(normalized.pathname).toBe(alias);
        expect.soft(Array.from(normalized.searchParams.entries())).toEqual([['campaign', 'one'], ['campaign', 'two']]);
      } finally { await attach(testInfo, observations); }
    });

    test('RSC-09 unsafe return queries stay local while signed queries and identity survive', async ({ page, anonymousPage, baseURL }, testInfo) => {
      const { origin, owner } = await restoreOwner(page, baseURL);
      const observations: Observation[] = [];
      try {
        for (const search of ['?opaque=%2f%2F', '?next=https://foreign.invalid/path', '?next=%252f%252fforeign.invalid']) {
          const response = await observed(page.request, observations, alias + search);
          expect.soft(response.status()).toBe(307);
          const target = new URL(response.headers().location ?? '', origin);
          expect.soft(target.origin).toBe(origin);
          expect.soft(target.pathname).toBe(canonical);
          expect.soft(Array.from(target.searchParams.entries())).toEqual(Array.from(new URLSearchParams(search).entries()));
          const anonymous = await observed(anonymousPage.request, observations, alias + search);
          if (alias.startsWith('/api/')) loginBoundary(anonymous, alias, origin);
          else {
            expect.soft(anonymous.status()).toBe(307);
            privateResponse(anonymous);
            const login = new URL(anonymous.headers().location ?? '', origin);
            expect.soft(login.origin).toBe(origin);
            expect.soft(login.pathname).toBe('/login');
            expect.soft(login.searchParams.getAll('next')).toEqual(['/dashboard']);
          }
        }
        const identity = await observed(page.request, observations, `/api/v1/auth/session-identity?tenant=${E2E_UNLIMITED_ADMIN_USER.organizationId}`, {
          headers: { 'x-organization-id': E2E_UNLIMITED_ADMIN_USER.organizationId, 'x-user-id': 'untrusted-browser-user' },
        });
        expect(identity.status()).toBe(200);
        expect(await identity.json()).toMatchObject(owner);
      } finally { await attach(testInfo, observations); }
    });
  });
}

test('RSC-05/06 expired sealed credentials cannot fetch canonical protected Flight payloads', async ({ page, anonymousPage, baseURL }, testInfo) => {
  const { origin, session } = await restoreOwner(page, baseURL);
  const cookie = await expiredCookie(session, origin);
  const observations: Observation[] = [];
  try {
    for (const canonical of ['/integrations', '/api-docs', '/trial-extension', '/changelog', '/unified-feed', '/feed', '/services/new']) {
      for (const markers of [{ rsc: '1' }, { rsc: '1', 'next-router-prefetch': '1' }]) {
        const target = await flightPath(canonical, markers);
        const response = await observed(anonymousPage.request, observations, target, { headers: { ...markers, cookie } });
        loginBoundary(response, target, origin);
        expect.soft(response.headers()['set-cookie']).toContain(`${session.name}=;`);
        expect.soft(response.headers()['set-cookie']).toContain('Max-Age=0');
        expect.soft(response.headers()['content-type'] ?? '').not.toContain('text/x-component');
      }
    }
  } finally { await attach(testInfo, observations); }
});

test('RSC-08 public settings retain exact methods and reject action-shaped access', async ({ anonymousPage, baseURL }, testInfo) => {
  const origin = nativeOrigin(baseURL);
  const observations: Observation[] = [];
  try {
    const target = '/api/v1/auth/public-settings';
    const baseline = await observed(anonymousPage.request, observations, target);
    expect(baseline.status()).toBe(200);
    const settings = await baseline.json();
    expect(settings).toMatchObject({ registration_mode: expect.any(String), registration_available: expect.any(Boolean), email_verification_required: true, providers: expect.any(Array) });
    await malformedPublicRsc(anonymousPage.request, observations, target, origin);
    for (const headers of [{ rsc: '1' }, { 'next-router-prefetch': '1' }, { purpose: 'prefetch' }]) {
      const validTarget = headers.rsc === '1' ? await flightPath(target, headers) : target;
      const response = await observed(anonymousPage.request, observations, validTarget, { headers });
      expect(response.status()).toBe(200);
      expect(await response.json()).toEqual(settings);
      privateResponse(response);
      noRedirect(response);
    }
    for (const method of ['HEAD', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS']) {
      loginBoundary(await observed(anonymousPage.request, observations, target, { method }), target, origin);
    }
    loginBoundary(await observed(anonymousPage.request, observations, target, { headers: { 'next-action': 'unregistered-boundary-action' } }), target, origin, true);
    for (const loginPath of ['/api/v1/auth/login', '/api/v1/auth/registration/email/start', '/api/v1/auth/registration/email/verify', '/api/v1/auth/register']) {
      loginBoundary(await observed(anonymousPage.request, observations, loginPath), loginPath, origin);
      for (const markers of [{}, { rsc: '1' }, { 'next-router-prefetch': '1' }]) {
        // Reach each actual public handler, but reject media type before login,
        // email, registration or any backend/provider side effect is attempted.
        const validTarget = markers.rsc === '1' ? await flightPath(loginPath, markers) : loginPath;
        const bounded = await observed(anonymousPage.request, observations, validTarget, {
          method: 'POST', headers: { ...markers, origin, 'sec-fetch-site': 'same-origin', 'content-type': 'text/plain' }, data: 'boundary-media-type-probe',
        });
        expect.soft(bounded.status()).toBe(415);
        privateResponse(bounded);
        noRedirect(bounded);
      }
      loginBoundary(await observed(anonymousPage.request, observations, loginPath, {
        method: 'POST', headers: { 'next-action': 'unregistered-boundary-action', origin, 'sec-fetch-site': 'same-origin' },
      }), loginPath, origin, true);
      const rejected = await observed(anonymousPage.request, observations, loginPath, {
        method: 'POST', headers: { origin: 'https://foreign.invalid', 'sec-fetch-site': 'cross-site', 'content-type': 'application/json' }, data: {},
      });
      expect.soft(rejected.status()).toBe(403);
      noRedirect(rejected);
    }
  } finally { await attach(testInfo, observations); }
});

test('RSC-08 publication negative reads remain anonymous but do not broaden eligibility', async ({ anonymousPage, baseURL }, testInfo) => {
  const origin = nativeOrigin(baseURL);
  const observations: Observation[] = [];
  const target = '/api/v1/public/sites/00000000-0000-0000-0000-000000000000';
  try {
    for (const headers of [{}, { 'next-router-prefetch': '1' }, { cookie: 'omnisolo_session=damaged-native-session' }]) {
      const response = await observed(anonymousPage.request, observations, target, { headers });
      expect.soft(response.status()).toBe(404);
      expect.soft(await response.text()).toBe('Publication not found.');
      expect.soft(response.headers()['cache-control']).toBe('no-store');
      noRedirect(response);
    }
    const validRsc = await malformedPublicRsc(anonymousPage.request, observations, target, origin);
    // Public document transport deliberately rejects ALL query strings, even
    // Next's reserved cache key. Flight must not broaden document eligibility.
    const flight = await observed(anonymousPage.request, observations, validRsc, { headers: { rsc: '1' } });
    expect.soft(flight.status()).toBe(400);
    expect.soft(await flight.text()).toBe('Public document unavailable.');
    expect.soft(flight.headers()['cache-control']).toBe('no-store');
    noRedirect(flight);
    for (const search of ['?unreviewed=1', '?_rsc=opaque']) {
      const response = await observed(anonymousPage.request, observations, target + search);
      expect.soft(response.status()).toBe(400);
      noRedirect(response);
    }
    for (const method of ['HEAD', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS']) {
      loginBoundary(await observed(anonymousPage.request, observations, target, { method }), target, origin);
    }
    loginBoundary(await observed(anonymousPage.request, observations, target + '/private'), target + '/private', origin);
    loginBoundary(await observed(anonymousPage.request, observations, target, { headers: { 'next-action': 'unregistered-boundary-action' } }), target, origin, true);
  } finally { await attach(testInfo, observations); }
});

test('RSC-08 disabled OIDC entries and a missing-state callback stay local without provider calls', async ({ anonymousPage, baseURL }, testInfo) => {
  const origin = nativeOrigin(baseURL);
  // Fail before a request if this is not the native runner's disabled-provider
  // environment; never turn this boundary test into a provider flow.
  for (const name of ['OMNISOLO_OIDC_GOOGLE_CLIENT_ID', 'OMNISOLO_OIDC_GOOGLE_CLIENT_SECRET', 'OMNISOLO_OIDC_KEYCLOAK_ISSUER', 'OMNISOLO_OIDC_KEYCLOAK_CLIENT_ID', 'OMNISOLO_OIDC_KEYCLOAK_CLIENT_SECRET']) {
    expect(process.env[name], `${name} must be absent for the no-provider boundary probe`).toBeUndefined();
  }
  const observations: Observation[] = [];
  try {
    for (const provider of ['google', 'keycloak']) {
      const target = `/api/v1/auth/oidc/${provider}`;
      await malformedPublicRsc(anonymousPage.request, observations, target, origin);
      for (const headers of [{}, { rsc: '1' }, { 'next-router-prefetch': '1' }]) {
        const validTarget = headers.rsc === '1' ? await flightPath(target, headers) : target;
        const response = await observed(anonymousPage.request, observations, validTarget, { headers });
        expect.soft(response.status()).toBe(404);
        privateResponse(response);
        noRedirect(response);
      }
      loginBoundary(await observed(anonymousPage.request, observations, target, { method: 'POST' }), target, origin);
      loginBoundary(await observed(anonymousPage.request, observations, target, { headers: { 'next-action': 'unregistered-boundary-action' } }), target, origin, true);
    }
    await malformedPublicRsc(anonymousPage.request, observations, '/api/v1/auth/oidc/callback', origin);
    for (const headers of [{}, { rsc: '1' }, { 'next-router-prefetch': '1' }]) {
      const targetPath = '/api/v1/auth/oidc/callback';
      const validTarget = headers.rsc === '1' ? await flightPath(targetPath, headers) : targetPath;
      const response = await observed(anonymousPage.request, observations, validTarget, { headers });
      expect.soft(response.status()).toBe(302);
      const target = new URL(response.headers().location ?? '', origin);
      expect.soft(target.origin).toBe(origin);
      expect.soft(target.pathname).toBe('/login');
      expect.soft(Array.from(target.searchParams.entries())).toEqual([['error', 'oidc-denied']]);
      privateResponse(response);
    }
    loginBoundary(await observed(anonymousPage.request, observations, '/api/v1/auth/oidc/callback', { method: 'POST' }), '/api/v1/auth/oidc/callback', origin);
  } finally { await attach(testInfo, observations); }
});

test('RSC-08 real eligible publication reads preserve bounded document eligibility and revocation', async ({ page, anonymousPage, baseURL }, testInfo) => {
  const { origin, owner } = await restoreOwner(page, baseURL);
  const headline = `Proxy boundary publication ${randomUUID()}`;
  const { receipt, headers } = await publishOwnedStorefront(page, baseURL, owner, {
    domain: null, pages: [{ path: '/', title: headline, seo_metadata: {}, blocks: [{ block_type: 'HeroBlock', content: { headline }, sort_order: 0 }] }],
  });
  const observations: Observation[] = [];
  let revoked = false;
  try {
    for (const markers of [{}, { 'next-router-prefetch': '1' }, { cookie: 'omnisolo_session=damaged-native-session' }]) {
      const response = await observed(anonymousPage.request, observations, receipt.public_path, { headers: markers });
      expect.soft(response.status()).toBe(200);
      expect.soft(response.headers()['content-type']).toBe('text/html; charset=utf-8');
      expect.soft(response.headers()['cache-control']).toBe('no-store');
      expect.soft(await response.text()).toContain(headline);
      noRedirect(response);
    }
    const validRsc = await malformedPublicRsc(anonymousPage.request, observations, receipt.public_path, origin);
    const flight = await observed(anonymousPage.request, observations, validRsc, { headers: { rsc: '1' } });
    expect.soft(flight.status()).toBe(400);
    expect.soft(await flight.text()).toBe('Public document unavailable.');
    expect.soft(flight.headers()['cache-control']).toBe('no-store');
    noRedirect(flight);
    const removed = await observed(page.request, observations, `/api/v1/builder/publications/${receipt.publication_id}`, { method: 'DELETE', headers });
    expect(removed.status()).toBe(200);
    expect(await removed.json()).toMatchObject({ status: 'revoked' });
    revoked = true;
    for (const markers of [{}, { 'next-router-prefetch': '1' }]) {
      const response = await observed(anonymousPage.request, observations, receipt.public_path, { headers: markers });
      expect.soft(response.status()).toBe(404);
      expect.soft(await response.text()).toBe('Publication not found.');
      noRedirect(response);
    }
    const revokedFlight = await observed(anonymousPage.request, observations, validRsc, { headers: { rsc: '1' } });
    expect.soft(revokedFlight.status()).toBe(400);
    expect.soft(await revokedFlight.text()).toBe('Public document unavailable.');
    noRedirect(revokedFlight);
  } finally {
    if (!revoked) {
      // Cleanup can only revoke the publication ID returned for this real,
      // signed seed owner. The disposable database owns all retained history.
      const cleanup = await page.request.delete(`/api/v1/builder/publications/${receipt.publication_id}`, { headers, maxRedirects: 0 });
      expect(cleanup.status()).toBe(200);
      expect(await cleanup.json()).toMatchObject({ status: 'revoked' });
    }
    await attach(testInfo, observations);
  }
});

test('RSC-09 anonymous login loads actual framework assets without opening internal page data', async ({ anonymousPage, baseURL }, testInfo) => {
  const origin = nativeOrigin(baseURL);
  const observations: Observation[] = [];
  const failedAssets: string[] = [];
  anonymousPage.on('response', response => {
    if (['script', 'stylesheet'].includes(response.request().resourceType()) && response.status() >= 400) failedAssets.push(response.url());
  });
  anonymousPage.on('requestfailed', request => {
    if (['script', 'stylesheet'].includes(request.resourceType())) failedAssets.push(request.url());
  });
  try {
    const response = await anonymousPage.goto('/login');
    expect(response?.status()).toBe(200);
    await expect(anonymousPage.getByLabel('Email or username')).toBeVisible();
    const assets = await anonymousPage.locator('script[src], link[rel="stylesheet"][href]').evaluateAll(nodes => nodes.map(node => node.getAttribute('src') ?? node.getAttribute('href') ?? ''));
    const frameworkAssets = [...new Set(assets.map(asset => new URL(asset, origin)).filter(asset => asset.origin === origin && asset.pathname.startsWith('/_next/static/')).map(asset => asset.pathname + asset.search))];
    expect(frameworkAssets.length).toBeGreaterThan(0);
    for (const asset of frameworkAssets) {
      const actual = await observed(anonymousPage.request, observations, asset);
      expect.soft(actual.status()).toBe(200);
      noRedirect(actual);
      expect.soft((await actual.body()).length).toBeGreaterThan(0);
    }
    expect(failedAssets).toEqual([]);
    for (const target of ['/_next/data/boundary-build/login.json', '/_next/data/boundary-build/dashboard.json']) {
      loginBoundary(await observed(anonymousPage.request, observations, target), target, origin);
      loginBoundary(await observed(anonymousPage.request, observations, `${target}?_rsc=`, { headers: { rsc: '1' } }), `${target}?_rsc=`, origin);
      const data = await observed(anonymousPage.request, observations, target, { headers: { 'x-nextjs-data': '1' } });
      expect.soft(data.status()).toBe(307);
      expect.soft(data.headers().location).toBeUndefined();
      const redirect = data.headers()['x-nextjs-redirect'];
      expect.soft(redirect).toBeDefined();
      if (redirect !== undefined) {
        const login = new URL(redirect, origin);
        expect.soft(login.origin).toBe(origin);
        expect.soft(login.pathname).toBe('/login');
        expect.soft(login.searchParams.getAll('next')).toEqual([target]);
      }
    }
  } finally { await attach(testInfo, observations); }
});
