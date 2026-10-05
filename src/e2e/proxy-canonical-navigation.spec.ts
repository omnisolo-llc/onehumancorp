import { randomUUID } from 'node:crypto';
import type { Page, Request, Response, TestInfo } from '@playwright/test';
import { test, expect, E2E_ADMIN_USER } from './fixtures';
import { loadAuthenticatedState } from '../../scripts/playwright/session-state.mjs';
import { requireLoopbackUrl } from './support/recorded_invitation';
import { navigateToPublicAuth } from '../ui/next/src/lib/auth/publicNavigation';

// Native app/backend and real seeded sessions only. No substituted responses or
// registration/email requests. The existing booking suite owns committed receipts.
const canonicalPages = [
  { path: '/integrations', api: '/api/v1/integrations' },
  { path: '/api-docs', api: '/api/v1/api-docs-spec' },
  { path: '/trial-extension', api: '/api/v1/billing/my-plan' },
  { path: '/changelog', api: '/api/v1/changelog' },
  { path: '/unified-feed', api: '/api/v1/agent-feed' },
  { path: '/feed', api: '/api/v1/agent-feed' },
  { path: '/services/new', api: '/api/v1/auth/session-identity' },
] as const;
const flightHeaderNames = ['rsc', 'next-router-state-tree', 'next-router-prefetch', 'next-router-segment-prefetch', 'next-url'] as const;
type ClientRouter = { push(path: string): void; prefetch(path: string): void };
type FlightTraffic = Response[] & { requests: Request[]; failures: string[]; consoleErrors: string[] };

async function restoreSeedSession(page: Page, baseURL: string | undefined) {
  if (!baseURL) throw new Error('Canonical navigation requires the native loopback app');
  requireLoopbackUrl(baseURL);
  const directory = process.env.OMNISOLO_E2E_SESSION_STATE_DIR;
  if (!directory) throw new Error('Canonical navigation requires cached real seed sessions');
  const state = await loadAuthenticatedState(directory, new URL(baseURL).origin, E2E_ADMIN_USER);
  await page.context().clearCookies();
  await page.context().addCookies(state.cookies);
  const identity = await page.request.get('/api/v1/auth/session-identity');
  expect(identity.status()).toBe(200);
  expect(await identity.json()).toMatchObject({ tenantId: E2E_ADMIN_USER.organizationId });
}

function watchFlight(page: Page) {
  const responses = Object.assign([] as Response[], { requests: [] as Request[], failures: [] as string[], consoleErrors: [] as string[] }) as FlightTraffic;
  page.on('request', request => {
    if (request.headers().rsc === '1') responses.requests.push(request);
  });
  page.on('response', response => {
    if (response.request().headers().rsc === '1') responses.push(response);
  });
  page.on('requestfailed', request => {
    if (request.headers().rsc === '1' && request.failure()?.errorText !== 'net::ERR_ABORTED') {
      responses.failures.push(`${request.url()}: ${request.failure()?.errorText}`);
    }
  });
  page.on('console', message => {
    if (message.type() === 'error' && /Failed to fetch RSC payload|ERR_TOO_MANY_REDIRECTS/i.test(message.text())) responses.consoleErrors.push(message.text());
  });
  return responses;
}

function expectNoFlightLoop(traffic: FlightTraffic) {
  expect(traffic.failures, 'Client navigation must not fail and silently recover with a document request').toEqual([]);
  expect(traffic.consoleErrors, 'Next must not fall back after a failed Flight fetch').toEqual([]);
  for (const request of traffic.requests) {
    const chain: string[] = [];
    for (let hop: Request | null = request; hop; hop = hop.redirectedFrom()) chain.push(hop.url());
    expect(new Set(chain).size, `Repeated URL in Flight redirect chain: ${chain.join(' -> ')}`).toBe(chain.length);
    expect(chain.length, `Unexpected Flight redirect chain: ${chain.join(' -> ')}`).toBeLessThanOrEqual(3);
  }
}

function protocolHeaders(response: Response) {
  const headers: Record<string, string> = {};
  for (const name of flightHeaderNames) {
    const value = response.request().headers()[name];
    if (value !== undefined) headers[name] = value;
  }
  return headers;
}

