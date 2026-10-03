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
function fixture(routes = ['/', '/second']) {
  const context = { protocol: PROTOCOL, commit: 'a'.repeat(40), sourceDigest: 'b'.repeat(64), inventoryDigest: 'c'.repeat(64), runId: 'run-1', attempt: '1', routes };
  const item = (id, title, attachment) => ({ id, title, file: CONTRACT_FILE, status: 'passed', expectedStatus: 'passed', retry: 0, attachments: attachment ? [attachment] : [] });
  const route = route => ({ navigations: Array.from({ length: 2 }, () => ({ requestedUrl: `https://fixture.test${route}`, finalUrl: `https://fixture.test${protocol.expectedAuditPath(route)}`, redirected: protocol.expectedAuditPath(route) !== route })), protocol: PROTOCOL, kind: 'route', route, discoveredKeys: ['actual-button'], observations: [{ key: 'actual-button', completed: true, effect: { ...effect }, error: null }], exhausted: true, failures: [], assertionsPassed: true });
  const tests = [item('a', CLICK_TITLE + routes[0], route(routes[0])), item('b', CLICK_TITLE + routes[1], route(routes[1])), item('declaration', INVENTORY_TITLE, { protocol: PROTOCOL, kind: 'inventory', routes: context.routes, assertionsPassed: true })];
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
  ['nonboolean focus effect', f => f.shards[0].tests[0].attachments[0].observations[0].effect.focusSeen = 'true'],
  ['nonboolean effect', f => f.shards[0].tests[0].attachments[0].observations[0].effect.changed = 'true'],
  ['false inventory declaration', f => f.shards[1].tests[1].attachments[0].routes = ['/']],
  ['missing distinct global assertion', f => { f.shards[1].tests.pop(); f.shards[1].selection.pop(); }],
  ['skipped purpose assertion', f => f.shards[1].tests[2].status = 'skipped'],
  ['no actual targets anywhere', f => { for (const s of f.shards) for (const t of s.tests) if (t.attachments[0]?.kind === 'route') Object.assign(t.attachments[0], { observations: [], discoveredKeys: [], navigations: t.attachments[0].navigations.slice(0, 1) }); }],
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
      + cases.map(item => `test(${JSON.stringify(item.title)},async()=>{if(process.env.PROBE_DRIFT==='1'&&${JSON.stringify(item.id)}==='a')require('node:fs').writeFileSync('tracked-output.txt','after');expect(process.env.PROBE_FAIL==='1'&&${JSON.stringify(item.id)}==='b').toBe(false);${item.attachments.length ? `await test.info().attach(${JSON.stringify(ATTACHMENT)},{body:Buffer.from(${JSON.stringify(JSON.stringify(item.attachments[0]))}),contentType:'application/json'});` : ''}});`).join('\n');
    fs.writeFileSync(path.join(specDir, 'comprehensive_ui_contract.spec.ts'), testFile);
    fs.writeFileSync(path.join(root, 'playwright.config.cjs'), `module.exports={testDir:'./src/e2e',workers:1,retries:0,reporter:[[${JSON.stringify(require.resolve('./ui-click-audit-reporter.cjs'))}]],outputDir:'./untracked-results'};`);
    fs.writeFileSync(path.join(root,'tracked-output.txt'),'before');
    git(['init', '-q']); git(['add', '.']); git(['-c', 'user.name=fixture', '-c', 'user.email=fixture@localhost', 'commit', '-qm', 'fixture']);
    const context = makeRunContext(root, { GITHUB_RUN_ID: 'reporter-probe', GITHUB_RUN_ATTEMPT: '1' });
    for (const mode of ['pass','assertion-failure','source-drift']) {
      const fail=mode!=='pass'; const directory=path.join(root,mode+'-receipts');
      let rejected = false;
      try {
        execFileSync(process.execPath, [require.resolve('@playwright/test/cli'), 'test', '--config', 'playwright.config.cjs'], {
          cwd: root, encoding: 'utf8', timeout: 30000, stdio: ['ignore', 'pipe', 'pipe'],
          env: { ...process.env, PROBE_FAIL: mode==='assertion-failure' ? '1' : '0', PROBE_DRIFT: mode==='source-drift' ? '1' : '0', OHC_CLICK_AUDIT_ROOT: root, OHC_CLICK_AUDIT_DIRECTORY: directory, OHC_CLICK_AUDIT_CONTEXT: JSON.stringify(context) },
        });
      } catch (error) { rejected = true; if (!fail) throw error; }
      assert.equal(rejected, fail);
      const receipts = readReceipts(directory);
      if (mode==='source-drift') {assert.equal(receipts[0].complete,false);assert.equal(receipts[0].runStatus,'failed');assert.match(receipts[0].reporterError,/source or route inventory changed/);assert.equal(receipts[0].sourceDiagnostics.changes[0].path,'tracked-output.txt');assert(receipts[0].tests.every(t=>t.status==='passed'));}
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

test('source drift reports exact tracked output hashes without excluding runtime-looking paths', () => {
  const root=fs.mkdtempSync(path.join(os.tmpdir(),'click-drift-'));
  const git=args=>execFileSync('git',args,{cwd:root,encoding:'utf8'});
  try {
    fs.mkdirSync(path.join(root,'src/ui/next/src/app'),{recursive:true});
    fs.mkdirSync(path.join(root,'.agent-task/report'),{recursive:true});
    fs.writeFileSync(path.join(root,'src/ui/next/src/app/page.tsx'),'page');
    fs.writeFileSync(path.join(root,'.agent-task/report/task_output.md'),'before');
    fs.writeFileSync(path.join(root,'removed.txt'),'remove me');
    git(['init','-q']);git(['add','-f','.']);git(['-c','user.name=fixture','-c','user.email=fixture@localhost','commit','-qm','fixture']);
    const context=makeRunContext(root,{GITHUB_RUN_ID:'drift',GITHUB_RUN_ATTEMPT:'1'});
    const before=protocol.assertSource(root,context);
    fs.writeFileSync(path.join(root,'.agent-task/report/task_output.md'),'after');fs.unlinkSync(path.join(root,'removed.txt'));
    fs.mkdirSync(path.join(root,'src/ui/next/src/app/new-route'));fs.writeFileSync(path.join(root,'src/ui/next/src/app/new-route/page.tsx'),'untracked route');
    let error;try{protocol.assertSource(root,context,before);}catch(caught){error=caught;}
    assert(error);assert.match(error.message,/changed|ENOENT/);
    const detail=error.sourceDiagnostics;
    assert.equal(detail.changedFiles,2);assert.equal(detail.omittedChanges,0);
    const output=detail.changes.find(item=>item.path==='.agent-task/report/task_output.md');
    assert.equal(output.before.bytes,6);assert.equal(output.after.bytes,5);assert.notEqual(output.before.sha256,output.after.sha256);
    assert.equal(detail.changes.find(item=>item.path==='removed.txt').after,null);
    assert.deepEqual(detail.routesAdded,['/new-route']);assert.deepEqual(detail.routesRemoved,[]);
  } finally {fs.rmSync(root,{recursive:true,force:true});}
});

test('drift diagnostics bound listed paths while the full source still fails closed', () => {
 const root=fs.mkdtempSync(path.join(os.tmpdir(),'click-drift-bound-'));const git=args=>execFileSync('git',args,{cwd:root,encoding:'utf8'});
 try {
  fs.mkdirSync(path.join(root,'src/ui/next/src/app'),{recursive:true});fs.writeFileSync(path.join(root,'src/ui/next/src/app/page.tsx'),'page');
  for(let n=0;n<105;n++)fs.writeFileSync(path.join(root,`source-${n}.txt`),'before');
  git(['init','-q']);git(['add','.']);git(['-c','user.name=fixture','-c','user.email=fixture@localhost','commit','-qm','fixture']);
  const context=makeRunContext(root,{GITHUB_RUN_ID:'bounded',GITHUB_RUN_ATTEMPT:'1'});const before=protocol.assertSource(root,context);
  for(let n=0;n<105;n++)fs.writeFileSync(path.join(root,`source-${n}.txt`),'after');
  let error;try{protocol.assertSource(root,context,before);}catch(caught){error=caught;}
  assert(error);assert.equal(error.sourceDiagnostics.changedFiles,105);assert.equal(error.sourceDiagnostics.changes.length,100);assert.equal(error.sourceDiagnostics.omittedChanges,5);
 } finally {fs.rmSync(root,{recursive:true,force:true});}
});

test('a recorded trusted-click focus effect is meaningful while plain preparation remains inert', () => {
 const f=fixture(); const effect=f.shards[0].tests[0].attachments[0].observations[0].effect; effect.changed=false; effect.focusSeen=true;
 assert.equal(validateReceipts(f.shards,f.context,2).targets,2);
 effect.focusSeen=false; assert.throws(()=>validateReceipts(f.shards,f.context,2),/no meaningful/);
});

test('refuses route coverage without navigation evidence', () => {
  const f = fixture();
  delete f.shards[0].tests[0].attachments[0].navigations;
  assert.throws(() => validateReceipts(f.shards, f.context, 2), /navigation/i);
});
test('refuses navigation evidence for another origin or unexpected destination path', () => {
  for (const finalUrl of ['https://other.test/dashboard', 'https://fixture.test/unrelated']) {
    const f = fixture();
    f.shards[0].tests[0].attachments[0].navigations = [
      { requestedUrl: 'https://fixture.test/', finalUrl, redirected: true },
      { requestedUrl: 'https://fixture.test/', finalUrl, redirected: true },
    ];
    assert.throws(() => validateReceipts(f.shards, f.context, 2), /navigation/i);
  }
});


test('accepts the source-classified share redirect with all final-page targets observed', () => {
  const f = fixture(['/share-card', '/second']);
  assert.equal(validateReceipts(f.shards, f.context, 2).targets, 2);
});
for (const [name, mutate] of [
  ['parameterized share redirect', n => { n.requestedUrl += '?url=%2Fother'; }],
  ['fragment on share redirect', n => { n.requestedUrl += '#other'; }],
  ['credentials', n => { n.requestedUrl = 'https://user:pass@fixture.test/share-card'; }],
  ['false redirect flag', n => { n.redirected = false; }],
]) test(`refuses ${name} navigation evidence`, () => {
  const f = fixture(['/share-card', '/second']);
  mutate(f.shards[0].tests[0].attachments[0].navigations[0]);
  assert.throws(() => validateReceipts(f.shards, f.context, 2), /navigation/i);
});
test('requires a separate verified navigation after each observed target', () => {
  const f = fixture(['/share-card', '/second']);
  f.shards[0].tests[0].attachments[0].navigations.pop();
  assert.throws(() => validateReceipts(f.shards, f.context, 2), /navigation/i);
});

test('dynamic order route audits the canonical persisted fixture instead of an invented ID', () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'click-order-source-'));
  try {
    fs.mkdirSync(path.join(root, 'src/ui/next/src/app/orders/[id]'), { recursive: true });
    fs.writeFileSync(path.join(root, 'src/ui/next/src/app/orders/[id]/page.tsx'), 'order page');
    assert.deepEqual(protocol.discoverAppRoutes(root), ['/orders/e2e-seeded-record']);
    const seed = fs.readFileSync(new URL('../src/e2e/e2e-seed.sql', import.meta.url), 'utf8');
    assert.match(seed, /INSERT INTO orders[\s\S]*?'e2e-seeded-record'/);
  } finally { fs.rmSync(root, { recursive: true, force: true }); }
});
