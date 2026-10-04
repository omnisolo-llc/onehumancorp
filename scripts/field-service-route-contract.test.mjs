import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';

const roots = ['src/ui/next/public', 'src/ui/next/public/ui', 'src/ui/next/public/api/ui', 'src/ui/next/public/api/v1/ui', 'src/ui/tauri/src/ui'];
const turn = () => new Promise(resolve => setImmediate(resolve));
async function until(check) { for (let i = 0; i < 150; i++) { if (check()) return; await turn(); } assert.ok(check(), 'condition did not settle'); }
const version = '2026-10-03T08:00:00.000001Z';
const nextVersion = '2026-10-03T08:00:00.000002Z';
const owner = { userId: 'owner-a', tenantId: 'tenant-a', expiresAt: Date.now() + 600_000 };

// A transaction fixture, not a browser certification. Requests succeed before
// commit; failed commits roll back. Production code must wait for oncomplete.
function indexedDBFixture() {
  const state = { rows: new Map(), failCommit: false, clearCalls: 0, failOpen: false, holdOpen: false, pendingOpens: [] };
  let tail = Promise.resolve();
  const db = {
    objectStoreNames: { contains: () => true }, close() {},
    transaction(_stores, mode) {
      let ready;
      const started = new Promise(resolve => { ready = resolve; });
      let finish;
      const finished = new Promise(resolve => { finish = resolve; });
      const previous = tail; tail = finished;
      let rows, pending = 0, ended = false;
      const tx = { error: null };
      function settle() {
        setImmediate(() => {
          if (pending || ended) return;
          ended = true;
          if (mode === 'readwrite' && state.failCommit) {
            tx.error = new Error('fixture transaction aborted'); tx.onabort?.();
          } else { if (mode === 'readwrite') state.rows = rows; tx.oncomplete?.(); }
          finish();
        });
      }
      function request(operation) {
        const req = {}; pending++;
        started.then(() => setImmediate(() => {
          try { req.result = operation(); req.onsuccess?.({ target: req }); }
          catch (error) { req.error = error; req.onerror?.(); tx.error = error; tx.onabort?.(); ended = true; finish(); }
          pending--; settle();
        }));
        return req;
      }
      tx.objectStore = () => ({
        getAll: () => request(() => [...rows.values()].map(value => structuredClone(value))),
        put: value => request(() => rows.set(value.id, structuredClone(value))),
        delete: id => request(() => rows.delete(id)),
        clear: () => request(() => { state.clearCalls++; rows.clear(); }),
      });
      tx.abort = () => { ended = true; tx.error = new Error('aborted'); tx.onabort?.(); finish(); };
      previous.then(() => { rows = new Map(state.rows); ready(); settle(); });
      return tx;
    },
  };
  return { state, api: { open() {
    const req = {};
    const complete = () => setImmediate(() => {
      if (state.failOpen) { req.error = new Error('fixture storage unavailable'); req.onerror?.(); }
      else { req.result = db; req.onsuccess?.({ target: req }); }
    });
    if (state.holdOpen) state.pendingOpens.push(complete); else complete();
    return req;
  } } };
}
async function setup(root, options = {}) {
  const idb = options.idb ?? indexedDBFixture();
  const requests = [];
  let online = true, identity = { ...owner }, post = options.post, routesReply;
  const jobs = [1, 2].map(id => ({ id: `job-${id}`, status: 'pending', updated_at: version, job_title: `Job ${id}`, address: 'Test address' }));
  const dom = new JSDOM(await readFile(`${root}/field-service-route.html`, 'utf8'), {
    url: 'https://app.example.test/field-service-route.html', runScripts: 'dangerously', virtualConsole: new VirtualConsole(),
    beforeParse(window) {
      window.Headers = Headers;
      window.indexedDB = idb.api;
      Object.defineProperty(window.navigator, 'onLine', { get: () => online });
      window.fetch = async (url, init = {}) => {
        requests.push({ url: String(url), init });
        if (url === '/api/v1/auth/session-identity') return identity instanceof Response ? identity.clone() : Response.json(identity);
        if (String(url).endsWith('/routes/today')) return routesReply ? routesReply() : Response.json({ routes: [{ id: 'route-1', jobs }] });
        if (init.method === 'POST') {
          if (post) return post(url, init);
          const body = JSON.parse(init.body);
          const job = jobs.find(item => String(url).includes(`/${item.id}/`));
          if (body.expected_updated_at !== job.updated_at) return new Response('Reload before editing', { status: 428 });
          job.status = body.status; job.updated_at = job.updated_at.replace(/\.(\d+)Z$/, (_match, fraction) => `.${String(Number(fraction) + 1).padStart(fraction.length, '0')}Z`);
          return Response.json({ success: true, error: null, id: job.id, status: job.status, updated_at: job.updated_at });
        }
        throw new Error(`Unexpected request: ${url}`);
      };
    },
  });
  await until(() => dom.window.document.querySelector('[data-testid="job-card-job-1"]'));
  await dom.window.syncOfflineQueue();
  return { dom, requests, idb, jobs, posts: () => requests.filter(r => r.init.method === 'POST'),
    offline(value = true) { online = !value; dom.window.dispatchEvent(new dom.window.Event(value ? 'offline' : 'online')); },
    identity(value) { identity = value; }, post(value) { post = value; }, routes(value) { routesReply = value; },
    click(id = 'job-1', status = 'en_route') { return dom.window.updateJobStatus(id, status); },
    receipt(id = 'job-1') { return dom.window.document.querySelector(`[data-testid="job-receipt-${id}"]`)?.textContent ?? ''; },
  };
}
for (const root of roots) {
  test(`${root}: actual sender binds observed CAS version, current identity and durable request key`, async () => {
    const f = await setup(root);
    try {
      await f.click(); await until(() => f.posts().length === 1); await until(() => f.idb.state.rows.size === 0);
      const { init } = f.posts()[0];
      assert.deepEqual(JSON.parse(init.body), { status: 'en_route', expected_updated_at: version });
      const headers = new Headers(init.headers);
      assert.equal(headers.get('x-ohc-expected-user'), owner.userId);
      assert.equal(headers.get('x-ohc-expected-tenant'), owner.tenantId);
      assert.ok(headers.get('idempotency-key'));
      assert.equal(f.idb.state.clearCalls, 0);
      assert.match(f.receipt(), /Confirmed/);
    } finally { f.dom.window.close(); }
  });
  test(`${root}: rejected CAS remains visibly blocked and is never reported confirmed or auto-retried`, async () => {
    const f = await setup(root, { post: () => new Response('Reload before editing', { status: 428 }) });
    try {
      await f.click(); await until(() => f.posts().length === 1); for (let i = 0; i < 30; i++) await turn();
      assert.match(f.receipt(), /not saved|blocked/i);
      assert.equal(f.posts().length, 1);
      assert.equal([...f.idb.state.rows.values()][0]?.state, 'blocked');
      assert.doesNotMatch(f.receipt(), /Confirmed/);
    } finally { f.dom.window.close(); }
  });
  test(`${root}: offline chained actions bind the next CAS only to the prior exact receipt`, async () => {
    const f = await setup(root);
    try {
      f.offline(); await f.click(); await f.click('job-1', 'on_site');
      assert.match(f.receipt(), /pending/i);
      assert.equal(f.posts().length, 0);
      const queued = [...f.idb.state.rows.values()];
      assert.equal(queued.length, 2);
      f.offline(false); await f.dom.window.syncOfflineQueue();
      assert.equal(f.posts().length, 2); assert.equal(f.idb.state.rows.size, 0);
      assert.equal(JSON.parse(f.posts()[0].init.body).expected_updated_at, version);
      assert.equal(JSON.parse(f.posts()[1].init.body).expected_updated_at, nextVersion);
      assert.equal(f.jobs[0].status, 'on_site');
    } finally { f.dom.window.close(); }
  });
}

