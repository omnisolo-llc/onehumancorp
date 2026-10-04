import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { requireSubscriptionHealthDraft } from './subscription-health-readiness.mjs';

// Build observed evidence from the actual producer's persisted representation,
// not the enum/Serde spelling. A source mapping change must fail this contract.
const source = file => readFileSync(new URL(`../src/server/orchestration/departments/${file}`, import.meta.url), 'utf8');
const types = source('types.rs');
const display = types.slice(types.indexOf('impl std::fmt::Display for DepartmentType'), types.indexOf('pub struct DepartmentConfig'));
const departments = new Map([...display.matchAll(/DepartmentType::(\w+)\s*=>\s*"([^"]+)"/g)].map(match => [match[1], match[2]]));
const persistedDepartment = departments.get('CustomerSuccess');

test('subscription producer persists the CustomerSuccess Display value in both approval records', () => {
  assert.equal(persistedDepartment, 'customer_success');
  const agent = source('customer_success_agent.rs');
  const atRisk = agent.slice(agent.indexOf('if event.event_type == "tenant.subscription.at_risk"'), agent.indexOf('if event.event_type == "tenant.message.received"'));
  assert.match(atRisk, /\.execute_action\(\s*crate::orchestration::departments::types::DepartmentType::CustomerSuccess,/);
  const orchestrator = source('orchestrator.rs');
  const writer = orchestrator.slice(orchestrator.indexOf('pub async fn add_approval_request('), orchestrator.indexOf('// Publish SSE event'));
  for (const [table, column] of [['agent_feed_items', 'event_source'], ['agent_action_requests', 'department_type']]) {
    const insert = writer.match(new RegExp(String.raw`"INSERT INTO ${table} \(([^)]+)\)[^\n]+"([\s\S]*?)\.execute\(`));
    assert.ok(insert, `actual ${table} writer must be found`);
    const index = insert[1].split(',').map(value => value.trim()).indexOf(column);
    const binds = [...insert[2].matchAll(/^\s*\.bind\((.*)\)\s*$/gm)].map(match => match[1]);
    assert.ok(index >= 0, `actual ${column} column must be found`);
    assert.equal(binds[index], 'req.department.to_string()', `${table}.${column} must use the actual Display mapping`);
  }
});

const expected = { tenantId: 'owned-tenant', jobId: 'health-job', subscriberId: 'owned-subscriber', customerId: 'owned-customer', planId: 'owned-plan' };
function evidence() {
  const payload = { feature_type: 'subscription_win_back', subscriber_id: expected.subscriberId, customer_id: expected.customerId, health_score: 40, draft_reply: 'Test-only draft content', generated_response: 'Test-only draft content' };
  return [{
    job: { id: expected.jobId, tenant_id: expected.tenantId, job_type: 'subscription_health', status: 'COMPLETED', payload: { subscriber_id: expected.subscriberId, customer_id: expected.customerId } },
    subscriber: { id: expected.subscriberId, tenant_id: expected.tenantId, customer_id: expected.customerId, subscription_plan_id: expected.planId, status: 'PAST_DUE', health_score: 40 },
    customer: { id: expected.customerId, tenant_id: expected.tenantId },
    plan: { id: expected.planId, tenant_id: expected.tenantId },
    feed: { id: 'actual-action', tenant_id: expected.tenantId, event_source: persistedDepartment, lifecycle_state: 'PENDING_APPROVAL', context_payload: { description: `The Ambassador identified subscriber ${expected.subscriberId} as at-risk and drafted a win-back offer.` }, proposed_action: structuredClone(payload) },
    request: { id: 'actual-action', tenant_id: expected.tenantId, department_type: persistedDepartment, status: 'DRAFT', payload: structuredClone(payload) },
  }];
}

test('accepts one acknowledged worker result and returns its actual canonical draft', () => {
  const rows = evidence();
  assert.equal(requireSubscriptionHealthDraft(rows, expected), rows[0].feed);
});

for (const [name, mutate] of [
  ['legacy-only readiness before a canonical draft exists', rows => { rows[0].feed = null; }],
  ['legacy commit while the worker is still processing', rows => { rows[0].job.status = 'PROCESSING'; }],
  ['missing worker acknowledgement', rows => { rows[0].job = null; }],
  ['unprocessed subscriber health', rows => { rows[0].subscriber.health_score = 100; }],
  ['different subscriber in the job', rows => { rows[0].job.payload.subscriber_id = 'other-subscriber'; }],
  ['different customer in the draft', rows => { rows[0].feed.proposed_action.customer_id = 'other-customer'; }],
  ['a different canonical and legacy action', rows => { rows[0].request.id = 'other-action'; }],
  ['a stale approved canonical action', rows => { rows[0].feed.lifecycle_state = 'APPROVED'; }],
  ['a stale approved legacy action', rows => { rows[0].request.status = 'APPROVED'; }],
  ['mismatched canonical and legacy payloads', rows => { rows[0].request.payload.draft_reply = 'Old content'; }],
  ['missing actual draft content', rows => { rows[0].feed.proposed_action.draft_reply = ''; rows[0].request.payload.draft_reply = ''; }],
  ['duplicate action evidence', rows => { rows.push(structuredClone(rows[0])); }],
  ['no action evidence', rows => { rows.length = 0; }],
  ...['job', 'subscriber', 'customer', 'plan', 'feed', 'request'].map(kind => [`wrong-owner ${kind}`, rows => { rows[0][kind].tenant_id = 'other-tenant'; }]),
]) {
  test(`rejects ${name}`, () => {
    const rows = evidence(); mutate(rows);
    assert.throws(() => requireSubscriptionHealthDraft(rows, expected), /Subscription health readiness/);
  });
}

// Every other real department remains invalid, even if both representations
// agree with each other. Enum-name and case aliases are not stored wire values.
for (const department of [...departments.values()].filter(value => value !== persistedDepartment).concat(['CustomerSuccess', 'CUSTOMER_SUCCESS', ''])) {
  for (const [record, column] of [['feed', 'event_source'], ['request', 'department_type']]) {
    test(`rejects ${record} department ${JSON.stringify(department)}`, () => {
      const rows = evidence();
      rows[0][record][column] = department;
      assert.throws(() => requireSubscriptionHealthDraft(rows, expected), record === 'feed'
        ? /Subscription health readiness: wrong department/
        : /Subscription health readiness: wrong legacy department/);
    });
  }
}

test('rejects matching but incorrect departments in both records', () => {
  const rows = evidence();
  rows[0].feed.event_source = departments.get('Operations');
  rows[0].request.department_type = departments.get('Operations');
  assert.throws(() => requireSubscriptionHealthDraft(rows, expected), /Subscription health readiness: wrong department/);
});
