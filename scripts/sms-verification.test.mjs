import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { readFile } from 'node:fs/promises';

test('SMS schema and exact production query invariants execute against SQLite', () => {
  const result = spawnSync('python3', ['scripts/sms-verification-contract/schema_test.py'], { cwd: new URL('../', import.meta.url), encoding: 'utf8' });
  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`);
});
test('SMS mutations bind to selected canonical identity authority, never global settings', async () => {
  const source = await readFile(new URL('../src/server/api/sms_settings.rs', import.meta.url), 'utf8');
  for (const required of ['CanonicalPgAuthority::bind', 'CanonicalSqliteAuthority::bind', 'verify_owner', 'sms_notification_preferences', 'sms_notification_dispatches', 'sms_verification_challenges', 'verify_slice', 'checked_acceptance']) assert.ok(source.includes(required), required);
  assert.ok(!source.includes('settings::Store::global'));
  assert.ok(!source.includes('get_pool()'));
  const sendStart = source.indexOf('async fn send(');
  const sendEnd = source.indexOf('\nasync fn ', sendStart + 1);
  assert.ok(sendStart >= 0 && sendEnd > sendStart, 'the production SMS send handler must be present');
  const send = source.slice(sendStart, sendEnd);
  const claim = send.indexOf('INSERT INTO sms_verification_challenges');
  assert.ok(claim >= 0, 'the send handler must persist its verification challenge');
  const commit = /tx\s*\.commit\(\)\s*\.await\?;/.exec(send.slice(claim));
  const provider = /service\s*\.provider\s*\.send_sms\(/.exec(send);
  assert.ok(commit && provider, 'the send handler must commit its claim and call the provider');
  assert.ok(claim + commit.index < provider.index, 'the durable sending claim must commit before provider I/O');
  const portable = await readFile(new URL('../src/server/persistence/migration.rs', import.meta.url), 'utf8');
  assert.ok(portable.includes('include_str!("sms_verification_sqlite.sql")'));
});
test('production mounts canonical SMS routes and consumers never use global fallback', async () => {
  const main = await readFile(new URL('../src/server/lib.rs', import.meta.url), 'utf8');
  assert.ok(main.includes('sms_settings::SmsService::configured(http_auth_store.clone())'));
  assert.ok(main.includes('.merge(api::sms_settings::router(sms_service).route_layer'));
  assert.ok(!main.includes('OTP_STORE'));
  assert.ok(!main.includes('pub async fn dispatch_critical_sms('));
  for (const path of ['src/server/services/booking.rs', 'src/server/services/subscription/service.rs', 'src/server/api/billing_webhook.rs']) {
    const source = await readFile(new URL(`../${path}`, import.meta.url), 'utf8');
    assert.ok(!source.includes('crate::dispatch_critical_sms('), path);
    assert.ok(source.includes('crate::api::sms_settings::dispatch_critical_sms('), path);
  }
  const booking = await readFile(new URL('../src/server/services/booking.rs', import.meta.url), 'utf8');
  const create = booking.slice(booking.indexOf('pub async fn create_booking'));
  assert.ok(create.indexOf('tx.commit().await') < create.indexOf('sms_settings::dispatch_critical_sms'));
});

test('SMS PostgreSQL claim fences current user, roles and opt-in until commit', async () => {
  const source = await readFile(new URL('../src/server/api/sms_settings.rs', import.meta.url), 'utf8');
  assert.ok(source.includes('FOR SHARE OF u,r FOR UPDATE OF p'));
});
test('volatile department acknowledgement cannot authorize an order SMS', async () => {
  const source = await readFile(new URL('../src/server/api/agents/webhook.rs', import.meta.url), 'utf8');
  assert.ok(!source.includes('sms_settings::dispatch_critical_sms('));
  assert.ok(source.includes('persistent_order_receipt_required'));
});
test('required SMS PostgreSQL gate is wired and cannot silently skip unavailable prerequisites', async () => {
  const root = new URL('../', import.meta.url);
  const runner = await readFile(new URL('scripts/sms-verification-contract/run.sh', root), 'utf8');
  const workflow = await readFile(new URL('.github/workflows/ci.yml', root), 'utf8');
  const gates = await readFile(new URL('scripts/focused_ci_gate.py', root), 'utf8');
  assert.ok(gates.includes("'sms-verification-contract': (29, 'OHC_SMS_TEST_DATABASE_URL')"));
  assert.ok(workflow.includes('python3 scripts/focused_ci_gate.py sms-verification-contract'));
  assert.ok(runner.includes('--locked --offline'));
  assert.ok(runner.includes('Required PostgreSQL test did not execute successfully'));
  const env = { ...process.env }; delete env.OHC_SMS_TEST_DATABASE_URL;
  const missing = spawnSync('bash', ['scripts/sms-verification-contract/run.sh'], { cwd: root, env, encoding: 'utf8' });
  assert.notEqual(missing.status, 0); assert.match(missing.stderr, /Explicit owned SMS PostgreSQL URL required/);
  const external = spawnSync('bash', ['scripts/sms-verification-contract/run.sh'], { cwd: root, env: { ...env, OHC_SMS_TEST_DATABASE_URL: 'postgres://user@provider.example/ohc_sms_test' }, encoding: 'utf8' });
  assert.notEqual(external.status, 0); assert.match(external.stderr, /explicit loopback PostgreSQL/);
});
test('required billing persistence test uses isolated storage instead of silent prerequisite returns', async () => {
  const source = await readFile(new URL('../src/server/api/billing_webhook_test.rs', import.meta.url), 'utf8');
  const block = source.split('async fn payment_failure_marks_subscriber_past_due_and_sends_dunning()')[1].split('#[tokio::test]')[0];
  assert.ok(block.includes('DbStore::Sqlite(sqlite.clone())'));
  assert.ok(!/\breturn\s*;/.test(block));
  assert.ok(!block.includes('get_multiplexed_async_connection'));
});
