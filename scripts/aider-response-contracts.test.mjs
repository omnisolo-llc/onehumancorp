import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';
const roots = ['src/ui/tauri/src/ui', 'src/ui/next/public', 'src/ui/next/public/ui'];
async function load(root, response) {
  const dom = new JSDOM(await readFile(new URL(`../${root}/aider.html`, import.meta.url), 'utf8'), {
    url: 'https://workspace.example/aider.html', runScripts: 'dangerously', virtualConsole: new VirtualConsole(),
    beforeParse(window) { window.fetch = async () => response; },
  });
  dom.window.document.getElementById('generate-btn').click();
  for (let i = 0; i < 50 && dom.window.document.getElementById('generate-btn').disabled; i++) await new Promise(resolve => setTimeout(resolve, 2));
  return dom;
}
for (const root of roots) {
  test(`${root}: preserves the actual unconfigured runtime prerequisite`, async () => {
    const dom = await load(root, Response.json({ error: 'Agent runtime is not configured; no work was dispatched' }, { status: 503 }));
    try {
      assert.match(dom.window.document.getElementById('error-msg').textContent, /Agent runtime is not configured; no work was dispatched/);
      assert.equal(dom.window.document.getElementById('result-container').style.display, 'none');
    } finally { dom.window.close(); }
  });
  for (const [name, status, body] of [
    ['HTTP failure with forged result', 502, { result: 'False success' }],
    ['missing RPC acknowledgement', 200, { result: 'False success' }],
    ['wrong request identity', 200, { jsonrpc: '2.0', id: 'other', result: 'False success' }],
    ['empty map', 200, { jsonrpc: '2.0', id: 'aider-1', result: '' }],
  ]) test(`${root}: rejects ${name}`, async () => {
    const dom = await load(root, Response.json(body, { status }));
    try {
      assert.equal(dom.window.document.getElementById('result-container').style.display, 'none');
      assert.equal(dom.window.document.getElementById('error-msg').style.display, 'block');
    } finally { dom.window.close(); }
  });
  test(`${root}: renders only the confirmed RPC map`, async () => {
    const dom = await load(root, Response.json({ jsonrpc: '2.0', id: 'aider-1', result: 'src\n  main.rs\n    pub fn main()' }));
    try {
      assert.match(dom.window.document.getElementById('result-content').textContent, /main.rs/);
      assert.equal(dom.window.document.getElementById('result-container').style.display, 'block');
    } finally { dom.window.close(); }
  });
}
