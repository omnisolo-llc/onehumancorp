import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile, readdir, access } from 'node:fs/promises';
import { JSDOM } from './test-support/offline-dom.mjs';

const roots = ['src/ui/next/public', 'src/ui/next/public/ui', 'src/ui/tauri/src/ui'];
const publicRoot = new URL('../src/ui/next/public/', import.meta.url);
const retained = ['dashboard.html', 'ui/dashboard.html'];
const retired = ['api/ui/dashboard.html', 'api/v1/ui/dashboard.html'];
const item = (id = 'feed-1') => ({ id, tenant_id: 'tenant-a', lifecycle_state: 'PENDING_APPROVAL', proposed_action: { message: 'Original draft' } });
const receipt = (id = 'feed-1', state = 'APPROVED', edit) => ({
  ...item(id), lifecycle_state: state, decision_recorded: true,
  proposed_action: { message: edit ?? 'Original draft' },
  dispatch: { status: 'NOT_REQUESTED', receipt_id: null },
});

async function load(root, response, options = {}) {
  const html = await readFile(new URL(`../${root}/dashboard.html`, import.meta.url), 'utf8');
  const existing = html.indexOf('window.handleGroupAction = async function');
  const helper = html.indexOf('// Durable dashboard decisions');
  const start = helper >= 0 ? helper : existing;
  const end = html.indexOf('// Load on start', existing);
  assert.ok(start >= 0 && end > start);
  const dom = new JSDOM('<div id="triage-group-group"><div id="triage-feed-1"><button>Approve</button><textarea id="triage-textarea-feed-1">Edited draft</textarea></div><div id="triage-feed-2"><button>Approve</button></div></div><div id="modal"></div><div id="title"></div><div id="desc"></div><div id="upsell"></div>', { url: 'https://app.example.test/dashboard.html', runScripts: 'outside-only' });
  const w = dom.window; const requests = [], reads = [], messages = [], alerts = [];
  w.agentFeedItems = [item(), item('feed-2')];
  w.localStorage.setItem('tenant_id', 'tenant-a');
  for (const [key, value] of options.entries ?? []) w.localStorage.setItem(key, value);
  const locks = options.locks ?? new Set();
  Object.defineProperty(w.navigator, 'locks', { value: { request: async (key, _options, callback) => {
    if (locks.has(key)) return callback(null);
    locks.add(key); try { return await callback({}); } finally { locks.delete(key); }
  } } });
  w.Headers = Headers;
  w.fetch = async (url, init) => {
    if (url === '/api/v1/auth/session-identity') return Response.json({ userId: options.actor ?? 'owner-a', tenantId: 'tenant-a', expiresAt: Date.now() + 60000 });
    if (!init?.method || init.method === 'GET') { reads.push(url); return options.reconcileResponse ? options.reconcileResponse(url) : Response.json({ decision_recorded: false }); }
    requests.push({ url, body: JSON.parse(init.body), headers: init.headers }); return typeof response === 'function' ? response(url, init) : response;
  };
  w.loadUnifiedFeed = () => { w.reloads++; }; w.reloads = 0;
  w.showStatus = message => messages.push(message); w.alert = message => alerts.push(message);
  w.console.error = () => {}; w.setTimeout = () => 0;
  for (const [key, id] of Object.entries({ agentSuccessModal: 'modal', agentSuccessTitle: 'title', agentSuccessDesc: 'desc', agentSuccessUpsellContainer: 'upsell' })) w[key] = w.document.getElementById(id);
  w.eval(html.slice(start, end));
  const renderActual = () => {
    if (!w.triageQueue) {
      w.document.body.insertAdjacentHTML('afterbegin', '<div id="actual-queue"></div>');
      w.triageQueue = w.document.getElementById('actual-queue');
      w.document.getElementById('triage-group-group').remove();
      w.currentTriageTab = 'proposals'; w.renderAgentBadge = () => '';
      const rendererStart = html.indexOf('function renderTriageItems(items)');
      const rendererEnd = html.indexOf('window.showStatus = function', rendererStart);
      w.eval(html.slice(rendererStart, rendererEnd));
    }
    w.renderTriageItems(w.agentFeedItems);
  };
  return { w, requests, reads, messages, alerts, renderActual, close: () => w.close() };
}

