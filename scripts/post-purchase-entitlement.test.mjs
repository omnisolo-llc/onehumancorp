import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';
const roots = ['src/ui/next/public', 'src/ui/next/public/ui', 'src/ui/next/public/api/ui', 'src/ui/next/public/api/v1/ui', 'src/ui/tauri/src/ui'];
const turn = () => new Promise(resolve => setImmediate(resolve));
async function load(root, plan, forgedPro = false) {
  const requests = [];
  const dom = new JSDOM(await readFile(`${root}/post-purchase-share.html`, 'utf8'), {
    url: 'https://app.example.test/post-purchase-share.html', runScripts: 'dangerously', virtualConsole: new VirtualConsole(),
    beforeParse(window) {
      window.localStorage.setItem('has_pro', String(forgedPro));
      window.localStorage.setItem('tenant', 'forged-tenant');
      window.fetch = async (url, options) => {
        requests.push({ url: String(url), options });
        if (url === '/api/v1/auth/session-identity') return Response.json({ userId: 'owner-1', tenantId: 'verified-tenant', expiresAt: Date.now() + 60_000 });
        if (url === '/api/v1/billing/my-plan') return typeof plan === 'function' ? plan() : plan === null ? Response.json({ error: 'unavailable' }, { status: 503 }) : Response.json({ current_plan: plan });
        throw new Error(`Unexpected request: ${url}`);
      };
    },
  });
  if (dom.window.document.readyState === 'loading') await new Promise(resolve => dom.window.document.addEventListener('DOMContentLoaded', resolve, { once: true }));
  await turn(); await turn();
  return { dom, document: dom.window.document, requests };
}
for (const root of roots) {
  test(`${root}: a forged local Pro flag cannot remove Free account branding`, async () => {
    const { dom, document, requests } = await load(root, 'Free', true);
    try {
      document.getElementById('remove-branding').click(); await turn(); await turn();
      assert.equal(document.getElementById('remove-branding').checked, false);
      assert.ok(document.getElementById('paywall-modal').classList.contains('active'));
      assert.notEqual(document.getElementById('preview-branding').style.display, 'none');
      assert.ok(requests.some(request => request.url === '/api/v1/billing/my-plan'));
    } finally { dom.window.close(); }
  });
  for (const plan of ['Pro', 'Business']) {
    test(`${root}: preserves verified ${plan} branding removal without a local flag`, async () => {
      const { dom, document } = await load(root, plan);
      try {
        document.getElementById('remove-branding').click(); await turn(); await turn();
        assert.equal(document.getElementById('remove-branding').checked, true);
        assert.equal(document.getElementById('preview-branding').style.display, 'none');
        document.getElementById('get-code-btn').click(); await turn(); await turn();
        const url = new URL(document.getElementById('embed-code').value.match(/src="([^"]+)"/)[1].replaceAll('&amp;', '&'));
        assert.equal(url.origin, 'https://app.example.test');
        assert.equal(url.searchParams.get('tenant'), 'verified-tenant');
        assert.equal(url.searchParams.get('hideBranding'), 'true');
      } finally { dom.window.close(); }
    });
  }
  test(`${root}: unavailable plan blocks private embed generation and does not infer Free or Pro`, async () => {
    const { dom, document } = await load(root, null, true);
    try {
      assert.equal(document.getElementById('remove-branding').disabled, true);
      assert.equal(document.getElementById('get-code-btn').disabled, true);
      assert.equal(document.getElementById('embed-code').value, '');
      assert.match(document.getElementById('entitlement-status').textContent, /unavailable/i);
    } finally { dom.window.close(); }
  });
  test(`${root}: no share activation or invented coupon is offered`, async () => {
    const { dom, document, requests } = await load(root, 'Free');
    try {
      assert.equal(document.getElementById('share-to-unlock-btn'), null);
      assert.equal(document.getElementById('preview-link').value, '');
      assert.match(document.getElementById('preview-desc').textContent, /preview|draft/i);
      assert.match(document.querySelector('#embed-modal p').textContent, /authenticated preview/i);
      assert.ok(!requests.some(request => request.url.includes('trial-extension/claim')));
      document.getElementById('remove-branding').click(); await turn(); await turn();
      document.getElementById('close-paywall').click();
      assert.equal(document.getElementById('paywall-modal').classList.contains('active'), false);
      assert.equal(document.getElementById('remove-branding').checked, false);
    } finally { dom.window.close(); }
  });
}

