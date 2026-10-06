import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import vm from 'node:vm';
import test from 'node:test';
import ts from 'typescript';

const source = readFileSync(new URL('../src/e2e/support/integration_storage.ts', import.meta.url), 'utf8');
const compiled = ts.transpileModule(source, {
  compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
}).outputText;
const plain = value => JSON.parse(JSON.stringify(value));

// Execute the actual helper at its guarded database boundary. These checks prove
// its snapshot/error contracts, not a provider connection or browser journey.
function fixture({ legacy = false, vault = false, failAt, error, legacyRows, schemaRows } = {}) {
  const queries = [];
  let inTransaction = false, tenantSet = false;
  const query = async (sql, values) => {
    queries.push(sql);
    if (failAt && sql.includes(failAt)) throw error;
    if (sql.startsWith('SET TRANSACTION')) {
      assert.equal(sql, 'SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY');
      return [];
    }
    if (sql.includes('set_config')) {
      assert.equal(inTransaction, true, 'tenant scope must be transaction-local');
      assert.deepEqual(plain(values), ['tenant-a']);
      tenantSet = true;
      assert.match(sql, /set_config\('app.current_tenant',\$1,true\)/);
      return [];
    }
    assert.equal(inTransaction && tenantSet, true, 'every storage read must use the scoped transaction');
    if (sql.includes('to_regclass')) return schemaRows ?? [{ legacy_exists: legacy, vault_exists: vault }];
    assert.match(sql, /^SELECT /, 'snapshots must not create missing tables or mutate rows');
    assert.doesNotMatch(sql, /\*|bot_token|api_token|nonce_hex|ciphertext_hex|integration_code/);
    assert.deepEqual(plain(values), ['tenant-a', ['openai_api']]);
    assert.match(sql, /tenant_id=\$1/);
    if (sql.includes('FROM tool_integrations')) {
      return [{ id: 'openai_api', name: 'OpenAI API', status: 'verification_required' }];
    }
    if (sql.includes('FROM ohc_provider_connections')) {
      assert.equal(vault, true, 'an absent lazy vault must not be queried');
      assert.match(sql, /provider=ANY\(\$2::text\[\]\)/);
      assert.match(sql, /SELECT provider, revision, state, verified_at /);
      return [{ provider: 'openai_api', revision: 'revision-1', state: 'verified', verified_at: '123' }];
    }
    if (sql.includes('FROM integration_credentials')) {
      assert.equal(legacy, true, 'an absent legacy table must not be queried');
      assert.match(sql, /integration_id=ANY\(\$2::text\[\]\)/);
      return legacyRows ?? [{ id: 'legacy-1', integration_id: 'openai_api', updated_at: '2026-10-06' }];
    }
    assert.fail(`Unexpected query: ${sql}`);
  };
  const exports = {};
  vm.runInNewContext(compiled, { exports, require: dependency => {
    assert.equal(dependency, '../db_utils');
    return {
      e2eDbQuery: query,
      e2eDbTransaction: async operation => {
        inTransaction = true; tenantSet = false;
        try { return await operation(query); } finally { inTransaction = false; }
      },
    };
  } });
  return { snapshot: () => exports.integrationStorage('tenant-a', ['openai_api']).then(plain), queries };
}

test('missing legacy and lazy vault tables remain explicitly absent without DDL', async () => {
  const f = fixture();
  const before = await f.snapshot();
  assert.deepEqual(before.legacyCredentials, { exists: false, rows: [] });
  assert.deepEqual(before.vaultConnections, { exists: false, rows: [] });
  assert.deepEqual(await f.snapshot(), before);
  assert.ok(f.queries.every(sql => !/FROM (integration_credentials|ohc_provider_connections)/.test(sql)));
});

test('existing supported vault and legacy records retain non-secret metadata', async () => {
  const f = fixture({ legacy: true, vault: true });
  const snapshot = await f.snapshot();
  assert.match(f.queries.find(sql => sql.includes('FROM tool_integrations')), /id=ANY\(\$2::text\[\]\)/, 'the API uses provider IDs, not display names');
  assert.deepEqual(snapshot.connections, [{ id: 'openai_api', name: 'OpenAI API', status: 'verification_required' }]);
  assert.deepEqual(snapshot.vaultConnections, {
    exists: true, rows: [{ provider: 'openai_api', revision: 'revision-1', state: 'verified', verified_at: '123' }],
  });
  assert.deepEqual(snapshot.legacyCredentials, {
    exists: true, rows: [{ id: 'legacy-1', integration_id: 'openai_api', updated_at: '2026-10-06' }],
  });
});

test('table creation is distinguishable from an absent table even when no legacy rows exist', async () => {
  const absent = await fixture().snapshot();
  const present = await fixture({ legacy: true, legacyRows: [] }).snapshot();
  assert.deepEqual(present.legacyCredentials, { exists: true, rows: [] });
  assert.notDeepEqual(present, absent);
});

for (const [failAt, code] of [
  ['to_regclass', '08006'], ['FROM tool_integrations', '42P01'],
  ['FROM integration_credentials', '42501'], ['FROM integration_credentials', '42P01'],
  ['FROM ohc_provider_connections', '42703'], ['set_config', '42501'],
]) {
  test(`query failure ${code} at ${failAt} is never converted to empty storage`, async () => {
    const error = Object.assign(new Error('genuine query failure'), { code });
    await assert.rejects(fixture({ legacy: true, vault: true, failAt, error }).snapshot, actual => actual === error);
  });
}

for (const schemaRows of [[], [{ legacy_exists: null, vault_exists: false }]]) {
  test('invalid schema inspection results cannot pass as absent storage', async () => {
    await assert.rejects(fixture({ schemaRows }).snapshot, /schema inspection returned no valid result/);
  });
}
