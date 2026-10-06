import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

test('production inventory adjustment never creates a catalog product or a fixture balance', async () => {
  const source = await readFile(new URL('../src/server/api/pos.rs', import.meta.url), 'utf8');
  const start = source.indexOf('pub async fn post_inventory_handler');
  const end = source.indexOf('async fn get_orders_handler', start);
  const implementation = await readFile(new URL('../src/server/api/pos_inventory.rs', import.meta.url), 'utf8');
  const handler = source.slice(start, end) + implementation;
  assert.doesNotMatch(handler, /INSERT INTO products|Chocolate Cake|12 \+|2500, 12/);
  assert.doesNotMatch(handler, /let _ = tx\.commit|unwrap_or\(0\) as i32/);
});

test('inventory failure and emptiness cannot select fixture stock', async () => {
  const source = await readFile(new URL('../src/ui/next/src/app/inventory/page.tsx', import.meta.url), 'utf8');
  assert.doesNotMatch(source, /e2e-product-cake|Chocolate Cake|business_display_name/);
});

test('required hosted CI executes every PostgreSQL inventory case with restricted roles', async () => {
  const workflow = await readFile(new URL('../.github/workflows/ci.yml', import.meta.url), 'utf8');
  const runner = await readFile(new URL('./run-inventory-postgres.sh', import.meta.url), 'utf8');
  const fixture = await readFile(new URL('../src/server/api/pos_inventory_test.rs', import.meta.url), 'utf8');
  assert.match(workflow, /name: Verify inventory adjustments against restricted PostgreSQL roles/);
  assert.match(workflow, /OHC_INVENTORY_TEST_DATABASE_URL: postgres:\/\/postgres:ignored_fixture@127\.0\.0\.1:/);
  assert.match(workflow, /createdb[^\n]+ohc_inventory_test\n\s+bash scripts\/run-inventory-postgres\.sh/);
  assert.match(runner, /OHC_INVENTORY_TEST_DATABASE_URL:\?Supply an isolated/);
  assert.match(runner, /api::pos::inventory::tests::postgres_ -- --ignored --test-threads=2/);
  assert.match(runner, /sorted\(executed\)==sorted\(expected\)/);
  assert.match(fixture, /NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE NOINHERIT/);
  assert.match(fixture, /FORCE ROW LEVEL SECURITY/);
});

test('MySQL observed versions include a durable revision beyond rounded timestamps', async () => {
  const source = await readFile(new URL('../src/server/api/pos_inventory.rs', import.meta.url), 'utf8');
  assert.match(source, /fn mysql_state\(stock: i32, updated: Option<String>, revision: i64\)/);
  assert.match(source, /COUNT\(\*\) FROM inventory_adjustment_receipts r WHERE r\.tenant_id=p\.tenant_id AND r\.item_id=p\.id/);
  assert.match(source, /revision\s*\.checked_add\(1\)/);
});

test('durable receipt lookup precedes product existence and never dispatches a mutation', async () => {
  const source = await readFile(new URL('../src/server/api/pos_inventory.rs', import.meta.url), 'utf8');
  const postgres = source.slice(source.indexOf('pub async fn apply_postgres'), source.indexOf('pub async fn read_postgres'));
  const mysql = source.slice(source.indexOf('pub async fn apply_mysql'), source.indexOf('pub async fn receipt_postgres'));
  assert.ok(postgres.indexOf('if let Some(saved)') < postgres.indexOf('let product: Option<Value>'));
  assert.ok(mysql.indexOf('if let Some(saved)') < mysql.indexOf('let state: Option'));
  assert.match(postgres, /A concurrent identical request may have committed/);
  const lookups = source.slice(source.indexOf('pub async fn receipt_postgres'), source.indexOf('#[cfg(test)]'));
  assert.doesNotMatch(lookups, /INSERT INTO|UPDATE products|apply_postgres|apply_mysql/);
  assert.match(source, /Error::Unconfirmed\("request_identity_changed"\)/);
  const fixture = await readFile(new URL('../src/server/api/pos_inventory_test.rs', import.meta.url), 'utf8');
  assert.match(fixture, /async fn postgres_receipt_lookup_and_replay_survive_later_product_deletion/);
});

test('post-lock missing products are interpreted only after the final receipt recheck', async () => {
  const source = await readFile(new URL('../src/server/api/pos_inventory.rs', import.meta.url), 'utf8');
  const pg = source.slice(source.indexOf('pub async fn apply_postgres'), source.indexOf('pub async fn read_postgres'));
  const mysql = source.slice(source.indexOf('pub async fn apply_mysql'), source.indexOf('pub async fn receipt_postgres'));
  for (const body of [pg, mysql]) {
    const rechecks = [...body.matchAll(/if let Some\(saved\)/g)];
    assert.ok(rechecks.length >= 2);
    assert.ok(body.indexOf('ok_or(Error::Blocked("product_not_found"))') > rechecks[1].index, 'a committed receipt must win even when a queued delete wins the product lock');
  }
});

test('PN oversell debt cannot be turned into invented available stock', async () => {
  const source = await readFile(new URL('../src/server/api/pos_inventory.rs', import.meta.url), 'utf8');
  assert.match(source, /if p < n\s*\{\s*return Err\(Error::Blocked\("inventory_debt_requires_reconciliation"\)\);\s*\}/);
  assert.doesNotMatch(source, /p\.checked_sub\(n\)\.map\(\|v\| v\.max\(0\)\)/);
});
