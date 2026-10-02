import { randomUUID } from 'node:crypto';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { test, expect } from './fixtures';
import { e2eDbQuery, e2eDbTransaction } from './db_utils';

// Execute the actual production SQL against PostgreSQL's real types, locking
// and uniqueness rules. This is a storage contract, not a mocked worker run.
test('recorded operations storage enforces tenant facts, concurrent dedupe and source rechecks', async () => {
  const source = readFileSync(path.resolve(__dirname, '../server/workers/proactive_operations_worker.rs'), 'utf8');
  const sql = (name: string) => {
    const match = source.match(new RegExp(`const ${name}: &str = r#"([\\s\\S]*?)"#;`));
    if (!match) throw new Error(`Production operation SQL ${name} is missing`);
    return match[1];
  };
  const schema = `ops_probe_${randomUUID().replaceAll('-', '')}`;
  // These helpers refuse any DB without the native runner's disposable proof.
  // No fallback to public tables is allowed on either independent connection.
  await e2eDbQuery(`CREATE SCHEMA "${schema}"`);
  const scoped = <T>(operation: (query: typeof e2eDbQuery) => Promise<T>) => e2eDbTransaction(async query => {
    await query(`SET LOCAL search_path TO "${schema}", pg_catalog`);
    return operation(query);
  });
  try {
    await scoped(async query => {
      await query(`CREATE TABLE raw_materials(id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT,
        current_quantity INTEGER, reorder_threshold INTEGER, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ);
        CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, event_source TEXT,
        context_payload JSONB, proposed_action JSONB, lifecycle_state TEXT,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP)`);
      await query(`INSERT INTO raw_materials VALUES
        ('source-a','a','Recorded flour',2,5,'2026-01-01 00:00:00+00','2026-01-01 00:00:00+00'),
        ('healthy-a','a','Recorded sugar',10,5,'2026-01-01 00:00:00+00','2026-01-01 00:00:00+00'),
        ('unconfigured-a','a','Unconfigured item',0,0,'2026-01-01 00:00:00+00','2026-01-01 00:00:00+00'),
        ('missing-revision','a','Unversioned item',1,5,NULL,NULL),
        ('source-b','b','Another tenant material',1,8,'2026-01-01 00:00:00+00','2026-01-01 00:00:00+00')`);
    });
    const observed = await scoped(async query => {
      await query("SET LOCAL TIME ZONE 'UTC'");
      const rows = await query(sql('STOCK_POSTGRES'), ['a', 100]);
      expect(rows).toHaveLength(1);
      expect(rows[0].id).toBe('source-a');
      await query("SET LOCAL TIME ZONE 'Pacific/Honolulu'");
      expect((await query(sql('STOCK_POSTGRES'), ['a', 100]))[0].version).toBe(rows[0].version);
      return rows[0];
    });
    const evidence = JSON.stringify({ feature_type: 'proactive_ops', source_table: 'raw_materials', source_id: observed.id,
      source_version: observed.version, quantity: Number(observed.quantity), threshold: Number(observed.threshold) });
    const parameters = ['same-source-key', 'a', evidence, '{}', observed.id, observed.quantity, observed.threshold, observed.version];
    await Promise.all([
      scoped(query => query(sql('INSERT_POSTGRES'), parameters)),
      scoped(query => query(sql('INSERT_POSTGRES'), parameters)),
    ]);
    await scoped(async query => {
      expect(await query('SELECT id,tenant_id FROM agent_feed_items')).toEqual([{ id: 'same-source-key', tenant_id: 'a' }]);
      expect(await query(sql('STOCK_POSTGRES'), ['a', 100])).toEqual([]);
      expect((await query(sql('STOCK_POSTGRES'), ['b', 100])).map(row => row.id)).toEqual(['source-b']);
      await query(sql('INSERT_POSTGRES'), ['wrong-tenant', 'b', evidence, '{}', observed.id, observed.quantity, observed.threshold, observed.version]);
      expect(await query("SELECT id FROM agent_feed_items WHERE id='wrong-tenant'")).toEqual([]);
      await query("UPDATE raw_materials SET current_quantity=8,updated_at='2026-01-02 00:00:00+00' WHERE id='source-a'");
      await query(sql('RETIRE_POSTGRES'), ['a', 100]);
      expect(await query("SELECT lifecycle_state FROM agent_feed_items WHERE id='same-source-key'")).toEqual([{ lifecycle_state: 'PAUSED' }]);
      await query(sql('INSERT_POSTGRES'), ['stale-source', ...parameters.slice(1)]);
      expect(await query("SELECT id FROM agent_feed_items WHERE id='stale-source'")).toEqual([]);
      await query("UPDATE raw_materials SET current_quantity=2,updated_at='2026-01-03 00:00:00+00' WHERE id='source-a'");
      const latest = (await query(sql('STOCK_POSTGRES'), ['a', 100]))[0];
      expect(latest.version).not.toBe(observed.version);
      const revised = JSON.stringify({ ...JSON.parse(evidence), source_version: latest.version });
      await query(sql('INSERT_POSTGRES'), ['new-source-version', 'a', revised, '{}', latest.id, latest.quantity, latest.threshold, latest.version]);
      await query("UPDATE agent_feed_items SET lifecycle_state='APPROVED' WHERE id='new-source-version'");
      await query("UPDATE raw_materials SET current_quantity=9,updated_at='2026-01-04 00:00:00+00' WHERE id='source-a'");
      await query(sql('RETIRE_POSTGRES'), ['a', 100]);
      expect(await query("SELECT lifecycle_state FROM agent_feed_items WHERE id='new-source-version'")).toEqual([{ lifecycle_state: 'APPROVED' }]);
    });
  } finally { await e2eDbQuery(`DROP SCHEMA "${schema}" CASCADE`); }
});
