import assert from 'node:assert/strict';
import { test } from 'node:test';
import { readFile } from 'node:fs/promises';
import { JSDOM, VirtualConsole } from 'jsdom';

const builders = ['src/ui/next/public/ai-lead-magnet-builder.html', 'src/ui/next/public/ui/ai-lead-magnet-builder.html', 'src/ui/tauri/src/ui/ai-lead-magnet-builder.html'];
const dashboards = ['src/ui/next/public/dashboard.html', 'src/ui/next/public/ui/dashboard.html', 'src/ui/tauri/src/ui/dashboard.html'];
const turn = () => new Promise(resolve => setImmediate(resolve));
async function load(path, options = {}) {
  const calls = [], opens = [], copies = [];
  let identityReads = 0;
  const tenant = options.tenant ?? 'tenant-a';
  const identity = { userId: 'owner-a', tenantId: tenant, expiresAt: Date.now() + 60_000 };
  const dom = new JSDOM(await readFile(path, 'utf8'), {
    url: 'https://example.test/ai-lead-magnet-builder.html', runScripts: 'dangerously',
    virtualConsole: new VirtualConsole(),
    beforeParse(window) {
      window.localStorage.setItem('tenant', 'untrusted-other-tenant');
      window.localStorage.setItem('has_pro', 'true');
      window.Headers = Headers;
      window.open = (...args) => { opens.push(args); return null; };
      window.fetch = async (url, init) => {
        calls.push({ url, init });
        if (String(url).endsWith('/auth/session-identity')) { identityReads += 1; return Response.json(options.changeOwner && identityReads > 1 ? { ...identity, userId: 'different-owner' } : identity); }
        if (String(url).endsWith('/billing/my-plan')) {
          const headers = new Headers(init?.headers);
          if (options.foreignPlan && headers.has('x-ohc-expected-user')) return Response.json({ error: 'queued owner does not match the current session' }, { status: 409 });
          return Response.json({ current_plan: options.plan ?? 'free' });
        }
        return Response.json({ success: true });
      };
      Object.defineProperty(window.navigator, 'clipboard', { value: { writeText: async value => { copies.push(value); if (options.copyDelay) await options.copyDelay(); if (options.copyFailure) throw new Error('denied'); } } });
      window.document.execCommand = () => false;
    },
  });
  if (dom.window.document.readyState === 'loading') await new Promise(resolve => dom.window.document.addEventListener('DOMContentLoaded', resolve, { once: true }));
  await turn(); await turn();
  return { dom, document: dom.window.document, calls, opens, copies, tenant };
}
for (const path of dashboards) test(`${path}: navigation reaches the actual lead-magnet builder`, async () => {
  const dom = new JSDOM(await readFile(path, 'utf8'), { url: 'https://example.test/dashboard.html' });
  try { assert.equal(dom.window.document.querySelector('#ai-lead-magnet-link')?.href, 'https://example.test/ai-lead-magnet-builder.html'); }
  finally { dom.window.close(); }
});
for (const path of builders) {
  test(`${path}: local Pro flag cannot hide branding for a verified free owner`, async () => {
    const { dom, document, calls } = await load(path);
    try {
      document.querySelector('#remove-branding').click();
      assert.equal(document.querySelector('#remove-branding').checked, false);
      assert.notEqual(document.querySelector('#preview-branding').style.display, 'none');
      assert.ok(calls.some(call => String(call.url).endsWith('/billing/my-plan')));
    } finally { dom.window.close(); }
  });
  test(`${path}: share intent cannot claim a trial or change entitlement`, async () => {
    const { dom, document, calls, opens } = await load(path);
    try {
      document.querySelector('#share-to-unlock-btn').click(); await turn();
      assert.equal(opens.length, 1);
      assert.equal(new URL(opens[0][0]).hostname, 'twitter.com');
      assert.equal(calls.some(call => call.init?.method === 'POST'), false);
      assert.match(document.querySelector('#soft-paywall-status').textContent, /not verified|unavailable/i);
      assert.doesNotMatch(document.querySelector('#soft-paywall-status').textContent, /unlocked|verifying share/i);
      assert.notEqual(document.querySelector('#preview-branding').style.display, 'none');
    } finally { dom.window.close(); }
  });
  test(`${path}: embed URL preserves the verified tenant and supported query fields`, async () => {
    const { dom, document, tenant } = await load(path, { tenant: `owner'&雪` });
    try {
      const description = document.querySelector('#magnet-desc'); description.value = 'A & B "雪"'; description.dispatchEvent(new dom.window.Event('input'));
      document.querySelector('#get-code-btn').click();
      const fragment = new JSDOM(document.querySelector('#embed-code').value);
      try {
        const url = new URL(fragment.window.document.querySelector('iframe').src);
        assert.equal(url.origin, 'https://example.test'); assert.equal(url.searchParams.get('tenant'), tenant);
        assert.equal(url.searchParams.get('description'), 'A & B "雪"');
        assert.equal(url.searchParams.get('buttonText'), 'Generate Free Audit');
        assert.equal(url.searchParams.get('hideBranding'), 'false');
      } finally { fragment.window.close(); }
    } finally { dom.window.close(); }
  });
  test(`${path}: clipboard failure cannot be reported as copied`, async () => {
    const { dom, document } = await load(path, { copyFailure: true });
    try {
      document.querySelector('#get-code-btn').click(); document.querySelector('#copy-code-btn').click(); await turn();
      assert.doesNotMatch(document.querySelector('#copy-code-btn').textContent, /^Copied/);
      assert.match(document.querySelector('#embed-copy-status')?.textContent ?? '', /could not copy/i);
    } finally { dom.window.close(); }
  });
}

