import { readQueueOwner, sameOwner, type QueueOwner } from '@/lib/sync/queueIdentity';

export type Appointment = {
  id: string;
  customer_id: string;
  customer_name: string;
  job_template_id: string;
  job_name: string;
  status: string;
  scheduled_start_time: string | null;
  scheduled_end_time: string | null;
  updated_at?: string | null;
  location_address?: string | null;
  location_lat?: number | null;
  location_lng?: number | null;
  notes?: string | null;
  version?: number;
  base_version?: number;
  actual_start_time?: string;
  actual_end_time?: string;
  tenant_id?: string;
};
export type AppointmentWrite = {
  id: string;
  status: string;
  expected_updated_at: string;
  notes?: string;
  scheduled_start_time?: string | null;
  scheduled_end_time?: string | null;
};
const statuses = new Set(['Requested', 'Pending', 'Scheduled', 'Confirmed', 'En-Route', 'In-Progress', 'Completed', 'Cancelled']);
function canonicalReadStatus(value: unknown): string | null {
  if (typeof value !== 'string') return null;
  if (statuses.has(value)) return value;
  // The persisted API deliberately retains these historical terminal spellings.
  // Normalize only their display/route snapshots, never reopen them or widen writes.
  if (['completed', 'done'].includes(value.toLowerCase())) return 'Completed';
  if (['cancelled', 'canceled'].includes(value.toLowerCase())) return 'Cancelled';
  return null;
}
export const terminalJob = (job: Appointment) => job.status === 'Completed' || job.status === 'Cancelled';
const object = (value: unknown): Record<string, unknown> => {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Invalid schedule response. Reload to verify the saved schedule.');
  return value as Record<string, unknown>;
};
const timestamp = (value: unknown): value is string => typeof value === 'string' && /^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d(?:\.\d+)?(?:Z|[+-]\d\d:\d\d)$/.test(value) && Number.isFinite(Date.parse(value));
const nullableTime = (value: unknown) => value === null || timestamp(value);
export function readAppointments(body: unknown): Appointment[] {
  const data = object(body);
  if (data.error != null || data.success === false || !Array.isArray(data.appointments)) throw new Error('Invalid schedule response. Reload to verify the saved schedule.');
  const ids = new Set<string>();
  return data.appointments.map(value => {
    const row = object(value);
    if (!['id', 'customer_id', 'customer_name', 'job_template_id', 'job_name', 'status'].every(key => typeof row[key] === 'string') || !row.id || ids.has(row.id as string) || !canonicalReadStatus(row.status) || !nullableTime(row.scheduled_start_time) || !nullableTime(row.scheduled_end_time) || (row.updated_at != null && !timestamp(row.updated_at)) || (row.notes != null && typeof row.notes !== 'string')) throw new Error('Invalid appointment response. Reload to verify the saved schedule.');
    ids.add(row.id as string);
    return { ...row, status: canonicalReadStatus(row.status)! } as Appointment;
  });
}
export function observedTime(job: Appointment): string {
  if (!timestamp(job.updated_at)) throw new Error('This job has no verified saved version. Reload the schedule before changing it.');
  return job.updated_at;
}
export function readAppointmentUpdate(body: unknown, original: Appointment, requested: AppointmentWrite): Appointment {
  const data = object(body);
  if (data.success !== true || data.error != null || data.id !== requested.id || data.status !== requested.status || !timestamp(data.updated_at) || !nullableTime(data.scheduled_start_time) || !nullableTime(data.scheduled_end_time) || (data.notes != null && typeof data.notes !== 'string') || ('notes' in requested && data.notes !== requested.notes) || (['scheduled_start_time', 'scheduled_end_time'] as const).some(key => key in requested && (requested[key] === null ? data[key] !== null : !timestamp(data[key]) || Date.parse(data[key]) !== Date.parse(requested[key]!)))) throw new Error('The saved change could not be confirmed. Keep these inputs and reload the schedule before making another change.');
  // Display fields come from the observed row; mutation fields only from its acknowledgement.
  return { ...original, status: data.status as string, notes: (data.notes ?? null) as string | null, location_lat: data.location_lat as number | null | undefined, location_lng: data.location_lng as number | null | undefined, scheduled_start_time: data.scheduled_start_time as string | null, scheduled_end_time: data.scheduled_end_time as string | null, updated_at: data.updated_at };
}
export function readRouteProposal(body: unknown, observed: Appointment[], committed: boolean): Appointment[] {
  const data = object(body);
  if (data.success !== true || data.error != null || data.committed !== committed || (committed && (typeof data.routeId !== 'string' || !data.routeId.trim())) || (!committed && data.routeId != null)) throw new Error('The route result could not be confirmed. Your current schedule has been kept.');
  const rows = readAppointments({ appointments: data.optimizedRoute });
  if (rows.length !== observed.length || rows.some(row => {
    const before = observed.find(job => job.id === row.id);
    return !before || row.updated_at !== before.updated_at || row.status !== before.status;
  })) throw new Error('The schedule changed while calculating this proposal. Reload and review it again.');
  return rows;
}
export async function readFieldOwner(): Promise<QueueOwner> {
  try { return await readQueueOwner(); }
  catch { throw new Error('Your session could not be verified. Reload the schedule.'); }
}

/** Verified identity is a precondition. Only the sealed proxy session authorizes access. */
export async function fetchFieldJob(url: string, options: RequestInit, owner: QueueOwner, isCurrent: () => boolean, onDispatch?: () => void): Promise<unknown> {
  if (!['/api/v1/field-ops/appointments', '/api/v1/field-ops/optimize-route', '/api/v1/field-ops/running-late', '/api/v1/quotes/draft_agent'].includes(url)) throw new Error('Invalid field action destination');
  if (!isCurrent()) throw new Error('Your session or selection changed. Reopen this view.');
  const verified = await readFieldOwner();
  if (!isCurrent() || !sameOwner(verified, owner)) throw new Error('Your session changed. Reopen this view.');
  const headers = new Headers(options.headers);
  headers.set('x-ohc-expected-user', owner.userId); headers.set('x-ohc-expected-tenant', owner.tenantId);
  onDispatch?.();
  if (!isCurrent()) throw new Error('Your session or selection changed. This request was not sent.');
  const response = await fetch(url, { ...options, headers, credentials: 'same-origin', redirect: 'error', cache: 'no-store' });
  if (!isCurrent()) throw new Error('Your session or selection changed. Reopen this view.');
  if (response.status === 401 || response.status === 403) throw new Error('Your session no longer permits this request. Reload to verify your business.');
  if (!response.ok) throw new Error(response.status === 409 ? 'This job changed or the request conflicts with a saved change. Reload and review the schedule.' : response.status === 404 ? 'This job is no longer available in your business.' : `The change could not be confirmed (HTTP ${response.status}). Your inputs have been kept.`);
  const data: unknown = await response.json();
  if (!isCurrent()) throw new Error('Your session or selection changed. Reopen this view.');
  const after = await readFieldOwner();
  if (!isCurrent() || !sameOwner(after, owner)) throw new Error('Your session changed. Reopen this view.');
  return data;
}
