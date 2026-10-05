import { randomUUID } from 'node:crypto';
import type { Page, Request } from '@playwright/test';
import { test, expect, E2E_ADMIN_USER, E2E_UNLIMITED_ADMIN_USER } from '../../../../e2e/fixtures';
import { e2eDbTransaction } from '../../../../e2e/db_utils';
import { requireLoopbackUrl } from '../../../../e2e/support/recorded_invitation';
import { loadAuthenticatedState } from '../../../../../scripts/playwright/session-state.mjs';

// Run with the native runner's fresh standalone app, disposable PostgreSQL proof
// and cached real seed sessions. These cases never sign up/login an extra owner,
// substitute an API response, or delete a row without its returned service ID.
const aliases = ['/booking-create.html', '/ui/booking-create.html'] as const;
const endpoint = '/api/v1/booking/services';
const query = `?tenant=${E2E_UNLIMITED_ADMIN_USER.organizationId}&campaign=retirement&campaign=again&opaque=%e2%9c%93+%20`;
// Next may normalize percent encoding and spaces; retain every decoded entry in order.
const queryEntries = Array.from(new URLSearchParams(query).entries());
const uuid = /^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$/i;
type Owner = { userId: string; tenantId: string };
type Service = { id: string; title: string };
type Actor = typeof E2E_ADMIN_USER | typeof E2E_UNLIMITED_ADMIN_USER;

async function restoreActor(page: Page, baseURL: string | undefined, actor: Actor = E2E_ADMIN_USER): Promise<Owner> {
  if (!baseURL) throw new Error('Booking retirement needs the native runner loopback app');
  requireLoopbackUrl(baseURL);
  const directory = process.env.OMNISOLO_E2E_SESSION_STATE_DIR;
  if (!directory) throw new Error('Booking retirement requires cached real seed actor sessions');
  const state = await loadAuthenticatedState(directory, new URL(baseURL).origin, actor);
  // Preserve this context's actual drafts/receipts during an account switch.
  await page.context().clearCookies();
  await page.context().addCookies(state.cookies);
  const identity = await page.request.get('/api/v1/auth/session-identity');
  expect(identity.status()).toBe(200);
  const owner = await identity.json();
  expect(owner.tenantId).toBe(actor.organizationId);
  expect(typeof owner.userId).toBe('string');
  expect(owner.userId).not.toBe('');
  return { userId: owner.userId, tenantId: owner.tenantId };
}

function servicePosts(page: Page) {
  const requests: Request[] = [];
  page.on('request', request => {
    if (request.method() === 'POST' && new URL(request.url()).pathname === endpoint) requests.push(request);
  });
  return requests;
}

async function openForm(page: Page, alias: string, baseURL: string | undefined) {
  const origin = new URL('/services/new', baseURL).origin;
  await page.goto(alias + query + '#details', { waitUntil: 'domcontentloaded' });
  await expect(page).toHaveURL(url => url.origin === origin && url.pathname === '/services/new' && url.hash === '#details');
  expect(Array.from(new URL(page.url()).searchParams.entries())).toEqual(queryEntries);
  await expect(page.getByRole('heading', { name: 'Add Service', exact: true })).toBeVisible();
  await expect(page.getByLabel('Service Title', { exact: true })).toBeEnabled();
}

async function fillService(page: Page, title: string, description: string, price: string) {
  await page.getByLabel('Service Title', { exact: true }).fill(title);
  await page.getByLabel('Description', { exact: true }).fill(description);
  await page.getByLabel('Price', { exact: true }).fill(price);
}

async function ownedQuery(tenant: string, sql: string, values: unknown[]) {
  // The shared helper verifies the runner-owned container/proof before any I/O.
  return e2eDbTransaction(async execute => {
    await execute("SELECT set_config('app.current_tenant', $1, true)", [tenant]);
    return execute(sql, values);
  });
}

async function assertNoService(owner: Owner, title: string) {
  expect(await ownedQuery(owner.tenantId, 'SELECT id FROM services WHERE tenant_id=$1 AND name=$2', [owner.tenantId, title])).toEqual([]);
}

