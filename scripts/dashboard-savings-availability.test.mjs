import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';

const files = [
  'src/ui/next/public/dashboard.html',
  'src/ui/next/public/ui/dashboard.html',
  'src/ui/tauri/src/ui/dashboard.html',
];
const turn = () => new Promise(resolve => setImmediate(resolve));
async function load(file, reply) {
  const requests = [];
  const dom = new JSDOM(await readFile(file, 'utf8'), {
    url: 'https://app.example.test/dashboard.html', runScripts: 'dangerously',
    virtualConsole: new VirtualConsole(),
    beforeParse(window) {
      window.Headers = Headers;
      window.Response = Response;
      window.alert = () => {};
      window.matchMedia = () => ({ matches: false, addEventListener() {}, removeEventListener() {} });
      window.fetch = async (url, options) => {
        requests.push({ url: String(url), options });
        if (String(url).endsWith('/growth/time-savings')) return reply();
        return Response.json({});
      };
    },
  });
  if (dom.window.document.readyState === 'loading') await new Promise(resolve => dom.window.document.addEventListener('DOMContentLoaded', resolve, { once: true }));
  await turn(); await turn();
  assert.ok(requests.some(request => request.url.endsWith('/growth/time-savings')), 'the actual dashboard savings reader must execute');
  return { dom, document: dom.window.document, requests };
}

for (const file of files) {
  test(`${file}: a valid server estimate remains visible without assuming missing counters`, async () => {
    const { dom, document } = await load(file, () => Response.json({ hours_saved: 1.75, inquiries_handled: 3, appointments_scheduled: 2 }));
    try {
      const widget = document.getElementById('ai-savings-widget');
      assert.notEqual(widget.parentElement.style.display, 'none');
      assert.equal(document.getElementById('ai-savings-title').textContent, 'Recorded estimate: 1.75 hours saved.');
      assert.equal(document.getElementById('ai-savings-desc').textContent, 'Reported counters: 3 customer inquiries; 2 appointments; not reported recovered carts.');
      assert.equal(document.getElementById('ai-savings-success-widget'), null);
      assert.equal(document.getElementById('share-savings-btn'), null);
    } finally { dom.window.close(); }
  });
  for (const [scenario, reply] of [
    ['unavailable501', () => Response.json({ success: false, code: 'capability_unavailable' }, { status: 501 })],
    ['server failure', () => Response.json({ error: 'unavailable' }, { status: 503 })],
    ['network failure', () => { throw new Error('offline'); }],
    ['malformed data', () => Response.json({ hours_saved: '12' })],
    ['nonterminal response', () => Response.json({ hours_saved: 12 }, { status: 202 })],
  ]) {
    test(`${file}: ${scenario} leaves the dashboard controls visible and reports unknown savings`, async () => {
      const { dom, document, requests } = await load(file, reply);
      try {
        const widget = document.getElementById('ai-savings-widget');
        const container = widget.parentElement;
        assert.ok(container.querySelectorAll('button,a,input').length > 100, 'regression must cover the real full dashboard container');
        assert.notEqual(container.style.display, 'none', 'a savings error must never hide the dashboard container');
        for (const control of [document.getElementById('generate-cloud-bridge-btn'), document.getElementById('cloud-bridge-email'), document.querySelector('a[href="booking-dashboard.html"]')]) {
          assert.ok(control, 'the existing Cloud Bridge and booking controls remain mounted');
          for (let node = control; node; node = node.parentElement) assert.notEqual(node.style.display, 'none', 'unrelated dashboard controls must remain visible');
        }
        assert.equal(widget.style.display, '');
        assert.equal(document.getElementById('ai-savings-title').textContent, 'Recorded time savings');
        assert.equal(document.getElementById('ai-savings-desc').textContent, 'Recorded time-savings data is unavailable.');
        assert.equal(widget.querySelector('a').getAttribute('href'), '/trial-extension');
        assert.doesNotMatch(widget.textContent, /You saved 0|7 Days Pro|Trial Extended|undefined/);
        assert.equal(requests.some(request => request.options?.method === 'POST' && request.url.includes('trial-extension')), false);
      } finally { dom.window.close(); }
    });
  }
}
