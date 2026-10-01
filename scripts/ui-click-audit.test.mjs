import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createRequire } from 'node:module';
import { execFileSync } from 'node:child_process';
const require = createRequire(import.meta.url);
const protocol = require('./ui-click-audit.cjs');
const { PROTOCOL, ATTACHMENT, CLICK_TITLE, INVENTORY_TITLE, PURPOSE_TITLE, GLOBAL_TITLES, CONTRACT_FILE, validateReceipts, makeRunContext, readReceipts } = protocol;
const effect = { changed: true, requestSeen: false, downloadSeen: false, fileChooserSeen: false, popupSeen: false, validationSeen: false, dialogSeen: false, decisionSeen: false };
function fixture() {
  const context = { protocol: PROTOCOL, commit: 'a'.repeat(40), sourceDigest: 'b'.repeat(64), inventoryDigest: 'c'.repeat(64), runId: 'run-1', attempt: '1', routes: ['/', '/second'] };
  const item = (id, title, attachment) => ({ id, title, file: CONTRACT_FILE, status: 'passed', expectedStatus: 'passed', retry: 0, attachments: attachment ? [attachment] : [] });
  const route = route => ({ protocol: PROTOCOL, kind: 'route', route, discoveredKeys: ['actual-button'], observations: [{ key: 'actual-button', completed: true, effect: { ...effect }, error: null }], exhausted: true, failures: [], assertionsPassed: true });
  const tests = [item('a', CLICK_TITLE + '/', route('/')), item('b', CLICK_TITLE + '/second', route('/second')), item('declaration', INVENTORY_TITLE, { protocol: PROTOCOL, kind: 'inventory', routes: context.routes, assertionsPassed: true })];
  tests.push(...[...context.routes.map(route => PURPOSE_TITLE + route), ...GLOBAL_TITLES].map((title, index) => item(`contract-${index}`, title)));
  const shards = [tests.slice(0, 1), tests.slice(1)].map((selected, index) => ({ protocol: PROTOCOL, context: structuredClone(context), shard: { index: index + 1, total: 2 }, selection: selected.map(({ id, title, file }) => ({ id, title, file })), tests: structuredClone(selected), complete: true, runStatus: 'passed' }));
  return { context, shards };
}

test('requires completed per-route assertions across every shard', () => {
  const { context, shards } = fixture();
  assert.deepEqual(validateReceipts(shards, context, 2), { shards: 2, routes: 2, targets: 2, tests: 11 });
});

const corruptions = [
  ['missing shard', f => f.shards.pop()],
  ['duplicate shard', f => f.shards[1].shard.index = 1],
  ['wrong total', f => f.shards[1].shard.total = 3],
  ['stale commit', f => f.shards[1].context.commit = 'd'.repeat(40)],
  ['changed source', f => f.shards[1].context.sourceDigest = 'd'.repeat(64)],
  ['wrong inventory', f => f.shards[1].context.inventoryDigest = 'd'.repeat(64)],
  ['wrong run', f => f.shards[1].context.runId = 'previous-run'],
  ['wrong attempt', f => f.shards[1].context.attempt = '2'],
  ['unfinished receipt', f => f.shards[1].complete = false],
  ['failed run', f => f.shards[1].runStatus = 'failed'],
  ['missing execution', f => f.shards[1].tests.pop()],
  ['undeclared execution', f => f.shards[0].selection = []],
  ['duplicate test execution', f => f.shards[0].tests.push(f.shards[0].tests[0])],
  ['duplicate cross-shard test', f => { const t = structuredClone(f.shards[0].tests[0]); f.shards[1].tests.push(t); f.shards[1].selection.push({ id: t.id, title: t.title, file: t.file }); }],
  ['failed route', f => f.shards[0].tests[0].status = 'failed'],
  ['skipped route', f => f.shards[0].tests[0].status = 'skipped'],
  ['expected failure', f => f.shards[0].tests[0].expectedStatus = 'failed'],
  ['retried route', f => f.shards[0].tests[0].retry = 1],
  ['missing attachment', f => f.shards[0].tests[0].attachments = []],
  ['foreign test route', f => f.shards[0].tests[0].attachments[0].route = '/second'],
  ['wrong source file', f => { f.shards[0].tests[0].file = 'fake.spec.ts'; f.shards[0].selection[0].file = 'fake.spec.ts'; }],
  ['missing target observation', f => f.shards[0].tests[0].attachments[0].observations = []],
  ['undiscovered observation', f => f.shards[0].tests[0].attachments[0].discoveredKeys = []],
  ['duplicate discovered key', f => f.shards[0].tests[0].attachments[0].discoveredKeys.push('actual-button')],
  ['incomplete target', f => f.shards[0].tests[0].attachments[0].observations[0].completed = false],
  ['target error', f => f.shards[0].tests[0].attachments[0].observations[0].error = 'detached'],
  ['failed assertion', f => f.shards[0].tests[0].attachments[0].assertionsPassed = false],
  ['unexhausted discovery', f => f.shards[0].tests[0].attachments[0].exhausted = false],
  ['route failure', f => f.shards[0].tests[0].attachments[0].failures = ['dead button']],
  ['alert-only fake effect', f => { const e = f.shards[0].tests[0].attachments[0].observations[0].effect; e.changed = false; e.dialogSeen = true; }],
  ['failed selection restoration', f => f.shards[0].tests[0].attachments[0].observations[0].effect.selectionRestored = false],
  ['nonboolean effect', f => f.shards[0].tests[0].attachments[0].observations[0].effect.changed = 'true'],
  ['false inventory declaration', f => f.shards[1].tests[1].attachments[0].routes = ['/']],
  ['missing distinct global assertion', f => { f.shards[1].tests.pop(); f.shards[1].selection.pop(); }],
  ['skipped purpose assertion', f => f.shards[1].tests[2].status = 'skipped'],
  ['no actual targets anywhere', f => { for (const s of f.shards) for (const t of s.tests) if (t.attachments[0]?.kind === 'route') Object.assign(t.attachments[0], { observations: [], discoveredKeys: [] }); }],
];
for (const [name, mutate] of corruptions) test(`refuses ${name}`, () => { const f = fixture(); mutate(f); assert.throws(() => validateReceipts(f.shards, f.context, 2)); });

