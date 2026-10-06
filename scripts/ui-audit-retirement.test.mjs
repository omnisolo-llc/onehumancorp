import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import vm from 'node:vm';
import ts from 'typescript';

// Load the entire production module and its real local dependencies. Only the
// Playwright transport boundary is simulated; retirement logic is not copied.
function load(filename) {
  const exports = {};
  vm.runInNewContext(ts.transpileModule(fs.readFileSync(filename, 'utf8'), {
    compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
  }).outputText, {
    exports, require: name => load(path.resolve(path.dirname(filename), name + '.ts')),
    URL, Date, Error, setTimeout, clearTimeout,
  }, { filename });
  return exports;
}
const { replaceAuditDocument } = load(path.resolve('src/e2e/support/ui_click_audit.ts'));
const origin = 'http://127.0.0.1:38357';
const interruption = destination => new Error(`page.goto: Navigation to "about:blank" is interrupted by another navigation to "${destination}"`);
function fixture() {
  const events = [];
  let current = `${origin}/link-in-bio-generator`, closed = false;
  const context = { newPage: async () => { events.push('new-page'); throw new Error('Late replacement must not be allocated'); } };
  const page = {
    url: () => current,
    viewportSize: () => ({ width: 1280, height: 720 }),
    isClosed: () => closed,
    context: () => context,
    goto: async (url, options) => { events.push(['goto', url, options]); current = url; },
    waitForURL: async (url, options) => { events.push(['wait', url, options]); current = url; },
    // Hosted Chromium hung for 127553ms here while recording a page whose
    // navigation had just been interrupted. Never enter that close path.
    close: async () => { events.push('close'); closed = true; throw new Error('Recorded-page close stalled'); },
  };
  return { page, events, close: () => { closed = true; } };
}

test('normal retirement commits a fresh document on the same recorded page within a finite budget', async () => {
  const f = fixture();
  assert.equal(await replaceAuditDocument(f.page), f.page);
  assert.equal(f.page.url(), 'about:blank');
  assert.equal(f.events.length, 1);
  assert.equal(f.events[0][2].waitUntil, 'commit');
  assert.ok(f.events[0][2].timeout > 0 && f.events[0][2].timeout <= 10000);
});

test('classified logout interruption settles then retires its realm without closing or replacing the page', async () => {
  const f = fixture();
  const goto = f.page.goto;
  let attempts = 0;
  f.page.goto = async (...args) => {
    if (++attempts === 1) { f.events.push('logout-interruption'); throw interruption(`${origin}/login`); }
    return goto(...args);
  };
  assert.equal(await replaceAuditDocument(f.page), f.page);
  assert.equal(attempts, 2);
  assert.equal(f.page.isClosed(), false);
  assert.equal(f.page.url(), 'about:blank');
  assert.deepEqual(f.events.map(event => Array.isArray(event) ? event.slice(0, 2) : event), [
    'logout-interruption', ['wait', `${origin}/login`], ['goto', 'about:blank'],
  ]);
  assert.equal(f.events[1][2].waitUntil, 'commit');
});

test('all retirement phases share one deadline rather than extending the route budget', async t => {
  let now = 0;
  t.mock.method(Date, 'now', () => now);
  const f = fixture();
  const goto = f.page.goto;
  let attempts = 0;
  f.page.goto = async (...args) => {
    if (++attempts === 1) { now += 4000; throw interruption(`${origin}/login`); }
    return goto(...args);
  };
  f.page.waitForURL = async (_url, options) => { assert.equal(options.timeout, 6000); now += 3000; };
  await replaceAuditDocument(f.page);
  assert.equal(f.events[0][2].timeout, 3000);
});

test('deadline expiry after logout settling cannot start another navigation or allocate a late page', async t => {
  let now = 0;
  t.mock.method(Date, 'now', () => now);
  const f = fixture();
  let attempts = 0;
  f.page.goto = async () => { attempts += 1; throw interruption(`${origin}/login`); };
  f.page.waitForURL = async () => { now = 10001; };
  await assert.rejects(replaceAuditDocument(f.page), /retirement.*deadline/i);
  assert.equal(attempts, 1);
  assert.deepEqual(f.events, []);
});

test('cancellation while logout settles does not resume retirement or create a late page', async () => {
  const f = fixture();
  let settle;
  let attempts = 0;
  f.page.goto = async () => { attempts += 1; throw interruption(`${origin}/login`); };
  f.page.waitForURL = () => new Promise(resolve => { settle = resolve; });
  const result = assert.rejects(replaceAuditDocument(f.page), /closed|cancel/i);
  await new Promise(resolve => setImmediate(resolve));
  f.close();
  settle();
  await result;
  assert.equal(attempts, 1);
  assert.deepEqual(f.events, []);
});

test('a rejected logout wait remains the original failure and never starts late work', async () => {
  const f = fixture();
  const failure = new Error('Target page, context or browser has been closed');
  f.page.goto = async () => { throw interruption(`${origin}/login`); };
  f.page.waitForURL = async () => { throw failure; };
  await assert.rejects(replaceAuditDocument(f.page), error => error === failure);
  assert.deepEqual(f.events, []);
});

test('unclassified failures retain their original error without resetting or replacing anything', async () => {
  for (const failure of [
    interruption('https://other.invalid/login'), interruption(`${origin}/settings`),
    interruption(`${origin}/login?next=/private`), interruption(`${origin}/login#fragment`),
    new Error('page.goto: net::ERR_ABORTED at about:blank'),
    new Error('page.goto: Timeout 5000ms exceeded'),
    new Error('Target page, context or browser has been closed'),
    interruption(`${origin}/login`).message,
  ]) {
    const f = fixture();
    f.page.goto = async () => { throw failure; };
    await assert.rejects(replaceAuditDocument(f.page), error => error === failure);
    assert.deepEqual(f.events, []);
  }
});

test('a second interruption propagates without a retry loop or replacement allocation', async () => {
  const f = fixture();
  let attempts = 0;
  const failure = interruption(`${origin}/login`);
  f.page.goto = async () => { attempts += 1; throw failure; };
  await assert.rejects(replaceAuditDocument(f.page), error => error === failure);
  assert.equal(attempts, 2);
  assert.equal(f.events.length, 1);
  assert.equal(f.events[0][0], 'wait');
});
