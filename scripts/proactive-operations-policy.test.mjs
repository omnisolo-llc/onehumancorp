import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const source = name => readFileSync(new URL(`../src/server/workers/${name}.rs`, import.meta.url), 'utf8');

test('production proactive operations never manufactures supplier or staffing facts', () => {
  const worker = source('proactive_operations_worker');
  assert.doesNotMatch(worker, /Follow up on delayed supplier delivery from yesterday|Only 1 person scheduled for closing shift|Generate the 3 specific CUJ tasks/);
});

test('the active daily routine never invents an unconfigured checklist', () => {
  const worker = source('daily_ops_routine_worker');
  assert.doesNotMatch(worker, /"feature_type": "daily_prep_checklist"|"message": "Review Daily Prep Checklist"/);
  assert.match(worker, /booking_reengagement_check/);
  assert.match(worker, /FROM bookings/);
});

test('the unmounted legacy name is an alias, not another synthetic generator', () => {
  const modules = readFileSync(new URL('../src/server/workers/mod.rs', import.meta.url), 'utf8');
  assert.doesNotMatch(modules, /pub\s+mod\s+daily_operations_worker\s*;/);
  const legacy = source('daily_operations_worker');
  assert.match(legacy, /pub use super::proactive_operations_worker::ProactiveOperationsWorker as DailyOperationsWorker;/);
  assert.doesNotMatch(legacy, /INSERT INTO|Uuid::new_v4|tokio::spawn/);
});
