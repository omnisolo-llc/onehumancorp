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
  for (const field of ['bot_token', 'chat_id', 'webhook_url', 'api_token', 'from_phone']) {
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
  ['razorpay', 'RazorpayProvider', ['api_token', 'api_token']],
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
