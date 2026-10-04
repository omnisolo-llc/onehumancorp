'use strict';
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { createOwnedAuditSeed, quoteAuditRoutes, quoteAuditRoute, ownedQuoteAuditRecord } = require('./ui-audit-fixture.cjs');
const { execFileSync } = require('node:child_process');
const PROTOCOL = 1;
const ATTACHMENT = 'ohc-click-coverage-v1';
const CONTRACT_FILE = 'src/e2e/comprehensive_ui_contract.spec.ts';
const CLICK_TITLE = 'all visible enabled buttons and click targets have an effect on ';
const INVENTORY_TITLE = 'declares complete click coverage for required post-run verification';
const PURPOSE_TITLE = 'all visible interactive elements declare their designed purpose on ';
const GLOBAL_TITLES = [
  'per-route exhaustive UI element contracts cover at least 100 additional checks',
  'every app page loads without visible crash output',
  'visible internal links resolve to real pages',
  'visible external and protocol links use expected destinations',
  'all visible interactive elements are usable and named',
  'layouts do not overflow or overlap click targets on desktop and mobile',
];
// Only these source-defined pages intentionally redirect before any control discovery.
function expectedAuditPath(route) { return route === '/' ? '/dashboard' : route === '/share-card' ? '/onboarding' : route; }
const hash = value => crypto.createHash('sha256').update(value).digest('hex');
function requireTrue(condition, reason) { if (!condition) throw new Error(`Click coverage: ${reason}`); }
function exact(a, b) { return JSON.stringify(a) === JSON.stringify(b); }
function uniqueStrings(values, name) {
  requireTrue(Array.isArray(values) && values.every(value => typeof value === 'string' && value.length > 0), `${name} must contain strings`);
  requireTrue(new Set(values).size === values.length, `${name} contains duplicates`);
  return values;
}
function discoverAppRoutes(root) {
  const app = path.join(root, 'src/ui/next/src/app');
  const examples = { '[articleId]': 'getting-started-1', '[tenant]': 'default', '[id]': 'e2e-id' };
  const walk = dir => fs.readdirSync(dir, { withFileTypes: true }).flatMap(entry => {
    const file = path.join(dir, entry.name);
    return entry.isDirectory() ? walk(file) : entry.isFile() || entry.isSymbolicLink() ? [file] : [];
  });
  const routes = walk(app).filter(file => file.endsWith(`${path.sep}page.tsx`)).map(file => {
    const relative = path.relative(app, path.dirname(file));
    const segments = relative === '' ? [] : relative.split(path.sep);
    if (segments.some(segment => segment === 'api' || segment.startsWith('('))) return null;
    // Dynamic record pages must use a real canonical database fixture.
    if (segments.join('/') === 'orders/[id]') return '/orders/e2e-seeded-record';
    return `/${segments.filter(segment => !segment.startsWith('_')).map(segment => examples[segment] || segment).join('/')}`.replace(/\/$/, '') || '/';
  }).filter(Boolean);
  return [...new Set(routes)].sort();
}
function completeSelection(args) {
  let shards = 0;
  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    if (arg === '--headed') continue;
    const match = /^--(workers|retries|shard|grouped-shard)(?:=(.+))?$/.exec(arg);
    if (!match) return false;
    const value = match[2] ?? args[++index];
    if (match[1] === 'shard' || match[1] === 'grouped-shard') {
      if (++shards > 1) return false;
      const shard = /^(\d+)\/(\d+)$/.exec(value || '');
      if (!shard || Number(shard[1]) < 1 || Number(shard[1]) > Number(shard[2])) return false;
      if (match[1] === 'grouped-shard' && (Number(shard[2]) !== 3 || Number(shard[1]) > 3)) return false;
    } else if (!/^\d+$/.test(value || '')) return false;
  }
  return true;
}

