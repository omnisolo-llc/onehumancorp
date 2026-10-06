import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, existsSync } from 'node:fs';
const read = path => readFileSync(new URL(`../${path}`, import.meta.url), 'utf8');
const assertManualCommitBeforeProvider = source => {
  const start = source.indexOf('async fn apply_with(');
  assert.ok(start >= 0, 'manual request implementation is required');
  const dispatch = source.slice(start);
  const claim = dispatch.indexOf('INSERT INTO department_message_dispatches');
  assert.ok(claim >= 0, 'manual dispatch must persist the shared fence');
  const commit = /tx\s*\.commit\(\)\s*\.await\?;/.exec(dispatch.slice(claim));
  const provider = /provider\s*\.send\(/.exec(dispatch);
  assert.ok(commit && provider, 'manual dispatch must commit and invoke the provider');
  assert.ok(claim + commit.index < provider.index, 'manual claim must commit before provider I/O');
};
test('manual route awaits durable explicit-user request instead of detached optimistic sends', () => {
  const source = read('src/server/lib.rs');
  const route = source.slice(source.indexOf('pub async fn update_ui_omni_inbox_action_handler('), source.indexOf('pub(crate) struct UiDashboardMetrics'));
  assert.match(route, /manual_inbox::apply/);
  assert.match(route, /&claims.sub/);
  assert.doesNotMatch(source, /dispatch_omni_reply|struct OmniReplyDispatch/);
  assert.doesNotMatch(route, /tokio::spawn|"success": true/);
});
test('manual immutable requests reuse provider receipt and shared no-replay fence', () => {
  const path = 'src/server/orchestration/departments/manual_inbox.rs';
  assert.ok(existsSync(new URL(`../${path}`, import.meta.url)), 'manual immutable request implementation required');
  const source = read(path);
  assert.match(source, /manual_inbox_requests/);
  assert.match(source, /department_message_dispatches/);
  assert.match(source, /actor_id/);
  assert.match(source, /provider\s*\.send/);
  assert.doesNotMatch(source, /INSERT INTO agent_(feed_items|action_requests)/);
  assertManualCommitBeforeProvider(source);
});
test('manual request schema forces tenant isolation and immutable routing', () => {
  const source = read('src/server/migrations/1045_manual_inbox_requests.sql');
  assert.match(source, /FORCE ROW LEVEL SECURITY/);
  for (const field of ['actor_id','request_id','inbox_message_id','payload_hash','provider_binding','recipient','body']) assert.match(source,new RegExp(field));
});
test('manual ownership uses durable revision and department review freezes it', () => {
  const source=read('src/server/orchestration/departments/manual_inbox.rs');
  assert.match(source,/manual_inbox_intents/);
  assert.match(source,/revision=manual_inbox_intents.revision\+1/);
  const department=read('src/server/orchestration/departments/message_delivery.rs');
  assert.match(department,/manual_intent_revision/);
});
test('exact manual request SQLite DDL and production SQL enforce receipts and ownership', async () => {
  const {execFileSync}=await import('node:child_process');
  const output=execFileSync('python3',[new URL('./department-delivery-contract/manual_sqlite_probe.py',import.meta.url).pathname],{encoding:'utf8'});
  assert.match(output,/8 exact SQLite checks passed/);
});
test('late department receipts preserve a newer manual dismissal', () => {
  const source=read('src/server/orchestration/departments/message_delivery.rs');
  assert.match(source,/!current\.terminal/);
});
test('mandatory native gate includes manual PostgreSQL concurrency and owned HTTP cases', () => {
  const gates=read('scripts/focused_ci_gate.py');
  assert.match(gates, /'department-delivery-contract': \(61, 'OHC_DEPARTMENT_TEST_DATABASE_URL'\)/);
  const runner=read('scripts/department-delivery-contract/run.sh');
  for (const name of ['pg_manual_concurrent_legacy_retry_cross_tab_and_rls','pg_manual_rejection_retires_old_department_approval_but_new_review_recovers','pg_manual_department_concurrent_attempts_share_one_effect_fence','manual_owned_http_validated_acceptance_and_malformed_rejection_unknown']) assert.ok(runner.includes(name));
  assert.match(runner,/--include-ignored --test-threads=1/);
  assert.match(read('.github/workflows/ci.yml'),/python3 scripts\/focused_ci_gate.py department-delivery-contract/);
});
test('retired department approval cannot overwrite newer manual draft/status', () => {
  const source=read('src/server/orchestration/departments/message_delivery.rs');
  assert.match(source,/if\s+!terminal\s*&&\s*!superseded\s*\{\s*set_inbox_state/);
});
test('mixed inbox mirrors retain independent terminal and historical evidence', () => {
  const source=read('src/server/orchestration/departments/message_delivery.rs');
  assert.match(source,/let\s+terminal\s*=\s*rows\s*\.iter\(\)\s*\.any/);
  assert.match(source,/if !current\.terminal/);
  assert.match(read('src/server/orchestration/departments/manual_inbox.rs'),/if !current\.terminal/);
});

test('manual committed-fence guard tolerates formatting and rejects missing required operations', () => {
  const source = read('src/server/orchestration/departments/manual_inbox.rs');
  const claim = source.indexOf('INSERT INTO department_message_dispatches');
  const commit = /tx\s*\.commit\(\)\s*\.await\?;/.exec(source.slice(claim));
  assert.ok(claim >= 0 && commit);
  const at = claim + commit.index;
  const without = source.slice(0, at) + source.slice(at + commit[0].length);
  assertManualCommitBeforeProvider(source);
  assertManualCommitBeforeProvider(source.slice(0, at) + 'tx\n    .commit()\n    .await?;' + source.slice(at + commit[0].length));
  assert.throws(() => assertManualCommitBeforeProvider(without));
  assert.throws(() => assertManualCommitBeforeProvider('tx.commit().await?;\n' + without));
  assert.throws(() => assertManualCommitBeforeProvider(source.replace('INSERT INTO department_message_dispatches', 'MISSING_FENCE')));
  assert.throws(() => assertManualCommitBeforeProvider(source.replace(/provider\s*\.send\(/, 'provider.missing(')));
});
