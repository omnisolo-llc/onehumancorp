import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';

const roots = ['src/ui/next/public', 'src/ui/next/public/ui', 'src/ui/tauri/src/ui'];
const turn = () => new Promise(resolve => setImmediate(resolve));
const settle = async () => { await turn(); await turn(); };
const unavailable = () => Response.json({ success: false, code: 'capability_unavailable', capability: 'trial_entitlement' }, { status: 501 });

async function load(root, plan = 'Free', claim = unavailable) {
  const requests = [];
  let providerWindows = 0;
  let currentOwner = { userId: 'owner-1', tenantId: 'verified-tenant', expiresAt: Date.now() + 60_000 };
  const dom = new JSDOM(await readFile(`${root}/work-intake-widget.html`, 'utf8'), {
    url: 'https://app.example.test/work-intake-widget.html', runScripts: 'dangerously', virtualConsole: new VirtualConsole(),
    beforeParse(window) {
      window.localStorage.setItem('has_pro', 'true');
      window.open = () => { providerWindows += 1; return null; };
      window.fetch = async (url, options) => {
        requests.push({ url: String(url), options });
        if (url === '/api/v1/auth/session-identity') return Response.json(currentOwner);
        if (url === '/api/v1/billing/my-plan') return typeof plan === 'function' ? plan() : Response.json({ current_plan: plan });
        if (url === '/api/v1/growth/trial-extension/claim') return claim();
        throw new Error(`Unexpected request: ${url}`);
      };
    },
  });
  await settle();
  const document = dom.window.document;
  return {
    dom, document, requests, byId: id => document.getElementById(id),
    providerWindows: () => providerWindows,
    changeOwner() { currentOwner = { ...currentOwner, userId: 'owner-2', tenantId: 'other-tenant' }; },
  };
}