async function attachFlight(testInfo: TestInfo, responses: FlightTraffic) {
  await testInfo.attach('canonical-flight-traffic', {
    body: Buffer.from(JSON.stringify({ responses: responses.map(response => ({
      url: response.url(), headers: protocolHeaders(response), status: response.status(),
      contentType: response.headers()['content-type'], location: response.headers().location,
    })), requests: responses.requests.map(request => ({ url: request.url(), redirectedFrom: request.redirectedFrom()?.url() })),
    failures: responses.failures, consoleErrors: responses.consoleErrors }, null, 2)), contentType: 'application/json',
  });
}

async function navigateWithRouter(page: Page, method: 'push' | 'prefetch', path: string) {
  // Pinned Next exposes its actual publicAppRouterInstance for debugging. This
  // drives the same client router and network protocol as useRouter, without
  // manufacturing a component, a Flight body, or a registration challenge.
  await page.waitForFunction(() => typeof (window as unknown as { next?: { router?: ClientRouter } }).next?.router?.push === 'function');
  await page.evaluate(({ operation, destination }) => {
    const router = (window as unknown as { next: { router: ClientRouter } }).next.router;
    router[operation](destination);
  }, { operation: method, destination: path });
}

async function expectCanonicalContent(page: Page, path: string) {
  if (path === '/integrations') await expect(page.getByRole('heading', { name: 'Verified business connections', exact: true })).toBeVisible();
  else if (path === '/api-docs') await expect(page.locator('.swagger-ui')).toBeVisible();
  else if (path === '/trial-extension') {
    await expect(page.getByRole('heading', { name: 'Plan and Trial Availability', exact: true })).toBeVisible();
    await expect(page.getByRole('status', { name: 'Current plan' })).toContainText('Current verified plan:');
  } else if (path === '/changelog') {
    await expect(page.getByTestId('changelog-title')).toBeVisible();
    const response = await page.request.get('/api/v1/changelog');
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toContain('application/json');
    const sections = await response.json();
    expect(Array.isArray(sections)).toBe(true);
    expect(sections.length).toBeGreaterThan(0);
    expect(typeof sections[0].version).toBe('string');
    await expect(page.getByRole('heading', { name: sections[0].version, exact: true }).first()).toBeVisible();
  } else if (path === '/unified-feed') {
    await expect(page.getByRole('heading', { name: 'Today', exact: true })).toBeVisible();
    await expect(page.getByTestId('agent-feed')).toBeVisible();
  } else if (path === '/feed') await expect(page.getByTestId('agent-feed')).toBeVisible();
  else if (path === '/services/new') {
    await expect(page.getByRole('heading', { name: 'Add Service', exact: true })).toBeVisible();
    await expect(page.getByLabel('Service Title', { exact: true })).toBeEnabled();
  } else throw new Error(`Unreviewed canonical target ${path}`);
}

function apiRead(page: Page, path: string) {
  return page.waitForResponse(response => new URL(response.url()).pathname === path && response.request().method() === 'GET');
}

async function expectLoginDenial(response: Awaited<ReturnType<Page['request']['get']>>, origin: string) {
  expect(response.status()).toBe(307);
  const target = new URL(response.headers().location, origin);
  expect(target.origin).toBe(origin);
  expect(target.pathname).toBe('/login');
  expect(response.headers()['content-type'] ?? '').not.toContain('text/x-component');
  expect(response.headers()['cache-control']).toContain('no-store');
}

