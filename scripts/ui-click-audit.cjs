'use strict';
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
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
    return `/${segments.filter(segment => !segment.startsWith('_')).map(segment => examples[segment] || segment).join('/')}`.replace(/\/$/, '') || '/';
  }).filter(Boolean);
  return [...new Set(routes)].sort();
}
function completeSelection(args) {
  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    if (arg === '--headed') continue;
    const match = /^--(workers|retries|shard)(?:=(.+))?$/.exec(arg);
    if (!match) return false;
    const value = match[2] ?? args[++index];
    if (match[1] === 'shard') {
      const shard = /^(\d+)\/(\d+)$/.exec(value || '');
      if (!shard || Number(shard[1]) < 1 || Number(shard[1]) > Number(shard[2])) return false;
    } else if (!/^\d+$/.test(value || '')) return false;
  }
  return true;
}

function sourceIdentity(root) {
  const commit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim();
  const files = execFileSync('git', ['ls-files', '-z'], { cwd: root, encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 }).split('\0').filter(Boolean).sort();
  requireTrue(/^[a-f0-9]{40,64}$/.test(commit) && files.length > 0, 'actual Git source identity required');
  const digest = crypto.createHash('sha256');
  for (const file of files) {
    const full = path.join(root, file), stat = fs.lstatSync(full);
    requireTrue(stat.isFile() || stat.isSymbolicLink(), `unsupported tracked source ${file}`);
    const bytes = stat.isSymbolicLink() ? Buffer.from(fs.readlinkSync(full)) : fs.readFileSync(full);
    digest.update(JSON.stringify([file, stat.isSymbolicLink() ? 'link' : 'file', bytes.length]));
    digest.update(bytes);
  }
  return { commit, sourceDigest: digest.digest('hex') };
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
function assertSource(root, context) {
  validateContext(context);
  const current = sourceIdentity(root), routes = discoverAppRoutes(root);
  requireTrue(current.commit === context.commit && current.sourceDigest === context.sourceDigest
    && exact(routes, context.routes) && hash(JSON.stringify(routes)) === context.inventoryDigest, 'source or route inventory changed during execution');
}
function meaningful(effect) {
  const keys = ['changed', 'requestSeen', 'downloadSeen', 'fileChooserSeen', 'popupSeen', 'validationSeen', 'dialogSeen', 'decisionSeen'];
  requireTrue(effect && keys.every(key => typeof effect[key] === 'boolean')
    && (effect.selectionRestored === undefined || typeof effect.selectionRestored === 'boolean'), 'invalid observed effect');
  return effect.selectionRestored !== false && keys.filter(key => key !== 'dialogSeen').some(key => effect[key]);
}
function validateReceipts(receipts, context, totalShards) {
  validateContext(context);
  requireTrue(Number.isInteger(totalShards) && totalShards > 0 && Array.isArray(receipts) && receipts.length === totalShards, 'missing or extra shards');
  const shards = new Set(), allIds = new Set(), covered = new Set();
  const expectedContracts = new Set([INVENTORY_TITLE, ...GLOBAL_TITLES, ...context.routes.flatMap(route => [CLICK_TITLE + route, PURPOSE_TITLE + route])]);
  const completedContracts = new Set();
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
module.exports = { PROTOCOL, ATTACHMENT, CLICK_TITLE, INVENTORY_TITLE, PURPOSE_TITLE, GLOBAL_TITLES, CONTRACT_FILE, discoverAppRoutes,
  sourceIdentity, makeRunContext, completeSelection, validateContext, assertSource, validateReceipts, readReceipts, writeReceipt };
if (require.main === module) {
  try {
    const [directory, count] = process.argv.slice(2);
    requireTrue(directory && /^[1-9]\d*$/.test(count || ''), 'usage: ui-click-audit.cjs RECEIPT_DIRECTORY SHARD_COUNT');
    const root = path.resolve(__dirname, '..');
    console.log(JSON.stringify(validateReceipts(readReceipts(path.resolve(directory)), makeRunContext(root), Number(count)), null, 2));
  } catch (error) { console.error(error.message); process.exitCode = 1; }
}