async function cleanupServices(owner: Owner, created: Service[]) {
  for (const service of created) {
    const removed = await ownedQuery(owner.tenantId,
      'DELETE FROM services WHERE tenant_id=$1 AND id=$2 AND name=$3 RETURNING id',
      [owner.tenantId, service.id, service.title]);
    expect(removed).toEqual([{ id: service.id }]);
  }
}

async function saveAndVerify(page: Page, owner: Owner, created: Service[], title: string, description: string, cents: number) {
  const responsePromise = page.waitForResponse(response => response.request().method() === 'POST' && new URL(response.url()).pathname === endpoint);
  await page.getByRole('button', { name: 'Save Service', exact: true }).click();
  const response = await responsePromise;
  const receipt = await response.json();
  // Retain only a real successful returned ID so later assertion failures still
  // clean up the row they created. Ambiguous outcomes require investigation.
  if (response.status() === 200 && receipt.success === true && typeof receipt.service_id === 'string' && uuid.test(receipt.service_id)) {
    created.push({ id: receipt.service_id, title });
  }
  expect(response.status()).toBe(200);
  expect(receipt).toEqual({ success: true, service_id: expect.stringMatching(uuid), error: null });
  expect(response.request().postDataJSON()).toEqual({ title, description, price_cents: cents });
  expect(response.request().headers()['x-ohc-expected-user']).toBe(owner.userId);
  expect(response.request().headers()['x-ohc-expected-tenant']).toBe(owner.tenantId);
  const rows = await ownedQuery(owner.tenantId,
    'SELECT id, tenant_id, name, description, (price * 100)::bigint::text AS cents, price * 100 = $3::numeric AS exact_cents FROM services WHERE id=$1 AND tenant_id=$2',
    [receipt.service_id, owner.tenantId, cents]);
  expect(rows).toEqual([{ id: receipt.service_id, tenant_id: owner.tenantId, name: title, description, cents: String(cents), exact_cents: true }]);
  const foreign = E2E_UNLIMITED_ADMIN_USER.organizationId;
  expect(await ownedQuery(foreign, 'SELECT id FROM services WHERE id=$1 AND tenant_id=$2', [receipt.service_id, foreign])).toEqual([]);
  expect(await ownedQuery(owner.tenantId, 'SELECT id FROM bookings WHERE tenant_id=$1 AND service_id=$2', [owner.tenantId, receipt.service_id])).toEqual([]);
  await expect(page).toHaveURL(url => url.pathname === '/dashboard');
  return receipt.service_id as string;
}