test('receipt loading refuses truncated, pending, duplicate and symlink files', () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'click-receipts-'));
  try {
    const receipt = fixture().shards[0];
    fs.writeFileSync(path.join(dir, 'one.json'), JSON.stringify(receipt));
    assert.equal(readReceipts(dir).length, 1);
    for (const [name, contents] of [['partial.json', '{'], ['one.json.pending', '{}']]) {
      const file = path.join(dir, name); fs.writeFileSync(file, contents); assert.throws(() => readReceipts(dir)); fs.unlinkSync(file);
    }
    fs.symlinkSync(path.join(dir, 'one.json'), path.join(dir, 'linked.json')); assert.throws(() => readReceipts(dir)); fs.unlinkSync(path.join(dir, 'linked.json'));
    fs.copyFileSync(path.join(dir, 'one.json'), path.join(dir, 'copy.json')); assert.throws(() => readReceipts(dir));
  } finally { fs.rmSync(dir, { recursive: true, force: true }); }
});

test('context binds actual tracked bytes, exact routes, HEAD and hosted SHA', () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'click-source-'));
  const git = args => execFileSync('git', args, { cwd: root, encoding: 'utf8' });
  try {
    fs.mkdirSync(path.join(root, 'src/ui/next/src/app/second'), { recursive: true });
    fs.writeFileSync(path.join(root, 'src/ui/next/src/app/page.tsx'), 'root');
    fs.writeFileSync(path.join(root, 'src/ui/next/src/app/second/page.tsx'), 'second');
    git(['init', '-q']); git(['add', '.']); git(['-c', 'user.name=fixture', '-c', 'user.email=fixture@localhost', 'commit', '-qm', 'fixture']);
    const env = { GITHUB_RUN_ID: 'fixture', GITHUB_RUN_ATTEMPT: '1', GITHUB_SHA: git(['rev-parse', 'HEAD']).trim() };
    const before = makeRunContext(root, env); assert.deepEqual(before.routes, ['/', '/second']);
    fs.writeFileSync(path.join(root, 'src/ui/next/src/app/page.tsx'), 'changed');
    assert.notEqual(makeRunContext(root, env).sourceDigest, before.sourceDigest);
    assert.throws(() => makeRunContext(root, { ...env, GITHUB_SHA: 'f'.repeat(40) }));
  } finally { fs.rmSync(root, { recursive: true, force: true }); }
});

export { ATTACHMENT };