for (const root of roots) {
  test(`${root}: forged local Pro cannot remove Free branding or change the embed owner`, async () => {
    const page = await load(root);
    try {
      page.byId('remove-branding-checkbox').click(); await settle();
      assert.equal(page.byId('remove-branding-checkbox').checked, false);
      assert.equal(page.byId('paywall-modal').style.display, 'flex');
      page.byId('close-paywall-btn').click();
      page.byId('tenant-input').value = 'forged-tenant';
      page.byId('get-code-btn').click(); await settle();
      const code = page.byId('embed-textarea').value;
      assert.match(code, /OmniSolo/);
      const url = new URL(code.match(/src="([^"]+)"/)[1]);
      assert.equal(url.origin, 'https://app.example.test');
      assert.equal(url.searchParams.get('tenant'), 'verified-tenant');
      assert.equal(url.searchParams.get('branding'), 'true');
      const planRequests = page.requests.filter(request => request.url === '/api/v1/billing/my-plan');
      assert.ok(planRequests.length >= 3);
      for (const { options } of planRequests) {
        assert.equal(options.headers['x-ohc-expected-user'], 'owner-1');
        assert.equal(options.headers['x-ohc-expected-tenant'], 'verified-tenant');
      }
    } finally { page.dom.window.close(); }
  });

  for (const [name, claim] of [
    ['501 rejection', unavailable],
    ['network failure', () => { throw new Error('offline'); }],
    ['unverified success response', () => Response.json({ success: true })],
  ]) {
    test(`${root}: ${name} retains the paywall, unchecked branding and visible explanation`, async () => {
      const page = await load(root, 'Free', claim);
      try {
        page.dom.window.localStorage.removeItem('has_pro');
        page.byId('remove-branding-checkbox').click(); await settle();
        page.byId('share-to-unlock-btn').click(); await settle();
        assert.equal(page.byId('paywall-modal').style.display, 'flex');
        assert.equal(page.byId('remove-branding-checkbox').checked, false);
        assert.match(page.byId('soft-paywall-status').textContent, /unavailable|not verified/i);
        assert.equal(page.byId('soft-paywall-status').style.display, 'block');
        assert.equal(page.byId('share-to-unlock-btn').disabled, false);
        assert.equal(page.dom.window.localStorage.getItem('has_pro'), null);
        assert.equal(page.providerWindows(), 0);
        assert.equal(new URL(page.byId('preview-iframe').src).searchParams.get('branding'), 'true');
      } finally { page.dom.window.close(); }
    });
  }

  for (const plan of ['Pro', 'Business']) {
    test(`${root}: an existing verified ${plan} plan can remove branding without a trial request`, async () => {
      const page = await load(root, plan);
      try {
        page.dom.window.localStorage.removeItem('has_pro');
        page.byId('remove-branding-checkbox').click(); await settle();
        assert.equal(page.byId('remove-branding-checkbox').checked, true);
        assert.equal(new URL(page.byId('preview-iframe').src).searchParams.get('branding'), 'false');
        page.byId('get-code-btn').click(); await settle();
        const code = page.byId('embed-textarea').value;
        assert.doesNotMatch(code, /OmniSolo/);
        assert.equal(new URL(code.match(/src="([^"]+)"/)[1]).searchParams.get('branding'), 'false');
        assert.equal(page.requests.filter(request => request.url.includes('trial-extension/claim')).length, 0);
      } finally { page.dom.window.close(); }
    });
  }

  for (const response of [() => Response.json({ error: 'unavailable' }, { status: 503 }), () => Response.json({ current_plan: 'Enterprise?' })]) {
    test(`${root}: unavailable or unknown plan blocks branding and embed generation (${response().status})`, async () => {
      const page = await load(root, response);
      try {
        assert.equal(page.byId('remove-branding-checkbox').disabled, true);
        assert.equal(page.byId('get-code-btn').disabled, true);
        assert.equal(page.byId('embed-textarea').value, '');
        assert.match(page.byId('entitlement-status').textContent, /unavailable/i);
      } finally { page.dom.window.close(); }
    });
  }

  test(`${root}: a revoked paid plan is reread before generating an embed`, async () => {
    let plan = 'Pro';
    const page = await load(root, () => Response.json({ current_plan: plan }));
    try {
      page.byId('remove-branding-checkbox').click(); await settle();
      assert.equal(page.byId('remove-branding-checkbox').checked, true);
      plan = 'Free';
      page.byId('get-code-btn').click(); await settle();
      assert.equal(page.byId('remove-branding-checkbox').checked, false);
      assert.match(page.byId('embed-textarea').value, /OmniSolo/);
      assert.equal(new URL(page.byId('preview-iframe').src).searchParams.get('branding'), 'true');
    } finally { page.dom.window.close(); }
  });

  for (const event of ['omnisolo_auth_changed', 'pagehide', 'storage', 'identity-response']) {
    test(`${root}: ${event} fences a late Pro plan response and clears old embed code`, async () => {
      let answer = () => Response.json({ current_plan: 'Pro' });
      const page = await load(root, () => answer());
      try {
        page.byId('get-code-btn').click(); await settle();
        assert.notEqual(page.byId('embed-textarea').value, '');
        let resolve;
        answer = () => new Promise(done => { resolve = done; });
        page.byId('get-code-btn').click(); await settle();
        assert.equal(typeof resolve, 'function');
        if (event === 'identity-response') page.changeOwner();
        else page.dom.window.dispatchEvent(event === 'storage' ? new page.dom.window.StorageEvent('storage', { key: null }) : new page.dom.window.Event(event));
        resolve(Response.json({ current_plan: 'Pro' })); await settle();
        assert.equal(page.byId('remove-branding-checkbox').checked, false);
        assert.equal(page.byId('get-code-btn').disabled, true);
        assert.equal(page.byId('embed-textarea').value, '');
        assert.equal(page.byId('code-modal').style.display, 'none');
      } finally { page.dom.window.close(); }
    });
  }

  test(`${root}: cancelling a pending trial check cannot reopen the paywall or unlock branding`, async () => {
    let resolve;
    const page = await load(root, 'Free', () => new Promise(done => { resolve = done; }));
    try {
      page.byId('remove-branding-checkbox').click(); await settle();
      page.byId('share-to-unlock-btn').click(); await settle();
      assert.equal(typeof resolve, 'function');
      page.byId('close-paywall-btn').click();
      resolve(unavailable()); await settle();
      assert.equal(page.byId('paywall-modal').style.display, 'none');
      assert.equal(page.byId('remove-branding-checkbox').checked, false);
      assert.equal(page.providerWindows(), 0);
    } finally { page.dom.window.close(); }
  });
}