for (const path of builders) {
  test(`${path}: verified Pro preview remains available without any grant`, async () => {
    const { dom, document, calls } = await load(path, { plan: 'pro' });
    try {
      document.querySelector('#remove-branding').click();
      assert.equal(document.querySelector('#remove-branding').checked, true);
      assert.equal(document.querySelector('#preview-branding').style.display, 'none');
      document.querySelector('#get-code-btn').click();
      const fragment = new JSDOM(document.querySelector('#embed-code').value);
      try { assert.equal(new URL(fragment.window.document.querySelector('iframe').src).searchParams.get('hideBranding'), 'true'); }
      finally { fragment.window.close(); }
      assert.equal(calls.some(call => call.init?.method === 'POST'), false);
    } finally { dom.window.close(); }
  });
  test(`${path}: an owner change during plan verification holds embed generation`, async () => {
    const { dom, document } = await load(path, { plan: 'pro', changeOwner: true });
    try {
      assert.equal(document.querySelector('#get-code-btn').disabled, true);
      assert.equal(document.querySelector('#remove-branding').disabled, true);
      assert.notEqual(document.querySelector('#preview-branding').style.display, 'none');
    } finally { dom.window.close(); }
  });
  test(`${path}: canonical logout clears the owner binding and generated snippet`, async () => {
    const { dom, document } = await load(path, { plan: 'pro' });
    try {
      document.querySelector('#get-code-btn').click(); assert.ok(document.querySelector('#embed-code').value);
      dom.window.dispatchEvent(new dom.window.StorageEvent('storage', { key: 'omnisolo_queue_identity_epoch_v2' }));
      assert.equal(document.querySelector('#embed-code').value, '');
      assert.equal(document.querySelector('#get-code-btn').disabled, true);
      assert.equal(document.querySelector('#share-to-unlock-btn').disabled, true);
    } finally { dom.window.close(); }
  });
  test(`${path}: late clipboard completion cannot acknowledge a reopened modal`, async () => {
    let finish;
    const { dom, document } = await load(path, { copyDelay: () => new Promise(resolve => { finish = resolve; }) });
    try {
      document.querySelector('#get-code-btn').click(); document.querySelector('#copy-code-btn').click();
      assert.equal(typeof finish, 'function');
      document.querySelector('#close-embed-btn').click(); document.querySelector('#get-code-btn').click();
      finish(); await turn();
      assert.equal(document.querySelector('#embed-copy-status').textContent, '');
      assert.equal(document.querySelector('#copy-code-btn').disabled, false);
      assert.equal(document.querySelector('#preview-btn-text').disabled, true);
    } finally { dom.window.close(); }
  });
}

for (const path of builders) {
  test(`${path}: an ABA session switch cannot import another owner's Pro plan`, async () => {
    const { dom, document, calls } = await load(path, { plan: 'pro', foreignPlan: true });
    try {
      document.querySelector('#remove-branding').click();
      assert.equal(document.querySelector('#remove-branding').checked, false);
      const request = calls.find(call => String(call.url).endsWith('/billing/my-plan'));
      assert.equal(new Headers(request.init?.headers).get('x-ohc-expected-user'), 'owner-a');
      assert.equal(new Headers(request.init?.headers).get('x-ohc-expected-tenant'), 'tenant-a');
      assert.notEqual(document.querySelector('#preview-branding').style.display, 'none');
    } finally { dom.window.close(); }
  });
  test(`${path}: an identity that cannot round-trip as headers keeps plan eligibility held`, async () => {
    const { dom, document, calls } = await load(path, { plan: 'pro', tenant: 'owner雪' });
    try {
      document.querySelector('#remove-branding').click();
      assert.equal(document.querySelector('#remove-branding').checked, false);
      assert.equal(calls.some(call => String(call.url).endsWith('/billing/my-plan')), false);
      assert.equal(document.querySelector('#get-code-btn').disabled, false);
    } finally { dom.window.close(); }
  });
}
