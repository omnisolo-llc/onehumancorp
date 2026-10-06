import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import protocol from './ui-click-audit.cjs';

function fixture(t) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'ohc-route-discovery-'));
  t.after(() => fs.rmSync(root, { recursive: true, force: true }));
  const add = name => {
    const file = path.join(root, 'src/ui/next/src/app', name);
    fs.mkdirSync(path.dirname(file), { recursive: true });
    fs.writeFileSync(file, 'export default function Page() { return null; }');
  };
  return { root, add };
}

test('route groups and parallel slots are pathless and future pages are rediscovered', t => {
  const { root, add } = fixture(t);
  add('page.tsx');
  assert.deepEqual(protocol.discoverAppRoutes(root), ['/']);
  for (const file of ['(main)/tasks/page.tsx', '(main)/(nested)/shifts/page.tsx',
    '(main)/tasks/@details/page.tsx', '(future)/notes/page.jsx', 'reports/page.js',
    'settings/page.ts', '%5Fpublic/page.tsx']) add(file);
  assert.deepEqual(protocol.discoverAppRoutes(root), ['/', '/_public', '/notes', '/reports', '/settings', '/shifts', '/tasks']);
});

test('grouped dynamic examples retain seeded orders and deduplicate static aliases', t => {
  const { root, add } = fixture(t);
  for (const file of ['(main)/orders/[id]/page.tsx', '(main)/quote/[id]/page.tsx',
    '(main)/quotes/[id]/page.tsx', '(docs)/help/[articleId]/page.tsx',
    'help/getting-started-1/page.tsx', '(public)/bio/[tenant]/page.tsx']) add(file);
  assert.deepEqual(protocol.discoverAppRoutes(root), ['/bio/default', '/help/getting-started-1', '/orders/e2e-seeded-record', '/quote/e2e-id', '/quotes/e2e-id']);
});

test('private subtrees and actual route handlers are excluded while api page paths remain routable', t => {
  const { root, add } = fixture(t);
  for (const file of ['page.tsx', '_private/secret/page.tsx', '(main)/_private/nested/page.tsx',
    '(main)/api/internal/page.tsx', 'docs/api/page.tsx', 'api/data/route.ts']) add(file);
  assert.deepEqual(protocol.discoverAppRoutes(root), ['/', '/api/internal', '/docs/api']);
  const entries = protocol.discoverAppRouteInventory(root);
  assert.equal(entries.length, 5);
  const excluded = entries.filter(entry => entry.exclusionReason);
  assert.equal(excluded.length, 2);
  assert.ok(excluded.every(entry => entry.auditRoute === null && entry.exclusionReason === 'private_directory'));
});

for (const [file, requirement] of [
  ['(main)/@modal/(.)photo/page.tsx', 'intercepting_route_requires_navigation_fixture'],
  ['(main)/@modal/(..)photo/page.tsx', 'intercepting_route_requires_navigation_fixture'],
  ['(main)/@modal/(...)photo/page.tsx', 'intercepting_route_requires_navigation_fixture'],
  ['articles/[slug]/page.tsx', 'dynamic_route_requires_fixture'],
  ['docs/[...slug]/page.tsx', 'dynamic_route_requires_fixture'],
  ['optional/[[...slug]]/page.tsx', 'dynamic_route_requires_fixture'],
  ['future/[id]/page.tsx', 'dynamic_route_requires_fixture'],
]) test(`new unclassified reachable page fails main discovery: ${file}`, t => {
  const { root, add } = fixture(t);
  add('page.tsx');
  assert.deepEqual(protocol.discoverAppRoutes(root), ['/']);
  add(file);
  assert.throws(() => protocol.discoverAppRoutes(root), error => {
    assert.match(error.message, /requires an explicit/);
    assert.ok(error.message.includes(file));
    assert.ok(error.message.includes(requirement));
    return true;
  });
  const pending = protocol.discoverAppRouteInventory(root).find(entry => entry.file.endsWith(file));
  assert.equal(pending.fixtureRequirement, requirement);
  assert.equal(pending.exclusionReason, null, 'a missing fixture is a failed gate, not an authorized exclusion');
  assert.equal(pending.auditRoute, null);
});

test('inventory retains every source behind a deduplicated route without claiming feature coverage', t => {
  const { root, add } = fixture(t);
  add('help/[articleId]/page.tsx');
  add('(docs)/help/getting-started-1/page.tsx');
  const entries = protocol.discoverAppRouteInventory(root);
  assert.equal(entries.length, 2);
  assert.deepEqual(entries.map(entry => entry.auditRoute), ['/help/getting-started-1', '/help/getting-started-1']);
  assert.deepEqual(entries.map(entry => entry.routePattern).sort(), ['/help/[articleId]', '/help/getting-started-1']);
  assert.ok(entries.every(entry => entry.exclusionReason === null));
  assert.deepEqual(protocol.discoverAppRoutes(root), ['/help/getting-started-1']);
});

test('current grouped task and shift pages stay in the required click inventory', () => {
  const routes = protocol.discoverAppRoutes(path.resolve(import.meta.dirname, '..'));
  assert.ok(routes.includes('/tasks'));
  assert.ok(routes.includes('/shifts'));
  assert.equal(routes.filter(route => route === '/help/getting-started-1').length, 1);
});

test('static route names never resolve through inherited dynamic-example properties', t => {
  const { root, add } = fixture(t);
  for (const name of ['constructor', 'toString', 'hasOwnProperty']) add(`${name}/page.tsx`);
  assert.deepEqual(protocol.discoverAppRoutes(root), ['/constructor', '/hasOwnProperty', '/toString']);
});