for (const route of canonicalPages) {
  test(`RSC-06 ${route.path} supports document/client navigation and denies anonymous or browser-expired Flight`, async ({ page, anonymousPage, baseURL }, testInfo) => {
    await restoreSeedSession(page, baseURL);
    const origin = new URL(baseURL!).origin;
    const flights = watchFlight(page);
    try {
      const read = apiRead(page, route.api);
      const documentResponse = await page.goto(route.path, { waitUntil: 'domcontentloaded' });
      expect(documentResponse?.status()).toBe(200);
      expect(documentResponse?.headers()['content-type']).toContain('text/html');
      const api = await read;
      expect(api.status()).toBe(200);
      expect(api.headers()['content-type']).toContain('application/json');
      await expectCanonicalContent(page, route.path);

      await page.goto('/dashboard', { waitUntil: 'domcontentloaded' });
      await expect(page.getByRole('heading', { name: 'Dashboard', exact: true }).first()).toBeVisible();
      const nonce = randomUUID();
      const destination = `${route.path}?navigation-probe=${nonce}`;
      await page.evaluate(value => { document.documentElement.dataset.navigationProbe = value; }, nonce);
      await navigateWithRouter(page, 'prefetch', destination);
      await expect.poll(() => flights.filter(response => {
        const url = new URL(response.url());
        return url.pathname === route.path && url.searchParams.get('navigation-probe') === nonce
          && response.request().headers()['next-router-prefetch'] !== undefined;
      }).length).toBeGreaterThan(0);
      await navigateWithRouter(page, 'push', destination);
      await expect(page).toHaveURL(url => url.origin === origin && url.pathname === route.path && url.searchParams.get('navigation-probe') === nonce);
      await expectCanonicalContent(page, route.path);
      expect(await page.evaluate(() => document.documentElement.dataset.navigationProbe)).toBe(nonce);
      const canonicalFlight = flights.filter(response => {
        const url = new URL(response.url());
        return url.pathname === route.path && url.searchParams.get('navigation-probe') === nonce;
      });
      expect(canonicalFlight.length).toBeGreaterThan(0);
      for (const response of canonicalFlight) {
        expect(response.status()).toBe(200);
        expect(response.headers()['content-type']).toContain('text/x-component');
        expect(new URL(response.url()).searchParams.has('_rsc')).toBe(true);
      }
      const actualFlight = canonicalFlight.at(-1)!;
      const headers = protocolHeaders(actualFlight);
      await expectLoginDenial(await anonymousPage.request.get(actualFlight.url(), { headers, maxRedirects: 0 }), origin);

      // Expire actual server-issued browser cookies without fabricating a seal.
      // This proves browser-cookie expiry, not server-side cryptographic expiry.
      const cookies = (await page.context().cookies()).filter(cookie => /^(?:__Host-)?omnisolo_session$/.test(cookie.name));
      expect(cookies.length).toBeGreaterThan(0);
      await page.context().addCookies(cookies.map(cookie => ({ ...cookie, expires: Math.floor(Date.now() / 1000) - 1 })));
      expect((await page.context().cookies()).filter(cookie => /^(?:__Host-)?omnisolo_session$/.test(cookie.name))).toEqual([]);
      await expectLoginDenial(await page.request.get(actualFlight.url(), { headers, maxRedirects: 0 }), origin);
    } finally { await attachFlight(testInfo, flights); }
  });
}

for (const route of [
  { source: '/dashboard', target: '/integrations' },
  { source: '/settings', target: '/api-docs' },
  { source: '/dashboard', target: '/trial-extension' },
  { source: '/dashboard', target: '/changelog' },
  { source: '/dashboard/bookings', target: '/feed' },
  { source: '/dashboard', target: '/services/new' },
]) {
  test(`RSC-06 existing Next Link reaches ${route.target}`, async ({ page, baseURL }, testInfo) => {
    await restoreSeedSession(page, baseURL);
    const flights = watchFlight(page);
    try {
      await page.goto(route.source, { waitUntil: 'domcontentloaded' });
      if (route.target === '/services/new') await page.getByRole('button', { name: 'Quick Actions', exact: true }).click();
      const link = page.locator(`a[href="${route.target}"]:visible`).first();
      await expect(link).toBeVisible();
      const nonce = randomUUID();
      await page.evaluate(value => { document.documentElement.dataset.navigationProbe = value; }, nonce);
      await link.hover();
      await link.click();
      await expect(page).toHaveURL(url => url.origin === new URL(baseURL!).origin && url.pathname === route.target);
      await expectCanonicalContent(page, route.target);
      expect(await page.evaluate(() => document.documentElement.dataset.navigationProbe)).toBe(nonce);
      expect(flights.some(response => new URL(response.url()).pathname === route.target && response.status() === 200
        && response.headers()['content-type']?.includes('text/x-component'))).toBe(true);
    } finally { await attachFlight(testInfo, flights); }
  });
}