for (const root of roots) {
  for (const result of [Response.json({ success: false, error: 'rejected' }), Response.json({ success: true, error: null, id: 'other-job', status: 'en_route', updated_at: nextVersion }), Response.json({ success: true, error: null, id: 'job-1', status: 'en_route', updated_at: nextVersion }, { status: 202 }), new Response('{broken', { status: 200 })]) {
    test(`${root}: non-exact ${result.status} receipt stays unknown with the identical retry payload/key`, async () => {
      const f = await setup(root, { post: () => result.clone() });
      try {
        await f.click();
        assert.equal(f.posts().length, 1);
        assert.match(f.receipt(), /unknown/i);
        const row = [...f.idb.state.rows.values()][0];
        assert.equal(row.state, 'unknown');
        await f.dom.window.syncOfflineQueue();
        assert.equal(f.posts().length, 2);
        assert.equal(f.posts()[1].init.body, f.posts()[0].init.body);
        assert.equal(new Headers(f.posts()[1].init.headers).get('idempotency-key'), row.id);
        assert.equal(f.idb.state.rows.size, 1);
      } finally { f.dom.window.close(); }
    });
  }
  test(`${root}: a failed storage commit never claims a saved change or sends a mutation`, async () => {
    const f = await setup(root);
    try {
      f.idb.state.failCommit = true;
      await f.click();
      assert.equal(f.posts().length, 0);
      assert.equal(f.idb.state.rows.size, 0);
      assert.match(f.dom.window.document.getElementById('network-status-text').textContent, /failed|aborted|unavailable/i);
      assert.equal(f.dom.window.document.querySelector('[data-testid="job-status-job-1"]').textContent, 'pending');
    } finally { f.dom.window.close(); }
  });
  test(`${root}: legacy and another owner's queues are never adopted or cleared`, async () => {
    const f = await setup(root);
    try {
      f.idb.state.rows.set('legacy', { id: 'legacy', type: 'job_status', jobId: 'job-1', status: 'done' });
      f.idb.state.rows.set('foreign', { schema: 2, id: 'foreign', owner: { userId: 'owner-b', tenantId: 'tenant-b' }, type: 'job_status', jobId: 'job-1', status: 'done', expected_updated_at: version, state: 'pending', timestamp: 1 });
      await f.click();
      assert.equal(f.posts().length, 1);
      assert.equal(f.idb.state.rows.size, 2);
      assert.equal(f.idb.state.clearCalls, 0);
    } finally { f.dom.window.close(); }
  });
  test(`${root}: a changed canonical owner blocks queued replay before transport`, async () => {
    const f = await setup(root);
    try {
      f.offline(); await f.click();
      f.identity({ ...owner, userId: 'owner-b', tenantId: 'tenant-b' });
      f.offline(false); await f.dom.window.syncOfflineQueue();
      assert.equal(f.posts().length, 0);
      assert.equal(f.idb.state.rows.size, 1);
      assert.equal(f.dom.window.document.getElementById('route-container').textContent, '');
    } finally { f.dom.window.close(); }
  });
  for (const event of ['omnisolo_auth_changed', 'pagehide', 'storage']) {
    test(`${root}: ${event} retires delayed mutation receipt and private UI`, async () => {
      let finish;
      const f = await setup(root, { post: () => new Promise(resolve => { finish = resolve; }) });
      try {
        const pending = f.click(); await until(() => typeof finish === 'function');
        f.dom.window.dispatchEvent(event === 'storage' ? new f.dom.window.StorageEvent('storage', { key: null }) : new f.dom.window.Event(event));
        finish(Response.json({ success: true, error: null, id: 'job-1', status: 'en_route', updated_at: nextVersion }));
        await pending;
        assert.equal(f.dom.window.document.getElementById('route-container').textContent, '');
        assert.equal(f.idb.state.rows.size, 1);
        assert.equal([...f.idb.state.rows.values()][0].state, 'unknown');
      } finally { f.dom.window.close(); }
    });
  }
  test(`${root}: request body failure retains the durable intent for restart`, async () => {
    const idb = indexedDBFixture();
    const first = await setup(root, { idb, post: () => { throw new Error('network lost after commit'); } });
    await first.click();
    const original = first.posts()[0];
    const originalId = [...idb.state.rows.values()][0].id;
    first.dom.window.close();
    const resumed = await setup(root, { idb, post: () => Response.json({ success: true, error: null, id: 'job-1', status: 'en_route', updated_at: nextVersion }) });
    try {
      await until(() => resumed.posts().length === 1); await resumed.dom.window.syncOfflineQueue(); assert.equal(idb.state.rows.size, 0);
      assert.equal(resumed.posts()[0].init.body, original.init.body);
      assert.equal(new Headers(resumed.posts()[0].init.headers).get('idempotency-key'), originalId);
    } finally { resumed.dom.window.close(); }
  });
  test(`${root}: a new job queued mid-pass survives another job's acknowledgment`, async () => {
    let finish;
    const f = await setup(root, { post: (url, init) => String(url).includes('/job-1/') ? new Promise(resolve => { finish = resolve; })
      : Response.json({ success: true, error: null, id: 'job-2', status: JSON.parse(init.body).status, updated_at: nextVersion }) });
    try {
      const first = f.click(); await until(() => typeof finish === 'function');
      const second = f.click('job-2');
      await until(() => f.idb.state.rows.size === 2);
      finish(Response.json({ success: true, error: null, id: 'job-1', status: 'en_route', updated_at: nextVersion }));
      await Promise.all([first, second]);
      assert.equal(f.posts().length, 2);
      assert.equal(f.idb.state.rows.size, 0);
      assert.match(f.receipt('job-2'), /Confirmed/);
      assert.equal(f.idb.state.clearCalls, 0);
    } finally { f.dom.window.close(); }
  });
}
for (const root of roots) {
  test(`${root}: unrelated queued work cannot automatically retry an unknown outcome in the same pass`, async () => {
    let finish;
    const f = await setup(root, { post: url => String(url).includes('/job-1/') ? new Promise(resolve => { finish = resolve; })
      : Response.json({ success: true, error: null, id: 'job-2', status: 'en_route', updated_at: nextVersion }) });
    try {
      const first = f.click(); await until(() => typeof finish === 'function');
      const second = f.click('job-2'); await until(() => f.idb.state.rows.size === 2);
      f.post(url => String(url).includes('/job-1/') ? new Response('Unavailable', { status: 503 }) : Response.json({ success: true, error: null, id: 'job-2', status: 'en_route', updated_at: nextVersion }));
      finish(new Response('Unavailable', { status: 503 }));
      await Promise.all([first, second]);
      assert.equal(f.posts().filter(row => row.url.includes('/job-1/')).length, 1);
      assert.equal(f.posts().length, 2);
    } finally { f.dom.window.close(); }
  });
  test(`${root}: malformed identity response retires private data and never dispatches a queued mutation`, async () => {
    const f = await setup(root);
    try {
      f.offline(); await f.click(); f.identity(new Response('{bad', { status: 200 }));
      f.offline(false); await f.dom.window.syncOfflineQueue();
      assert.equal(f.posts().length, 0);
      assert.equal(f.dom.window.document.getElementById('route-container').textContent, '');
      assert.equal(f.idb.state.rows.size, 1);
    } finally { f.dom.window.close(); }
  });
  test(`${root}: a late route snapshot cannot replace a subsequently committed receipt version`, async () => {
    let finish;
    const f = await setup(root);
    try {
      const stale = Response.json({ routes: [{ jobs: structuredClone(f.jobs) }] });
      f.routes(() => new Promise(resolve => { finish = resolve; }));
      const read = f.dom.window.fetchRoutes(); await until(() => typeof finish === 'function');
      await f.click(); finish(stale); await read;
      f.routes(null);
      await f.click('job-1', 'on_site');
      assert.equal(JSON.parse(f.posts()[1].init.body).expected_updated_at, nextVersion);
    } finally { f.dom.window.close(); }
  });
}
for (const root of roots) {
  test(`${root}: a denied route read retires the prior private snapshot`, async () => {
    const f = await setup(root);
    try {
      f.routes(() => new Response('Authority revoked', { status: 403 }));
      await f.dom.window.fetchRoutes();
      assert.equal(f.dom.window.document.getElementById('route-container').textContent, '');
      await f.click(); assert.equal(f.posts().length, 0);
    } finally { f.dom.window.close(); }
  });
}
test('all supported standalone aliases preserve the same maintained source', async () => {
  const files = await Promise.all(roots.map(root => readFile(`${root}/field-service-route.html`, 'utf8')));
  for (const file of files.slice(1)) assert.equal(file, files[0]);
});
for (const root of roots) {
  test(`${root}: reconciliation control is visually hidden without an unknown request`, async () => {
    const f = await setup(root);
    try { assert.equal(f.dom.window.getComputedStyle(f.dom.window.document.getElementById('retry-sync')).display, 'none'); }
    finally { f.dom.window.close(); }
  });
}
for (const root of roots) {
  for (const identity of ['unexpected identity', 17, true, []]) {
    test(`${root}: non-object canonical identity ${JSON.stringify(identity)} retires private state before further work`, async () => {
      const f = await setup(root);
      try {
        f.identity(identity); await f.dom.window.fetchRoutes();
        assert.equal(f.dom.window.document.getElementById('route-container').textContent, '');
        await f.click(); assert.equal(f.posts().length, 0);
        assert.match(f.dom.window.document.getElementById('network-status-text').textContent, /identity|verify/i);
      } finally { f.dom.window.close(); }
    });
  }
  for (const returned of [version, '2026-10-03T09:00:00.000002+01:00', '2026-10-03T08:00:00.000001999Z']) {
    test(`${root}: a non-advanced precise receipt ${returned} retains the unknown intent and dependent CAS`, async () => {
      const f = await setup(root, { post: () => Response.json({ success: true, error: null, id: 'job-1', status: 'en_route', updated_at: returned }) });
      try {
        f.jobs[0].updated_at = nextVersion; await f.dom.window.fetchRoutes();
        f.offline(); await f.click(); await f.click('job-1', 'on_site');
        f.offline(false); await f.dom.window.syncOfflineQueue();
        assert.equal(f.posts().length, 1);
        assert.equal(f.idb.state.rows.size, 2);
        const rows = [...f.idb.state.rows.values()];
        assert.equal(rows.find(row => row.status === 'en_route').state, 'unknown');
        assert.equal(rows.find(row => row.status === 'on_site').expected_updated_at, null);
        assert.match(f.receipt(), /unknown/i);
      } finally { f.dom.window.close(); }
    });
  }
  test(`${root}: an advanced submillisecond receipt with an offset commits its exact next version`, async () => {
    const advanced = '2026-10-03T09:00:00.000001001+01:00';
    const f = await setup(root, { post: () => Response.json({ success: true, error: null, id: 'job-1', status: 'en_route', updated_at: advanced }) });
    try { await f.click(); assert.equal(f.idb.state.rows.size, 0); assert.match(f.receipt(), /Confirmed/); }
    finally { f.dom.window.close(); }
  });
}