for (const root of roots) {
  test(`${root}: approval needs a durable matching receipt and never claims execution`, async () => {
    const x = await load(root, Response.json(receipt('feed-1', 'APPROVED', 'Edited draft')));
    try {
      await x.w.handleTriageAction('feed-1', true);
      assert.deepEqual(x.requests[0].body, { state: 'APPROVED', modified_content: 'Edited draft' });
      assert.match(x.messages.join(' '), /Approval recorded/);
      assert.doesNotMatch(x.w.agentSuccessTitle.innerText || '', /Executed|Completed/);
      assert.equal(x.w.agentSuccessModal.style.display, '');
    } finally { x.close(); }
  });
  test(`${root}: invalid receipts retain cards and edits`, async () => {
    for (const response of [new Response('failed', { status: 500 }), new Response('{'), Response.json({}),
      Response.json({ ...receipt(), decision_recorded: false }), Response.json(receipt('foreign')),
      Response.json({ ...receipt(), tenant_id: 'other-tenant' }), Response.json(receipt('feed-1', 'DISMISSED')),
      Response.json(receipt('feed-1', 'APPROVED', 'Wrong edit'))]) {
      const x = await load(root, response);
      try {
        await x.w.handleTriageAction('feed-1', true);
        assert.equal(x.messages.length, 0);
        assert.equal(x.w.reloads, 0);
        assert.equal(x.w.document.querySelector('textarea').value, 'Edited draft');
        assert.match(x.alerts.join(' '), /unconfirmed|retained/i);
      } finally { x.close(); }
    }
  });
  test(`${root}: failed batch responses cannot remove the group or claim success`, async () => {
    const x = await load(root, () => new Response('failure', { status: 500 }));
    try {
      await x.w.handleGroupAction('group', '["feed-1","feed-2"]', 'APPROVED');
      assert.ok(x.w.document.getElementById('triage-group-group'));
      assert.equal(x.messages.length, 0);
      assert.match(x.alerts.join(' '), /unconfirmed|retained/i);
    } finally { x.close(); }
  });
  test(`${root}: batch waits for every body and reports partial receipts honestly`, async () => {
    let release; const body = new Promise(resolve => { release = resolve; });
    const x = await load(root, url => url.includes('feed-1') ? { status: 200, ok: true, json: () => body } : Response.json({ error: 'not committed' }, { status: 503 }));
    try {
      const action = x.w.handleGroupAction('group', '["feed-1","feed-2"]', 'APPROVED');
      await new Promise(resolve => setImmediate(resolve));
      assert.ok(x.w.document.getElementById('triage-group-group'));
      assert.equal(x.messages.length, 0);
      release(receipt()); await action;
      assert.ok(x.w.document.getElementById('triage-group-group'));
      assert.match(x.alerts.join(' '), /1 of 2.*recorded.*unconfirmed/i);
      assert.doesNotMatch(x.alerts.join(' '), /executed|completed/i);
    } finally { release(receipt()); x.close(); }
  });
  test(`${root}: dismissal excludes edits and reports the correct decision`, async () => {
    const x = await load(root, () => Response.json(receipt('feed-1', 'DISMISSED')));
    try {
      await x.w.handleGroupAction('group', '["feed-1"]', 'DISMISSED');
      assert.deepEqual(x.requests[0].body, { state: 'DISMISSED' });
      assert.match(x.messages.join(' '), /Dismissal recorded/);
      assert.doesNotMatch(x.messages.join(' '), /approv/i);
    } finally { x.close(); }
  });
}

