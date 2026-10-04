import { test, expect } from './fixtures';
import type { Browser, BrowserContextOptions, Page, Request } from '@playwright/test';
import { withOwnedBrowserContexts } from '../../scripts/playwright/owned-contexts.mjs';
import { startConfiguredCheckoutFixture } from './support/configured_checkout_fixture';
import { createGrowthOwner } from './growth_owner';
import { e2eDbQuery } from './db_utils';
import {
  TIMECARD_PATH, assertClockPost, assertClockReceipt, assertPersistedClock, eventFromClockRow,
  expectClockRoute, readClockRows, seedHistoricalClocks, unlockClockTerminal, waitForClockPost,
  type ClockOwner, type ClockEvent,
} from './support/clock_receipts';

type Runtime = Awaited<ReturnType<typeof startConfiguredCheckoutFixture>>;
type ForwardedClocks = () => { method: string; path: string }[];

async function withClockOwner(browser: Browser, contextOptions: BrowserContextOptions, fixture: Runtime, expectedPosts: number,
  use: (page: Page, owner: ClockOwner, posts: Request[], forwardedClocks: ForwardedClocks) => Promise<void>) {
  const before = fixture.evidence().forwardedHttp.length;
  const forwardedClocks: ForwardedClocks = () => fixture.evidence().forwardedHttp.slice(before)
    .filter(request => request.method === 'POST' && request.path === TIMECARD_PATH);
  await withOwnedBrowserContexts(browser, { ...contextOptions, baseURL: fixture.origin, proxy: fixture.proxy,
    storageState: { cookies: [], origins: [] }, serviceWorkers: 'block' as const }, async ([context]) => {
    // Select an existing production adapter in a new empty context. All queue
    // writes, claims and receipts still run through the shipped application.
    await context.addInitScript(origin => {
      if (location.origin === origin && localStorage.getItem('omnisolo_queue_adapter_v2') === null) {
        localStorage.setItem('omnisolo_queue_adapter_v2', 'indexeddb');
      }
    }, fixture.origin);
    const page = await context.newPage();
    const { userId, tenantId } = await createGrowthOwner(page, fixture.origin);
    const owner = { userId, tenantId };
    const posts: Request[] = [];
    page.on('request', request => {
      if (new URL(request.url()).origin === fixture.origin && request.method() === 'POST'
          && new URL(request.url()).pathname === TIMECARD_PATH) posts.push(request);
    });
    await unlockClockTerminal(page, owner);
    expect(await readClockRows(page)).toEqual([]);
    expect(await e2eDbQuery('SELECT id FROM ohc_timecard_event WHERE tenant_id=$1', [owner.tenantId])).toEqual([]);
    // The supported subject is the authenticated actor, not a fabricated PIN staff row.
    expect(await e2eDbQuery('SELECT id FROM ohc_staff_member WHERE tenant_id=$1 AND id=$2', [owner.tenantId, owner.userId])).toEqual([]);
    await use(page, owner, posts, forwardedClocks);
  });
  // Count actual proxy dispatches too: browser request events may hide a
  // transport-level retry of the same logical fetch.
  expect(forwardedClocks()).toHaveLength(expectedPosts);
  const forwarded = fixture.evidence().forwardedHttp.slice(before);
  expect(forwarded.filter(request => ['/api/v1/sync/operation-intents', '/api/v1/sync/offline'].includes(request.path))).toEqual([]);
  expect(fixture.evidence().requests).toEqual([]);
}

async function readReceipt(page: Page, owner: ClockOwner, event: ClockEvent) {
  const response = await page.request.get(`${TIMECARD_PATH}/receipts/${event.id}`, {
    headers: { 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId },
  });
  await assertClockReceipt(response, owner, event);
  await assertPersistedClock(owner, event);
}

async function queueOfflineClockIn(page: Page, owner: ClockOwner, posts: Request[], forwardedClocks: ForwardedClocks) {
  await page.context().setOffline(true);
  await expect.poll(() => page.evaluate(() => navigator.onLine)).toBe(false);
  await page.getByRole('button', { name: 'Clock In', exact: true }).click();
  await expect(page.getByRole('heading', { name: 'Clocked In', exact: true })).toBeVisible();
  await expect(page.getByText('Clock-in saved on this device. Server confirmation is shown below.', { exact: true })).toBeVisible();
  await expect(page.getByText('1 saved clock change awaiting server confirmation.', { exact: true })).toBeVisible();
  await expect(page.getByText(/saved clock changes? confirmed by the server\./)).toHaveCount(0);
  const rows = await readClockRows(page);
  expect(rows).toHaveLength(1);
  const event = eventFromClockRow(rows[0], owner, 'CLOCK_IN');
  await expectClockRoute(page, event.id, 'pending', 0);
  expect(posts).toHaveLength(0);
  expect(forwardedClocks()).toEqual([]);
  expect(await e2eDbQuery('SELECT id FROM ohc_timecard_event WHERE tenant_id=$1', [owner.tenantId])).toEqual([]);
  return { event, original: rows[0] };
}

