import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';

const roots = ['src/ui/tauri/src/ui', 'src/ui/next/public', 'src/ui/next/public/ui'];
const fullRoots = [...roots, 'src/ui/next/public/api/ui', 'src/ui/next/public/api/v1/ui'];

async function loadPage(root, name, url = 'http://127.0.0.1:18789/') {
  const errors = [];
  const virtualConsole = new VirtualConsole();
  virtualConsole.on('jsdomError', (error) => errors.push(error));
  const html = await readFile(new URL(`../${root}/${name}.html`, import.meta.url), 'utf8');
  const dom = new JSDOM(html, {
    url: new URL(`${name}.html`, url).href,
    runScripts: 'dangerously',
    pretendToBeVisual: true,
    virtualConsole,
    beforeParse(window) {
      // Unit boundary only: no application backend is claimed by these DOM tests.
      window.fetch = async url => {
        if (url === '/api/v1/auth/session-identity') return Response.json({ userId: 'modal-owner', tenantId: 'modal-tenant', expiresAt: Date.now() + 60_000 });
        if (url === '/api/v1/billing/my-plan') return Response.json({ current_plan: 'Free' });
        return { ok: true, json: async () => [] };
      };
    },
  });
  await new Promise(resolve => dom.window.addEventListener('load', resolve, { once: true }));
  await new Promise(resolve => setImmediate(resolve));
  return { dom, errors };
}

for (const root of roots) {
  test(`${root}: generated certificate link uses the serving origin and entered values`, async () => {
    for (const origin of ['http://127.0.0.1:39151/', 'http://localhost:3000/', 'https://business.example/']) {
      const { dom, errors } = await loadPage(root, 'viral-certificate-generator', origin);
      try {
        const document = dom.window.document;
        for (const [id, value] of Object.entries({ tenant: 'my-business', 'cert-title': 'A & B Certificate', recipient: 'Jane Doe', 'course-name': 'Course 1' })) {
          document.getElementById(id).value = value;
        }
        document.getElementById('generate-btn').click();
        const embed = document.getElementById('embed-code').innerText;
        const url = new URL(embed.match(/src="([^"]+)"/)[1]);
        assert.equal(url.origin, new URL(origin).origin);
        assert.equal(url.pathname, '/api/v1/growth/certificate/embed');
        assert.deepEqual(Object.fromEntries(url.searchParams), {
          tenant: 'my-business', title: 'A & B Certificate', recipient: 'Jane Doe', course: 'Course 1', theme: 'light',
        });
        assert.deepEqual(errors, []);
      } finally { dom.window.close(); }
    }
  });
}

for (const root of fullRoots) {
  test(`${root}: the Help launcher is fixed and Ask AI opens and focuses its chat`, async () => {
    const { dom, errors } = await loadPage(root, 'help');
    try {
      const { document } = dom.window;
      const launcher = document.getElementById('ohc-floating-help-btn');
      assert.ok(launcher);
      assert.equal(dom.window.getComputedStyle(launcher).position, 'fixed');
      dom.window.openAISupport();
      // Chat selection and focus run on successive rendering frames. A fixed
      // timer can fire between them when the first frame is delayed.
      await new Promise(resolve => dom.window.requestAnimationFrame(() => dom.window.requestAnimationFrame(resolve)));
      assert.equal(document.querySelector('#ohc-floating-help-widget, #omnisolo-floating-help-widget').style.display, 'flex');
      assert.equal(document.getElementById('tab-chat').classList.contains('active'), true);
      assert.equal(document.activeElement, document.getElementById('ohc-help-chat-input'));
      assert.deepEqual(errors, []);
    } finally { dom.window.close(); }
  });
}

for (const root of fullRoots) {
  test(`${root}: closed post-purchase modals are hidden from keyboard and accessibility navigation`, async () => {
    const { dom, errors } = await loadPage(root, 'post-purchase-share');
    try {
      const { document } = dom.window;
      const paywall = document.getElementById('paywall-modal');
      const embed = document.getElementById('embed-modal');
      const visibility = element => dom.window.getComputedStyle(element).visibility;
      assert.equal(visibility(paywall), 'hidden');
      assert.equal(visibility(embed), 'hidden');
      document.getElementById('remove-branding').click();
      await new Promise(resolve => setImmediate(resolve));
      assert.equal(visibility(paywall), 'visible');
      document.getElementById('close-paywall').click();
      assert.equal(visibility(paywall), 'hidden');
      document.getElementById('get-code-btn').click();
      await new Promise(resolve => setImmediate(resolve));
      assert.equal(visibility(embed), 'visible');
      document.getElementById('close-embed-btn').click();
      assert.equal(visibility(embed), 'hidden');
      assert.deepEqual(errors, []);
    } finally { dom.window.close(); }
  });
}

for (const root of roots) {
  for (const [name, inputIds] of [
    ['viral-wifi-qr-generator', ['network-name', 'remove-branding']],
    ['viral-secret-menu-generator', ['itemName', 'itemDesc', 'accessCode', 'sharesReq', 'copyBtn']],
  ]) {
    test(`${root}: ${name} accepts edits only after preview initialization`, async () => {
      const html = await readFile(new URL(`../${root}/${name}.html`, import.meta.url), 'utf8');
      // A linked stylesheet can delay the bottom script while its form is already visible.
      const delayedHtml = html.replaceAll('<script>', '<script type="text/plain">');
      const dom = new JSDOM(delayedHtml, { url: `http://127.0.0.1:39151/${name}.html`, runScripts: 'dangerously' });
      try {
        const { document } = dom.window;
        for (const id of inputIds) assert.equal(document.getElementById(id).disabled, true, `${id} before initialization`);
        // Browser-restored/autofilled values must survive initialization too.
        if (name.includes('wifi')) document.getElementById('network-name').value = 'Restored WiFi';
        const ready = new Promise(resolve => dom.window.addEventListener('load', resolve, { once: true }));
        for (const script of document.querySelectorAll('script:not([src])')) dom.window.eval(script.textContent);
        await ready;
        for (const id of inputIds) assert.equal(document.getElementById(id).disabled, false, `${id} after initialization`);
        if (name.includes('wifi')) {
          const qrPayload = () => new URL(document.getElementById('qr-image').src).searchParams.get('data');
          assert.equal(qrPayload(), 'https://omnisolo.co/checkout?product=Restored%20WiFi');
          document.getElementById('network-name').value = 'CoffeeShop 5G';
          document.getElementById('network-name').dispatchEvent(new dom.window.Event('input'));
          assert.equal(document.getElementById('preview-network-name').textContent, 'CoffeeShop 5G');
          assert.equal(qrPayload(), 'https://omnisolo.co/checkout?product=CoffeeShop%205G');
        } else {
          assert.ok(document.getElementById('previewFrame').getAttribute('src'));
          for (const [id, value] of Object.entries({ itemName: 'Double Smash Burger', itemDesc: 'Extra cheese, extra smash.', accessCode: 'SMASHX2', sharesReq: '4' })) {
            document.getElementById(id).value = value;
            document.getElementById(id).dispatchEvent(new dom.window.Event('input'));
          }
          const preview = new URL(document.getElementById('previewFrame').src);
          assert.equal(preview.searchParams.get('item_name'), 'Double Smash Burger');
          assert.equal(preview.searchParams.get('item_desc'), 'Extra cheese, extra smash.');
          assert.equal(preview.searchParams.get('access_code'), 'SMASHX2');
          assert.equal(preview.searchParams.get('shares_req'), '4');
          assert.equal(document.getElementById('shareLink').innerText, preview.href);
        }
      } finally { dom.window.close(); }
    });
  }
}
