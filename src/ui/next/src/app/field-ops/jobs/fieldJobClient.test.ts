import { beforeEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner } from '@/lib/sync/queueIdentity';
import { readAppointments, readAppointmentUpdate, readRouteProposal, fetchFieldJob } from './fieldJobClient';

const owner = { userId: 'owner-a', tenantId: 'tenant-a' };
const observed = '2026-10-03T10:00:00Z';
const job = { id: 'job-1', customer_id: 'customer-1', customer_name: 'Test Customer', job_template_id: 'template-1', job_name: 'Test Service', status: 'Scheduled', notes: '', scheduled_start_time: '2026-10-03T12:00:00Z', scheduled_end_time: '2026-10-03T13:00:00Z', updated_at: observed };
const update = { id: job.id, status: 'En-Route', notes: 'Saved note', expected_updated_at: observed };
const saved = { ...job, ...update, success: true, updated_at: '2026-10-03T10:01:00Z' };
beforeEach(() => { invalidateQueueOwner(); Object.defineProperty(navigator, 'onLine', { configurable: true, value: true }); });
it('accepts only an explicit valid appointments list while preserving legacy missing versions', () => {
  expect(readAppointments({ appointments: [job] })).toEqual([expect.objectContaining(job)]);
  expect(readAppointments({ appointments: [{ ...job, updated_at: null }] })[0].updated_at).toBeNull();
  for (const body of [{}, { appointments: [job, job] }, { appointments: [{ ...job, status: 'made-up' }] }, { appointments: [{ ...job, scheduled_start_time: 'bad' }] }, { appointments: [job], success: false }]) expect(() => readAppointments(body)).toThrow();
});
it('requires persisted acknowledgement of the exact requested row and fields', () => {
  expect(readAppointmentUpdate(saved, job, update)).toMatchObject({ id: job.id, status: 'En-Route', notes: 'Saved note', updated_at: saved.updated_at });
  for (const body of [{ success: true }, { ...saved, id: 'other' }, { ...saved, status: 'Scheduled' }, { ...saved, notes: 'not saved' }, { ...saved, updated_at: null }, { ...saved, success: false }, { ...saved, error: 'failure' }]) expect(() => readAppointmentUpdate(body, job, update)).toThrow();
});
it('requires each proposed row to match an observed id and timestamp and an explicit preview marker', () => {
  const proposed = { ...job, scheduled_start_time: '2026-10-03T12:30:00Z', scheduled_end_time: '2026-10-03T13:30:00Z' };
  expect(readRouteProposal({ success: true, committed: false, optimizedRoute: [proposed] }, [job], false)).toHaveLength(1);
  for (const body of [{ success: true, optimizedRoute: [proposed] }, { success: true, committed: true, optimizedRoute: [proposed] }, { success: true, committed: false, optimizedRoute: [{ ...proposed, id: 'foreign' }] }, { success: true, committed: false, optimizedRoute: [{ ...proposed, updated_at: '2026-10-03T11:00:00Z' }] }, { success: true, committed: false, optimizedRoute: [] }]) expect(() => readRouteProposal(body, [job], false)).toThrow();
  expect(() => readRouteProposal({ success: true, committed: true, optimizedRoute: [job] }, [job], true)).toThrow();
  expect(readRouteProposal({ success: true, committed: true, routeId: 'route-1', optimizedRoute: [job] }, [job], true)).toHaveLength(1);
});
it('binds requests to the verified owner and prevents a changed owner from receiving a mutation', async () => {
  const transport = vi.fn(async (url: string) => Response.json(url.includes('session-identity') ? { ...owner, expiresAt: Date.now() + 60_000 } : saved));
  vi.stubGlobal('fetch', transport);
  await fetchFieldJob('/api/v1/field-ops/appointments', { method: 'POST', body: JSON.stringify(update) }, owner, () => true);
  const sent = transport.mock.calls.find(([url]) => url === '/api/v1/field-ops/appointments');
  expect(new Headers((sent as unknown as [string, RequestInit])[1].headers).get('x-ohc-expected-tenant')).toBe(owner.tenantId);
  transport.mockClear(); transport.mockResolvedValue(Response.json({ ...owner, tenantId: 'other', expiresAt: Date.now() + 60_000 }));
  await expect(fetchFieldJob('/api/v1/field-ops/appointments', { method: 'POST' }, owner, () => true)).rejects.toThrow(/session/i);
  expect(transport.mock.calls).toHaveLength(1);
});
it('fences cancellation during identity verification and rejects unknown destinations', async () => {
  let current = true;
  vi.stubGlobal('fetch', vi.fn(async () => { current = false; return Response.json({ ...owner, expiresAt: Date.now() + 60_000 }); }));
  await expect(fetchFieldJob('/api/v1/field-ops/appointments', { method: 'POST' }, owner, () => current)).rejects.toThrow();
  expect(fetch).toHaveBeenCalledTimes(1);
  await expect(fetchFieldJob('https://other.example', {}, owner, () => true)).rejects.toThrow();
  expect(fetch).toHaveBeenCalledTimes(1);
});

it.each([['completed', 'Completed'], ['COMPLETED', 'Completed'], ['done', 'Completed'], ['DONE', 'Completed'], ['cancelled', 'Cancelled'], ['canceled', 'Cancelled']])('normalizes the known legacy terminal state %s without dropping appointment evidence', (status, canonical) => {
  const raw = { ...job, status, notes: 'Keep saved notes', location_address: 'Confirmed address' };
  expect(readAppointments({ appointments: [raw] })).toEqual([{ ...raw, status: canonical }]);
  expect(readRouteProposal({ success: true, committed: false, optimizedRoute: [raw] }, readAppointments({ appointments: [raw] }), false)[0].status).toBe(canonical);
});
it.each(['complete-ish', 'unknown', 'schedule', 'reopen', ' done '])('continues rejecting unsupported status %s', status => {
  expect(() => readAppointments({ appointments: [{ ...job, status }] })).toThrow();
});
