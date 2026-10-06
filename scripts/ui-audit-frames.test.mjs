import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import vm from 'node:vm';
import ts from 'typescript';
import { JSDOM } from './test-support/offline-dom.mjs';
function load(filename) {
  const exports = {};
  vm.runInNewContext(ts.transpileModule(fs.readFileSync(filename, 'utf8'), { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 } }).outputText,
    { exports, require: name => load(path.resolve(path.dirname(filename), name + '.ts')), URL, setTimeout, clearTimeout }, { filename });
  return exports;
}
const audit = load(path.resolve('src/e2e/support/ui_click_audit.ts'));
const documents = load(path.resolve('src/e2e/support/ui_audit_documents.ts'));
function fixture() {
  const dom = new JSDOM('<button>Parent control</button><iframe data-ohc-api-docs-viewer="true" src="/api-docs/viewer"></iframe>', { url: 'https://app.test/api-docs', runScripts: 'outside-only' });
  const frame = dom.window.document.querySelector('iframe');
  const child = frame.contentWindow;
  child.document.open(); child.document.write('<body><button>Embedded operation</button><input required><a href="#details">Details</a><div id="details">Detail content</div></body>'); child.document.close();
  for (const win of [dom.window, child]) win.HTMLElement.prototype.getBoundingClientRect = () => ({ x: 0, y: 0, left: 0, top: 0, right: 100, bottom: 30, width: 100, height: 30 });
  const invoke = (fn, ...args) => vm.runInContext(`(${fn.toString()})`, dom.getInternalVMContext())(...args);
  const unwrap = value => value?._value ?? value;
  const handle = value => ({ _value: value,
    evaluate: async (fn, arg) => invoke(fn, value, unwrap(arg)),
    evaluateHandle: async (fn, arg) => handle(invoke(fn, value, unwrap(arg))),
    getProperties: async () => new Map(Object.entries(value).map(([key, entry]) => [key, handle(entry)])),
    asElement: () => value?.nodeType === 1 ? handle(value) : null,
    dispose: async () => {},
  });
  const page = { url: () => dom.window.location.href,
    evaluate: async (fn, arg) => invoke(fn, unwrap(arg)),
    evaluateHandle: async (fn, arg) => handle(invoke(fn, unwrap(arg))),
    locator: selector => ({ elementHandles: async () => Array.from(dom.window.document.querySelectorAll(selector.replaceAll(':visible', '')), handle) }),
    frameLocator: () => ({ locator: selector => ({ elementHandles: async () => Array.from(child.document.querySelectorAll(selector.replaceAll(':visible', '')), handle) }) }),
  };
  return { dom, child, frame, page, invoke, close: () => dom.window.close() };
}
test('click discovery retains parent controls and actual embedded operation controls', async t => {
  const f = fixture(); t.after(f.close);
  const targets = await audit.tagClickTargets(f.page);
  assert.deepEqual(Array.from(targets, target => target.label), ['Parent control', 'Embedded operation']);
  assert.equal(new Set(Array.from(targets, target => target.key)).size, 2);
  assert.ok(f.child.document.querySelector('button').hasAttribute('data-ui-audit-click-key'));
});
test('document signatures see iframe-only effects', async t => {
  const f = fixture(); t.after(f.close);
  const before = await audit.auditPageSignature(f.page);
  f.child.document.querySelector('button').textContent = 'Operation opened';
  assert.notEqual(await audit.auditPageSignature(f.page), before);
});
test('an unexpected frame destination fails discovery rather than removing its targets', async t => {
  const f = fixture(); t.after(f.close);
  f.frame.setAttribute('src', 'https://foreign.test/api-docs/viewer');
  await assert.rejects(audit.tagClickTargets(f.page), /frame|origin|destination/i);
});

test('trusted target handles are resolved through the owning frame', async t => {
  const f = fixture(); t.after(f.close);
  const targets = await audit.tagClickTargets(f.page);
  const handles = await documents.auditElementHandles(f.page, `[data-ui-audit-click-index="${targets[1].index}"]`);
  assert.equal(handles.length, 1);
  assert.equal(handles[0]._value.ownerDocument, f.child.document);
});
test('link inventory and fragment validation retain their owning document', async t => {
  const f = fixture(); t.after(f.close);
  const links = await documents.evaluateAuditElements(f.page, 'a[href]', elements => elements.map(element => ({ href: element.getAttribute('href'), embedded: element.ownerDocument !== document })));
  assert.equal(links.length, 1); assert.equal(links[0].embedded, true);
  assert.equal(await f.page.evaluate(audit.hasFragmentTarget, links[0]), true);
  assert.equal(await f.page.evaluate(audit.hasFragmentTarget, { href: '#details', embedded: false }), false);
  f.child.document.querySelector('#details').remove();
  assert.equal(await f.page.evaluate(audit.hasFragmentTarget, links[0]), false);
});

test('layout audit detects parent controls obscuring framed operations in top-page coordinates', t => {
  const f = fixture(); t.after(f.close);
  const rect = (left, top, width, height) => ({ left, top, right: left + width, bottom: top + height, width, height });
  f.dom.window.document.querySelector('button').getBoundingClientRect = () => rect(110, 70, 50, 20);
  f.frame.getBoundingClientRect = () => rect(100, 50, 200, 100);
  for (const [name, value] of Object.entries({ clientWidth: 200, clientHeight: 100, offsetWidth: 200, offsetHeight: 100 })) Object.defineProperty(f.frame, name, { value });
  const operation = f.child.document.querySelector('button');
  operation.getBoundingClientRect = () => rect(10, 20, 50, 20);
  const layouts = () => f.invoke(documents.measureAuditLayouts, [f.dom.window.document, f.child.document], 'button');
  assert.match(layouts()[0].overlaps.join('\n'), /Parent control.*Embedded operation/);
  operation.getBoundingClientRect = () => rect(10, 110, 50, 20);
  f.dom.window.document.querySelector('button').getBoundingClientRect = () => rect(110, 160, 50, 20);
  assert.equal(layouts()[0].overlaps.length, 0, 'a control fully clipped by its frame cannot obscure the parent');
  operation.getBoundingClientRect = () => rect(10, 90, 50, 20);
  f.dom.window.document.querySelector('button').getBoundingClientRect = () => rect(110, 140, 50, 10);
  assert.match(layouts()[0].overlaps.join('\n'), /Parent control.*Embedded operation/, 'partially clipped controls retain visible overlap coverage');
  assert.equal(layouts().length, 2, 'both documents retain individual overflow measurements');
});
