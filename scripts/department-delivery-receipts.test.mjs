import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, existsSync } from 'node:fs';
const read = (path) => readFileSync(new URL(`../${path}`, import.meta.url), 'utf8');
const agent = read('src/server/orchestration/departments/customer_success_agent.rs');
const orchestrator = read('src/server/orchestration/departments/orchestrator.rs');
test('approved Ambassador path awaits a canonical durable receipt and never preclaims sent', () => {
  const approved = agent.slice(agent.indexOf('if event.event_type == "agent:customer_success:approved"'), agent.indexOf('if event.event_type == "tenant.subscription.churn_risk"'));
  assert.match(approved, /message_delivery::dispatch/);
  assert.doesNotMatch(approved, /tokio::spawn|Sent response to customer|"sent"/);
  const ambassador = orchestrator.slice(orchestrator.indexOf('// If this is an Ambassador Reply approval'), orchestrator.indexOf('== Some("invoice_followup")', orchestrator.indexOf('// If this is an Ambassador Reply approval')));
  assert.doesNotMatch(ambassador, /"auto_replied"/);
});
test('ambassador preparation uses canonical inbox binding and cannot mint auto-execute authority', () => {
  const execute = orchestrator.slice(orchestrator.indexOf('pub async fn execute_action('),orchestrator.indexOf('pub async fn notify_owner('));
  assert.match(execute,/message_delivery::prepare/);
  assert.match(execute,/ActionRisk::DraftForReview/);
  assert.doesNotMatch(agent, /"Auto-replied to message:|update_inbox_message_status\(inbox_id, &event.tenant_id, "auto_replied"\)/);
});
test('canonical dispatch commits its unknown replay fence before provider invocation', () => {
  const path = new URL('../src/server/orchestration/departments/message_delivery.rs', import.meta.url);
  assert.ok(existsSync(path), 'durable department dispatch implementation is required');
  const source=readFileSync(path,'utf8');
  assert.match(source,/ON CONFLICT.*DO NOTHING/);
  assert.match(source,/provider_accepted/);
  assert.match(source,/delivery_unknown/);
  assert.ok(source.indexOf('tx.commit().await?; // Durable replay fence') < source.indexOf('provider.send('));
});
test('modern inbox adapter does not bypass canonical receipt authority', () => {
  const source=read('src/server/domain/inbox.rs');
  assert.doesNotMatch(source, /tokio::spawn|status = 'sent'|status = 'replied'/);
  assert.match(source,/canonical.*action|canonical.*identity/i);
});
test('dashboard and inbox distinguish provider acceptance from delivery', () => {
  for (const name of ['dashboard','inbox']) {
    const source=read(`src/ui/next/src/app/${name}/page.tsx`);
    assert.doesNotMatch(source,/✨ AI Handled/);
    assert.match(source,/messageDeliveryStatus/);
  }
});
test('new message triage drafts freeze provider routing before pending review', () => {
  const source=read('src/server/workers/message_triage_worker.rs');
  assert.match(source,/message_delivery::prepare/);
  assert.match(source,/record_pending_review/);
  assert.equal((source.match(/message_reply_payload\.clone\(\)\.unwrap_or_else/g)||[]).length,4);
});
test('inbox approval acknowledgement never substitutes for a send receipt', () => {
  const source=read('src/ui/next/src/app/inbox/page.tsx');
  assert.doesNotMatch(source,/Draft approved and sent\./);
  const ambassador=orchestrator.slice(orchestrator.indexOf('// If this is an Ambassador Reply approval'),orchestrator.indexOf('== Some("invoice_followup")',orchestrator.indexOf('// If this is an Ambassador Reply approval')));
  assert.match(ambassador,/message_delivery::dispatch/);
  assert.doesNotMatch(ambassador,/dispatch_event\(/);
});
test('existing feed-worker fixture rejects the new canonical provider boundary', () => {
  const source=read('scripts/agent-feed-decision-contract/prepare.py');
  assert.match(source,/pub async fn handle_approved_inbox_action\(_tenant:&str,_action:&str,_job:&str,_admitted:&serde_json::Value,_pool:&sqlx::PgPool\)->Result<\(\),String>\{panic!/);
});
