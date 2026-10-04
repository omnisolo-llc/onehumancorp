import test from 'node:test';
import assert from 'node:assert/strict';
import { requireSubscriptionHealthDraft } from './subscription-health-readiness.mjs';

const expected = { tenantId: 'owned-tenant', jobId: 'health-job', subscriberId: 'owned-subscriber', customerId: 'owned-customer', planId: 'owned-plan' };
function evidence() {
  const payload = { feature_type: 'subscription_win_back', subscriber_id: expected.subscriberId, customer_id: expected.customerId, health_score: 40, draft_reply: 'Test-only draft content', generated_response: 'Test-only draft content' };
  return [{
    job: { id: expected.jobId, tenant_id: expected.tenantId, job_type: 'subscription_health', status: 'COMPLETED', payload: { subscriber_id: expected.subscriberId, customer_id: expected.customerId } },
    subscriber: { id: expected.subscriberId, tenant_id: expected.tenantId, customer_id: expected.customerId, subscription_plan_id: expected.planId, status: 'PAST_DUE', health_score: 40 },
    customer: { id: expected.customerId, tenant_id: expected.tenantId },
    plan: { id: expected.planId, tenant_id: expected.tenantId },
    feed: { id: 'actual-action', tenant_id: expected.tenantId, event_source: 'CustomerSuccess', lifecycle_state: 'PENDING_APPROVAL', context_payload: { description: `The Ambassador identified subscriber ${expected.subscriberId} as at-risk and drafted a win-back offer.` }, proposed_action: structuredClone(payload) },
    request: { id: 'actual-action', tenant_id: expected.tenantId, status: 'DRAFT', payload: structuredClone(payload) },
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
