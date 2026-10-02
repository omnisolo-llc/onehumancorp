import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const source = await readFile(new URL('../src/server/integrations/registry.rs', import.meta.url), 'utf8');
const production = source.split('#[cfg(test)]')[0];
const connectStart = production.indexOf('    pub fn connect(');
const connectEnd = production.indexOf('    pub fn disconnect(', connectStart);
const connect = production.slice(connectStart, connectEnd);
const clone = (field) => `creds\\.${field}\\.clone\\(\\)`;

// Source contracts protect the credential handoff at every constructor, including
// providers that cannot be contacted by an offline test. They do not certify
// provider authentication, permissions, or successful external actions.
test('registry source retains every supplied credential in the connection record', () => {
  assert.ok(connectStart >= 0 && connectEnd > connectStart, 'locate the real connection method');
  const record = connect.match(/IntegrationCredentials\s*\{([^}]+)\}/)?.[1];
  assert.ok(record, 'connection must retain the supplied credentials');
  for (const field of ['bot_token', 'chat_id', 'webhook_url', 'api_token', 'from_phone', 'api_key', 'api_secret']) {
    assert.match(record, new RegExp(`\\b${field}:\\s*${clone(field)}\\s*,`), field);
  }
});

test('production registry contains no test-token substitution', () => {
  assert.equal(production.includes('"fake_token"'), false, 'test placeholders must not replace supplied production credentials');
});

const providers = [
  ['trello', 'TrelloProvider', ['api_token', 'bot_token']],
  ['twilio', 'TwilioProvider', ['bot_token', 'api_token']],
  ['meta', 'MetaProvider', ['api_token'], 2],
  ['calendly', 'CalendlyProvider', ['api_token']],
  ['cal_com', 'CalComProvider', ['api_token']],
  ['google_workspace', 'GoogleWorkspaceProvider', ['api_token']],
  ['google_calendar', 'GoogleCalendarProvider', ['api_token']],
  ['mailchimp', 'MailchimpProvider', ['api_token']],
  ['alipay', 'AlipayProvider', ['api_token']],
  ['mercadopago', 'MercadoPagoProvider', ['api_token']],
  ['razorpay', 'RazorpayProvider', ['api_key', 'api_secret']],
  ['shippo', 'ShippoProvider', ['api_token']],
  ['taxjar', 'TaxJarProvider', ['api_token']],
  ['zoom', 'ZoomProvider', ['api_token']],
  ['jitsi', 'JitsiProvider', ['api_token']],
  ['ayrshare', 'AyrshareProvider', ['api_token']],
  ['listmonk', 'ListmonkProvider', ['api_token']],
  ['doordash', 'DoorDashProvider', ['api_token']],
  ['easypost', 'EasyPostProvider', ['api_token']],
  ['manychat', 'ManychatProvider', ['api_token']],
  ['resend', 'ResendProvider', ['api_token']],
  ['sendgrid', 'SendGridProvider', ['api_token']],
  ['slack', 'SlackProvider', ['bot_token']],
  ['google_analytics', 'GoogleAnalyticsProvider', ['api_token', 'chat_id']],
  ['github_api', 'GitHubProvider', ['api_token']],
  ['outlook_calendar', 'OutlookCalendarProvider', ['api_token']],
];

for (const [module, provider, fields, expectedCalls = 1] of providers) {
  test(`registry source forwards supplied credentials to ${provider}`, () => {
    const constructor = new RegExp(`crate::integrations::${module}::provider::${provider}::new\\(`, 'g');
    const calls = [...production.matchAll(constructor)];
    assert.equal(calls.length, expectedCalls, 'retain each connection path without replacing clients during another operation');
    const argumentsPrefix = new RegExp(`^\\s*${fields.map(clone).join('\\s*,\\s*')}\\s*[,)]`);
    for (const call of calls) {
      assert.ok(call.index >= connectStart && call.index < connectEnd, 'only connect may initialize the credential-backed client');
      assert.ok(argumentsPrefix.test(production.slice(call.index + call[0].length)), `${provider} must use the supplied ${fields.join(', ')}`);
    }
  });
}

test('connection source validates supported configuration before mutating registry state', () => {
  const validation = connect.indexOf('validate_registry_connection(');
  assert.ok(validation >= 0, 'the real connection path must validate its inputs');
  assert.ok(validation < connect.indexOf('self.instances.write()'), 'invalid configuration must have no registry side effects');
  for (const field of ['bot_token', 'webhook_url', 'api_token', 'api_key', 'api_secret']) {
    assert.match(connect.slice(validation), new RegExp(`${field}:\\s*&creds\\.${field}`), field);
  }
  assert.match(connect.slice(validation), /\)\s*\.map_err\([^;]+\)\?;/);
});

test('connection source distinguishes unverified configuration from verified connection', () => {
  assert.ok(connect.includes('"configured".to_string()'));
  assert.equal(connect.includes('"connected".to_string()'), false);
  const testConnection = production.slice(production.indexOf('    pub fn test_connection('), production.indexOf('    pub fn chat_messages('));
  assert.ok(testConnection.includes('Err('), 'a provider verification result cannot be invented');
  assert.equal(testConnection.includes('Ok(())'), false, 'no unperformed provider check may succeed');
});

test('Razorpay configuration has explicit additive key and secret fields on the wire', async () => {
  const proto = await readFile(new URL('../src/proto/hub.proto', import.meta.url), 'utf8');
  const request = proto.match(/message ConnectIntegrationRequest\s*\{([^}]+)\}/)?.[1];
  assert.ok(request);
  assert.match(request, /string api_token = 6;/, 'retain the existing generic API token field');
  assert.match(request, /string api_key = 8;/);
  assert.match(request, /string api_secret = 9;/);
});