test('every shipped dashboard alias has an explicit reviewed disposition', async () => {
  const shipped = (await readdir(publicRoot, { recursive: true })).filter(path => /(^|\/)dashboard\.html$/.test(path)).sort();
  assert.deepEqual(shipped, retained);
  const map = await readFile(new URL('../src/ui/next/src/lib/auth/retiredPageRoutes.ts', import.meta.url), 'utf8');
  const aliases = Array.from(map.matchAll(/\["([^"]*\/dashboard\.html)", "\/dashboard"\]/g), match => match[1].slice(1)).sort();
  assert.deepEqual(aliases, retired);
  for (const path of retired) await assert.rejects(access(new URL(path, publicRoot)), { code: 'ENOENT' });
  for (const path of ['help-widget.mjs', 'ui/help-widget.mjs', 'api/v1/ui/help-widget.mjs', 'agent-card.html', 'viral-certificate-generator.html']) await access(new URL(path, publicRoot));
});

for (const root of roots) {
  test(`${root}: pending single decisions cannot be resubmitted or acknowledged for another identity`, async () => {
    let finish; const body = new Promise(resolve => { finish = resolve; });
    const x = await load(root, { status: 200, ok: true, json: () => body });
    try {
      const first = x.w.handleTriageAction('feed-1', true);
      await new Promise(resolve => setImmediate(resolve));
      await x.w.handleTriageAction('feed-1', true);
      await x.w.handleGroupAction('group', '["feed-1"]', 'APPROVED');
      await x.w.handleTriageAction('feed-1', true);
      assert.equal(x.requests.length, 1);
      x.w.localStorage.setItem('omnisolo_queue_identity_epoch_v2', 'different-login');
      finish(receipt('feed-1', 'APPROVED', 'Edited draft')); await first;
      assert.equal(x.messages.length, 0); assert.equal(x.w.reloads, 0); assert.equal(x.alerts.length, 0);
    } finally { finish(receipt()); x.close(); }
  });
  test(`${root}: legacy decisions require the tenant-bound nested receipt`, async () => {
    const payload = { success: true, decision_recorded: true, item: { id: 'triage-owned', tenant_id: 'tenant-a', lifecycle_state: 'DISMISSED', edited_payload: null } };
    const x = await load(root, Response.json(payload));
    try {
      x.w.agentFeedItems.push(item('triage-owned'));
      await x.w.handleTriageAction('triage-owned', false);
      assert.deepEqual(x.requests[0].body, { triage_item_id: 'triage-owned', approved: false });
      assert.equal(x.requests[0].url, '/api/v1/ui/triage/action');
      assert.match(x.messages.join(' '), /Dismissal recorded/);
    } finally { x.close(); }
  });
  test(`${root}: feed failure never fabricates business or storefront readiness`, async () => {
    const html = await readFile(new URL(`../${root}/dashboard.html`, import.meta.url), 'utf8');
    const dom = new JSDOM('<div id="triage-section"></div><div id="queue"></div>', { url: 'https://app.example.test/dashboard.html', runScripts: 'outside-only' });
    const w = dom.window; w.triageSection = w.document.getElementById('triage-section'); w.triageQueue = w.document.getElementById('queue');
    w.fetch = async () => new Response('unavailable', { status: 503 }); w.console.error = () => {};
    const start = html.indexOf('async function loadUnifiedFeed()'); const end = html.indexOf('let activeAgentFeedWs', start);
    try {
      w.eval(html.slice(start, end)); await w.loadUnifiedFeed();
      assert.match(w.triageQueue.textContent, /unavailable|retry/i);
      assert.doesNotMatch(w.triageQueue.textContent, /business is ready|generated storefront/i);
      assert.equal(w.triageQueue.querySelector('a')?.getAttribute('href'), '/dashboard');
      assert.doesNotMatch(html, /invite\/test-|invite\/fallback/);
    } finally { w.close(); }
  });
}

test('retired dashboard cases and canonical links cannot drift out of browser coverage', async () => {
  const spec = await readFile(new URL('../src/ui/next/src/e2e/legacy-dashboard-retirement.spec.ts', import.meta.url), 'utf8');
  for (const alias of retired) assert.ok(spec.includes(`'/${alias}'`), `Missing fixed real-stack case for ${alias}`);
  for (const path of await readdir(publicRoot, { recursive: true })) {
    if (!/\.(html|mjs|js)$/.test(path)) continue;
    const source = await readFile(new URL(path, publicRoot), 'utf8');
    assert.doesNotMatch(source, /['"`]\/api\/(?:v1\/)?ui\/dashboard\.html(?:[?#][^'"`]*)?['"`]/, path);
    if (/^api\/(?:v1\/)?ui\//.test(path)) assert.doesNotMatch(source, /['"`]dashboard\.html(?:[?#][^'"`]*)?['"`]/, path);
  }
});

test('retained dashboard implementations share receipt logic and never construct invite URLs', async () => {
  const handlers = [];
  const bridges = [];
  for (const root of roots) {
    const html = await readFile(new URL(`../${root}/dashboard.html`, import.meta.url), 'utf8');
    const start = html.indexOf('// Durable dashboard decisions');
    const end = html.indexOf('// Load on start', start);
    assert.ok(start >= 0 && end > start, root);
    handlers.push(html.slice(start, end));
    assert.doesNotMatch(html, /https?:\/\/[^\s'"`<>]*\/(?:invite\/|join\?ref)/, root);
    assert.ok(html.includes('getVerifiedDashboardInvitation'), root);
    bridges.push(await readFile(new URL(`../${root}/invite-bridge.mjs`, import.meta.url), 'utf8'));
  }
  assert.equal(new Set(handlers).size, 1, 'All retained copies must enforce the same durable receipt contract');
  assert.equal(new Set(bridges).size, 1, 'All retained copies must share owner-bound invitation validation');
});

for (const root of roots) {
  test(`${root}: a lost acknowledgement survives reload and reconciles with GET only`, async () => {
    const x = await load(root, () => { throw new Error('lost after commit'); });
    let entries;
    try {
      await x.w.handleTriageAction('feed-1', true);
      assert.equal(x.requests.length, 1);
      assert.equal(x.w.document.querySelector('textarea').disabled, true);
      entries = Object.entries(x.w.localStorage);
      assert.ok(entries.some(([key]) => key.startsWith('omnisolo_dashboard_decision_v1:')));
      await x.w.handleTriageAction('feed-1', false);
      assert.equal(x.requests.length, 1, 'different intent cannot release an unknown request');
    } finally { x.close(); }
    const restored = await load(root, () => { throw new Error('No replay is allowed'); }, { entries, reconcileResponse: () => Response.json(receipt('feed-1', 'APPROVED', 'Edited draft')) });
    try {
      await restored.w.reconcileDashboardDecisions();
      assert.deepEqual(restored.reads, ['/api/v1/agent-feed/feed-1/decision']);
      assert.equal(restored.requests.length, 0);
      assert.match(restored.messages.join(' '), /recorded/);
    } finally { restored.close(); }
  });
  test(`${root}: no-record and changed actors cannot replay or clear the original request`, async () => {
    const x = await load(root, () => new Response('unavailable', { status: 503 }));
    let entries;
    try { await x.w.handleTriageAction('feed-1', true); entries = Object.entries(x.w.localStorage); } finally { x.close(); }
    for (const actor of ['owner-a', 'owner-b']) {
      const y = await load(root, () => Response.json(receipt()), { entries, actor });
      try {
        await y.w.reconcileDashboardDecisions();
        y.w.document.querySelector('textarea').value = 'Different intent';
        await y.w.handleTriageAction('feed-1', true);
        assert.equal(y.requests.length, 0);
        assert.ok(Object.entries(y.w.localStorage).some(([key, value]) => key.startsWith('omnisolo_dashboard_decision_v1:') && value.includes('Edited draft')));
      } finally { y.close(); }
    }
  });
  test(`${root}: known rejection permits a corrected intent but partial batches never replay confirmed items`, async () => {
    let reject = true;
    const x = await load(root, () => reject ? Response.json({ decision_recorded: false, error: 'invalid draft' }, { status: 422 }) : Response.json(receipt('feed-1', 'APPROVED', 'Corrected draft')));
    try {
      await x.w.handleTriageAction('feed-1', true);
      reject = false; x.w.document.querySelector('textarea').value = 'Corrected draft';
      await x.w.handleTriageAction('feed-1', true);
      assert.equal(x.requests.length, 2);
    } finally { x.close(); }
    const y = await load(root, url => url.includes('feed-1') ? Response.json(receipt()) : new Response('unknown', { status: 503 }), { reconcileResponse: url => url.includes('feed-1') ? Response.json(receipt()) : Response.json({ decision_recorded: false }) });
    try {
      await y.w.handleGroupAction('group', '["feed-1","feed-2"]', 'APPROVED');
      await y.w.handleGroupAction('group', '["feed-1","feed-2"]', 'APPROVED');
      assert.equal(y.requests.filter(request => request.url.includes('feed-1')).length, 1);
      assert.equal(y.requests.filter(request => request.url.includes('feed-2')).length, 1);
    } finally { y.close(); }
  });
}

for (const root of roots) {
  test(`${root}: a held legacy decision reads its own endpoint and preserves its edit`, async () => {
    const x = await load(root, () => { throw new Error('acknowledgement lost'); }); let entries;
    try {
      x.w.agentFeedItems.push(item('triage-owned'));
      await x.w.handleTriageAction('triage-owned', true);
      entries = Object.entries(x.w.localStorage);
    } finally { x.close(); }
    const y = await load(root, () => { throw new Error('Unexpected replay'); }, { entries, reconcileResponse: () => Response.json({ success: true, decision_recorded: true, item: { id: 'triage-owned', tenant_id: 'tenant-a', lifecycle_state: 'APPROVED', edited_payload: null } }) });
    try {
      await y.w.reconcileDashboardDecisions();
      assert.deepEqual(y.reads, ['/api/v1/ui/triage/decisions/triage-owned']);
      assert.equal(y.requests.length, 0); assert.match(y.messages.join(' '), /recorded/);
    } finally { y.close(); }
  });
  test(`${root}: storage or lock failure cannot dispatch an unrecoverable decision`, async () => {
    const x = await load(root, () => Response.json(receipt()));
    try {
      x.w.Storage.prototype.setItem = () => { throw new Error('storage denied'); };
      await x.w.handleTriageAction('feed-1', true);
      assert.equal(x.requests.length, 0); assert.equal(x.messages.length, 0);
    } finally { x.close(); }
    const y = await load(root, () => Response.json(receipt()));
    try {
      y.w.navigator.locks.request = async (_key, _options, callback) => callback(null);
      await y.w.handleTriageAction('feed-1', true);
      assert.equal(y.requests.length, 0); assert.equal(y.messages.length, 0);
    } finally { y.close(); }
  });
}

for (const root of roots) {
  test(`${root}: an actor switch during receipt GET cannot release the old durable hold`, async () => {
    const first = await load(root, () => { throw new Error('lost acknowledgement'); }); let entries;
    try { await first.w.handleTriageAction('feed-1', true); entries = Object.entries(first.w.localStorage); } finally { first.close(); }
    let finish; const deferred = new Promise(resolve => { finish = resolve; });
    const x = await load(root, () => { throw new Error('No replay'); }, { entries, reconcileResponse: () => deferred });
    try {
      const reading = x.w.reconcileDashboardDecisions();
      await new Promise(resolve => setImmediate(resolve));
      x.w.dispatchEvent(new x.w.Event('omnisolo_auth_changed'));
      finish(Response.json(receipt('feed-1', 'APPROVED', 'Edited draft'))); await reading;
      const hold = JSON.parse(Object.entries(x.w.localStorage).find(([key]) => key.startsWith('omnisolo_dashboard_decision_v1:'))[1]);
      assert.equal(hold.phase, 'unknown');
      assert.equal(x.w.document.querySelector('textarea').disabled, true);
      assert.equal(x.messages.length, 0); assert.equal(x.requests.length, 0);
    } finally { finish(Response.json(receipt())); x.close(); }
  });
  test(`${root}: an actor switch during write acknowledgement cannot release or reject the old hold`, async () => {
    for (const reply of [() => Response.json(receipt('feed-1', 'APPROVED', 'Edited draft')), () => Response.json({ decision_recorded: false, error: 'denied' }, { status: 403 })]) {
      let finish; const deferred = new Promise(resolve => { finish = resolve; });
      const x = await load(root, () => deferred);
      try {
        const writing = x.w.handleTriageAction('feed-1', true);
        await new Promise(resolve => setImmediate(resolve));
        assert.equal(x.requests.length, 1);
        x.w.dispatchEvent(new x.w.Event('omnisolo_auth_changed'));
        finish(reply()); await writing;
        const hold = JSON.parse(Object.entries(x.w.localStorage).find(([key]) => key.startsWith('omnisolo_dashboard_decision_v1:'))[1]);
        assert.equal(hold.phase, 'unknown');
        assert.equal(x.w.document.querySelector('textarea').disabled, true);
        assert.equal(x.messages.length, 0);
      } finally { finish(reply()); x.close(); }
    }
  });
}


for (const root of roots) {
  test(`${root}: actual tab and websocket-style renders retain unknown draft holds`, async () => {
    const x = await load(root, () => new Response('unknown', { status: 503 }));
    try {
      x.w.agentFeedItems[0].event_source = 'Simulated Webhook';
      x.renderActual();
      x.w.document.querySelector('textarea').value = 'Exact held draft';
      await x.w.handleTriageAction('feed-1', true);
      for (const tab of ['activity', 'proposals', 'proposals']) {
        x.w.currentTriageTab = tab; x.renderActual();
        if (tab !== 'proposals') continue;
        assert.equal(x.w.document.querySelector('textarea').disabled, true);
        assert.equal(x.w.document.querySelector('textarea').value, 'Exact held draft');
        assert.ok([...x.w.document.getElementById('triage-feed-1').querySelectorAll('button')].every(button => button.disabled));
      }
      assert.equal(x.requests.length, 1);
      const hold = JSON.parse(Object.entries(x.w.localStorage).find(([key]) => key.startsWith('omnisolo_dashboard_decision_v1:'))[1]);
      assert.equal(hold.phase, 'unknown'); assert.equal(hold.edit, 'Exact held draft');
    } finally { x.close(); }
  });
  test(`${root}: actual group controls stay held through partial-batch rerenders`, async () => {
    const x = await load(root, url => url.includes('feed-1') ? Response.json(receipt()) : new Response('unknown', { status: 503 }));
    try {
      for (const item of x.w.agentFeedItems) item.proposed_action.group_key = 'group';
      x.renderActual();
      await x.w.handleGroupAction('group', '["feed-1","feed-2"]', 'APPROVED');
      for (let render = 0; render < 2; render++) {
        x.renderActual();
        assert.ok([...x.w.document.getElementById('triage-group-group').querySelectorAll('button')].every(button => button.disabled));
      }
      assert.equal(x.requests.length, 2);
    } finally { x.close(); }
  });
}

for (const root of roots) {
  test(`${root}: restored drafts remain hidden and held until the original owner is verified`, async () => {
    const first = await load(root, () => new Response('unknown', { status: 503 })); let entries;
    try { await first.w.handleTriageAction('feed-1', true); entries = Object.entries(first.w.localStorage); }
    finally { first.close(); }
    for (const actor of ['owner-a', 'owner-b']) {
      const x = await load(root, () => { throw new Error('No replay'); }, { entries, actor });
      try {
        x.w.agentFeedItems[0].event_source = 'Simulated Webhook'; x.renderActual();
        assert.equal(x.w.document.querySelector('textarea').value, '');
        assert.equal(x.w.document.querySelector('textarea').disabled, true);
        await x.w.reconcileDashboardDecisions();
        assert.equal(x.w.document.querySelector('textarea').value, actor === 'owner-a' ? 'Edited draft' : '');
        assert.equal(x.w.document.querySelector('textarea').disabled, true);
        assert.equal(x.requests.length, 0);
      } finally { x.close(); }
    }
  });
  test(`${root}: an actual held group releases only after its final receipt reconciles`, async () => {
    const x = await load(root, url => url.includes('feed-1') ? Response.json(receipt()) : new Response('unknown', { status: 503 }), { reconcileResponse: () => Response.json(receipt('feed-2')) });
    try {
      for (const item of x.w.agentFeedItems) item.proposed_action.group_key = 'group';
      x.renderActual(); await x.w.handleGroupAction('group', '["feed-1","feed-2"]', 'APPROVED');
      assert.ok([...x.w.document.getElementById('triage-group-group').querySelectorAll('button')].every(button => button.disabled));
      await x.w.reconcileDashboardDecisions();
      assert.ok([...x.w.document.getElementById('triage-group-group').querySelectorAll('button')].every(button => !button.disabled));
      assert.equal(x.requests.length, 2);
    } finally { x.close(); }
  });
}