test('real Playwright reporter retains actual successful assertions and refuses an actual failed assertion', () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'click-reporter-'));
  const git = args => execFileSync('git', args, { cwd: root, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
  try {
    const specDir = path.join(root, 'src/e2e'); fs.mkdirSync(specDir, { recursive: true });
    fs.mkdirSync(path.join(root, 'src/ui/next/src/app/second'), { recursive: true });
    for (const file of ['page.tsx', 'second/page.tsx']) fs.writeFileSync(path.join(root, 'src/ui/next/src/app', file), 'fixture');
    const { shards } = fixture(); const cases = shards.flatMap(shard => shard.tests);
    const testFile = `const {test,expect}=require(${JSON.stringify(require.resolve('@playwright/test'))});\n`
      + cases.map(item => `test(${JSON.stringify(item.title)},async()=>{expect(process.env.PROBE_FAIL==='1'&&${JSON.stringify(item.id)}==='b').toBe(false);${item.attachments.length ? `await test.info().attach(${JSON.stringify(ATTACHMENT)},{body:Buffer.from(${JSON.stringify(JSON.stringify(item.attachments[0]))}),contentType:'application/json'});` : ''}});`).join('\n');
    fs.writeFileSync(path.join(specDir, 'comprehensive_ui_contract.spec.ts'), testFile);
    fs.writeFileSync(path.join(root, 'playwright.config.cjs'), `module.exports={testDir:'./src/e2e',workers:1,retries:0,reporter:[[${JSON.stringify(require.resolve('./ui-click-audit-reporter.cjs'))}]],outputDir:'./untracked-results'};`);
    git(['init', '-q']); git(['add', '.']); git(['-c', 'user.name=fixture', '-c', 'user.email=fixture@localhost', 'commit', '-qm', 'fixture']);
    const context = makeRunContext(root, { GITHUB_RUN_ID: 'reporter-probe', GITHUB_RUN_ATTEMPT: '1' });
    for (const fail of [false, true]) {
      const directory = path.join(root, fail ? 'failed-receipts' : 'passed-receipts');
      let rejected = false;
      try {
        execFileSync(process.execPath, [require.resolve('@playwright/test/cli'), 'test', '--config', 'playwright.config.cjs'], {
          cwd: root, encoding: 'utf8', timeout: 30000, stdio: ['ignore', 'pipe', 'pipe'],
          env: { ...process.env, PROBE_FAIL: fail ? '1' : '0', OHC_CLICK_AUDIT_ROOT: root, OHC_CLICK_AUDIT_DIRECTORY: directory, OHC_CLICK_AUDIT_CONTEXT: JSON.stringify(context) },
        });
      } catch (error) { rejected = true; if (!fail) throw error; }
      assert.equal(rejected, fail);
      const receipts = readReceipts(directory);
      if (fail) assert.throws(() => validateReceipts(receipts, context, 1));
      else assert.deepEqual(validateReceipts(receipts, context, 1), { shards: 1, routes: 2, targets: 2, tests: 11 });
    }
  } finally { fs.rmSync(root, { recursive: true, force: true }); }
});


test('required native selection cannot conceal filtering or reporter overrides', () => {
  for (const args of [[], ['--workers=2', '--retries=0'], ['--shard', '2/12', '--workers', '1'], ['--headed']]) assert.equal(protocol.completeSelection(args), true);
  for (const args of [['one.spec.ts'], ['--grep=single'], ['--project=chromium'], ['--last-failed'], ['--reporter=html'], ['--shard=0/12'], ['--shard=13/12'], ['--workers']]) assert.equal(protocol.completeSelection(args), false);
});

test('required workflow aggregates all twelve independently retained shard receipts', () => {
  const root = path.resolve(import.meta.dirname, '..');
  const jobs = require('js-yaml').load(fs.readFileSync(path.join(root, '.github/workflows/ci.yml'), 'utf8')).jobs;
  const e2e = jobs['native-e2e'], aggregate = jobs['native-click-coverage'], required = jobs['ci-required'];
  assert.deepEqual(e2e.strategy.matrix.shard, Array.from({ length: 12 }, (_, index) => index + 1));
  const upload = e2e.steps.find(step => step.with?.name === 'native-click-receipts-${{ matrix.shard }}');
  assert.equal(upload.if, 'always()'); assert.equal(upload.with['if-no-files-found'], 'error');
  assert.deepEqual(aggregate.needs, ['check-changes', 'native-e2e']); assert.match(aggregate.if, /!cancelled\(\)/);
  const download = aggregate.steps.find(step => step.uses?.startsWith('actions/download-artifact@'));
  assert.equal(download.with.pattern, 'native-click-receipts-*'); assert.equal(download.with['merge-multiple'], false);
  assert(aggregate.steps.some(step => /ui-click-audit\.cjs target\/click-receipts 12/.test(step.run || '')));
  assert(required.needs.includes('native-click-coverage'));
  assert(required.steps.some(step => /require_success "native-click-coverage"/.test(step.run || '')));
});
