import type { ActionPayload } from '@/lib/agent-feed-types';
export type DepartmentId = 'operations' | 'marketing' | 'sales' | 'customer_success' | 'finance' | 'legal' | 'business_advisory';
export type Approval = { id: string; tenant_id: string; department: DepartmentId; description: string; status: 'PENDING_APPROVAL' | 'APPROVED' | 'DISMISSED' | 'PAUSED'; action_risk: 'HIGH' | 'LOW'; payload?: ActionPayload; created_at?: string };
export type Decision = { state: 'APPROVED' | 'DISMISSED'; proposed_action?: ActionPayload };
export const object = (value: unknown): Record<string, unknown> | null => value !== null && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : null;
export const validId = (value: unknown): value is string => typeof value === 'string' && /^[A-Za-z0-9._-]{1,128}$/.test(value) && !['.', '..'].includes(value);
const departments: Record<string, DepartmentId> = { Operations: 'operations', Marketing: 'marketing', Sales: 'sales', CustomerSuccess: 'customer_success', Finance: 'finance', Legal: 'legal', BusinessAdvisory: 'business_advisory' };
const states: Record<string, Approval['status']> = { PendingApproval: 'PENDING_APPROVAL', Approved: 'APPROVED', Rejected: 'DISMISSED', Paused: 'PAUSED', PENDING_APPROVAL: 'PENDING_APPROVAL', APPROVED: 'APPROVED', REJECTED: 'DISMISSED', DISMISSED: 'DISMISSED', PAUSED: 'PAUSED' };
export function departmentId(value: unknown): DepartmentId | undefined { return typeof value === 'string' ? (Object.hasOwn(departments, value) ? departments[value] : Object.values(departments).find(id => id === value)) : undefined; }
export function readApproval(value: unknown, tenant: string): Approval {
  const row = object(value), department = departmentId(row?.department);
  const status = typeof row?.status === 'string' && Object.hasOwn(states, row.status) ? states[row.status] : undefined;
  const risk = row?.action_risk === 'DraftForReview' || row?.action_risk === 'HIGH' ? 'HIGH' : row?.action_risk === 'AutoExecute' || row?.action_risk === 'LOW' ? 'LOW' : null;
  if (!row || !validId(row.id) || row.tenant_id !== tenant || !department || !status || !risk || typeof row.description !== 'string' || !row.description.trim() || (row.payload != null && !object(row.payload))) throw Error('Approval data is unavailable or belongs to another account.');
  const payload = row.payload as ActionPayload | undefined;
  // These fields are rendered as text by the existing department review cards.
  for (const [key, value] of Object.entries(payload ?? {})) {
    if (['feature_type', 'description', 'original_message', 'generated_response', 'draft_reply', 'draft_copy', 'scope', 'service', 'service_name', 'media_url', 'image_url', 'instagram', 'facebook', 'tiktok', 'product_name', 'customer_inquiry', 'suggested_time', 'proposed_slot_id', 'product_id', 'suggested_action', 'reply'].includes(key) && value != null && typeof value !== 'string') throw Error('Approval payload is unavailable.');
    if (['suggested_price', 'remaining_stock'].includes(key) && value != null && (typeof value !== 'number' || !Number.isFinite(value))) throw Error('Approval payload is unavailable.');
  }
  return { id: row.id, tenant_id: tenant, department, description: row.description, status, action_risk: risk, ...(payload ? { payload } : {}) };
}
export function readApprovalPage(value: unknown, tenant: string): { items: Approval[]; next: string | null } {
  const data = object(value);
  if (!data || data.error != null || ('success' in data && data.success !== true) || !Array.isArray(data.pending_approvals) || (data.next_cursor != null && !validId(data.next_cursor))) throw Error('Approval list is unavailable.');
  const items = data.pending_approvals.map(item => readApproval(item, tenant));
  if (items.some(item => item.status !== 'PENDING_APPROVAL') || (data.next_cursor && !items.length)) throw Error('Approval list is unavailable.');
  return { items, next: data.next_cursor as string | null ?? null };
}
export function canonical(value: unknown): string {
  if (Array.isArray(value)) return '[' + value.map(canonical).join(',') + ']';
  const row = object(value);
  return row ? '{' + Object.keys(row).sort().map(key => JSON.stringify(key) + ':' + canonical(row[key])).join(',') + '}' : JSON.stringify(value);
}
export function readDecision(value: unknown, id: string, tenant: string, expected: Decision): string {
  const row = object(value), dispatch = object(row?.dispatch);
  if (!row || row.error != null || ('success' in row && row.success !== true) || row.id !== id || row.tenant_id !== tenant || row.lifecycle_state !== expected.state || row.decision_recorded !== true || !dispatch || typeof dispatch.status !== 'string' || !['NOT_REQUESTED','PENDING','ATTEMPTING','DISPATCH_RETURNED','RECONCILIATION_REQUIRED','CANCELLED'].includes(dispatch.status)) throw Error('Decision outcome is unconfirmed.');
  if (expected.proposed_action && canonical(row.proposed_action) !== canonical(expected.proposed_action)) throw Error('Edited decision outcome is unconfirmed.');
  if (['PENDING','ATTEMPTING','DISPATCH_RETURNED'].includes(dispatch.status) && !validId(dispatch.job_id)) throw Error('Dispatch receipt is incomplete.');
  const date = (value: unknown) => typeof value === 'string' && value.length > 0 && Number.isFinite(Date.parse(value));
  if (!Object.hasOwn(dispatch, 'job_id') || !(dispatch.job_id === null || validId(dispatch.job_id)) || !(dispatch.attempted_at === null || date(dispatch.attempted_at)) || !(dispatch.dispatch_returned_at === null || date(dispatch.dispatch_returned_at)) || !(dispatch.detail === null || typeof dispatch.detail === 'string')) throw Error('Dispatch receipt is incomplete.');
  if (['ATTEMPTING', 'DISPATCH_RETURNED'].includes(dispatch.status) && !date(dispatch.attempted_at)) throw Error('Dispatch attempt is unconfirmed.');
  if (dispatch.status === 'DISPATCH_RETURNED' && !date(dispatch.dispatch_returned_at)) throw Error('Dispatch return is unconfirmed.');
  if (expected.state === 'DISMISSED') return 'Dismissal recorded. This is not a delivery receipt.';
  switch (dispatch.status) {
    case 'PENDING': return 'Approval recorded; action queued. Execution or delivery is not verified.';
    case 'ATTEMPTING': return 'Approval recorded; dispatch is being attempted. Execution or delivery is not verified.';
    case 'DISPATCH_RETURNED': return 'Dispatch returned. Business execution or delivery is not verified.';
    case 'RECONCILIATION_REQUIRED': return 'Approval recorded; dispatch outcome is unknown and needs reconciliation. Do not resubmit.';
    case 'CANCELLED': return 'Approval recorded; dispatch was cancelled. No execution or delivery is verified.';
    default: return 'Approval recorded. No dispatch was requested; execution or delivery is not verified.';
  }
}
