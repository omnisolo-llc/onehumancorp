import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const source = await readFile(new URL('../src/server/lib.rs', import.meta.url), 'utf8');
const loader = source.slice(source.indexOf('    async fn load_ui_supply_from_db('), source.indexOf('    async fn load_ui_agent_approvals_from_db('));
const handler = source.slice(source.indexOf('    async fn list_ui_supply_handler('), source.indexOf('    async fn create_ui_supply_vendor_handler('));

test('supply reads propagate query and decode failures instead of fabricating empty rows', () => {
  assert.doesNotMatch(loader, /unwrap_or_default|unwrap_or_else|row\.get::/);
  assert.match(loader, /set_org_context\(&mut \*tx, tenant_id\)\.await\?/);
  assert.match(loader, /tx\.commit\(\)\.await\?/);
  for (const table of ['vendors', 'raw_materials', 'bom_items']) {
    assert.match(loader, new RegExp(`FROM ${table} WHERE tenant_id = \\$1`));
    assert.match(loader, new RegExp(`FROM ${table} WHERE tenant_id = \\?`));
  }
});

test('explicit supply refresh is authoritative and failure is an error envelope', () => {
  assert.doesNotMatch(handler, /get_or_fetch_with_swr|RowNotFound|"vendors":\s*\[\]/);
  assert.match(handler, /load_ui_supply_from_db\(&db, &tenant_id, mobile_optimized\)\.await/);
  assert.match(handler, /"error": "supply_unavailable"/);
  assert.match(handler, /"success": false/);
});

test('mobile supply retains the recorded threshold needed for real stock warnings', () => {
  assert.doesNotMatch(loader, /if !mobile_optimized\s*\{[^}]*reorder_threshold/);
  assert.match(loader, /"reorder_threshold": row\.try_get::<i32, _>\("reorder_threshold"\)\?/);
});