test('RSC-07 anonymous document entries remain available without protected content', async ({ anonymousPage }) => {
  for (const path of ['/login', '/register', '/verify-email', '/healthz']) {
    const response = await anonymousPage.request.get(path, { maxRedirects: 0 });
    expect(response.status(), path).toBe(200);
    expect(response.headers()['content-type'] ?? '').not.toContain('text/x-component');
  }
  expect((await anonymousPage.request.get('/api/v1/auth/session-identity')).status()).toBe(401);
});

for (const path of ['/login', '/register', '/verify-email', '/healthz']) {
  test(`RSC-07 ${path} retains the explicit anonymous Flight/prefetch policy`, async ({ anonymousPage, baseURL }, testInfo) => {
    const observed = [];
    for (const variant of [
      { path: path + '?_rsc', headers: { rsc: '1' } },
      { path, headers: { 'next-router-prefetch': '1' } },
      { path, headers: { purpose: 'prefetch' } },
    ]) {
      const response = await anonymousPage.request.get(variant.path, { headers: variant.headers, maxRedirects: 0 });
      observed.push({ path: variant.path, headers: variant.headers, status: response.status(),
        location: response.headers().location, contentType: response.headers()['content-type'] });
      expect.soft(response.status()).toBe(307);
      const location = response.headers().location;
      expect.soft(location).toBeDefined();
      if (location) {
        const destination = new URL(location, baseURL);
        expect.soft(destination.origin).toBe(new URL(baseURL!).origin);
        expect.soft(destination.pathname).toBe('/login');
      }
      expect.soft(response.headers()['content-type'] ?? '').not.toContain('text/x-component');
    }
    await testInfo.attach('public-invocation-policy', { body: Buffer.from(JSON.stringify(observed, null, 2)), contentType: 'application/json' });
  });
}

for (const route of [
  { source: '/login', link: 'Check registration', target: '/register', heading: 'Create your account' },
  { source: '/register', link: 'Sign in', target: '/login', heading: 'Sign in to OmniSolo OneHumanCorp' },
  { source: '/verify-email', link: 'Start again', target: '/register', heading: 'Create your account' },
]) {
  test(`RSC-07 anonymous ${route.link} navigation reaches ${route.target}`, async ({ anonymousPage, baseURL }, testInfo) => {
    const flights = watchFlight(anonymousPage);
    try {
      await anonymousPage.goto(route.source, { waitUntil: 'domcontentloaded' });
      await anonymousPage.getByRole('link', { name: route.link, exact: true }).click();
      await expect(anonymousPage).toHaveURL(url => url.origin === new URL(baseURL!).origin && url.pathname === route.target);
      await expect(anonymousPage.getByRole('heading', { name: route.heading, exact: true })).toBeVisible();
      await expect(anonymousPage.locator('[data-auth-shell]')).toBeVisible();
      await expect(anonymousPage.getByRole('navigation', { name: 'Primary' })).toHaveCount(0);
      expect((await anonymousPage.request.get('/api/v1/auth/session-identity')).status()).toBe(401);
      expectNoFlightLoop(flights);
    } finally { await attachFlight(testInfo, flights); }
  });
}

test('RSC-07 production auth navigation reaches verify-email without sending verification mail', async ({ anonymousPage, baseURL }, testInfo) => {
  const flights = watchFlight(anonymousPage);
  const registrationPosts: string[] = [];
  anonymousPage.on('request', request => {
    if (request.method() === 'POST' && new URL(request.url()).pathname.startsWith('/api/v1/auth/registration/')) registrationPosts.push(request.url());
  });
  try {
    await anonymousPage.goto('/register', { waitUntil: 'domcontentloaded' });
    await expect(anonymousPage.getByRole('heading', { name: 'Create your account', exact: true })).toBeVisible();
    // Exercise the same production transition used after registration creates a
    // challenge. This isolated navigation check does not claim email issuance.
    const verificationDocument = anonymousPage.waitForResponse(response => new URL(response.url()).pathname === '/verify-email'
      && response.request().resourceType() === 'document');
    await anonymousPage.evaluate(navigateToPublicAuth, '/verify-email');
    const response = await verificationDocument;
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toContain('text/html');
    await expect(anonymousPage).toHaveURL(url => url.origin === new URL(baseURL!).origin && url.pathname === '/verify-email');
    await expect(anonymousPage.getByRole('heading', { name: 'Verify your email', exact: true })).toBeVisible();
    await expect(anonymousPage.getByRole('alert').filter({ hasText: 'No active email verification was found.' })).toBeVisible();
    expect(registrationPosts).toEqual([]);
    expectNoFlightLoop(flights);
  } finally { await attachFlight(testInfo, flights); }
});

