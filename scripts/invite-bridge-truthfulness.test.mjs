import assert from 'node:assert/strict';
import { test } from 'node:test';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';
const pages = ['src/ui/next/public/dashboard.html', 'src/ui/next/public/ui/dashboard.html', 'src/ui/tauri/src/ui/dashboard.html'];
const turn = () => new Promise(resolve => setImmediate(resolve));
const deferred = () => { let resolve; let reject; const promise = new Promise((yes, no) => { resolve = yes; reject = no; }); return { promise, resolve, reject }; };
const invites = context => context.requests.filter(request => request.init?.method === 'POST' && /(?:cloud-bridge\/invite|team-invites)$/.test(request.url));
const markerEntries = context => Object.entries(context.window.localStorage).filter(([key]) => key.startsWith('omnisolo_invite_creation_v1:'));
const generate = async context => { context.document.getElementById('generate-link-btn').click(); await turn(); await turn(); };
async function dashboard(path, options = {}) {
  const requests = [], copied = [], opened = [], scriptErrors = [], expiryCallbacks = [];
  const console = new VirtualConsole(); console.on('jsdomError', error => scriptErrors.push(error.message));
  const window = new JSDOM(await readFile(path, 'utf8'), {
    url: 'https://example.test/dashboard.html', runScripts: 'dangerously', virtualConsole: console,
    beforeParse(window) {
      window.Headers = Headers; window.Response = Response;
      const locks = options.sharedLocks ?? new Set();
      if (!options.noLocks) Object.defineProperty(window.navigator, 'locks', { value: { request: async (name, _options, callback) => {
        if (locks.has(name)) return callback(null);
        locks.add(name); try { return await callback({ name }); } finally { locks.delete(name); }
      } } });
      for (const [key, value] of options.entries ?? []) window.localStorage.setItem(key, value);
      if (options.storageFailure) window.Storage.prototype.setItem = () => { throw new Error('Storage unavailable'); };
      if (!options.storageFailure) window.localStorage.setItem('tenant', 'untrusted-browser-tenant');
      if (!options.storageFailure) window.localStorage.setItem('tenant_id', 'untrusted-browser-tenant');
      window.alert = () => {};
      window.matchMedia = () => ({ matches: false, addEventListener() {}, removeEventListener() {} });
      window.fetch = async (url, init) => {
        requests.push({ url: String(url), init });
        if (String(url).endsWith('/auth/session-identity')) return options.identityReply ? options.identityReply() : Response.json({ userId: 'owner-a', tenantId: 'tenant-a', expiresAt: Date.now() + 60_000 });
        if (String(url).includes('/growth/milestones/check') && options.milestoneReply) return options.milestoneReply();
        if (init?.method === 'POST' && /(?:cloud-bridge\/invite|team-invites)$/.test(String(url))) return options.inviteReply ? options.inviteReply(init) : options.rejectInvite ? Response.json({ error: 'unavailable' }, { status: 503 }) : Response.json({ invite_link: 'https://omnisolo.co/invite/inv-recorded' });
        return Response.json({});
      };
      Object.defineProperty(window.navigator, 'clipboard', { value: { writeText: async value => { copied.push(value); if (options.copyReply) return options.copyReply(value); } } });
      window.open = (...args) => { opened.push(args); return null; };
    },
  }).window;
  if (window.document.readyState === 'loading') await new Promise(resolve => window.document.addEventListener('DOMContentLoaded', resolve, { once: true }));
  const moduleScript = window.document.querySelector('script[type="module"][src="invite-bridge.mjs"]');
  if (moduleScript) {
    const bridge = await import(pathToFileURL(resolve(dirname(path), 'invite-bridge.mjs')).href);
    if (options.captureExpiry) window.setTimeout = (callback, delay) => { expiryCallbacks.push({ callback, delay }); return 0; };
    bridge.installInviteBridge(window);
  }
  await turn(); await turn();
  return { window, document: window.document, requests, copied, opened, scriptErrors, expiryCallbacks };
}
for (const path of pages) {
  test(`${path}: one Generate click dispatches only one invitation request`, async () => {
    const context = await dashboard(path);
    try {
      context.document.getElementById('generate-link-btn').click(); await turn(); await turn();
      const invitations = context.requests.filter(request => request.init?.method === 'POST' && /(?:cloud-bridge\/invite|team-invites)$/.test(request.url));
      assert.equal(invitations.length, 1, JSON.stringify({ invitations, scriptErrors: context.scriptErrors }));
    } finally { context.window.close(); }
  });
  test(`${path}: copy and WhatsApp use one consistent recorded invitation`, async () => {
    const context = await dashboard(path);
    try {
      context.document.getElementById('generate-link-btn').click(); await turn(); await turn();
      context.document.getElementById('copy-btn').click(); await turn();
      context.document.getElementById('share-whatsapp-btn').click();
      assert.equal(context.copied.length, 1);
      assert.equal(context.opened.length, 1);
      assert.equal(new URL(context.opened[0][0]).searchParams.get('text'), context.copied[0]);
      assert.ok(context.copied[0].includes('https://omnisolo.co/invite/inv-recorded'));
    } finally { context.window.close(); }
  });
  test(`${path}: an unconfirmed request never invents a link or enables sharing`, async () => {
    const context = await dashboard(path, { rejectInvite: true });
    try {
      context.document.getElementById('generate-link-btn').click(); await turn(); await turn();
      assert.equal(context.document.getElementById('referral-link').value, '');
      assert.equal(context.document.getElementById('copy-btn').disabled, true);
      assert.equal(context.document.getElementById('share-whatsapp-btn').disabled, true);
      assert.equal(context.copied.length, 0);
    } finally { context.window.close(); }
  });
}

