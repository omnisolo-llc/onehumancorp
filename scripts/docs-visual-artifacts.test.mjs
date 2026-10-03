import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import vm from 'node:vm';
import ts from 'typescript';

const spec = new URL('../src/e2e/docs_visual_audit.spec.ts', import.meta.url);
const captures = [
  { width: 375, height: 800 },
  { width: 768, height: 1024 },
  { width: 1440, height: 900 },
];
async function registeredCallback() {
  const source = await readFile(spec, 'utf8');
  const code = ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.CommonJS } }).outputText;
  const registrations = [];
  vm.runInNewContext(code, {
    exports: {},
    require(name) {
      assert.equal(name, './fixtures');
      return { test: (name, callback) => registrations.push({ name, callback }) };
    },
  }, { filename: spec.pathname });
  assert.deepEqual(registrations.map(value => value.name), ['Generate visual screenshots for User Guide']);
  return registrations[0].callback;
}

test('the maintained visual audit keeps all captures in per-test artifacts without changing documentation', async () => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'ohc-visual-artifacts-'));
  try {
    const docs = path.join(root, 'docs/app');
    await mkdir(docs, { recursive: true });
    for (const viewport of captures) await writeFile(path.join(docs, `ux_audit_${viewport.width}.png`), 'tracked documentation sentinel');
    const callback = await registeredCallback();
    for (const attempt of ['test-worker-one', 'test-worker-two']) {
      const output = path.join(root, 'test-results', attempt);
      const events = [], paths = [];
      // Output-path contract only: this boundary writes fixture bytes and does
      // not claim browser screenshot execution or visual acceptance.
      await callback({ page: {
        goto: async route => { events.push(['goto', route]); },
        waitForLoadState: async state => { events.push(['load', state]); },
        waitForTimeout: async timeout => { events.push(['wait', timeout]); },
        setViewportSize: async viewport => { events.push(['viewport', JSON.parse(JSON.stringify(viewport))]); },
        screenshot: async options => {
          const destination = path.resolve(root, options.path);
          await mkdir(path.dirname(destination), { recursive: true });
          await writeFile(destination, 'captured fixture bytes');
          paths.push(destination);
        },
      } }, { outputPath: name => path.join(output, name) });
      assert.deepEqual(events, [
        ['goto', '/dashboard'], ['load', 'domcontentloaded'], ['wait', 5000],
        ...captures.map(viewport => ['viewport', viewport]),
      ]);
      assert.deepEqual(paths, captures.map(viewport => path.join(output, `ux_audit_${viewport.width}.png`)),
        'routine browser execution must use its per-test output directory, never tracked docs');
      for (const destination of paths) assert.equal(await readFile(destination, 'utf8'), 'captured fixture bytes');
    }
    for (const viewport of captures) assert.equal(await readFile(path.join(docs, `ux_audit_${viewport.width}.png`), 'utf8'), 'tracked documentation sentinel');
  } finally { await rm(root, { recursive: true, force: true }); }
});