for (const root of roots) {
  test(`${root}: Job Done immediately retires the prior confirmation before delayed local storage`, async () => {
    const f = await setup(root);
    let pending;
    try {
      await f.click(); await f.click('job-1', 'on_site');
      assert.equal(f.receipt(), 'Confirmed by server.');
      const button = f.dom.window.document.querySelector('[data-testid="btn-job-done-job-1"]');
      f.idb.state.holdOpen = true;
      pending = button.onclick();
      await button.onclick(); // A repeated click cannot stage another request.
      assert.equal(f.receipt(), 'Saving change locally; confirmation pending.');
      assert.equal(button.disabled, true);
      assert.equal(f.posts().length, 2);
      assert.equal(f.idb.state.rows.size, 0); // It is not saved locally yet.
      f.idb.state.holdOpen = false;
      f.idb.state.pendingOpens.splice(0).forEach(complete => complete());
      await pending;
      assert.equal(f.posts().length, 3);
      assert.deepEqual(JSON.parse(f.posts()[2].init.body), { status: 'done', expected_updated_at: '2026-10-03T08:00:00.000003Z' });
      assert.equal(f.receipt(), 'Confirmed by server.');
      assert.equal(f.dom.window.document.querySelector('[data-testid="job-status-job-1"]').textContent, 'done');
      assert.equal(f.idb.state.rows.size, 0);
    } finally {
      f.idb.state.holdOpen = false;
      f.idb.state.pendingOpens.splice(0).forEach(complete => complete());
      await pending; f.dom.window.close();
    }
  });
  test(`${root}: a failed local save never restores the previous transition confirmation`, async () => {
    const f = await setup(root);
    try {
      await f.click(); assert.equal(f.receipt(), 'Confirmed by server.');
      f.idb.state.failCommit = true;
      await f.click('job-1', 'on_site');
      assert.equal(f.posts().length, 1);
      assert.equal(f.idb.state.rows.size, 0);
      assert.doesNotMatch(f.receipt(), /Confirmed|Saved locally|Saving change/);
      assert.equal(f.dom.window.document.querySelector('[data-testid="job-status-job-1"]').textContent, 'en route');
      assert.match(f.dom.window.document.getElementById('network-status-text').textContent, /aborted/);
      assert.equal(f.dom.window.document.querySelector('[data-testid="btn-arrived-job-1"]').disabled, false);
    } finally { f.dom.window.close(); }
  });
}

for (const root of roots) {
  test(`${root}: persistent storage failure ends the saving claim without restoring a prior receipt`, async () => {
    const f = await setup(root);
    try {
      await f.click(); assert.equal(f.receipt(), 'Confirmed by server.');
      f.idb.state.failOpen = true;
      await f.click('job-1', 'on_site');
      assert.equal(f.posts().length, 1);
      assert.equal(f.idb.state.rows.size, 0);
      assert.equal(f.receipt(), 'Offline storage unavailable; confirmation unavailable.');
      assert.equal(f.dom.window.document.querySelector('[data-testid="btn-arrived-job-1"]').disabled, true);
      assert.equal(f.dom.window.document.querySelector('[data-testid="job-status-job-1"]').textContent, 'en route');
      assert.match(f.dom.window.document.getElementById('network-status-text').textContent, /cannot be confirmed/);
      f.idb.state.failOpen = false;
      await f.dom.window.fetchRoutes();
      assert.equal(f.receipt(), 'Current server snapshot.');
      assert.equal(f.dom.window.document.querySelector('[data-testid="btn-arrived-job-1"]').disabled, false);
    } finally { f.dom.window.close(); }
  });
}
