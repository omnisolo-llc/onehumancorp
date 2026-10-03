import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { JSDOM, VirtualConsole } from 'jsdom';
import { build } from 'esbuild';

const roots = ['src/ui/tauri/src/ui', 'src/ui/next/public', 'src/ui/next/public/ui', 'src/ui/next/public/api/ui', 'src/ui/next/public/api/v1/ui'];
for (const root of roots) {
 for (const moduleFirst of [false, true]) {
  test(`${root}: API reference and actual Help module share one tooltip, module first=${moduleFirst}`, async () => {
    const html = await readFile(new URL(`../${root}/api-docs.html`, import.meta.url), 'utf8');
    const errors = [];
    const console = new VirtualConsole();
    console.on('jsdomError', error => errors.push(error));
    const assetRoot = root.startsWith('src/ui/next/public') ? 'src/ui/next/public' : 'src/ui/tauri/src/ui';
    const pagePath = `${root.slice(assetRoot.length)}/api-docs.html`;
    const dom = new JSDOM(html, { url: `https://workspace.example${pagePath}`, runScripts: 'outside-only', pretendToBeVisual: true, virtualConsole: console });
    const loaded = new Promise(resolve => dom.window.addEventListener('load', resolve, { once: true }));
    try {
      const module = dom.window.document.querySelector('script[type="module"][src$="help-widget.mjs"]');
      assert.ok(module, 'Use the module URL actually declared by the served page');
      const modulePath = new URL(module.src).pathname;
      const bundled = await build({ entryPoints: [fileURLToPath(new URL(`../${assetRoot}${modulePath}`, import.meta.url))], bundle: true, write: false, format: 'iife', platform: 'browser' });
      // Execute the actual imported Help module too. Swagger and browser
      // networking remain outside this DOM contract.
      dom.window.fetch = async () => ({ ok: true, json: async () => ({}) });
      const scripts = [...dom.window.document.querySelectorAll('script:not([src])')]
        .map(script => script.textContent)
        .filter(source => source.includes("const tooltipEl = document.createElement('div')"));
      assert.ok(scripts.length > 0);
      await loaded;
      if (moduleFirst) dom.window.eval(bundled.outputFiles[0].text);
      for (const source of scripts) dom.window.eval(source);
      if (!moduleFirst) dom.window.eval(bundled.outputFiles[0].text);
      dom.window.document.dispatchEvent(new dom.window.Event('DOMContentLoaded'));
      const doc = dom.window.document;
      assert.equal(doc.querySelectorAll('.omnisolo-tooltip').length, 1);
      dom.window.initOmniSoloTooltips();
      assert.equal(doc.querySelectorAll('.omnisolo-tooltip').length, 1, 'Repeated initialization must not duplicate listeners or surfaces');
      const target = doc.getElementById('api-docs-tooltip');
      target.setAttribute('aria-describedby', 'existing-description');
      target.dispatchEvent(new dom.window.MouseEvent('mouseover', { bubbles: true }));
      const visible = doc.querySelectorAll('.omnisolo-tooltip.visible');
      assert.equal(visible.length, 1);
      assert.equal(visible[0].textContent, 'Direct API access is only for custom integrations.');
      assert.equal(visible[0].getAttribute('role'), 'tooltip');
      assert.equal(visible[0].getAttribute('aria-hidden'), 'false');
      assert.equal(target.getAttribute('aria-describedby'), `existing-description ${visible[0].id}`);
      target.dispatchEvent(new dom.window.MouseEvent('mouseout', { bubbles: true }));
      await new Promise(resolve => dom.window.setTimeout(resolve, 120));
      assert.equal(doc.querySelectorAll('.omnisolo-tooltip.visible').length, 0);
      assert.equal(target.getAttribute('aria-describedby'), 'existing-description');
      assert.equal(target.tabIndex, 0, 'The API explanation is reachable by keyboard');
      target.focus();
      assert.equal(doc.querySelectorAll('.omnisolo-tooltip.visible').length, 1);
      target.dispatchEvent(new dom.window.KeyboardEvent('keydown', { key: 'Escape', bubbles: true }));
      assert.equal(doc.querySelectorAll('.omnisolo-tooltip.visible').length, 0);
      assert.equal(doc.activeElement, target);
      target.blur(); target.focus();
      assert.equal(doc.querySelectorAll('.omnisolo-tooltip.visible').length, 1);
      target.blur();
      await new Promise(resolve => dom.window.setTimeout(resolve, 120));
      assert.equal(doc.querySelectorAll('.omnisolo-tooltip.visible').length, 0);
      const launcher = doc.getElementById('ohc-floating-help-btn');
      assert.ok(launcher, 'Sharing tooltip ownership must retain the actual Help widget');
      launcher.click();
      assert.equal(doc.getElementById('ohc-floating-help-widget').style.display, 'flex');
      assert.deepEqual(errors, []);
    } finally { dom.window.close(); }
  });
 }
}
