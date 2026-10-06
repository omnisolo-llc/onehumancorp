import test from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import vm from 'node:vm';
import ts from 'typescript';
import { parse } from 'jsonc-parser';
import { JSDOM } from './test-support/offline-dom.mjs';
const source = 'src/ui/next/src/lib/swaggerViewerDocument.ts';
function documentSource() {
  assert.ok(existsSync(source), 'the same-origin Swagger viewer must have a maintained document and runtime');
  const result = { exports: {} };
  vm.runInNewContext(ts.transpileModule(readFileSync(source, 'utf8'), { compilerOptions: { module: ts.ModuleKind.CommonJS } }).outputText, result);
  return result.exports;
}
test('viewer is same-origin and embeds no spec data or remote scripts', () => {
  const { VIEWER_HTML } = documentSource();
  const dom = new JSDOM(VIEWER_HTML, { url: 'https://app.test/api-docs/viewer' });
  try {
    assert.equal(dom.window.document.title, 'Interactive API documentation');
    assert.equal(dom.window.document.querySelector('script[src]').getAttribute('src'), '/vendor/swagger-ui/dist/swagger-ui-bundle.js');
    assert.equal(dom.window.document.querySelector('link[rel=stylesheet]').getAttribute('href'), '/vendor/swagger-ui/dist/swagger-ui.css');
    assert.equal(dom.window.document.querySelectorAll('script[src]').length, 1);
  } finally { dom.window.close(); }
});
test('viewer ignores foreign initialization, renders actual operations, and preserves safe Markdown', async () => {
  const { VIEWER_HTML, VIEWER_SCRIPT } = documentSource();
  const parent = new JSDOM('<iframe src="/api-docs/viewer"></iframe>', { url: 'https://app.test/api-docs', runScripts: 'outside-only' });
  const child = parent.window.document.querySelector('iframe').contentWindow;
  try {
    child.document.open(); child.document.write(VIEWER_HTML); child.document.close();
    child.scrollTo = () => {};
    child.ResizeObserver = class { observe() {} disconnect() {} };
    child.eval(readFileSync('src/ui/next/public/vendor/swagger-ui/dist/swagger-ui-bundle.js', 'utf8'));
    child.eval(VIEWER_SCRIPT);
    const spec = { openapi: '3.0.0', info: { title: 'Real renderer fixture', version: '1', description: '<img src=x onerror="alert(1)"><script>alert(1)</script>' }, paths: { '/api/v1/help': { get: { summary: 'Read help', responses: { '200': { description: 'Help' } } } }, '/api/v1/tooltips': { get: { summary: 'Read tooltips', responses: { '200': { description: 'Tooltips' } } } } } };
    const receive = (origin, sender, data) => child.dispatchEvent(new child.MessageEvent('message', { origin, source: sender, data }));
    receive('https://foreign.test', parent.window, { type: 'ohc-api-docs:init', spec });
    receive('https://app.test', child, { type: 'ohc-api-docs:init', spec });
    assert.equal(child.document.querySelectorAll('.opblock-summary-control').length, 0);
    receive('https://app.test', parent.window, { type: 'ohc-api-docs:init', spec, dark: true });
    const deadline = Date.now() + 5000;
    while (child.document.querySelectorAll('.opblock-summary-control').length !== 2 && Date.now() < deadline) await new Promise(resolve => setTimeout(resolve, 10));
    assert.equal(child.document.querySelectorAll('.opblock-summary-control').length, 2);
    assert.match(child.document.body.textContent, /Read help/);
    assert.match(child.document.body.textContent, /Read tooltips/);
    assert.equal(child.document.querySelector('.info [onerror]'), null);
    assert.equal(child.document.querySelector('.info script'), null);
    assert.equal(child.document.documentElement.classList.contains('dark'), true);
    receive('https://app.test', parent.window, { type: 'ohc-api-docs:theme', dark: false });
    assert.equal(child.document.documentElement.classList.contains('dark'), false);
  } finally { parent.window.close(); }
});

