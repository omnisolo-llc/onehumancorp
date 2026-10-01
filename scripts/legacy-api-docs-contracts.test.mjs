import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { JSDOM, VirtualConsole } from 'jsdom';

const roots = ['src/ui/tauri/src/ui', 'src/ui/next/public', 'src/ui/next/public/ui', 'src/ui/next/public/api/ui', 'src/ui/next/public/api/v1/ui'];
for (const root of roots) {
  test(`${root}: API reference has one functional tooltip controller`, async () => {
    const html = await readFile(new URL(`../${root}/api-docs.html`, import.meta.url), 'utf8');
    const errors = [];
    const console = new VirtualConsole();
    console.on('jsdomError', error => errors.push(error));
    const dom = new JSDOM(html, { url: 'https://workspace.example/api-docs.html', runScripts: 'outside-only', pretendToBeVisual: true, virtualConsole: console });
    try {
      // Exercise the real inline tooltip components. Swagger and its network
      // bundle are outside this DOM contract and are not replaced or certified.
      dom.window.fetch = async () => ({ ok: true, json: async () => ({}) });
      const scripts = [...dom.window.document.querySelectorAll('script:not([src])')]
        .map(script => script.textContent)
        .filter(source => source.includes("const tooltipEl = document.createElement('div')"));
      assert.ok(scripts.length > 0);
      const loaded = new Promise(resolve => dom.window.addEventListener('load', resolve, { once: true }));
      for (const source of scripts) dom.window.eval(source);
      await loaded;
      const doc = dom.window.document;
      assert.equal(doc.querySelectorAll('.omnisolo-tooltip').length, 1);
      dom.window.initOmniSoloTooltips();
      assert.equal(doc.querySelectorAll('.omnisolo-tooltip').length, 1, 'Repeated initialization must not duplicate listeners or surfaces');
      const target = doc.getElementById('api-docs-tooltip');
      target.dispatchEvent(new dom.window.MouseEvent('mouseover', { bubbles: true }));
      const visible = doc.querySelectorAll('.omnisolo-tooltip.visible');
      assert.equal(visible.length, 1);
      assert.equal(visible[0].textContent, 'Direct API access is only for custom integrations.');
      target.dispatchEvent(new dom.window.MouseEvent('mouseout', { bubbles: true }));
      await new Promise(resolve => dom.window.setTimeout(resolve, 120));
      assert.equal(doc.querySelectorAll('.omnisolo-tooltip.visible').length, 0);
      assert.deepEqual(errors, []);
    } finally { dom.window.close(); }
  });
}