function sourceIdentity(root, captureFiles = false) {
  const commit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim();
  const files = execFileSync('git', ['ls-files', '-z'], { cwd: root, encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 }).split('\0').filter(Boolean).sort();
  requireTrue(/^[a-f0-9]{40,64}$/.test(commit) && files.length > 0, 'actual Git source identity required');
  const digest = crypto.createHash('sha256'), sourceFiles = Object.create(null);
  for (const file of files) {
    const full = path.join(root, file), stat = fs.lstatSync(full);
    requireTrue(stat.isFile() || stat.isSymbolicLink(), `unsupported tracked source ${file}`);
    const bytes = stat.isSymbolicLink() ? Buffer.from(fs.readlinkSync(full)) : fs.readFileSync(full);
    digest.update(JSON.stringify([file, stat.isSymbolicLink() ? 'link' : 'file', bytes.length]));
    digest.update(bytes);
    if (captureFiles) sourceFiles[file] = { kind: stat.isSymbolicLink() ? 'link' : 'file', bytes: bytes.length, sha256: hash(bytes) };
  }
  return { commit, sourceDigest: digest.digest('hex'), ...(captureFiles ? { files: sourceFiles } : {}) };
}
function makeRunContext(root, environment = process.env) {
  const identity = sourceIdentity(root), routes = discoverAppRoutes(root);
  requireTrue(routes.length > 0, 'source route discovery was empty');
  if (environment.GITHUB_SHA) requireTrue(environment.GITHUB_SHA === identity.commit, 'hosted commit differs from checkout');
  const context = { protocol: PROTOCOL, ...identity, inventoryDigest: hash(JSON.stringify(routes)),
    runId: environment.GITHUB_RUN_ID || `local-${crypto.randomUUID()}`, attempt: environment.GITHUB_RUN_ATTEMPT || '1', routes };
  validateContext(context);
  return context;
}
function validateContext(context) {
  requireTrue(context && context.protocol === PROTOCOL && /^[a-f0-9]{40,64}$/.test(context.commit)
    && /^[a-f0-9]{64}$/.test(context.sourceDigest) && /^[a-f0-9]{64}$/.test(context.inventoryDigest)
    && typeof context.runId === 'string' && /^[A-Za-z0-9_-]+$/.test(context.runId) && typeof context.attempt === 'string' && /^[1-9]\d*$/.test(context.attempt), 'invalid run/source context');
  requireTrue(uniqueStrings(context.routes, 'source routes').length > 0, 'empty source route inventory');
}
// Diagnostics never substitute for the full digest. Keep the complete failure
// and report at most100 changed paths, hashes only, with explicit omitted counts.
function describeSourceDrift(root, context, before, observed) {
  const afterFiles=observed?.files || Object.create(null);
  let commit=observed?.commit || null, diagnosticError=null;
  if (!observed) {
    try {
      commit=execFileSync('git',['rev-parse','HEAD'],{cwd:root,encoding:'utf8'}).trim();
      const files=execFileSync('git',['ls-files','-z'],{cwd:root,encoding:'utf8',maxBuffer:16*1024*1024}).split('\0').filter(Boolean);
      for (const file of files) {
        try {
          const full=path.join(root,file), stat=fs.lstatSync(full);
          if (!stat.isFile() && !stat.isSymbolicLink()) {afterFiles[file]={kind:'unsupported'}; continue;}
          const bytes=stat.isSymbolicLink()?Buffer.from(fs.readlinkSync(full)):fs.readFileSync(full);
          afterFiles[file]={kind:stat.isSymbolicLink()?'link':'file',bytes:bytes.length,sha256:hash(bytes)};
        } catch(error) {afterFiles[file]=error.code==='ENOENT'?null:{kind:'unreadable',error:String(error.code || 'read_failed').slice(0,80)};}
      }
    } catch(error) {diagnosticError=String(error.message).slice(0,500);}
  }
  const previous=before?.files || Object.create(null), changes=[];
  for (const file of [...new Set([...Object.keys(previous),...Object.keys(afterFiles)])].sort()) {
    const prior=previous[file] || null, current=afterFiles[file] || null;
    if (!exact(prior,current)) changes.push({path:file.slice(0,1024),...(file.length>1024?{pathTruncated:true}:{}),before:prior,after:current});
  }
  let routes=null;
  try {routes=discoverAppRoutes(root);} catch(error) {diagnosticError ||= String(error.message).slice(0,500);}
  const added=routes?.filter(route=>!context.routes.includes(route)) || [], removed=routes?context.routes.filter(route=>!routes.includes(route)):[];
  return { commitBefore:context.commit,commitAfter:commit,sourceDigestBefore:context.sourceDigest,sourceDigestAfter:observed?.sourceDigest || null,
    filesBefore:Object.keys(previous).length,filesAfter:Object.keys(afterFiles).length,changedFiles:changes.length,changes:changes.slice(0,100),omittedChanges:Math.max(0,changes.length-100),
    routesAdded:added.slice(0,100).map(route=>route.slice(0,1024)),routesRemoved:removed.slice(0,100).map(route=>route.slice(0,1024)),omittedRoutes:Math.max(0,added.length-100)+Math.max(0,removed.length-100),diagnosticError };
}
function assertSource(root, context, before) {
  validateContext(context);
  let current;
  try {
    current=sourceIdentity(root,true);
    const routes=discoverAppRoutes(root);
    requireTrue(current.commit === context.commit && current.sourceDigest === context.sourceDigest
      && exact(routes, context.routes) && hash(JSON.stringify(routes)) === context.inventoryDigest, 'source or route inventory changed during execution');
    return current;
  } catch(error) {
    if (before) error.sourceDiagnostics=describeSourceDrift(root,context,before,current);
    throw error;
  }
}

