import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';
import { transformSync } from 'esbuild';

const source = await readFile('src/e2e/support/team_fixture.ts', 'utf8');
const helper = source.slice(source.indexOf('export function captureTeamResponse'), source.indexOf('export async function readTeamPage')).replace('export ', '');
const { code } = transformSync(helper, { loader: 'ts', target: 'es2022' });
const capture = new Function('expect', `${code}; return captureTeamResponse;`)(value => ({ toBe: expected => assert.equal(value, expected) }));
const owner = { origin: 'http://127.0.0.1:12345', userId: 'owned-user', tenantId: 'owned-tenant' };
function fixture() {
  const listeners = [];
  const page = { waitForEvent(event, { predicate }) { return new Promise(resolve => listeners.push({ event, predicate, resolve })); } };
  const emit = (event, value) => { for (const listener of listeners) if (listener.event === event && listener.predicate(value)) listener.resolve(value); };
  return { page, emit };
}
function request({ method = 'GET', status = 200, tenant = owner.tenantId } = {}) {
  const response = { status: () => status, request: () => ({ headers: () => ({ 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': tenant }) }), json: async () => ({ pending_approvals: [], next_cursor: null }) };
  return { url: () => owner.origin + '/api/v1/agents/approvals', method: () => method, response: async () => response };
}
test('headers or failed navigation requests cannot satisfy the body receipt capture', async () => {
  const { page, emit } = fixture(); let settled = false;
  const pending = capture(page, owner, '/api/v1/agents/approvals').then(value => { settled = true; return value; });
  emit('response', request()); emit('requestfailed', request());
  emit('requestfinished', request({ method: 'POST' }));
  await new Promise(resolve => setImmediate(resolve)); assert.equal(settled, false);
  emit('requestfinished', request());
  assert.deepEqual(await (await pending).json(), { pending_approvals: [], next_cursor: null });
});
for (const fault of [{ status: 400 }, { tenant: 'foreign' }]) {
  test(`completed Team response still rejects ${JSON.stringify(fault)}`, async () => {
    const { page, emit } = fixture(); const pending = capture(page, owner, '/api/v1/agents/approvals');
    const rejected = assert.rejects(pending, assert.AssertionError);
    emit('requestfinished', request(fault)); await rejected;
  });
}
test('Team keeps exact response-body and database checks after completed capture', () => {
  assert.match(source, /expect\(await response\.json\(\)\)\.toEqual\(\{ pending_approvals:/);
  assert.match(source, /expect\(await e2eDbQuery\(`SELECT id,tenant_id,event_source/);
  assert.match(source, /expect\(body\.dispatch\)\.toEqual\(\{ status: 'NOT_REQUESTED'/);
});
