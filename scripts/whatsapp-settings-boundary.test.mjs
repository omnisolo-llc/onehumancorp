import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const root = new URL('../', import.meta.url);
const wrapper = await readFile(new URL('src/server/api/settings/integrations/whatsapp.rs', root), 'utf8');
const canonical = await readFile(new URL('src/server/api/integrations_settings.rs', root), 'utf8');

test('mounted WhatsApp settings adapters cannot store plaintext or invent provider verification', () => {
  assert.doesNotMatch(wrapper, /sqlx::|integration_code|integration_credentials|'connected'|StatusCode::OK/);
  assert.match(wrapper, /verification_unavailable\(\)/);
  assert.match(wrapper, /organization_id/);
  assert.match(wrapper, /eq_ignore_ascii_case\("owner"\)/);
  assert.match(wrapper, /eq_ignore_ascii_case\("admin"\)/);
  assert.match(canonical, /StatusCode::NOT_IMPLEMENTED/);
  assert.match(canonical, /status: "pending_verification"/);
  assert.match(canonical, /usable: false/);
});

test('WhatsApp adapters and native negative HTTP tests are compiled by the settings module', async () => {
  const settings = await readFile(new URL('src/server/api/settings/mod.rs', root), 'utf8');
  const integrations = await readFile(new URL('src/server/api/settings/integrations/mod.rs', root), 'utf8');
  assert.match(settings, /pub mod integrations;/);
  assert.match(integrations, /#\[cfg\(test\)\]\s*pub mod whatsapp_test;/);
});

test('production settings routes mount the guarded adapters rather than an unverified storage path', async () => {
  const server = await readFile(new URL('src/server/lib.rs', root), 'utf8');
  assert.match(server, /\.route\("\/api\/v1\/settings\/integrations\/whatsapp_cloud_api", axum::routing::post\(api::settings::integrations::whatsapp::connect_whatsapp_cloud_api\)/);
  assert.match(server, /\.route\("\/api\/v1\/settings\/integrations\/whatsapp", axum::routing::post\(api::settings::integrations::whatsapp::connect_whatsapp_twilio\)/);
});