for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
  test.describe(`booking aliases at ${viewport.width}px`, () => {
    test.use({ viewport });
    for (const alias of aliases) {
      test(`BC-01 ${alias} preserves query/hash and renders the maintained form without mutation`, async ({ page, baseURL }, testInfo) => {
        await restoreActor(page, baseURL);
        const posts = servicePosts(page);
        const brokenAssets: string[] = [];
        page.on('response', response => {
          if (response.status() >= 400 && ['script', 'stylesheet'].includes(response.request().resourceType())) brokenAssets.push(response.url());
        });
        page.on('requestfailed', request => {
          if (['script', 'stylesheet'].includes(request.resourceType())) brokenAssets.push(request.url());
        });
        const redirectPromise = page.waitForResponse(response => new URL(response.url()).pathname === alias);
        await openForm(page, alias, baseURL);
        const redirect = await redirectPromise;
        expect(redirect.status()).toBe(307);
        const location = new URL(redirect.headers().location, redirect.url());
        expect(location.origin).toBe(new URL(baseURL!).origin);
        expect(location.pathname).toBe('/services/new');
        expect(Array.from(location.searchParams.entries())).toEqual(queryEntries);
        expect(location.hash).toBe('');
        expect(redirect.headers()['cache-control']).toBe('private, no-store');
        await expect(page.getByLabel('Description', { exact: true })).not.toHaveAttribute('required', '');
        await expect(page.getByLabel('Price', { exact: true })).not.toHaveAttribute('required', '');
        await expect(page.getByText('Leave blank for $0. Saving a service does not charge a customer.')).toBeVisible();
        await expect(page.getByRole('checkbox', { name: 'Recurring payment' })).toBeDisabled();
        expect(posts).toHaveLength(0);
        expect(brokenAssets).toEqual([]);
        expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
        await testInfo.attach('service-form', { body: await page.screenshot(), contentType: 'image/png' });
      });

      test(`BC-03 ${alias} commits exactly 1299 cents to the signed tenant and retains its actual receipt`, async ({ page, baseURL }, testInfo) => {
        const owner = await restoreActor(page, baseURL);
        const created: Service[] = [];
        const posts = servicePosts(page);
        const sideEffectRequests: string[] = [];
        page.on('request', request => {
          const path = new URL(request.url()).pathname;
          if (!['GET', 'HEAD', 'OPTIONS'].includes(request.method())
            && /\/(?:checkout|payments?|subscriptions?|reservations?|messages?|send|bookings)(?:\/|$)/.test(path)) sideEffectRequests.push(path);
        });
        const title = `Booking retirement café 音楽 ${randomUUID()}`;
        const description = 'Séance de musique 🎵 — résumé, 日本語';
        try {
          await openForm(page, alias, baseURL);
          await fillService(page, title, description, '12.99');
          const id = await saveAndVerify(page, owner, created, title, description, 1299);
          expect(posts).toHaveLength(1);
          expect(sideEffectRequests).toEqual([]);
          await page.goto(alias + query);
          await expect(page.getByRole('heading', { name: 'Previously acknowledged service' })).toBeVisible();
          await expect(page.getByText(id, { exact: true })).toBeVisible();
          await page.reload();
          await expect(page.getByText(id, { exact: true })).toBeVisible();
          expect(posts).toHaveLength(1);
          await testInfo.attach('persisted-service-receipt', { body: await page.screenshot(), contentType: 'image/png' });
          await restoreActor(page, baseURL, E2E_UNLIMITED_ADMIN_USER);
          await openForm(page, alias, baseURL);
          await expect(page.getByLabel('Service Title', { exact: true })).toHaveValue('');
          await expect(page.getByText(id, { exact: true })).toHaveCount(0);
          expect(posts).toHaveLength(1);
        } finally { await cleanupServices(owner, created); }
      });
    }
  });
}

