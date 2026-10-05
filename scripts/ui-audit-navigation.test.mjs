import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import vm from 'node:vm';
import { EventEmitter } from 'node:events';
import { createRequire } from 'node:module';
import { setImmediate as nextTurn } from 'node:timers/promises';
import { JSDOM } from './test-support/offline-dom.mjs';
import ts from 'typescript';

const filename = path.resolve('src/e2e/support/ui_audit_navigation.ts');
const compiled = ts.transpileModule(fs.readFileSync(filename, 'utf8'), {
  compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
}).outputText;
const exported = {};
vm.runInNewContext(compiled, {
  exports: exported, require: createRequire(filename), URL, setTimeout, clearTimeout,
}, { filename });
const { createAuditNavigation } = exported;
const origin = 'http://127.0.0.1:44041';
const spec = { openapi: '3.0.0', paths: {
  '/api/v1/one': { parameters: [], get: { tags: ['First'] } },
  '/api/v1/two': { post: { tags: ['First', 'Second'] } },
} };
const operations = [['First', 'GET', '/api/v1/one'], ['First', 'POST', '/api/v1/two'], ['Second', 'POST', '/api/v1/two']];

// Execute the maintained navigation helper and its browser predicates against
// actual DOM fixtures. The Page boundary controls source and renderer completion
// independently, without requiring a backend or substituting an E2E response.
function browserBoundary() {
  const dom = new JSDOM('<body><button>Voice Assistant</button></body>', { url: `${origin}/api-docs`, runScripts: 'outside-only' });
  const context = new EventEmitter();
  const page = new EventEmitter();
  const checks = new Set();
  let currentUrl = 'about:blank';
  let navigations = 0;
  Object.assign(page, {
    context: () => context,
    url: () => currentUrl,
    goto: async url => { currentUrl = url; navigations += 1; return { status: () => 200 }; },
    waitForLoadState: async () => {},
    waitForTimeout: async () => {},
    waitForFunction: (predicate, argument) => new Promise((resolve, reject) => {
      const evaluate = vm.runInContext(`(${predicate.toString()})`, dom.getInternalVMContext());
      const check = { run: () => {
        try { if (evaluate(argument)) { checks.delete(check); resolve({ dispose: async () => {} }); } }
        catch (error) { checks.delete(check); reject(error); }
      }, reject };
      checks.add(check); check.run();
    }),
  });
  const flush = () => { for (const check of [...checks]) check.run(); };
  const render = rows => {
    dom.window.document.body.innerHTML = '<button>Voice Assistant</button><div class="swagger-ui"></div>';
    const root = dom.window.document.querySelector('.swagger-ui');
    for (const [tag, method, route] of rows) {
      const section = dom.window.document.createElement('section');
      section.className = 'opblock-tag-section';
      const heading = dom.window.document.createElement('h3');
      heading.className = 'opblock-tag'; heading.dataset.tag = tag;
      const button = dom.window.document.createElement('button');
      button.className = 'opblock-summary-control';
      const methodLabel = dom.window.document.createElement('span');
      methodLabel.className = 'opblock-summary-method'; methodLabel.textContent = method;
      const pathLabel = dom.window.document.createElement('span'); pathLabel.dataset.path = route;
      button.append(methodLabel, pathLabel); section.append(heading, button); root.append(section);
    }
    flush();
  };
  const respond = (value = spec, status = 200, url = `${origin}/api/v1/api-docs-spec`) => {
    const response = { url: () => url, request: () => ({ method: () => 'GET' }), status: () => status, json: async () => value };
    page.emit('response', response); context.emit('response', response);
  };
  const begin = () => {
    let settled = false;
    const result = createAuditNavigation(origin, async () => {})(page, '/api-docs');
    result.then(() => { settled = true; }, () => { settled = true; });
    return { result, settled: () => settled };
  };
  return { page, begin, render, respond, moveTo: url => { currentUrl = url; }, navigations: () => navigations, close: () => {
    for (const check of checks) check.reject(new Error('Test boundary closed'));
    checks.clear(); dom.window.close();
  } };
}

test('documentation navigation waits for the fetched source and every rendered operation before discovery', async t => {
  const browser = browserBoundary(); t.after(browser.close);
  const pending = browser.begin();
  await nextTurn();
  assert.equal(pending.settled(), false, 'the loading skeleton must not become the click baseline');
  browser.render([]); browser.respond(spec, 200, 'https://unrelated.invalid/api/v1/api-docs-spec');
  await nextTurn();
  assert.equal(pending.settled(), false, 'a Swagger shell and foreign response do not establish readiness');
  browser.respond(); await nextTurn();
  assert.equal(pending.settled(), false, 'the source response does not prove Swagger finished rendering');
  browser.render(operations.slice(0, 1)); await nextTurn();
  assert.equal(pending.settled(), false, 'partial operation rendering must remain held');
  browser.render([operations[0], operations[1], ['Second', 'POST', '/wrong']]); await nextTurn();
  assert.equal(pending.settled(), false, 'equal counts cannot substitute for the actual operation identities');
  browser.render([...operations].reverse());
  const receipt = await pending.result;
  assert.equal(receipt.finalUrl, `${origin}/api-docs`);
  assert.equal(browser.navigations(), 1, 'readiness must not retry a navigation or any click');
  assert.equal(browser.page.listenerCount('response'), 0, 'source observation ends when navigation is ready');
});

test('documentation readiness is required again after each document reset', async t => {
  const browser = browserBoundary(); t.after(browser.close);
  for (let index = 0; index < 2; index += 1) {
    browser.render([]);
    const pending = browser.begin(); await nextTurn(); browser.respond(); await nextTurn();
    assert.equal(pending.settled(), false, `document ${index + 1} must render its own complete operation set`);
    browser.render(operations); await pending.result;
  }
  assert.equal(browser.navigations(), 2);
});

test('documentation readiness cannot conceal a navigation to an unclassified document', async t => {
  const browser = browserBoundary(); t.after(browser.close);
  const pending = browser.begin(); await nextTurn();
  browser.moveTo(`${origin}/wrong`);
  const failed = assert.rejects(pending.result, /destination/i); failed.catch(() => {});
  browser.respond(); browser.render(operations); await failed;
  assert.equal(browser.page.listenerCount('response'), 0);
});

for (const [label, status, source] of [
  ['failed source response', 503, { error: 'unavailable' }],
  ['empty operation source', 200, { openapi: '3.0.0', paths: {} }],
  ['malformed operation source', 200, { openapi: '3.0.0', paths: { '/bad': { get: null } } }],
]) {
  test(`documentation navigation rejects a ${label} before discovery`, async t => {
    const browser = browserBoundary(); t.after(browser.close);
    const pending = browser.begin();
    const failed = assert.rejects(pending.result, /documentation|operation|spec/i);
    failed.catch(() => {});
    await nextTurn(); browser.respond(source, status); await failed;
    assert.equal(browser.navigations(), 1);
    assert.equal(browser.page.listenerCount('response'), 0);
  });
}
