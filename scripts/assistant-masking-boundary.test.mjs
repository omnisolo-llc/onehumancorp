import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
const source = readFileSync('src/server/api/assistant.rs', 'utf8');
const handler = source.split('async fn update_assistant_settings(', 2)[1].split('\n#[derive', 1)[0];
test('unused masking is rejected before any supported settings write, without echo-success', () => {
  const rejection = handler.indexOf('if payload.get("observationMasking").is_some()');
  assert.ok(rejection >= 0 && rejection < handler.indexOf('INSERT INTO application_settings'));
  assert.match(handler.slice(rejection, handler.indexOf('let agent_name')), /return Err\(StatusCode::BAD_REQUEST\)/);
  assert.doesNotMatch(handler, /settings\.insert\("observationMasking"|let observation_masking/);
  assert.match(handler, /settings\.insert\("agentName"/);
});
test('mounted native regression preserves real persisted name and rejects mixed no-op writes', () => {
  const native = source.split('async fn text_settings_reject_unused_masking_and_preserve_real_name()', 2)[1].split('async fn task_mutations_use_database', 1)[0];
  for (const value of ['true', 'false', 'null']) assert.ok(native.includes(`"observationMasking": ${value}`));
  assert.match(native, /"agentName": "Must not replace", "observationMasking": true/);
  assert.match(native, /assert_eq!\(read, saved\)/);
  assert.match(native, /SELECT key, value, updated_by FROM application_settings/);
});
test('real browser coverage checks rejection, zero writes and PostgreSQL/reload name persistence', () => {
  const browser = readFileSync('src/ui/next/src/e2e/test_observation_masking.spec.ts', 'utf8');
  assert.match(browser, /expect\(response\.status\(\)\)\.toBe\(400\)/);
  assert.match(browser, /expect\(await savedRows\(\)\)\.toEqual\(\[\]\)/);
  assert.match(browser, /updated_by: owner\.userId/);
  assert.match(browser, /await page\.reload\(\)/);
  assert.doesNotMatch(browser, /route\.fulfill|\.route\(|test\.skip|force:\s*true/);
});
