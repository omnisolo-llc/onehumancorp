import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import vm from 'node:vm';

const files = [
  'src/ui/next/public/triage.html',
  'src/ui/next/public/ui/triage.html',
  'src/ui/next/public/api/ui/triage.html',
  'src/ui/next/public/api/v1/ui/triage.html',
  'src/ui/tauri/src/ui/triage.html',
];
const receipt = (approved = true, edit = '') => ({
  status: 'success', success: true, decision_recorded: true,
  item: { id: 'owned', tenant_id: 'tenant-a', lifecycle_state: approved ? 'APPROVED' : 'DISMISSED', edited_payload: edit, proposed_action: { message: edit } },
  dispatch: { status: 'NOT_REQUESTED', detail: 'Decision and draft saved; no execution was requested', receipt_id: null },
});

// Execute the maintained handler unchanged; isolate only its browser/network
// dependencies. Removing its JSON await or identity check must fail these tests.
async function load(file, response) {
  const html = await readFile(new URL(`../${file}`, import.meta.url), 'utf8');
  const start = html.indexOf('      async function handleDecision(');
  const end = html.indexOf('      function startEdit(', start);
  assert.ok(start >= 0 && end > start, 'the actual static handler must be present');
  const requests = [], alerts = [];
  const context = vm.createContext({
    items: [{ id: 'owned', tenant_id: 'tenant-a' }, { id: 'remaining', tenant_id: 'tenant-a' }],
    editingId: 'owned', reviewingId: 'owned', currentTenant: 'tenant-a', renderCount: 0,
    fetch: async (url, init) => { requests.push({ url, body: JSON.parse(init.body) }); return typeof response === 'function' ? response() : response; },
    alert: message => alerts.push(message), console: { error() {} },
  });
  vm.runInContext(`function tenantId() { return currentTenant; } function renderUI() { ++renderCount; }\n${html.slice(start, end)}`, context);
  return { context, requests, alerts, decide: (approved = true, edit = '') => context.handleDecision('owned', approved, edit) };
}
function retained(x) {
  assert.deepEqual(Array.from(x.context.items, item => item.id), ['owned', 'remaining']);
  assert.equal(x.context.editingId, 'owned');
  assert.equal(x.context.reviewingId, 'owned');
  assert.equal(x.context.renderCount, 0);
}
for (const file of files) {
  test(`${file}: a pending body retains the card and draft until a matching receipt arrives`, async () => {
    let deliver;
    const body = new ReadableStream({ start(controller) { deliver = value => { controller.enqueue(new TextEncoder().encode(JSON.stringify(value))); controller.close(); }; } });
    const x = await load(file, new Response(body, { status: 200 }));
    const pending = x.decide();
    try { await new Promise(resolve => setImmediate(resolve)); retained(x); }
    finally { deliver(receipt()); await pending; }
    assert.deepEqual(Array.from(x.context.items, item => item.id), ['remaining']);
    assert.equal(x.context.editingId, null);
    assert.equal(x.requests.length, 1);
    assert.deepEqual(x.requests[0].body, { triage_item_id: 'owned', approved: true, edited_payload: '' });
  });
  test(`${file}: malformed and contradictory HTTP 200 receipts retain the draft`, async () => {
    const invalid = [null, [], {}, { ...receipt(), success: false }, { ...receipt(), decision_recorded: false },
      { ...receipt(), error: 'unconfirmed' },
      { ...receipt(), item: { ...receipt().item, id: 'another' } },
      { ...receipt(), item: { ...receipt().item, tenant_id: 'another-tenant' } },
      { ...receipt(), item: { ...receipt().item, lifecycle_state: 'PENDING' } },
      { ...receipt(), item: { ...receipt().item, edited_payload: 'stale draft' } }];
    for (const value of invalid) {
      const x = await load(file, Response.json(value)); await x.decide(); retained(x);
      assert.equal(x.alerts.length, 1);
    }
    const x = await load(file, new Response('{', { status: 200 })); await x.decide(); retained(x);
  });
  test(`${file}: a dismissal needs its matching receipt and does not send an edit`, async () => {
    const x = await load(file, Response.json(receipt(false, null)));
    await x.decide(false, 'ignored edit');
    assert.deepEqual(Array.from(x.context.items, item => item.id), ['remaining']);
    assert.deepEqual(x.requests[0].body, { triage_item_id: 'owned', approved: false });
    assert.deepEqual(x.alerts, []);
    const y = await load(file, Response.json(receipt(true, null)));
    await y.decide(false); retained(y);
  });
  test(`${file}: an approval without an edit accepts its original durable replay`, async () => {
    const x = await load(file, Response.json(receipt(true, 'previously recorded edit')));
    await x.context.handleDecision('owned', true);
    assert.deepEqual(Array.from(x.context.items, item => item.id), ['remaining']);
    assert.deepEqual(x.requests[0].body, { triage_item_id: 'owned', approved: true });
    assert.deepEqual(x.alerts, []);
    for (const malformedEdit of [undefined, false, {}]) {
      const invalid = receipt(true, malformedEdit);
      invalid.item.edited_payload = malformedEdit;
      const y = await load(file, Response.json(invalid));
      await y.context.handleDecision('owned', true); retained(y);
    }
  });
  test(`${file}: rejected transport and HTTP failures retain the card and draft`, async () => {
    for (const response of [() => { throw new Error('offline'); }, new Response('denied', { status: 403 })]) {
      const x = await load(file, response); await x.decide(); retained(x); assert.equal(x.alerts.length, 1);
    }
  });
  test(`${file}: a receipt from the retired tenant cannot remove the current card`, async () => {
    let finish;
    const x = await load(file, () => new Promise(resolve => { finish = resolve; }));
    const pending = x.decide(); x.context.currentTenant = 'tenant-b';
    finish(Response.json(receipt())); await pending; retained(x);
  });
}