test.describe('Terminal clocks have actual authenticated timecard receipts', () => {
  test.describe.configure({ mode: 'default' });
  let fixture: Runtime;
  test.beforeAll(async () => { fixture = await startConfiguredCheckoutFixture(); });
  test.afterAll(async () => { await fixture?.close(); });
  test.afterEach(async ({ browserName }, testInfo) => {
    if (fixture) await testInfo.attach(`clock-receipt-boundary-evidence-${browserName}`, {
      body: JSON.stringify(fixture.evidence(), null, 2), contentType: 'application/json',
    });
  });

  test('online Clock In and Clock Out each acknowledge a distinct persisted event without generic sync or provider calls', async ({ browser, contextOptions }) => {
    await withClockOwner(browser, contextOptions, fixture, 2, async (page, owner, posts) => {
      const events: ClockEvent[] = [];
      for (const kind of ['CLOCK_IN', 'CLOCK_OUT'] as const) {
        const submitted = waitForClockPost(page);
        await page.getByRole('button', { name: kind === 'CLOCK_IN' ? 'Clock In' : 'Clock Out', exact: true }).click();
        const event = await assertClockPost(await submitted, owner, kind);
        events.push(event);
        await expectClockRoute(page, event.id, 'acknowledged', 1);
        await readReceipt(page, owner, event);
        await expect(page.getByText(`${events.length} saved clock ${events.length === 1 ? 'change' : 'changes'} confirmed by the server.`, { exact: true })).toBeVisible();
      }
      expect(events[0].id).not.toBe(events[1].id);
      expect(posts.map(post => post.postDataJSON())).toEqual(events.map(event => ({ events: [event] })));
      const rows = await readClockRows(page);
      expect(rows).toHaveLength(2);
      for (const event of events) expect(eventFromClockRow(rows.find(row => row.id === event.id)!, owner, event.event_type)).toEqual(event);
      expect(await e2eDbQuery('SELECT id FROM ohc_timecard_event WHERE tenant_id=$1 ORDER BY id', [owner.tenantId]))
        .toEqual(events.map(event => ({ id: event.id })).sort((a, b) => a.id.localeCompare(b.id)));
      await expect(page.getByText(/saved clock changes? awaiting server confirmation\./)).toHaveCount(0);
      await expect(page.getByRole('heading', { name: 'Not Clocked In', exact: true })).toBeVisible();
    });
  });

  test('offline Clock In is locally held until connectivity returns and sends its frozen event exactly once', async ({ browser, contextOptions }) => {
    await withClockOwner(browser, contextOptions, fixture, 1, async (page, owner, posts, forwardedClocks) => {
      const { event, original } = await queueOfflineClockIn(page, owner, posts, forwardedClocks);
      const submitted = waitForClockPost(page);
      await page.context().setOffline(false);
      expect(await assertClockPost(await submitted, owner, 'CLOCK_IN')).toEqual(event);
      await expectClockRoute(page, event.id, 'acknowledged', 1);
      await readReceipt(page, owner, event);
      await expect(page.getByText('1 saved clock change confirmed by the server.', { exact: true })).toBeVisible();
      const [saved] = await readClockRows(page);
      expect(JSON.parse(saved.payload).action).toEqual(JSON.parse(original.payload).action);
      // Another genuine browser online transition cannot claim the acknowledged tombstone.
      await page.context().setOffline(true); await page.context().setOffline(false);
      await page.getByRole('button', { name: 'Check saved clock status', exact: true }).click();
      await expect(page.getByRole('button', { name: 'Check saved clock status', exact: true })).toBeEnabled();
      expect(posts.map(post => post.postDataJSON())).toEqual([{ events: [event] }]);
      expect(await e2eDbQuery('SELECT id FROM ohc_timecard_event WHERE tenant_id=$1', [owner.tenantId])).toEqual([{ id: event.id }]);
    });
  });

  test('a genuinely committed clock with a lost reply recovers by receipt GET after reload with one POST total', async ({ browser, contextOptions }) => {
    await withClockOwner(browser, contextOptions, fixture, 1, async (page, owner, posts, forwardedClocks) => {
      const { event, original } = await queueOfflineClockIn(page, owner, posts, forwardedClocks);
      // The actual committed local ID is known before any network send. Never
      // patch UUID/fetch or fabricate an action just to target the fault.
      fixture.armClockResponseLoss({ tenantId: owner.tenantId, actorId: owner.userId, event });
      const failed = page.waitForEvent('requestfailed', { predicate: request => request.method() === 'POST' && new URL(request.url()).pathname === TIMECARD_PATH });
      await page.context().setOffline(false);
      const request = await failed;
      expect(request.postDataJSON()).toEqual({ events: [event] });
      await expectClockRoute(page, event.id, 'reconciliation', 1);
      expect(posts).toHaveLength(1);
      expect(fixture.evidence().clockResponseLoss).toMatchObject({ eventId: event.id, requestBody: JSON.stringify({ events: [event] }), status: 200, complete: true, dropped: true, error: null });
      // Prove the effect exists before any readback can change local acknowledgment.
      await assertPersistedClock(owner, event);
      await expect(page.getByText('1 saved clock change awaiting server confirmation.', { exact: true })).toBeVisible();
      await expect(page.getByText(/saved clock changes? confirmed by the server\./)).toHaveCount(0);
      await page.reload();
      await unlockClockTerminal(page, owner);
      await expectClockRoute(page, event.id, 'reconciliation', 1);
      const receiptPath = `${TIMECARD_PATH}/receipts/${event.id}`;
      const receiptRequests: Request[] = [];
      page.on('request', request => { if (new URL(request.url()).pathname === receiptPath) receiptRequests.push(request); });
      const readback = page.waitForResponse(response => new URL(response.url()).pathname === receiptPath && response.request().method() === 'GET');
      await page.getByRole('button', { name: 'Check saved clock status', exact: true }).click();
      const receipt = await readback;
      await assertClockReceipt(receipt, owner, event);
      expect(await receipt.request().allHeaders()).toMatchObject({ 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId });
      await expectClockRoute(page, event.id, 'acknowledged', 1);
      await expect(page.getByText('1 saved clock change confirmed by the server.', { exact: true })).toBeVisible();
      await page.getByRole('button', { name: 'Check saved clock status', exact: true }).click();
      await expect(page.getByRole('button', { name: 'Check saved clock status', exact: true })).toBeEnabled();
      expect(receiptRequests.map(request => request.method())).toEqual(['GET']);
      expect(posts.map(post => post.postDataJSON())).toEqual([{ events: [event] }]);
      const rows = await readClockRows(page);
      expect(rows).toHaveLength(1);
      expect(JSON.parse(rows[0].payload).action).toEqual(JSON.parse(original.payload).action);
      await assertPersistedClock(owner, event);
      expect(await e2eDbQuery('SELECT id FROM ohc_timecard_event WHERE tenant_id=$1', [owner.tenantId])).toEqual([{ id: event.id }]);
    });
  });

  test('both historical pending/pending and acknowledged/pending clocks remain byte-identical across reload and a new v1 enqueue', async ({ browser, contextOptions }) => {
    await withClockOwner(browser, contextOptions, fixture, 1, async (page, owner, posts) => {
      const originals = await seedHistoricalClocks(page, owner);
      const originalIds = originals.map(row => row.id);
      await page.reload();
      await unlockClockTerminal(page, owner);
      expect(await readClockRows(page)).toEqual(originals);
      await expect(page.getByText('2 historical clock changes require review. Original records are preserved.', { exact: true })).toBeVisible();
      await page.context().setOffline(true); await page.context().setOffline(false);
      const submitted = waitForClockPost(page);
      await page.getByRole('button', { name: 'Clock In', exact: true }).click();
      const event = await assertClockPost(await submitted, owner, 'CLOCK_IN');
      expect(originalIds).not.toContain(event.id);
      await expectClockRoute(page, event.id, 'acknowledged', 1);
      await readReceipt(page, owner, event);
      await page.getByRole('button', { name: 'Check saved clock status', exact: true }).click();
      await expect(page.getByRole('button', { name: 'Check saved clock status', exact: true })).toBeEnabled();
      expect((await readClockRows(page)).filter(row => originalIds.includes(row.id))).toEqual(originals);
      expect(await readClockRows(page)).toHaveLength(3);
      expect(posts.map(post => post.postDataJSON())).toEqual([{ events: [event] }]);
      expect(await e2eDbQuery('SELECT id FROM ohc_timecard_event WHERE tenant_id=$1', [owner.tenantId])).toEqual([{ id: event.id }]);
      expect(await e2eDbQuery('SELECT id FROM operation_intents WHERE tenant_id=$1 AND id=ANY($2::text[])', [owner.tenantId, originalIds])).toEqual([]);
      await page.reload();
      await unlockClockTerminal(page, owner);
      expect((await readClockRows(page)).filter(row => originalIds.includes(row.id))).toEqual(originals);
      expect(await readClockRows(page)).toHaveLength(3);
      await expect(page.getByText('2 historical clock changes require review. Original records are preserved.', { exact: true })).toBeVisible();
      expect(posts).toHaveLength(1);
    });
  });
});