for (const root of roots) {
  for (const event of ['omnisolo_auth_changed', 'pagehide', 'storage']) {
    test(`${root}: ${event} retires a delayed private embed response`, async () => {
      let answer = async () => Response.json({ current_plan: 'Pro' });
      const { dom, document } = await load(root, () => answer());
      try {
        let resolve;
        answer = () => new Promise(done => { resolve = done; });
        document.getElementById('get-code-btn').click(); await turn();
        assert.equal(typeof resolve, 'function');
        dom.window.dispatchEvent(event === 'storage' ? new dom.window.StorageEvent('storage', { key: null }) : new dom.window.Event(event));
        resolve(Response.json({ current_plan: 'Pro' })); await turn(); await turn();
        assert.equal(document.getElementById('embed-code').value, '');
        assert.equal(document.getElementById('get-code-btn').disabled, true);
        assert.equal(document.getElementById('remove-branding').checked, false);
        assert.equal(document.getElementById('embed-modal').classList.contains('active'), false);
      } finally { dom.window.close(); }
    });
  }
  test(`${root}: a revoked paid plan is reread before removing branding`, async () => {
    let plan = 'Pro';
    const { dom, document } = await load(root, async () => Response.json({ current_plan: plan }));
    try {
      plan = 'Free'; document.getElementById('remove-branding').click(); await turn(); await turn();
      assert.equal(document.getElementById('remove-branding').checked, false);
      assert.ok(document.getElementById('paywall-modal').classList.contains('active'));
    } finally { dom.window.close(); }
  });
}

for (const root of roots) {
  for (const result of ['accepted', 'denied', 'throws']) {
    test(`${root}: clipboard ${result} reports the actual copy outcome`, async () => {
      const { dom, document } = await load(root, 'Pro');
      try {
        document.execCommand = () => { if (result === 'throws') throw new Error('fixture clipboard denied'); return result === 'accepted'; };
        document.getElementById('get-code-btn').click(); await turn(); await turn();
        document.getElementById('copy-code-btn').click(); await turn(); await turn();
        const button = document.getElementById('copy-code-btn');
        if (result === 'accepted') assert.equal(button.textContent, 'Copied!');
        else assert.match(button.textContent, /copy failed/i, 'a denied copy must stay recoverable and must not claim success');
      } finally { dom.window.close(); }
    });
  }
  test(`${root}: a silent identity epoch change prevents copying the previous account draft`, async () => {
    const { dom, document } = await load(root, 'Pro');
    try {
      let copies = 0;
      document.execCommand = () => { copies++; return true; };
      document.getElementById('get-code-btn').click(); await turn(); await turn();
      assert.match(document.getElementById('embed-code').value, /verified-tenant/);
      dom.window.localStorage.setItem('omnisolo_queue_identity_epoch_v2', 'retired-owner-epoch');
      document.getElementById('copy-code-btn').click(); await turn(); await turn();
      assert.equal(copies, 0);
      assert.equal(document.getElementById('embed-code').value, '');
      assert.equal(document.getElementById('embed-modal').classList.contains('active'), false);
    } finally { dom.window.close(); }
  });
}

for (const root of roots) {
  test(`${root}: storage becoming unavailable retires visible private code without copying`, async () => {
    const { dom, document } = await load(root, 'Pro');
    try {
      let copies = 0;
      let errors = 0;
      document.execCommand = () => { copies++; return true; };
      dom.window.addEventListener('error', () => { errors++; });
      document.getElementById('get-code-btn').click(); await turn(); await turn();
      Object.defineProperty(dom.window.Storage.prototype, 'getItem', { configurable: true, value() { throw new Error('fixture storage unavailable'); } });
      document.getElementById('copy-code-btn').click(); await turn(); await turn();
      assert.equal(copies, 0);
      assert.equal(errors, 0, 'identity storage failure must retire cleanly');
      assert.equal(document.getElementById('embed-code').value, '');
      assert.equal(document.getElementById('embed-modal').classList.contains('active'), false);
    } finally { dom.window.close(); }
  });
  test(`${root}: explicitly closing a preview fences a pending Get Embed Code refresh`, async () => {
    let answer = async () => Response.json({ current_plan: 'Pro' });
    const { dom, document } = await load(root, () => answer());
    try {
      document.getElementById('get-code-btn').click(); await turn(); await turn();
      assert.equal(document.getElementById('embed-modal').classList.contains('active'), true);
      let resolve;
      answer = () => new Promise(done => { resolve = done; });
      document.getElementById('get-code-btn').click(); await turn();
      assert.equal(typeof resolve, 'function');
      document.getElementById('close-embed-btn').click();
      resolve(Response.json({ current_plan: 'Pro' })); await turn(); await turn();
      assert.equal(document.getElementById('embed-modal').classList.contains('active'), false);
      assert.equal(document.getElementById('embed-code').value, '');
      answer = async () => Response.json({ current_plan: 'Pro' });
      document.getElementById('get-code-btn').click(); await turn(); await turn();
      assert.equal(document.getElementById('embed-modal').classList.contains('active'), true, 'a new explicit request remains supported');
    } finally { dom.window.close(); }
  });
}