for (const path of pages) {
  test(`${path}: served module and controls have one binding, with no inline duplicate listeners`, async () => {
    const html = await readFile(path, 'utf8');
    assert.equal((html.match(/<script type="module" src="invite-bridge.mjs"><\/script>/g) ?? []).length, 1);
    assert.equal((html.match(/getElementById\(['"](?:generate-link-btn|copy-btn|share-whatsapp-btn)['"]\)/g) ?? []).length, 0);
    assert.equal(await readFile(resolve(dirname(path), 'invite-bridge.mjs'), 'utf8'), await readFile(resolve(dirname(pages[0]), 'invite-bridge.mjs'), 'utf8'));
  });
}
const primary = pages[0];
test('the request is bound to verified owner headers and ignores browser tenant authority', async () => {
  const c = await dashboard(primary);
  try {
    await generate(c);
    assert.equal(invites(c).length, 1);
    assert.equal(invites(c)[0].init.headers.get('x-ohc-expected-user'), 'owner-a');
    assert.equal(invites(c)[0].init.headers.get('x-ohc-expected-tenant'), 'tenant-a');
    assert.deepEqual(JSON.parse(invites(c)[0].init.body), { invitee_id: 'pending' });
    assert.equal(c.scriptErrors.length, 0, c.scriptErrors.join('\n'));
    const [[, bytes]] = markerEntries(c);
    assert.equal(JSON.parse(bytes).state, 'created');
    assert.equal(bytes.includes('inv-recorded'), false);
    assert.equal(bytes.includes('https:'), false);
  } finally { c.window.close(); }
});
for (const [name, options] of [
  ['missing origin locks', { noLocks: true }],
  ['failed pending-marker persistence', { storageFailure: true }],
  ['unsafe header identity', { identityReply: () => Response.json({ userId: 'owner-雪', tenantId: 'tenant-a', expiresAt: Date.now() + 60_000 }) }],
]) test(`${name} cannot dispatch an invitation`, async () => {
  const c = await dashboard(primary, options);
  try { await generate(c); assert.equal(invites(c).length, 0); assert.equal(c.document.getElementById('referral-link').value, ''); }
  finally { c.window.close(); }
});
test('identity changes before dispatch retire the old owner without creating an invitation', async () => {
  let identityCalls = 0;
  const c = await dashboard(primary, { identityReply: () => Response.json({ userId: ++identityCalls === 1 ? 'owner-a' : 'owner-b', tenantId: 'tenant-a', expiresAt: Date.now() + 60_000 }) });
  try { await generate(c); assert.equal(invites(c).length, 0); assert.equal(c.document.getElementById('generate-link-btn').disabled, true); }
  finally { c.window.close(); }
});
for (const event of ['auth', 'storage', 'null-storage', 'pagehide']) test(`${event} retires a late confirmed body without displaying the previous owner link`, async () => {
  const pending = deferred(); const c = await dashboard(primary, { inviteReply: () => pending.promise });
  try {
    await generate(c); assert.equal(invites(c).length, 1);
    c.window.dispatchEvent(event === 'auth' ? new c.window.Event('omnisolo_auth_changed') : event === 'pagehide' ? new c.window.Event('pagehide') : new c.window.StorageEvent('storage', { key: event === 'storage' ? 'omnisolo_queue_identity_epoch_v2' : null }));
    pending.resolve(Response.json({ invite_link: 'https://omnisolo.co/invite/inv-recorded' })); await turn(); await turn();
    assert.equal(c.document.getElementById('referral-link').value, '');
    assert.equal(c.document.getElementById('copy-btn').disabled, true);
    assert.equal(c.document.getElementById('generate-link-btn').disabled, true);
    assert.equal(JSON.parse(markerEntries(c)[0][1]).state, 'created');
  } finally { c.window.close(); }
});
for (const [name, body, status] of [
  ['server rejection', { error: 'unavailable' }, 503],
  ['contradictory receipt', { invite_link: 'https://omnisolo.co/invite/inv-recorded', error: 'denied' }, 200],
  ['wrong success status', { invite_link: 'https://omnisolo.co/invite/inv-recorded' }, 202],
  ['foreign destination', { invite_link: 'https://untrusted.test/invite/inv-recorded' }, 200],
  ['fabricated fallback', { invite_link: 'https://omnisolo.co/invite/fallback' }, 200],
  ['absent receipt', { success: true }, 200],
]) test(`${name} stays held across repeated clicks and reload`, async () => {
  const c = await dashboard(primary, { inviteReply: () => Response.json(body, { status }) }); let reopened;
  try {
    await generate(c); await generate(c); assert.equal(invites(c).length, 1);
    const entries = markerEntries(c); assert.equal(JSON.parse(entries[0][1]).state, 'pending');
    reopened = await dashboard(primary, { entries }); await generate(reopened);
    assert.equal(invites(reopened).length, 0);
    assert.equal(reopened.document.getElementById('referral-link').value, '');
    assert.equal(reopened.document.getElementById('copy-btn').disabled, true);
    assert.match(reopened.document.getElementById('invite-link-status').textContent, /previous invitation request is unconfirmed/i);
  } finally { c.window.close(); reopened?.window.close(); }
});
test('a created marker cannot become a new receipt or a repeated request on reload', async () => {
  const c = await dashboard(primary); let reopened;
  try {
    await generate(c);
    reopened = await dashboard(primary, { entries: markerEntries(c) }); await generate(reopened);
    assert.equal(invites(reopened).length, 0);
    assert.equal(reopened.document.getElementById('referral-link').value, '');
    assert.match(reopened.document.getElementById('invite-link-status').textContent, /already created/i);
  } finally { c.window.close(); reopened?.window.close(); }
});
test('origin lock excludes a simultaneous independent view before provider-facing dispatch', async () => {
  const pending = deferred(); const sharedLocks = new Set();
  const a = await dashboard(primary, { sharedLocks, inviteReply: () => pending.promise });
  const b = await dashboard(primary, { sharedLocks });
  try {
    await generate(a); await generate(b);
    assert.equal(invites(a).length, 1); assert.equal(invites(b).length, 0);
    pending.resolve(Response.json({ invite_link: 'https://omnisolo.co/invite/inv-recorded' })); await turn(); await turn();
  } finally { a.window.close(); b.window.close(); }
});
test('clipboard rejection never claims copied and late completion after auth is ignored', async () => {
  const pending = deferred(); const c = await dashboard(primary, { copyReply: () => pending.promise });
  try {
    await generate(c); c.document.getElementById('copy-btn').click(); await turn();
    assert.equal(c.document.getElementById('copy-btn').textContent, 'Copying…');
    c.window.dispatchEvent(new c.window.Event('omnisolo_auth_changed'));
    pending.resolve(); await turn();
    assert.equal(c.document.getElementById('copy-btn').textContent, 'Copy');
    assert.equal(c.document.getElementById('copy-btn').disabled, true);
  } finally { c.window.close(); }
  const denied = await dashboard(primary, { copyReply: () => Promise.reject(new Error('denied')) });
  try {
    await generate(denied); denied.document.getElementById('copy-btn').click(); await turn();
    assert.equal(denied.document.getElementById('copy-btn').textContent, 'Copy');
    assert.match(denied.document.getElementById('invite-link-status').textContent, /Copy failed/);
  } finally { denied.window.close(); }
});
test('foreign owner history remains private and unchanged while a new owner creates its own request', async () => {
  const a = await dashboard(primary); let b;
  try {
    await generate(a); const entries = markerEntries(a);
    b = await dashboard(primary, { entries, identityReply: () => Response.json({ userId: 'owner-b', tenantId: 'tenant-b', expiresAt: Date.now() + 60_000 }) });
    assert.equal(b.document.getElementById('referral-link').value, '');
    assert.equal(b.document.getElementById('generate-link-btn').disabled, false);
    await generate(b); assert.equal(invites(b).length, 1);
    assert.equal(invites(b)[0].init.headers.get('x-ohc-expected-user'), 'owner-b');
    assert.equal(b.window.localStorage.getItem(entries[0][0]), entries[0][1]);
    assert.equal(markerEntries(b).length, 2);
  } finally { a.window.close(); b?.window.close(); }
});
for (const [name, reply] of [
  ['lost network outcome', () => Promise.reject(new Error('connection lost after dispatch'))],
  ['invalid response body', () => new Response('{', { status: 200 })],
]) test(`${name} remains pending and blocks a new request`, async () => {
  const c = await dashboard(primary, { inviteReply: reply });
  try {
    await generate(c); await generate(c);
    assert.equal(invites(c).length, 1);
    assert.equal(JSON.parse(markerEntries(c)[0][1]).state, 'pending');
    assert.equal(c.document.getElementById('referral-link').value, '');
    assert.match(c.document.getElementById('invite-link-status').textContent, /could not be confirmed/);
  } finally { c.window.close(); }
});
test('corrupt own history is held without deleting it or dispatching', async () => {
  const key = 'omnisolo_invite_creation_v1:' + encodeURIComponent(JSON.stringify(['owner-a', 'tenant-a']));
  const c = await dashboard(primary, { entries: [[key, '{corrupt']] });
  try {
    await generate(c); assert.equal(invites(c).length, 0);
    assert.equal(c.window.localStorage.getItem(key), '{corrupt');
    assert.equal(c.document.getElementById('generate-link-btn').disabled, true);
  } finally { c.window.close(); }
});

test('legacy milestone consumers can read only the currently verified invitation', async () => {
  const c = await dashboard(primary);
  try {
    assert.equal(c.window.getVerifiedDashboardInvitation(), null);
    await generate(c);
    assert.equal(c.window.getVerifiedDashboardInvitation(), 'https://omnisolo.co/invite/inv-recorded');
    c.window.dispatchEvent(new c.window.Event('omnisolo_auth_changed'));
    assert.equal(c.window.getVerifiedDashboardInvitation(), null);
  } finally { c.window.close(); }
});


for (const path of pages) {
  for (const event of ['auth', 'storage', 'null-storage', 'pagehide', 'expiry']) test(`${path}: ${event} clears every actual invitation consumer`, async () => {
    const c = await dashboard(path, { captureExpiry: event === 'expiry' });
    try {
      await generate(c);
      c.document.getElementById('referral-tier-copy-btn').click(); await turn();
      assert.equal(c.document.getElementById('referral-link-input').value, 'https://omnisolo.co/invite/inv-recorded');
      if (event === 'expiry') {
        const timer = c.expiryCallbacks.find(timer => timer.delay > 50000 && timer.delay <= 60000);
        assert.ok(timer, 'The bridge must arm the verified owner expiry'); timer.callback();
      }
      else c.window.dispatchEvent(event === 'auth' ? new c.window.Event('omnisolo_auth_changed') : event === 'pagehide' ? new c.window.Event('pagehide') : new c.window.StorageEvent('storage', { key: event === 'storage' ? 'omnisolo_queue_identity_epoch_v2' : null }));
      assert.equal(c.window.getVerifiedDashboardInvitation(), null);
      assert.equal(c.document.getElementById('referral-link').value, '');
      assert.equal(c.document.getElementById('referral-link-input').value, '');
    } finally { c.window.close(); }
  });
  for (const outcome of ['resolve', 'reject']) test(`${path}: a late referral clipboard ${outcome} cannot report for a retired owner`, async () => {
    const pending = deferred(); const c = await dashboard(path, { copyReply: () => pending.promise });
    const messages = []; c.window.showStatus = message => messages.push(message);
    try {
      await generate(c); c.document.getElementById('referral-tier-copy-btn').click(); await turn();
      c.window.dispatchEvent(new c.window.Event('omnisolo_auth_changed'));
      if (outcome === 'resolve') pending.resolve(); else pending.reject(new Error('denied'));
      await turn();
      assert.deepEqual(messages, []);
      assert.equal(c.document.getElementById('referral-link-input').value, '');
    } finally { c.window.close(); }
  });
}

for (const path of pages) {
  for (const outcome of ['resolve', 'reject']) test(`${path}: a late milestone clipboard ${outcome} cannot release a retired owner control`, async () => {
    const pending = deferred(); const c = await dashboard(path, {
      copyReply: () => pending.promise,
      milestoneReply: () => Response.json({ milestones: [{ id: 'first_sale', title: 'First recorded order', description: 'Recorded milestone', reached: true }] }),
    });
    const messages = []; c.window.showStatus = message => messages.push(message);
    try {
      await generate(c); c.document.getElementById('milestone-copy-btn').click(); await turn();
      assert.equal(c.copied.length, 1);
      c.window.dispatchEvent(new c.window.Event('omnisolo_auth_changed'));
      if (outcome === 'resolve') pending.resolve(); else pending.reject(new Error('denied'));
      await turn();
      assert.deepEqual(messages, []);
      assert.equal(c.document.getElementById('milestone-copy-btn').disabled, true);
      assert.equal(c.window.localStorage.getItem('dismissed_milestone_first_sale'), null);
    } finally { c.window.close(); }
  });
}
