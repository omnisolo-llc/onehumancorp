import { randomUUID } from 'node:crypto';
import { expect, type Page, type Response, type APIResponse } from '@playwright/test';
import { e2eDbQuery } from '../db_utils';

export const TIMECARD_PATH = '/api/v1/staff/timecard';
export type ClockOwner = { userId: string; tenantId: string };
export type ClockKind = 'CLOCK_IN' | 'CLOCK_OUT';
export type ClockEvent = { id: string; staff_id: string; event_type: ClockKind; offline_timestamp: string };
export type ClockRow = { id: string; type: string; timestamp: number; payload: string };
export type ClockEnvelope = {
  version: number; adapter: string; owner: ClockOwner;
  action: { id: string; type: string; timestamp: number; payload: { staff_id: string; event_type?: ClockKind; timestamp?: number } };
  context: Record<string, unknown>;
  routes: { plan: { id: string; method: string; body: unknown; maxAttempts: number }; status: string; attempts: number; attemptToken?: string }[];
};

export async function unlockClockTerminal(page: Page, owner: ClockOwner) {
  await page.goto('/pos/terminal');
  await expect(page.locator('#pos-keypad')).toBeVisible();
  const authenticated = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/pos/auth' && response.request().method() === 'POST');
  for (const digit of ['1', '2', '3', '4']) await page.locator('#pos-keypad').getByRole('button', { name: digit, exact: true }).click();
  const response = await authenticated;
  expect(response.status()).toBe(200);
  expect(await response.json()).toMatchObject({ success: true, staff: { id: owner.userId, tenant_id: owner.tenantId, role: 'ADMIN' } });
  await expect(page.getByText('Offline queue ready for this session.', { exact: true })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Clock In', exact: true })).toBeEnabled();
}

export function waitForClockPost(page: Page) {
  const origin = new URL(page.url()).origin;
  return page.waitForResponse(response => new URL(response.url()).origin === origin
    && new URL(response.url()).pathname === TIMECARD_PATH && response.request().method() === 'POST');
}

export function clockAcknowledgment(event: ClockEvent) {
  return { success: true, outcomes: [{ id: event.id, route: TIMECARD_PATH, status: 'acknowledged' }] };
}

export async function assertClockPost(response: Response, owner: ClockOwner, kind: ClockKind): Promise<ClockEvent> {
  expect(response.status()).toBe(200);
  const submitted = response.request().postDataJSON();
  expect(submitted).toEqual({ events: [{ id: expect.stringMatching(/^[A-Za-z0-9_-]{1,128}$/),
    staff_id: owner.userId, event_type: kind, offline_timestamp: expect.stringMatching(/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$/) }] });
  const event = submitted.events[0] as ClockEvent;
  expect(Number.isSafeInteger(Date.parse(event.offline_timestamp))).toBe(true);
  expect(await response.request().allHeaders()).toMatchObject({
    'idempotency-key': event.id, 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId,
  });
  expect(await response.json()).toEqual(clockAcknowledgment(event));
  return event;
}

export async function assertClockReceipt(response: Response | APIResponse, owner: ClockOwner, event: ClockEvent) {
  expect(response.status()).toBe(200);
  expect(response.headers()['cache-control']).toBe('private, no-store');
  expect(await response.json()).toEqual({ ...clockAcknowledgment(event), receipt: { version: 1, actor_id: owner.userId, ...event } });
}

export async function assertPersistedClock(owner: ClockOwner, event: ClockEvent) {
  const rows = await e2eDbQuery(`SELECT staff_id, event_type, event_time = $3::timestamptz AS exact_instant,
    request_identity::jsonb AS identity FROM ohc_timecard_event WHERE tenant_id=$1 AND id=$2`,
  [owner.tenantId, event.id, event.offline_timestamp]);
  expect(rows).toEqual([{ staff_id: owner.userId, event_type: event.event_type, exact_instant: true,
    identity: { version: 1, actor_id: owner.userId, ...event } }]);
}

/** Shared with checkout: strengthen only its existing Clock In step. */
export async function assertConfirmedClockIn(page: Page, response: Response, owner: ClockOwner) {
  const event = await assertClockPost(response, owner, 'CLOCK_IN');
  const receipt = await page.request.get(`${TIMECARD_PATH}/receipts/${event.id}`, {
    headers: { 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId },
  });
  await assertClockReceipt(receipt, owner, event);
  await assertPersistedClock(owner, event);
  await expect(page.getByText('1 saved clock change confirmed by the server.', { exact: true })).toBeVisible();
  await expect(page.getByText(/saved clock changes? awaiting server confirmation\./)).toHaveCount(0);
  return event;
}

/** Read the real persisted adapter without running queue methods or completing claims. */
export async function readClockRows(page: Page): Promise<ClockRow[]> {
  return page.evaluate(async () => {
    if (localStorage.getItem('omnisolo_queue_adapter_v2') !== 'indexeddb') throw new Error('Clock proof requires its selected real IndexedDB adapter');
    const db = await new Promise<IDBDatabase>((resolve, reject) => {
      const opening = indexedDB.open('OMNISOLO_Offline_Queue', 1);
      opening.onsuccess = () => resolve(opening.result); opening.onerror = () => reject(opening.error);
    });
    return new Promise<ClockRow[]>((resolve, reject) => {
      const tx = db.transaction('actions', 'readonly');
      const request = tx.objectStore('actions').getAll();
      tx.oncomplete = () => { db.close(); resolve((request.result as ClockRow[]).sort((a, b) => a.id.localeCompare(b.id))); };
      tx.onabort = () => { db.close(); reject(tx.error); };
    });
  });
}

export function eventFromClockRow(row: ClockRow, owner: ClockOwner, kind: ClockKind): ClockEvent {
  const envelope = JSON.parse(row.payload) as ClockEnvelope;
  const event = { id: row.id, staff_id: owner.userId, event_type: kind, offline_timestamp: new Date(row.timestamp).toISOString() };
  expect(row.type).toBe('staff_clock_event_v1');
  expect(envelope).toMatchObject({ version: 2, adapter: 'indexeddb', owner,
    action: { id: row.id, type: 'staff_clock_event_v1', timestamp: row.timestamp, payload: { staff_id: owner.userId, event_type: kind } }, context: {} });
  expect(envelope.action.payload).toEqual({ staff_id: owner.userId, event_type: kind });
  expect(envelope.routes.map(route => route.plan)).toEqual([{ id: TIMECARD_PATH, method: 'POST', body: { events: [event] }, maxAttempts: 1 }]);
  return event;
}

export async function expectClockRoute(page: Page, id: string, status: string, attempts: number) {
  await expect.poll(async () => {
    const row = (await readClockRows(page)).find(row => row.id === id);
    if (!row) return null;
    const envelope = JSON.parse(row.payload) as ClockEnvelope;
    return envelope.routes.map(route => ({ id: route.plan.id, status: route.status, attempts: route.attempts }));
  }).toEqual([{ id: TIMECARD_PATH, status, attempts }]);
}

/** Recreate two frozen pre-upgrade journals, never an acknowledgment of a clock effect. */
export async function seedHistoricalClocks(page: Page, owner: ClockOwner): Promise<ClockRow[]> {
  const rows = (['CLOCK_IN', 'CLOCK_OUT'] as const).map((kind, index) => {
    const timestamp = Date.now() - 120000 + index;
    const action = { id: `historical-${randomUUID()}`, type: kind, payload: { staff_id: owner.userId, timestamp }, timestamp };
    const plans = [
      { id: '/api/v1/sync/operation-intents', method: 'POST', body: { intents: [{ id: action.id, action_type: kind, payload: action.payload, timestamp: new Date(timestamp).toISOString() }] }, maxAttempts: 1 },
      { id: '/api/v1/sync/offline', method: 'POST', body: { mutations: [action] }, maxAttempts: 1 },
    ];
    const envelope = { version: 2, adapter: 'indexeddb', owner, action, context: {}, routes: plans.map((plan, route) =>
      index === 1 && route === 0 ? { plan, status: 'acknowledged', attempts: 1, attemptToken: `original-${randomUUID()}` }
        : { plan, status: 'pending', attempts: 0 }) };
    // Deliberate whitespace makes accidental parse-and-rewrite detectable too.
    return { id: action.id, type: kind, timestamp, payload: JSON.stringify(envelope, null, 2) };
  });
  await page.evaluate(async rows => {
    const db = await new Promise<IDBDatabase>((resolve, reject) => {
      const opening = indexedDB.open('OMNISOLO_Offline_Queue', 1);
      opening.onsuccess = () => resolve(opening.result); opening.onerror = () => reject(opening.error);
    });
    await new Promise<void>((resolve, reject) => {
      const tx = db.transaction('actions', 'readwrite');
      for (const row of rows) tx.objectStore('actions').add(row);
      tx.oncomplete = () => { db.close(); resolve(); }; tx.onabort = () => { db.close(); reject(tx.error); };
    });
  }, rows);
  return rows.sort((a, b) => a.id.localeCompare(b.id));
}
