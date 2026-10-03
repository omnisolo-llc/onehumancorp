import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';

const roots = ['src/ui/next/public', 'src/ui/next/public/ui', 'src/ui/tauri/src/ui'];
const fields = ['tenant', 'cert-title', 'recipient', 'course-name', 'theme'];
const values = { tenant: 'e2e-demo-tenant', 'cert-title': 'Certificate of Achievement', recipient: 'John E2E Doe', 'course-name': 'E2E Mastery Course', theme: 'gold' };
async function delayedEditor(root, storageUnavailable = false) {
  const errors = []; const console = new VirtualConsole(); console.on('jsdomError', error => errors.push(error.message));
  const html = await readFile(new URL(`../${root}/viral-certificate-generator.html`, import.meta.url), 'utf8');
  // A preceding linked stylesheet can defer the script while the rendered
  // form is already visible. Model that lifecycle without external requests.
  const dom = new JSDOM(html.replaceAll('<script>', '<script type="text/plain">'), { url: 'http://127.0.0.1:39151/viral-certificate-generator.html', runScripts: 'dangerously', virtualConsole: console, beforeParse(window) { if (storageUnavailable) window.Storage.prototype.getItem = () => { throw new Error('storage denied'); }; } });
  return { dom, errors, initialize: () => { for (const script of dom.window.document.querySelectorAll('script:not([src])')) dom.window.eval(script.textContent); } };
}
for (const root of roots) {
  test(`${root}: certificate controls are held until preview listeners are initialized`, async () => {
    const { dom, errors, initialize } = await delayedEditor(root);
    try {
      const document = dom.window.document;
      for (const id of [...fields, 'generate-btn', 'copy-btn']) assert.equal(document.getElementById(id).disabled, true, `${id} before script`);
      initialize();
      for (const id of [...fields, 'generate-btn', 'copy-btn']) assert.equal(document.getElementById(id).disabled, false, `${id} after script`);
      assert.deepEqual(errors, []);
    } finally { dom.window.close(); }
  });
  test(`${root}: initialization preserves current field values in the actual preview and generated embed`, async () => {
    const { dom, errors, initialize } = await delayedEditor(root);
    try {
      const document = dom.window.document;
      // Restored/autofilled values can exist before event listeners. Rendering
      // the preview must not rely on receiving a later synthetic input event.
      for (const [id, value] of Object.entries(values)) document.getElementById(id).value = value;
      initialize();
      const url = new URL(document.getElementById('preview-frame').src);
      assert.equal(url.origin, dom.window.location.origin);
      assert.equal(url.pathname, '/api/v1/growth/certificate/embed');
      assert.deepEqual(Object.fromEntries(url.searchParams), { tenant: values.tenant, title: values['cert-title'], recipient: values.recipient, course: values['course-name'], theme: values.theme });
      document.getElementById('generate-btn').click();
      const embed = document.getElementById('embed-code').innerText;
      assert.equal(new URL(embed.match(/src="([^"]+)"/)[1]).href, url.href);
      assert.deepEqual(errors, []);
    } finally { dom.window.close(); }
  });
}

for (const root of roots) {
  test(`${root}: unavailable saved settings do not disable the editable certificate`, async () => {
    const { dom, errors, initialize } = await delayedEditor(root, true);
    try {
      const document = dom.window.document;
      for (const [id, value] of Object.entries(values)) document.getElementById(id).value = value;
      assert.doesNotThrow(initialize);
      for (const id of [...fields, 'generate-btn', 'copy-btn']) assert.equal(document.getElementById(id).disabled, false);
      const url = new URL(document.getElementById('preview-frame').src);
      assert.equal(url.searchParams.get('tenant'), values.tenant);
      assert.equal(url.searchParams.get('recipient'), values.recipient);
      assert.match(document.querySelector('[role="status"]').textContent, /Saved business settings are unavailable/);
      assert.deepEqual(errors, []);
    } finally { dom.window.close(); }
  });
}
