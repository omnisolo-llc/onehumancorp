import { expect, test } from 'vitest';
import { readApproval, readDecision } from './approvalContract';
const item = { id: 'id-1', tenant_id: 'tenant-a', department: 'Sales', status: 'PendingApproval', action_risk: 'DraftForReview', description: 'Review saved request', payload: { feature_type: 'quote_draft', suggested_price: 20, scope: 'Repair' } };
test.each(['__proto__', 'constructor', 'toString'])('unrecognized enum %s never grants authority', value => {
  expect(() => readApproval({ ...item, department: value }, 'tenant-a')).toThrow();
  expect(() => readApproval({ ...item, status: value }, 'tenant-a')).toThrow();
});
test.each([{}, { status: 'DISPATCH_RETURNED', job_id: 'job-1' }, { status: 'ATTEMPTING', job_id: 'job-1', attempted_at: 'invalid', dispatch_returned_at: null, detail: null }])('incomplete dispatch receipt is held: %j', dispatch => {
  expect(() => readDecision({ id: 'id-1', tenant_id: 'tenant-a', lifecycle_state: 'APPROVED', decision_recorded: true, dispatch }, 'id-1', 'tenant-a', { state: 'APPROVED' })).toThrow();
});
