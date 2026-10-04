import assert from 'node:assert/strict';
import { access, readFile } from 'node:fs/promises';
import { test } from 'node:test';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';

test('the mounted conversion RPC cannot issue an unverified permanent plan or ledger credit', async () => {
  const source = await readFile('src/server/services/growth/service.rs', 'utf8');
  const method = source.split('    async fn convert_referral(')[1]?.split('    async fn get_downloads(')[0];
  assert.ok(method, 'the existing conversion RPC remains implemented');
  assert.match(method, /self\.get_org_id\(request\.metadata\(\)\)\.await\?/);
  assert.match(method, /Err\(Status::failed_precondition\(/);
  assert.doesNotMatch(method, /sqlx::|\.pool|\.begin\(|\.commit\(|\.execute\(|14_day_pro_trial|Both parties received/);
  const server = await readFile('src/server/lib.rs', 'utf8');
  assert.match(server, /add_service\(GrowthServiceServer::with_interceptor\(growth_service, SpiffeInterceptor\)\)/);
});

for (const file of ['src/ui/next/public/trial-extension.html', 'src/ui/next/public/ui/trial-extension.html']) {
  test(`${file}: the web release no longer ships the plan interstitial`, async () => {
    await assert.rejects(access(file), { code: 'ENOENT' });
  });
}

// Preserve standalone compatibility; the web aliases are covered by the real
// canonical-page and redirect browser tests instead of loading deleted assets.
for (const file of ['src/ui/tauri/src/ui/trial-extension.html']) {
  test(`${file}: directs owners to verified plan reads without a share-to-grant claim`, async () => {
    const requests = [];
    const dom = new JSDOM(await readFile(file, 'utf8'), {
      url: 'https://app.example.test/trial-extension.html', runScripts: 'dangerously',
      virtualConsole: new VirtualConsole(),
      beforeParse(window) {
        window.fetch = async (url, options) => { requests.push({ url, options }); return Response.json({}); };
      },
    });
    try {
      const document = dom.window.document;
      assert.equal(document.querySelector('h1').textContent.trim(), 'Plan and Trial Availability');
      assert.match(document.querySelector('main').textContent, /durable grant is not verified/i);
      assert.equal(document.querySelector('a[data-current-plan]').getAttribute('href'), '/trial-extension');
      assert.equal(document.querySelector('a[data-review-plans]').getAttribute('href'), '/pricing');
      assert.equal(document.querySelector('#claim-btn'), null);
      assert.equal(document.querySelector('#claimed-state'), null);
      assert.doesNotMatch(document.querySelector('main').textContent, /7 (?:Extra )?Days|Trial Extended|Share on X to Unlock/);
      assert.equal(requests.some(({ url, options }) => String(url).includes('/trial-extension/claim') || options?.method === 'POST'), false);
    } finally { dom.window.close(); }
  });
}