for (const alias of aliases) {
  test(`BC-02 ${alias} protects methods and framework RSC normalization without alias migration for RSC/prefetch`, async ({ page, anonymousPage, baseURL }, testInfo) => {
    await restoreActor(page, baseURL);
    const origin = new URL(baseURL!).origin;
    for (const method of ['GET', 'HEAD']) {
      const authenticated = await page.request.fetch(alias + query, { method, maxRedirects: 0 });
      expect(authenticated.status()).toBe(307);
      const target = new URL(authenticated.headers().location, origin);
      expect(target.origin).toBe(origin);
      expect(target.pathname).toBe('/services/new');
      expect(target.hash).toBe('');
      expect(Array.from(target.searchParams.entries())).toEqual(queryEntries);
      expect(authenticated.headers()['cache-control']).toBe('private, no-store');
      if (method === 'HEAD') expect((await authenticated.body()).length).toBe(0);
      const anonymous = await anonymousPage.request.fetch(alias + query, { method, maxRedirects: 0 });
      expect(anonymous.status()).toBe(307);
      const login = new URL(anonymous.headers().location, origin);
      expect(login.origin).toBe(origin);
      expect(login.pathname).toBe('/login');
      expect(login.hash).toBe('');
      expect(login.searchParams.getAll('next')).toHaveLength(1);
      const returnTo = login.searchParams.get('next') ?? '';
      expect(returnTo.startsWith(alias + '?')).toBe(true);
      const next = new URL(returnTo, origin);
      expect(next.origin).toBe(origin);
      expect(next.pathname).toBe(alias);
      expect(next.hash).toBe('');
      expect(Array.from(next.searchParams.entries())).toEqual(queryEntries);
      expect(anonymous.headers()['cache-control']).toBe('private, no-store');
      const guarded = await anonymousPage.request.fetch(alias + '?opaque=%2f%2F', { method, maxRedirects: 0 });
      expect(guarded.status()).toBe(307);
      expect(new URL(guarded.headers().location, origin).searchParams.get('next')).toBe('/dashboard');
    }
    for (const method of ['POST', 'PUT', 'PATCH', 'DELETE']) {
      const trusted = await page.request.fetch(alias, { method, maxRedirects: 0, headers: { origin, 'sec-fetch-site': 'same-origin' } });
      // Removed public files are missing pages, not mutation handlers.
      expect(trusted.status()).toBe(404);
      expect(trusted.headers().location).toBeUndefined();
      const foreign = await page.request.fetch(alias, { method, maxRedirects: 0,
        headers: { origin: 'https://foreign.invalid', 'sec-fetch-site': 'cross-site' } });
      expect(foreign.status()).toBe(403);
      expect(foreign.headers().location).toBeUndefined();
    }
    // Next's dedicated 404 render path skips malformed RSC hash normalization.
    // Otherwise its normalization must stay on this alias with an empty _rsc
    // key; neither path may migrate to the maintained service form.
    const malformed = await page.request.get(alias, { headers: { rsc: '1' }, maxRedirects: 0 });
    const malformedBody = await malformed.text();
    const malformedLocation = malformed.headers().location;
    const protocolResults: Array<{ path: string; headers: Record<string, string | undefined>; status: number; location: string | undefined; query: [string, string][] | null }> = [{
      path: alias, headers: { rsc: '1' }, status: malformed.status(), location: malformedLocation,
      query: malformedLocation === undefined ? null : Array.from(new URL(malformedLocation, origin).searchParams.entries()),
    }];
    // Soft assertions collect every variant but still fail this test when any
    // boundary is violated. Do not accept a canonical migration as normalization.
    expect.soft(malformed.headers()['x-nextjs-redirect']).toBeUndefined();
    if (malformed.status() === 404) {
      expect.soft(malformedLocation).toBeUndefined();
    } else {
      expect(malformed.status(), JSON.stringify({ alias, headers: { rsc: '1' },
        status: malformed.status(), location: malformedLocation, body: malformedBody.slice(0, 200) })).toBe(307);
      expect(malformedLocation).toBeDefined();
      const normalized = new URL(malformedLocation, origin);
      expect.soft(normalized.origin).toBe(origin);
      expect.soft(normalized.pathname).toBe(alias);
      expect.soft(normalized.hash).toBe('');
      expect.soft(Array.from(normalized.searchParams.entries())).toEqual([['_rsc', '']]);
      expect.soft(malformedBody).toBe('');
    }
    // The independently constructed valid RSC request below must end at 404,
    // including after a malformed request received a normalization redirect.
    // No redirects are followed and no page scripts execute in these API
    // requests; none can dispatch the service form's POST.
    for (const { path, headers } of [
      { path: alias + '?_rsc', headers: { rsc: '1' } },
      { path: alias, headers: { purpose: 'prefetch' } },
      { path: alias, headers: { 'next-router-prefetch': '1' } },
    ]) {
      const response = await page.request.get(path, { headers, maxRedirects: 0 });
      const location = response.headers().location;
      protocolResults.push({ path, headers, status: response.status(), location,
        query: location === undefined ? null : Array.from(new URL(location, origin).searchParams.entries()) });
      expect.soft(response.status(), JSON.stringify({ path, headers, status: response.status(),
        location: response.headers().location, body: (await response.text()).slice(0, 200) })).toBe(404);
      expect.soft(response.headers().location).toBeUndefined();
    }
    await testInfo.attach('booking-protocol-responses', {
      body: Buffer.from(JSON.stringify(protocolResults, null, 2)), contentType: 'application/json',
    });
  });

  for (const price of ['', '0']) {
    test(`BC-04 ${alias} saves optional description and ${price === '' ? 'blank' : 'zero'} price as a real free service`, async ({ page, baseURL }) => {
      const owner = await restoreActor(page, baseURL);
      const created: Service[] = [];
      const posts = servicePosts(page);
      const title = `Booking retirement free ${randomUUID()}`;
      try {
        await openForm(page, alias, baseURL);
        await fillService(page, title, '', price);
        await saveAndVerify(page, owner, created, title, '', 0);
        expect(posts).toHaveLength(1);
      } finally { await cleanupServices(owner, created); }
    });
  }

  test(`BC-04 ${alias} keeps invalid local inputs editable without dispatch`, async ({ page, baseURL }) => {
    const owner = await restoreActor(page, baseURL);
    const title = `Booking retirement invalid ${randomUUID()}`;
    const posts = servicePosts(page);
    await openForm(page, alias, baseURL);
    for (const price of ['12.999', '-1', '10000000.01']) {
      await fillService(page, title, '', price);
      await page.getByRole('button', { name: 'Save Service', exact: true }).click();
      await expect(page.getByRole('status').filter({ hasText: 'Enter a price from' })).toHaveText('Enter a price from $0 to $10,000,000 with no more than two decimal places.');
      await expect(page.getByLabel('Price', { exact: true })).toBeEnabled();
      expect(posts).toHaveLength(0);
    }
    await fillService(page, '   ', '', '12.99');
    await page.getByRole('button', { name: 'Save Service', exact: true }).click();
    await expect(page.getByRole('status').filter({ hasText: 'Enter a service title' })).toHaveText('Enter a service title before saving.');
    expect(posts).toHaveLength(0);
    await assertNoService(owner, title);
  });

  test(`BC-05 ${alias} restores its draft and fences an account switch before dispatch`, async ({ page, baseURL }) => {
    const owner = await restoreActor(page, baseURL);
    const title = `Booking retirement owner draft ${randomUUID()}`;
    const posts = servicePosts(page);
    await openForm(page, alias, baseURL);
    await fillService(page, title, 'Only the signed owner may restore this draft', '12.99');
    await page.reload();
    await expect(page.getByLabel('Service Title', { exact: true })).toHaveValue(title);
    await expect(page.getByLabel('Description', { exact: true })).toHaveValue('Only the signed owner may restore this draft');
    await expect(page.getByLabel('Price', { exact: true })).toHaveValue('12.99');
    const other = await restoreActor(page, baseURL, E2E_UNLIMITED_ADMIN_USER);
    await page.getByRole('button', { name: 'Save Service', exact: true }).click();
    await expect(page.getByLabel('Service Title', { exact: true })).toBeEnabled();
    await expect(page.getByLabel('Service Title', { exact: true })).toHaveValue('');
    await expect(page.getByLabel('Description', { exact: true })).toHaveValue('');
    await expect(page.getByLabel('Price', { exact: true })).toHaveValue('');
    expect(posts).toHaveLength(0);
    await assertNoService(owner, title);
    await assertNoService(other, title);
    await restoreActor(page, baseURL);
    await page.reload();
    await expect(page.getByLabel('Service Title', { exact: true })).toHaveValue(title);
    expect(posts).toHaveLength(0);
  });

  test(`BC-05 ${alias} holds an offline save across reload without claiming success or retrying`, async ({ page, baseURL }) => {
    const owner = await restoreActor(page, baseURL);
    const title = `Booking retirement offline ${randomUUID()}`;
    const posts = servicePosts(page);
    await openForm(page, alias, baseURL);
    await fillService(page, title, '', '12.99');
    // Real browser disconnection before dispatch. This proves an offline failure,
    // not a response lost after a server commit; component tests cover that race.
    await page.context().setOffline(true);
    try {
      await page.getByRole('button', { name: 'Save Service', exact: true }).click();
      await expect(page.getByRole('status').filter({ hasText: 'Could not confirm whether the service was saved.' })).toHaveText('Could not confirm whether the service was saved. This request is on hold to avoid creating a duplicate.');
      await expect(page.getByRole('button', { name: 'Save Service', exact: true })).toBeDisabled();
      await expect(page.getByRole('heading', { name: 'Service Saved!' })).toHaveCount(0);
    } finally { await page.context().setOffline(false); }
    const attempts = posts.length;
    expect(attempts).toBe(1);
    await page.reload();
    await expect(page.getByLabel('Service Title', { exact: true })).toHaveValue(title);
    await expect(page.getByRole('button', { name: 'Save Service', exact: true })).toBeDisabled();
    await expect(page.getByRole('status').filter({ hasText: 'Could not confirm whether the service was saved.' })).toContainText('on hold to avoid creating a duplicate');
    expect(posts).toHaveLength(attempts);
    await assertNoService(owner, title);
  });
}