function meaningful(effect) {
  const keys = ['changed', 'requestSeen', 'downloadSeen', 'fileChooserSeen', 'popupSeen', 'validationSeen', 'dialogSeen', 'decisionSeen'];
  requireTrue(effect && keys.every(key => typeof effect[key] === 'boolean')
    && (effect.selectionRestored === undefined || typeof effect.selectionRestored === 'boolean')
    && (effect.focusSeen === undefined || typeof effect.focusSeen === 'boolean'), 'invalid observed effect');
  return effect.selectionRestored !== false && (effect.focusSeen === true || keys.filter(key => key !== 'dialogSeen').some(key => effect[key]));
}
function validateReceipts(receipts, context, totalShards, requireGrouped = false) {
  validateContext(context);
  require('./browser-shards.cjs').validateGroupedReceipts(receipts, context, requireGrouped);
  requireTrue(Number.isInteger(totalShards) && totalShards > 0 && Array.isArray(receipts) && receipts.length === totalShards, 'missing or extra shards');
  const shards = new Set(), allIds = new Set(), covered = new Set();
  const expectedContracts = new Set([INVENTORY_TITLE, ...GLOBAL_TITLES, ...context.routes.flatMap(route => [CLICK_TITLE + route, PURPOSE_TITLE + route])]);
  const completedContracts = new Set();
  const ownedQuoteNamespaces = new Set();
  let targets = 0, declarations = 0, tests = 0;
  for (const receipt of receipts) {
    requireTrue(receipt && receipt.protocol === PROTOCOL && exact(receipt.context, context), 'stale or foreign receipt identity');
    requireTrue(receipt.complete === true && receipt.runStatus === 'passed', 'run did not finish successfully');
    const shard = receipt.shard;
    requireTrue(shard && Number.isInteger(shard.index) && shard.index >= 1 && shard.index <= totalShards && shard.total === totalShards && !shards.has(shard.index), 'invalid or duplicate shard');
    shards.add(shard.index);
    requireTrue(Array.isArray(receipt.selection) && receipt.selection.length > 0 && Array.isArray(receipt.tests), 'missing actual test selection/execution');
    const selected = new Map();
    for (const item of receipt.selection) {
      requireTrue(item && typeof item.id === 'string' && item.id && typeof item.title === 'string' && typeof item.file === 'string' && !selected.has(item.id), 'invalid or duplicated selected test');
      selected.set(item.id, item);
    }
    requireTrue(receipt.tests.length === selected.size, 'not every selected test completed exactly once');
    for (const item of receipt.tests) {
      const declared = selected.get(item.id);
      requireTrue(declared && declared.title === item.title && declared.file === item.file && !allIds.has(item.id), 'unknown, mismatched or repeated test execution');
      allIds.add(item.id); selected.delete(item.id); tests += 1;
      requireTrue(['passed', 'failed', 'timedOut', 'skipped', 'interrupted'].includes(item.status), 'test execution is not terminal');
      requireTrue(Array.isArray(item.attachments), 'missing attachment record');
      if (item.file === CONTRACT_FILE) {
        requireTrue(expectedContracts.has(item.title) && !completedContracts.has(item.title), 'unknown or duplicate contract assertion');
        requireTrue(item.status === 'passed' && item.expectedStatus === 'passed' && item.retry === 0, 'required contract assertion did not pass once');
        completedContracts.add(item.title);
      }
      const isRoute = item.file === CONTRACT_FILE && item.title.startsWith(CLICK_TITLE);
      const isInventory = item.file === CONTRACT_FILE && item.title === INVENTORY_TITLE;
      if (!isRoute && !isInventory) { requireTrue(item.attachments.length === 0, 'coverage emitted by an unrelated test'); continue; }
      requireTrue(item.status === 'passed' && item.expectedStatus === 'passed' && item.retry === 0 && item.attachments.length === 1, 'required assertion did not pass once without retry');
      const audit = item.attachments[0];
      requireTrue(audit && audit.protocol === PROTOCOL && audit.assertionsPassed === true, 'required assertions were not completed');
      if (isInventory) {
        requireTrue(audit.kind === 'inventory' && exact(audit.routes, context.routes), 'inventory declaration differs from source'); declarations += 1; continue;
      }
      const route = item.title.slice(CLICK_TITLE.length);
      requireTrue(audit.kind === 'route' && audit.route === route && context.routes.includes(route) && !covered.has(route), 'unknown or duplicate route coverage');
      requireTrue(audit.exhausted === true && Array.isArray(audit.failures) && audit.failures.length === 0, 'enumeration or route assertions incomplete');
      const discovered = uniqueStrings(audit.discoveredKeys, 'discovered target keys');
      requireTrue(Array.isArray(audit.observations) && audit.observations.length === discovered.length, 'missing or invented target observations');
      const observed = new Set();
      for (const observation of audit.observations) {
        requireTrue(observation && discovered.includes(observation.key) && !observed.has(observation.key) && observation.completed === true && observation.error === null, 'target did not complete exactly once');
        requireTrue(meaningful(observation.effect), 'target has no meaningful observed click effect');
        observed.add(observation.key); targets += 1;
      }
      requireTrue(Array.isArray(audit.navigations) && audit.navigations.length === discovered.length + 1, 'navigation evidence must cover initial discovery and every document reset');
      const quoteRoute = quoteAuditRoutes.includes(route);
      const quoteStateKeys = new Map();
      if (quoteRoute) {
        const states = route === '/quotes/e2e-id' ? ['entry', 'editing'] : ['entry'];
        for (const state of states) quoteStateKeys.set(state, []);
        for (const key of discovered) {
          let scoped;
          try { scoped = JSON.parse(key); } catch { throw new Error('Click coverage: quote inventory key lacks its source view'); }
          requireTrue(Array.isArray(scoped) && scoped.length === 2 && quoteStateKeys.has(scoped[0])
            && typeof scoped[1] === 'string' && scoped[1].length > 0, 'quote inventory has an unclassified view');
          quoteStateKeys.get(scoped[0]).push(key);
        }
        requireTrue(states.every(state => quoteStateKeys.get(state).length > 0), 'quote inventory omitted a required view');
        requireTrue(audit.isolation?.kind === 'case-owned-postgres'
          && Array.isArray(audit.isolation.cases) && audit.isolation.cases.length === audit.navigations.length
          && audit.isolation.cases.at(-1).state === 'entry',
          'quote navigation requires a separately owned case for every discovery/reset, ending with the full entry view');
      }
      for (const [index, navigation] of audit.navigations.entries()) {
        requireTrue(navigation && typeof navigation.requestedUrl === 'string' && typeof navigation.finalUrl === 'string', 'invalid navigation evidence');
        let requested, final;
        try { requested = new URL(navigation.requestedUrl); final = new URL(navigation.finalUrl); }
        catch { throw new Error('Click coverage: invalid navigation URL'); }
        requireTrue(['http:', 'https:'].includes(requested.protocol) && !requested.username && !requested.password
          && !final.username && !final.password && requested.origin === final.origin,
          'navigation origin or credentials differ from the classified source route');
        if (quoteRoute) {
          const proof = navigation.quoteFixture;
          requireTrue(proof && navigation.sourceRoute === route && !ownedQuoteNamespaces.has(proof.namespace), 'quote navigation lacks a unique source-bound owned fixture');
          const actor = createOwnedAuditSeed(fs.readFileSync(path.join(__dirname, '../src/e2e/e2e-seed.sql'), 'utf8'), proof.namespace);
          const expected = ownedQuoteAuditRecord(actor);
          const isolated = audit.isolation.cases[index];
          requireTrue(audit.isolation.seedDigest === actor.sourceDigest && isolated.tenantId === actor.tenantId
            && isolated.userId === actor.userId && quoteStateKeys.has(isolated.state), 'quote navigation isolation differs from the canonical source fixture');
          const keys = uniqueStrings(isolated.keys, 'quote case inventory');
          requireTrue(exact([...keys].sort(), [...quoteStateKeys.get(isolated.state)].sort())
            && (index === discovered.length || keys.includes(audit.observations[index].key)),
            'quote case inventory changed before a click or reset');
          const expectedUrl = new URL(quoteAuditRoute(route, expected.quoteId), requested.origin).href;
          requireTrue(proof.quoteId === expected.quoteId && proof.customerId === expected.customerId && proof.tenantId === expected.tenantId
            && proof.status === 200 && proof.method === 'GET' && proof.detailUrl === `${requested.origin}/api/v1/quotes/${expected.quoteId}`
            && requested.href === expectedUrl && final.href === expectedUrl && navigation.redirected === false,
            'quote navigation or canonical detail proof differs from the source-bound owned record');
          ownedQuoteNamespaces.add(proof.namespace);
        } else {
          requireTrue(navigation.sourceRoute === undefined && navigation.quoteFixture === undefined
            && requested.pathname === route && final.pathname === expectedAuditPath(route)
            && (route !== '/share-card' || (!requested.search && !requested.hash))
            && navigation.redirected === (requested.href !== final.href), 'navigation destination or redirect evidence differs from the classified source route');
        }
      }
      covered.add(route);
    }
  }
  requireTrue(exact([...completedContracts].sort(), [...expectedContracts].sort()), 'missing purpose or distinct global contract assertions');
  requireTrue(declarations === 1 && exact([...covered].sort(), [...context.routes].sort()), 'missing required inventory declaration or routes');
  requireTrue(targets > 0, 'no actual target was observed');
  return { shards: shards.size, routes: covered.size, targets, tests };
}
function readReceipts(directory) {
  const receipts = [], seen = new Set();
  function walk(dir) {
    requireTrue(!fs.lstatSync(dir).isSymbolicLink(), 'receipt directories cannot be links');
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const file = path.join(dir, entry.name), stat = fs.lstatSync(file);
      requireTrue(!stat.isSymbolicLink(), 'receipt files cannot be links');
      if (stat.isDirectory()) { walk(file); continue; }
      requireTrue(stat.isFile() && entry.name.endsWith('.json') && stat.size <= 32 * 1024 * 1024, 'unknown, pending or oversized receipt file');
      const bytes = fs.readFileSync(file), digest = hash(bytes);
      requireTrue(!seen.has(digest), 'duplicate receipt files'); seen.add(digest);
      receipts.push(JSON.parse(bytes.toString('utf8')));
    }
  }
  walk(directory); requireTrue(receipts.length > 0, 'no receipt files'); return receipts;
}
function writeReceipt(file, value) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  const pending = `${file}.pending`;
  fs.writeFileSync(pending, JSON.stringify(value, null, 2) + '\n', { flag: 'wx' });
  fs.renameSync(pending, file);
}
module.exports = { PROTOCOL, ATTACHMENT, CLICK_TITLE, INVENTORY_TITLE, PURPOSE_TITLE, GLOBAL_TITLES, CONTRACT_FILE, discoverAppRoutes, expectedAuditPath,
  sourceIdentity, makeRunContext, completeSelection, validateContext, assertSource, validateReceipts, readReceipts, writeReceipt };
if (require.main === module) {
  try {
    const [directory, count, mode, ...extra] = process.argv.slice(2);
    requireTrue(extra.length === 0 && (mode === undefined || mode === '--grouped'), 'unknown coverage mode');
    requireTrue(directory && /^[1-9]\d*$/.test(count || ''), 'usage: ui-click-audit.cjs RECEIPT_DIRECTORY SHARD_COUNT');
    const root = path.resolve(__dirname, '..');
    console.log(JSON.stringify(validateReceipts(readReceipts(path.resolve(directory)), makeRunContext(root), Number(count), mode === '--grouped'), null, 2));
  } catch (error) { console.error(error.message); process.exitCode = 1; }
}