test('the rebuilt viewer renders every operation in the actual maintained server specification', async () => {
  const { VIEWER_HTML, VIEWER_SCRIPT } = documentSource();
  const server = readFileSync('src/server/api/docs.rs', 'utf8').split('pub async fn get_api_docs_spec()', 2)[1];
  const begin = server.indexOf('serde_json::json!(') + 'serde_json::json!('.length;
  const end = server.indexOf('\n    });', begin);
  assert.ok(begin > 0 && end > begin, 'read the maintained literal spec rather than replacing it with a page fixture');
  const errors = [];
  const spec = parse(server.slice(begin, end) + '\n    }', errors, { allowTrailingComma: true });
  assert.deepEqual(errors, []);
  const methods = new Set(['get', 'post', 'put', 'patch', 'delete', 'head', 'options', 'trace']);
  const expected = Object.entries(spec.paths).flatMap(([path, item]) => Object.entries(item).filter(([method]) => methods.has(method)).flatMap(([method, operation]) => [...new Set(operation.tags?.length ? operation.tags : ['default'])].map(tag => JSON.stringify([tag, method.toUpperCase(), path])))).sort();
  assert.ok(expected.length > 0);
  const parent = new JSDOM('<iframe src="/api-docs/viewer"></iframe>', { url: 'https://app.test/api-docs', runScripts: 'outside-only' });
  const child = parent.window.document.querySelector('iframe').contentWindow;
  try {
    child.document.open(); child.document.write(VIEWER_HTML); child.document.close();
    child.ResizeObserver = class { observe() {} disconnect() {} };
    child.scrollTo = () => {};
    child.eval(readFileSync('src/ui/next/public/vendor/swagger-ui/dist/swagger-ui-bundle.js', 'utf8'));
    child.eval(VIEWER_SCRIPT);
    child.dispatchEvent(new child.MessageEvent('message', { origin: 'https://app.test', source: parent.window, data: { type: 'ohc-api-docs:init', spec } }));
    const deadline = Date.now() + 5000;
    while (child.document.querySelectorAll('.opblock-summary-control').length !== expected.length && Date.now() < deadline) await new Promise(resolve => setTimeout(resolve, 10));
    const actual = Array.from(child.document.querySelectorAll('.opblock-summary-control'), element => JSON.stringify([
      element.closest('.opblock-tag-section')?.querySelector('.opblock-tag')?.getAttribute('data-tag'),
      element.querySelector('.opblock-summary-method')?.textContent?.trim(),
      element.querySelector('[data-path]')?.getAttribute('data-path'),
    ])).sort();
    assert.deepEqual(actual, expected);
  } finally { parent.window.close(); }
});

test('actual bundle acknowledges repeated initialization and replacement specifications', async () => {
  const { VIEWER_HTML, VIEWER_SCRIPT } = documentSource();
  const parent = new JSDOM('<iframe src="/api-docs/viewer"></iframe>', { url: 'https://app.test/api-docs', runScripts: 'outside-only' });
  const child = parent.window.document.querySelector('iframe').contentWindow;
  const messages = [];
  parent.window.postMessage = data => messages.push(data);
  const until = async predicate => {
    const deadline = Date.now() + 2000;
    while (!predicate() && Date.now() < deadline) await new Promise(resolve => setTimeout(resolve, 10));
    assert.ok(predicate(), 'the real renderer must acknowledge completion');
  };
  try {
    child.document.open(); child.document.write(VIEWER_HTML); child.document.close();
    child.ResizeObserver = class { observe() {} disconnect() {} };
    child.scrollTo = () => {};
    child.eval(readFileSync('src/ui/next/public/vendor/swagger-ui/dist/swagger-ui-bundle.js', 'utf8'));
    child.eval(VIEWER_SCRIPT);
    const spec = { openapi: '3.0.0', info: { title: 'First source', version: '1' }, paths: { '/first': { get: { responses: { '200': { description: 'First' } } } } } };
    const initialize = value => child.dispatchEvent(new child.MessageEvent('message', { origin: 'https://app.test', source: parent.window, data: { type: 'ohc-api-docs:init', spec: value } }));
    initialize(spec);
    await until(() => messages.filter(message => message.type === 'ohc-api-docs:ready').length === 1);
    initialize(spec);
    await until(() => messages.filter(message => message.type === 'ohc-api-docs:ready').length === 2);
    initialize({ ...spec, info: { title: 'Replacement source', version: '2' }, paths: { '/replacement': spec.paths['/first'] } });
    await until(() => messages.filter(message => message.type === 'ohc-api-docs:ready').length === 3);
    await until(() => child.document.querySelector('[data-path]')?.getAttribute('data-path') === '/replacement');
    assert.equal(child.document.querySelectorAll('.swagger-ui').length, 1);
    assert.equal(child.document.querySelectorAll('.opblock-summary-control').length, 1);
  } finally { parent.window.close(); }
});