test('BC-04 real invalid service API request returns 400 and commits no row', async ({ page, baseURL }) => {
  const owner = await restoreActor(page, baseURL);
  const title = `Booking retirement server validation ${randomUUID()}`;
  const created: Service[] = [];
  try {
    const response = await page.request.post(endpoint + query, {
      headers: { origin: new URL(baseURL!).origin, 'sec-fetch-site': 'same-origin',
        'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId },
      data: { title, description: '', price_cents: -1 },
    });
    const receipt = await response.json();
    if (response.status() === 200 && receipt.success === true && typeof receipt.service_id === 'string' && uuid.test(receipt.service_id)) {
      created.push({ id: receipt.service_id, title });
    }
    expect(response.status()).toBe(400);
    expect(receipt).toEqual({ success: false, service_id: null, error: 'invalid service fields' });
    await assertNoService(owner, title);
  } finally { await cleanupServices(owner, created); }
});

test('BC-06 loopback browser supports real Web Locks and owner draft storage', async ({ page, baseURL }) => {
  await restoreActor(page, baseURL);
  await openForm(page, aliases[0], baseURL);
  const capabilities = await page.evaluate(async suffix => {
    const key = `booking-retirement-capability:${suffix}`;
    let storage: boolean;
    try { localStorage.setItem(key, 'probe'); storage = localStorage.getItem(key) === 'probe'; }
    finally { localStorage.removeItem(key); }
    const locks = typeof navigator.locks?.request === 'function'
      && await navigator.locks.request(key, () => true);
    return { secureContext: window.isSecureContext, storage, locks };
  }, randomUUID());
  expect(capabilities).toEqual({ secureContext: true, storage: true, locks: true });
});

test('BC-06 unavailable Web Locks retain the draft without a service POST', async ({ page, baseURL }) => {
  const owner = await restoreActor(page, baseURL);
  await page.addInitScript(() => Object.defineProperty(navigator, 'locks', { value: undefined, configurable: true }));
  const posts = servicePosts(page);
  const title = `Booking retirement no locks ${randomUUID()}`;
  await openForm(page, aliases[0], baseURL);
  await fillService(page, title, '', '12.99');
  await page.getByRole('button', { name: 'Save Service', exact: true }).click();
  await expect(page.getByRole('status').filter({ hasText: 'Service saving is unavailable in this browser.' })).toHaveText('Service saving is unavailable in this browser. Your draft has been kept.');
  await page.reload();
  await expect(page.getByLabel('Service Title', { exact: true })).toHaveValue(title);
  expect(posts).toHaveLength(0);
  await assertNoService(owner, title);
});

test('BC-06 unavailable owner storage blocks restoration and dispatch', async ({ page, baseURL }) => {
  await restoreActor(page, baseURL);
  await page.addInitScript(() => {
    const read = Storage.prototype.getItem;
    Storage.prototype.getItem = function (key: string) {
      if (key.startsWith('omnisolo_onboarding_owned_v1:')) throw new DOMException('Storage unavailable', 'SecurityError');
      return read.call(this, key);
    };
  });
  const posts = servicePosts(page);
  await page.goto(aliases[1]);
  await expect(page.getByRole('status').filter({ hasText: 'Could not verify your session or restore its local request status.' })).toHaveText('Could not verify your session or restore its local request status. Saving is blocked.');
  await expect(page.getByRole('button', { name: 'Save Service', exact: true })).toBeDisabled();
  expect(posts).toHaveLength(0);
});

test('BC-06 public entries and publication eligibility stay bounded', async ({ anonymousPage }) => {
  for (const path of ['/login', '/register', '/verify-email', '/healthz']) {
    expect((await anonymousPage.request.get(path, { maxRedirects: 0 })).status(), path).toBe(200);
  }
  const publication = await anonymousPage.request.get('/api/v1/public/sites/00000000-0000-0000-0000-000000000000', { maxRedirects: 0 });
  expect(publication.status()).toBe(404);
  expect(publication.headers()['cache-control']).toContain('no-store');
  const canonical = await anonymousPage.request.get('/services/new', { maxRedirects: 0 });
  expect(canonical.status()).toBe(307);
  expect(new URL(canonical.headers().location, canonical.url()).pathname).toBe('/login');
});
