// Test-only acknowledgement contract shared by the real browser scenario and
// deterministic regressions. It validates observed rows; it never creates work.
import assert from 'node:assert/strict';

export function requireSubscriptionHealthDraft(rows, expected) {
  const message = detail => `Subscription health readiness: ${detail}`;
  assert.equal(rows.length, 1, message('one exact worker result is required'));
  const { job, subscriber, customer, plan, feed, request } = rows[0];
  for (const [name, row] of Object.entries({ job, subscriber, customer, plan, feed, request })) {
    assert.ok(row, message(`${name} is missing`));
    assert.equal(row.tenant_id, expected.tenantId, message(`${name} belongs to another tenant`));
  }
  assert.equal(job.id, expected.jobId, message('job identity mismatch'));
  assert.equal(job.job_type, 'subscription_health', message('wrong worker'));
  assert.equal(job.status, 'COMPLETED', message('worker has not acknowledged completion'));
  assert.deepEqual(job.payload, { subscriber_id: expected.subscriberId, customer_id: expected.customerId }, message('job input mismatch'));
  assert.equal(subscriber.id, expected.subscriberId, message('subscriber identity mismatch'));
  assert.equal(subscriber.customer_id, expected.customerId, message('subscriber customer mismatch'));
  assert.equal(subscriber.subscription_plan_id, expected.planId, message('subscriber plan mismatch'));
  assert.equal(subscriber.status, 'PAST_DUE', message('subscriber status mismatch'));
  assert.equal(subscriber.health_score, 40, message('worker health score is not persisted'));
  assert.equal(customer.id, expected.customerId, message('customer identity mismatch'));
  assert.equal(plan.id, expected.planId, message('plan identity mismatch'));
  assert.ok(typeof feed.id === 'string' && feed.id.length > 0, message('canonical action identity missing'));
  assert.equal(request.id, feed.id, message('legacy and canonical actions differ'));
  // DepartmentType::Display is persisted by the orchestrator in both rows.
  assert.equal(feed.event_source, 'customer_success', message('wrong department'));
  assert.equal(request.department_type, 'customer_success', message('wrong legacy department'));
  assert.equal(feed.lifecycle_state, 'PENDING_APPROVAL', message('canonical draft is not awaiting approval'));
  assert.equal(request.status, 'DRAFT', message('legacy draft is not awaiting approval'));
  assert.equal(feed.context_payload?.description,
    `The Ambassador identified subscriber ${expected.subscriberId} as at-risk and drafted a win-back offer.`,
    message('canonical description does not identify this subscriber'));
  const payload = feed.proposed_action;
  assert.equal(payload?.feature_type, 'subscription_win_back', message('wrong proposed action'));
  assert.equal(payload?.subscriber_id, expected.subscriberId, message('draft subscriber mismatch'));
  assert.equal(payload?.customer_id, expected.customerId, message('draft customer mismatch'));
  assert.equal(payload?.health_score, subscriber.health_score, message('draft health score mismatch'));
  assert.ok(typeof payload?.draft_reply === 'string' && payload.draft_reply.trim(), message('actual draft content missing'));
  assert.equal(payload.generated_response, payload.draft_reply, message('draft content mismatch'));
  assert.deepEqual(request.payload, payload, message('legacy and canonical payloads differ'));
  return feed;
}
