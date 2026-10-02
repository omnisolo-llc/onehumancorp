import assert from 'node:assert/strict';
import { readFileSync, existsSync } from 'node:fs';
import { createRequire } from 'node:module';
import test from 'node:test';
const require = createRequire(import.meta.url);
const seed = readFileSync(new URL('../src/e2e/e2e-seed.sql', import.meta.url), 'utf8');
const factoryPath = new URL('./ui-audit-fixture.cjs', import.meta.url);
function factory() {
  assert.ok(existsSync(factoryPath), 'The click audit needs an owned real-database fixture factory');
  return require('./ui-audit-fixture.cjs');
}

test('canonical fixture data is separated from runner-only schema and security changes', () => {
  const blocks = [...seed.matchAll(/-- audit-fixture-data:start\n([\s\S]*?)-- audit-fixture-data:end/g)];
  assert.equal(blocks.length, 2, 'Both canonical data sections must be reusable without rerunning DDL');
  for (const [, sql] of blocks) assert.doesNotMatch(sql, /^\s*(?:ALTER|CREATE|DROP|DO|BEGIN|COMMIT)\b/im);
  const schema = seed.replace(/-- audit-fixture-data:start\n[\s\S]*?-- audit-fixture-data:end/g, '').replace(/\$\$[\s\S]*?\$\$/g, '');
  assert.doesNotMatch(schema, /^\s*(?:INSERT INTO|UPDATE|DELETE FROM)\b/im, 'New fixture data must remain in the owned data sections');
});

test('independent cases reuse all canonical data with disjoint keys and matching foreign references', () => {
  const { createOwnedAuditSeed } = factory();
  const first = createOwnedAuditSeed(seed, 'audit-11111111-1111-4111-8111-111111111111');
  const second = createOwnedAuditSeed(seed, 'audit-22222222-2222-4222-8222-222222222222');
  assert.notEqual(first.tenantId, second.tenantId);
  assert.notEqual(first.email, second.email);
  assert.equal(first.sourceStatementCount, second.sourceStatementCount);
  assert.ok(first.sourceStatementCount >= 25, 'Preserve the full canonical data graph');
  for (const item of [first, second]) {
    assert.doesNotMatch(item.sql, /^\s*(?:ALTER|CREATE|DROP|DO)\b/im);
    assert.doesNotMatch(item.sql, /ON CONFLICT/i, 'A supposedly fresh case must fail on collisions, never overwrite an existing record');
    assert.doesNotMatch(item.sql, /'e2e-tenant'|'test@example\.com'|'opp-test-[12]'|'bm-default-/);
    assert.doesNotMatch(item.sql, /648d7c4a-8f5b-4c3e-908f-7c6d5e4f3a2b|823e4567-e89b-12d3-a456-426614174000/);
    assert.ok(item.sql.includes(`'${item.tenantId}'`));
    assert.ok(item.sql.includes(`'${item.userId}'`));
    assert.ok(item.sql.includes(`'${item.email}'`));
    assert.ok(item.sql.includes(`"inbox_message_id":"${item.namespace}-e2e-inbox-msg-1"`));
    assert.ok(item.sql.includes(`'${item.namespace}-e2e-inbox-msg-1'`));
    assert.match(item.sql, /\$2b\$10\$hmVhun/); // Use the real canonical login hash.
  }
  assert.ok(!first.sql.includes(second.namespace));
  assert.ok(!second.sql.includes(first.namespace));
});

test('rejects unsafe namespaces and any schema operation smuggled into a data section', () => {
  const { createOwnedAuditSeed } = factory();
  for (const namespace of ['', 'e2e-tenant', "audit-x';DELETE FROM users;--"]) {
    assert.throws(() => createOwnedAuditSeed(seed, namespace), /namespace/);
  }
  assert.throws(() => createOwnedAuditSeed(seed.replace('-- audit-fixture-data:start\n', '-- audit-fixture-data:start\nALTER TABLE users DISABLE ROW LEVEL SECURITY;\n'), 'audit-11111111-1111-4111-8111-111111111111'), /schema/);
});


test('a later case must retain every baseline target, even after an earlier case deleted a record', () => {
  const { assertSameClickInventory } = factory();
  assert.equal(typeof assertSameClickInventory, 'function');
  const baseline = ['approve-record', 'edit-record', 'reject-record'];
  assert.doesNotThrow(() => assertSameClickInventory(baseline, [...baseline].reverse()));
  assert.throws(() => assertSameClickInventory(baseline, ['approve-record']), /edit-record.*reject-record/);
  assert.throws(() => assertSameClickInventory(baseline, [...baseline, 'unexpected']), /unexpected/);
  assert.throws(() => assertSameClickInventory(baseline, [...baseline, 'edit-record']), /duplicate/);
});