async function signInThroughForm(page: Page) {
  await page.goto('/login', { waitUntil: 'domcontentloaded' });
  await page.getByLabel('Email or username', { exact: true }).fill(E2E_ADMIN_USER.email);
  await page.getByLabel('Password', { exact: true }).fill(E2E_ADMIN_USER.password);
  await page.getByLabel('Organization', { exact: true }).fill(E2E_ADMIN_USER.organizationId);
  const loginResponse = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/auth/login' && response.request().method() === 'POST');
  await page.getByRole('button', { name: 'Log in', exact: true }).click();
  expect((await loginResponse).status()).toBe(200);
  await expect(page).toHaveURL(url => url.pathname === '/dashboard');
  await expect(page.getByRole('heading', { name: 'Dashboard', exact: true }).first()).toBeVisible();
}

test('RSC-07 real login router transition reaches the authenticated dashboard', async ({ anonymousPage }, testInfo) => {
  const flights = watchFlight(anonymousPage);
  try {
    await signInThroughForm(anonymousPage);
    const identity = await anonymousPage.request.get('/api/v1/auth/session-identity');
    expect(identity.status()).toBe(200);
    expect(await identity.json()).toMatchObject({ tenantId: E2E_ADMIN_USER.organizationId });
    expect((await anonymousPage.context().cookies()).some(cookie => /^(?:__Host-)?omnisolo_session$/.test(cookie.name) && cookie.httpOnly)).toBe(true);
    expectNoFlightLoop(flights);
  } finally { await attachFlight(testInfo, flights); }
});

test('RSC-07 logout client transition reaches usable anonymous login', async ({ anonymousPage }, testInfo) => {
  // Use a freshly issued session; never revoke the cache shared by other cases.
  const flights = watchFlight(anonymousPage);
  try {
    await signInThroughForm(anonymousPage);
    const logout = anonymousPage.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/auth/logout' && response.request().method() === 'POST');
    await anonymousPage.getByRole('button', { name: 'Log out', exact: true }).first().click();
    expect((await logout).ok()).toBe(true);
    await expect(anonymousPage).toHaveURL(url => url.pathname === '/login');
    await expect(anonymousPage.getByLabel('Email or username', { exact: true })).toBeEnabled();
    expect((await anonymousPage.request.get('/api/v1/auth/session-identity')).status()).toBe(401);
    await expect(anonymousPage.getByRole('navigation', { name: 'Primary' })).toHaveCount(0);
    expectNoFlightLoop(flights);
  } finally { await attachFlight(testInfo, flights); }
});

for (const mode of ['document', 'client'] as const) {
  test(`RSC-07 signed-in login next remains usable through ${mode} navigation`, async ({ page, baseURL }, testInfo) => {
    await restoreSeedSession(page, baseURL);
    const flights = watchFlight(page);
    const target = `/changelog?navigation-probe=${randomUUID()}`;
    const login = `/login?next=${encodeURIComponent(target)}`;
    try {
      if (mode === 'document') await page.goto(login, { waitUntil: 'domcontentloaded' });
      else {
        await page.goto('/dashboard', { waitUntil: 'domcontentloaded' });
        await navigateWithRouter(page, 'push', login);
      }
      await expect(page).toHaveURL(url => url.origin === new URL(baseURL!).origin && url.pathname + url.search === target);
      await expectCanonicalContent(page, '/changelog');
      await expect(page.getByLabel('Email or username', { exact: true })).toHaveCount(0);
      expectNoFlightLoop(flights);
    } finally { await attachFlight(testInfo, flights); }
  });
}
