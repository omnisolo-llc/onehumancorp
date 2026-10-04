import { act, render, screen, fireEvent, waitFor, within } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import FieldOpsJobsPage from './page';
import { invalidateQueueOwner, notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';

const { enqueue, execute } = vi.hoisted(() => ({ enqueue: vi.fn(), execute: vi.fn() }));
vi.mock('../../../lib/sync/SyncManager', () => ({ SyncManager: { getInstance: () => ({ enqueue }) } }));
vi.mock('../../../lib/powersync/PowerSyncProvider', () => ({ PowerSyncProvider: ({ children }: import('react').PropsWithChildren) => <>{children}</>, isPowerSyncSupportedForLocation: () => true }));
vi.mock('../../../lib/powersync/db', () => ({ getPowerSyncDB: async () => ({ execute, writeTransaction: async (fn: (tx: { execute: typeof execute }) => Promise<void>) => fn({ execute }) }) }));
vi.mock('@powersync/react', () => ({ useQuery: () => ({ data: [] }) }));
const owner = { userId: 'owner-a', tenantId: 'tenant-a' };
const version = '2026-10-03T10:00:00Z';
const initial = [
  { id: 'job-1', customer_id: 'cust-1', customer_name: 'Alice Smith', job_template_id: 'template-1', job_name: 'Plumbing Repair', status: 'Scheduled', notes: '', scheduled_start_time: '2026-10-03T12:00:00Z', scheduled_end_time: '2026-10-03T13:00:00Z', updated_at: version, location_address: '123 Main St' },
  { id: 'job-2', customer_id: 'cust-2', customer_name: 'Bob Jones', job_template_id: 'template-2', job_name: 'Electrical Inspection', status: 'Scheduled', notes: '', scheduled_start_time: '2026-10-03T14:00:00Z', scheduled_end_time: '2026-10-03T15:00:00Z', updated_at: version, location_address: '456 Oak Ave' },
];
let rows: typeof initial;
let activeOwner = owner;
let handler: ((url: string, init?: RequestInit) => Promise<Response> | undefined) | undefined;
const calls = (url: string) => vi.mocked(fetch).mock.calls.filter(([target, init]) => target === url && init?.method === 'POST');
const bodyOf = (call: Parameters<typeof fetch>) => JSON.parse(String(call[1]?.body));
const deferred = () => { let resolve!: (value: Response) => void; const promise = new Promise<Response>(done => { resolve = done; }); return { promise, resolve }; };
const apptURL = '/api/v1/field-ops/appointments';
const delayURL = '/api/v1/field-ops/running-late';
const routeURL = '/api/v1/field-ops/optimize-route';
const card = (name = 'Alice Smith') => screen.getByRole('heading', { name }).closest('[data-testid^="job-card-"]') as HTMLElement;
const start = async () => { render(<FieldOpsJobsPage />); await screen.findByText('Alice Smith'); };
const delayRows = () => rows.map((job, i) => i === 0 ? job : { ...job, scheduled_start_time: '2026-10-03T14:30:00Z', scheduled_end_time: '2026-10-03T15:30:00Z' });
beforeEach(() => {
  vi.clearAllMocks(); localStorage.clear(); invalidateQueueOwner(); enqueue.mockReset(); enqueue.mockResolvedValue(undefined); execute.mockResolvedValue(undefined);
  Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
  activeOwner = owner; rows = structuredClone(initial); handler = undefined;
  vi.stubGlobal('fetch', vi.fn(async (target: RequestInfo | URL, init?: RequestInit) => {
    const url = String(target);
    const custom = handler?.(url, init); if (custom) return custom;
    if (url.endsWith('session-identity')) return Response.json({ ...activeOwner, expiresAt: Date.now() + 60_000 });
    if (url.startsWith(apptURL) && init?.method !== 'POST') return Response.json({ appointments: rows });
    if (url === apptURL) {
      const body = JSON.parse(String(init?.body)); const before = rows.find(job => job.id === body.id)!;
      const saved = { ...before, ...body, updated_at: '2026-10-03T10:01:00Z' };
      rows = rows.map(job => job.id === body.id ? saved : job); return Response.json({ success: true, ...saved });
    }
    if (url === delayURL) return Response.json({ success: true, committed: false, optimizedRoute: delayRows(), subsequentCount: 1 });
    if (url === routeURL) { const body = JSON.parse(String(init?.body)); return Response.json({ success: true, committed: body.commit, ...(body.commit ? { routeId: 'route-1' } : {}), optimizedRoute: rows }); }
    throw new Error(`Unexpected test request: ${url}`);
  }));
});
describe('persisted field jobs actions', () => {
  it('loads an owned schedule and never writes the default or another tenant cache', async () => {
    await start(); expect(screen.getByText('Bob Jones')).toBeInTheDocument();
    const read = vi.mocked(fetch).mock.calls.find(([url]) => url === apptURL);
    expect(read).toBeDefined(); expect(new Headers(read?.[1]?.headers).get('x-ohc-expected-tenant')).toBe(owner.tenantId);
    await waitFor(() => expect(execute).toHaveBeenCalled());
    expect(execute.mock.calls.some(([sql]) => sql === 'DELETE FROM appointments')).toBe(false);
    expect(execute.mock.calls.some(([, values]) => Array.isArray(values) && values.includes('default'))).toBe(false);
  });
  it('waits for persisted status, sends observed version and prevents double-click duplicates', async () => {
    const pending = deferred(); handler = (url, init) => url === apptURL && init?.method === 'POST' ? pending.promise : undefined;
    await start(); fireEvent.change(within(card()).getByRole('textbox'), { target: { value: 'New notes' } });
    const button = within(card()).getByText('Heading to Job'); fireEvent.click(button); fireEvent.click(button);
    await waitFor(() => expect(calls(apptURL)).toHaveLength(1));
    expect(within(card()).getByText('SCHEDULED')).toBeInTheDocument();
    expect(bodyOf(calls(apptURL)[0])).toMatchObject({ id: 'job-1', status: 'En-Route', expected_updated_at: version, notes: 'New notes' });
    expect(new Headers(calls(apptURL)[0][1]?.headers).get('Idempotency-Key')).toBeTruthy();
    await act(async () => pending.resolve(Response.json({ ...initial[0], success: true, status: 'En-Route', notes: 'New notes', updated_at: '2026-10-03T10:01:00Z' })));
    expect(await screen.findByText('Start Work')).toBeInTheDocument();
  });
  it.each([409, 503])('keeps status and typed notes on HTTP %s', async status => {
    handler = (url, init) => url === apptURL && init?.method === 'POST' ? Promise.resolve(new Response('failed', { status })) : undefined;
    await start(); fireEvent.change(within(card()).getByRole('textbox'), { target: { value: 'Keep me' } }); fireEvent.click(within(card()).getByText('Heading to Job'));
    expect(await screen.findByRole('alert')).toBeInTheDocument(); expect(within(card()).getByText('SCHEDULED')).toBeInTheDocument();
    expect(within(card()).getByRole('textbox')).toHaveValue('Keep me'); expect(enqueue).not.toHaveBeenCalled();
  });
  it('does not overwrite notes typed while a status save is pending', async () => {
    const pending = deferred(); handler = (url, init) => url === apptURL && init?.method === 'POST' ? pending.promise : undefined;
    await start(); fireEvent.click(within(card()).getByText('Heading to Job')); await waitFor(() => expect(calls(apptURL)).toHaveLength(1));
    fireEvent.change(within(card()).getByRole('textbox'), { target: { value: 'Newer unsaved note' } });
    await act(async () => pending.resolve(Response.json({ ...initial[0], success: true, status: 'En-Route', updated_at: '2026-10-03T10:01:00Z' })));
    expect(await screen.findByText('Start Work')).toBeInTheDocument(); expect(within(card()).getByRole('textbox')).toHaveValue('Newer unsaved note');
  });
  it('rejects success-shaped replies without a persisted row', async () => {
    handler = (url, init) => url === apptURL && init?.method === 'POST' ? Promise.resolve(Response.json({ success: true })) : undefined;
    await start(); fireEvent.click(within(card()).getByText('Heading to Job'));
    expect(await screen.findByRole('alert')).toHaveTextContent(/could not be confirmed/i); expect(screen.queryByText('Start Work')).not.toBeInTheDocument();
  });
  it('does not mutate rows without an observed timestamp', async () => {
    Object.assign(rows[0], { updated_at: null }); await start(); fireEvent.click(within(card()).getByText('Heading to Job'));
    expect(await screen.findByRole('alert')).toHaveTextContent(/saved version/i); expect(calls(apptURL)).toHaveLength(0);
  });
  it('does not queue follow-on work or optimize when completion fails', async () => {
    rows[0].status = 'In-Progress'; handler = (url, init) => url === apptURL && init?.method === 'POST' ? Promise.resolve(new Response('', { status: 409 })) : undefined;
    await start(); fireEvent.click(within(card()).getByText('Job Done')); await screen.findByRole('alert');
    expect(enqueue).not.toHaveBeenCalled(); expect(calls(routeURL)).toHaveLength(0); expect(within(card()).getByText('IN-PROGRESS')).toBeInTheDocument();
  });
  it('queues genuine completion follow-ups only after the saved acknowledgement and shows queue failures', async () => {
    rows[0].status = 'In-Progress'; rows[0].notes = 'Customer requested pipe estimate';
    enqueue.mockRejectedValueOnce(new Error('Queue storage unavailable'));
    await start(); fireEvent.click(within(card()).getByText('Job Done'));
    expect(await screen.findByRole('alert')).toHaveTextContent(/follow-up.*could not be confirmed/i);
    expect(within(card()).getByText('COMPLETED')).toBeInTheDocument();
    expect(enqueue).toHaveBeenCalledWith(expect.objectContaining({ type: 'generate_invoice', payload: { job_id: 'job-1', customer_id: 'cust-1' } }), owner);
    expect(enqueue).toHaveBeenCalledWith(expect.objectContaining({ type: 'draft_quote', notes: 'Follow up quote requested by field op for job job-1. Notes: Customer requested pipe estimate', payload: { notes: 'Follow up quote requested by field op for job job-1. Notes: Customer requested pipe estimate' } }), owner);
    expect(await screen.findByText('Save route')).toBeInTheDocument();
  });
  it('does not expose a response after a silent session change while saving', async () => {
    const pending = deferred(); handler = (url, init) => url === apptURL && init?.method === 'POST' ? pending.promise : undefined;
    await start(); fireEvent.click(within(card()).getByText('Heading to Job')); await waitFor(() => expect(calls(apptURL)).toHaveLength(1));
    activeOwner = { ...owner, tenantId: 'tenant-b' };
    await act(async () => pending.resolve(Response.json({ ...initial[0], success: true, status: 'En-Route', updated_at: '2026-10-03T10:01:00Z' })));
    await screen.findByRole('alert'); expect(screen.queryByText('Alice Smith')).not.toBeInTheDocument();
  });
  it('reviews a route preview after completion and requires a committed route acknowledgement', async () => {
    rows[0].status = 'In-Progress'; await start(); fireEvent.click(within(card()).getByText('Job Done'));
    const approve = await screen.findByText('Save route'); expect(bodyOf(calls(routeURL)[0])).toMatchObject({ commit: false }); expect(bodyOf(calls(routeURL)[0])).not.toHaveProperty('tenantId');
    fireEvent.click(approve); await waitFor(() => expect(calls(routeURL)).toHaveLength(2)); expect(bodyOf(calls(routeURL)[1])).toMatchObject({ commit: true });
    expect(await screen.findByRole('status')).toHaveTextContent(/route saved/i); expect(screen.queryByText('Yes, text them')).not.toBeInTheDocument();
  });
  it('keeps failed delay proposals visible and retries only unchanged original requests', async () => {
    let fail = true; handler = (url, init) => url === apptURL && init?.method === 'POST' && fail ? Promise.resolve(new Response('', { status: 503 })) : undefined;
    await start(); fireEvent.click(within(card()).getByText('Running Late')); const approve = await screen.findByText('Save schedule');
    expect(calls(apptURL)).toHaveLength(0); expect(screen.getByText(/No customer notifications are sent/i)).toBeInTheDocument();
    fireEvent.click(approve); await screen.findByText(/HTTP 503/); expect(screen.getByText('Save schedule')).toBeInTheDocument(); expect(calls(apptURL)).toHaveLength(1);
    const first = calls(apptURL)[0]; expect(bodyOf(first)).toEqual({ id: 'job-2', status: 'Scheduled', expected_updated_at: version, scheduled_start_time: '2026-10-03T14:30:00Z', scheduled_end_time: '2026-10-03T15:30:00Z' });
    fail = false; fireEvent.click(screen.getByText('Save schedule')); await waitFor(() => expect(screen.queryByText('Save schedule')).not.toBeInTheDocument());
    expect(new Headers(calls(apptURL)[1][1]?.headers).get('Idempotency-Key')).toBe(new Headers(first[1]?.headers).get('Idempotency-Key'));
  });
  it('keeps only failed entries after a partially saved delay', async () => {
    rows.push({ ...initial[1], id: 'job-3', customer_name: 'Charlie Test' });
    handler = (url, init) => {
      if (url === delayURL) return Promise.resolve(Response.json({ success: true, committed: false, subsequentCount: 2, optimizedRoute: delayRows() }));
      if (url === apptURL && init?.method === 'POST' && JSON.parse(String(init.body)).id === 'job-3') return Promise.resolve(new Response('', { status: 409 }));
    };
    await start(); fireEvent.click(within(card()).getByText('Running Late')); fireEvent.click(await screen.findByText('Save schedule'));
    await screen.findByText(/request conflicts/); const proposal = screen.getByRole('region', { name: 'Schedule proposal' });
    expect(within(proposal).queryByText('Bob Jones')).not.toBeInTheDocument(); expect(within(proposal).getByText('Charlie Test')).toBeInTheDocument(); expect(calls(apptURL)).toHaveLength(2);
  });
  it('clears a cancelled proposal and does not revive an older calculation', async () => {
    const pending = deferred(); handler = url => url === delayURL ? pending.promise : undefined;
    await start(); fireEvent.click(within(card()).getByText('Running Late')); await waitFor(() => expect(calls(delayURL)).toHaveLength(1));
    fireEvent.click(screen.getByText('Cancel calculation'));
    await act(async () => pending.resolve(Response.json({ success: true, committed: false, subsequentCount: 1, optimizedRoute: delayRows() })));
    expect(screen.queryByText('Save schedule')).not.toBeInTheDocument();
  });
  it('fences changed identities before dispatch and clears private rows', async () => {
    await start(); activeOwner = { ...owner, tenantId: 'tenant-b' }; fireEvent.click(within(card()).getByText('Heading to Job'));
    await screen.findByRole('alert'); expect(calls(apptURL)).toHaveLength(0); expect(screen.queryByText('Alice Smith')).not.toBeInTheDocument();
  });
  it('ignores an in-flight response after auth invalidation', async () => {
    const pending = deferred(); handler = (url, init) => url === apptURL && init?.method === 'POST' ? pending.promise : undefined;
    await start(); fireEvent.click(within(card()).getByText('Heading to Job')); await waitFor(() => expect(calls(apptURL)).toHaveLength(1));
    act(() => notifyQueueIdentityChange());
    await act(async () => pending.resolve(Response.json({ ...initial[0], success: true, status: 'En-Route', updated_at: '2026-10-03T10:01:00Z' })));
    expect(screen.queryByText('Alice Smith')).not.toBeInTheDocument(); expect(screen.queryByText('Start Work')).not.toBeInTheDocument();
  });
  it('queues offline changes against the observed owner and labels them pending', async () => {
    await start(); Object.defineProperty(navigator, 'onLine', { configurable: true, value: false }); fireEvent(window, new Event('offline'));
    fireEvent.change(within(card()).getByRole('textbox'), { target: { value: 'Offline note' } }); fireEvent.click(within(card()).getByText('Heading to Job'));
    await waitFor(() => expect(enqueue).toHaveBeenCalledWith(expect.objectContaining({ type: 'sync_event', payload: expect.objectContaining({ payload: expect.objectContaining({ expected_status: 'Scheduled', expected_notes: '', expected_updated_at: version, notes: 'Offline note' }) }) }), owner));
    expect(await screen.findByText(/awaiting server confirmation/i)).toBeInTheDocument();
  });
  it('reconciles queued offline changes after an early reload without losing newer notes', async () => {
    await start(); Object.defineProperty(navigator, 'onLine', { configurable: true, value: false }); fireEvent(window, new Event('offline'));
    fireEvent.change(within(card()).getByRole('textbox'), { target: { value: 'Queued notes' } }); fireEvent.click(within(card()).getByText('Heading to Job'));
    await screen.findByText(/awaiting server confirmation/i);
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true }); fireEvent(window, new Event('online'));
    fireEvent.click(screen.getByText('Reload schedule')); await screen.findByText('Alice Smith');
    fireEvent.change(within(card()).getByRole('textbox'), { target: { value: 'Even newer unsaved notes' } });
    rows[0] = { ...rows[0], status: 'En-Route', notes: 'Queued notes', updated_at: '2026-10-03T10:01:00Z' };
    fireEvent.click(screen.getByText('Reload schedule')); await screen.findByText('Alice Smith');
    expect(within(card()).getByText('Start Work')).toBeEnabled(); expect(within(card()).getByRole('textbox')).toHaveValue('Even newer unsaved notes');
    expect(screen.queryByText(/awaiting server confirmation/i)).not.toBeInTheDocument();
  });
  it('hides private rows when session verification is rejected after loading', async () => {
    await start(); handler = url => url.endsWith('session-identity') ? Promise.resolve(new Response('', { status: 401 })) : undefined;
    fireEvent.click(within(card()).getByText('Heading to Job')); await screen.findByRole('alert');
    expect(calls(apptURL)).toHaveLength(0); expect(screen.queryByText('Alice Smith')).not.toBeInTheDocument();
  });
  it('blocks service-note edits while completing so acknowledged notes remain saveable', async () => {
    rows[0].status = 'In-Progress'; const pending = deferred(); handler = (url, init) => url === apptURL && init?.method === 'POST' ? pending.promise : undefined;
    await start(); fireEvent.click(within(card()).getByText('Job Done')); await waitFor(() => expect(calls(apptURL)).toHaveLength(1));
    expect(within(card()).getByRole('textbox')).toBeDisabled();
  });
  it('hides private rows when a dispatched mutation is rejected as unauthorized', async () => {
    handler = (url, init) => url === apptURL && init?.method === 'POST' ? Promise.resolve(new Response('', { status: 403 })) : undefined;
    await start(); fireEvent.click(within(card()).getByText('Heading to Job')); await screen.findByRole('alert'); expect(screen.queryByText('Alice Smith')).not.toBeInTheDocument();
  });
  it('recognizes the real queued quote receipt without claiming a completed quote', async () => {
    const id = '123e4567-e89b-42d3-a456-426614174001';
    handler = url => url.endsWith('draft_agent') ? Promise.resolve(Response.json({ id })) : undefined;
    await start(); fireEvent.click(screen.getByTestId('voice-quote-btn-job-1')); fireEvent.change(screen.getByTestId('voice-transcript-input'), { target: { value: 'Confirmed customer request' } });
    fireEvent.click(screen.getByTestId('generate-quote-btn'));
    expect(await screen.findByTestId('draft-quote-result')).toHaveTextContent('Draft requested. Preparation is pending');
    expect(screen.getByText('Review requested quote')).toHaveAttribute('href', `/quoting?id=${id}`);
    expect(screen.queryByText(/Draft Quote Ready|Calculated Total|Approve & Send/)).not.toBeInTheDocument();
  });
  it('retains uncertain quote requests across dialog close and reload without redispatch', async () => {
    handler = url => url.endsWith('draft_agent') ? Promise.reject(new Error('Lost response')) : undefined;
    await start(); fireEvent.click(screen.getByTestId('voice-quote-btn-job-1')); fireEvent.change(screen.getByTestId('voice-transcript-input'), { target: { value: 'Keep these exact notes' } });
    fireEvent.click(screen.getByTestId('generate-quote-btn')); await screen.findByRole('alert');
    expect(screen.getByTestId('voice-transcript-input')).toHaveValue('Keep these exact notes'); expect(screen.getByTestId('generate-quote-btn')).toBeDisabled();
    fireEvent.click(screen.getByLabelText('Close quote draft')); fireEvent.click(screen.getByText('Reload schedule')); await screen.findByText('Alice Smith'); fireEvent.click(screen.getByTestId('voice-quote-btn-job-1'));
    expect(screen.getByTestId('generate-quote-btn')).toBeDisabled(); expect(screen.getByTestId('voice-transcript-input')).toHaveValue('Keep these exact notes');
    expect(calls('/api/v1/quotes/draft_agent')).toHaveLength(1);
  });
  it('holds a quote request recorded by another tab after the dialog opened', async () => {
    await start(); fireEvent.click(screen.getByTestId('voice-quote-btn-job-1')); fireEvent.change(screen.getByTestId('voice-transcript-input'), { target: { value: 'My pending notes' } });
    localStorage.setItem('ohc_field_quote_request_v1:' + JSON.stringify([owner.userId, owner.tenantId, 'job-1']), JSON.stringify({ transcript: 'Other tab notes' }));
    fireEvent.click(screen.getByTestId('generate-quote-btn')); await screen.findByRole('alert');
    expect(calls('/api/v1/quotes/draft_agent')).toHaveLength(0); expect(screen.getByTestId('generate-quote-btn')).toBeDisabled();
  });
  it('captures follow-ups in the durable completion event and never reconstructs them from current status', async () => {
    rows[0].status = 'In-Progress'; rows[0].notes = 'Follow-up requested';
    await start(); Object.defineProperty(navigator, 'onLine', { configurable: true, value: false }); fireEvent(window, new Event('offline'));
    fireEvent.click(within(card()).getByText('Job Done')); await screen.findByText(/awaiting server confirmation/i);
    expect(enqueue).toHaveBeenCalledTimes(1);
    expect(enqueue).toHaveBeenCalledWith(expect.objectContaining({ type: 'sync_event', field_completion: { job_id: 'job-1', customer_id: 'cust-1', notes: 'Follow-up requested' } }), owner);
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true }); fireEvent(window, new Event('online'));
    rows[0] = { ...rows[0], status: 'Completed', updated_at: '2026-10-03T10:01:00Z' };
    fireEvent.click(screen.getByText('Reload schedule')); await screen.findByText('Alice Smith');
    expect(enqueue).toHaveBeenCalledTimes(1);
    expect(screen.queryByText(/follow-up requests queued/)).not.toBeInTheDocument();
  });
  it.each(['done', 'completed', 'COMPLETED', 'cancelled', 'canceled'])('keeps legacy terminal %s rows visible without reopening controls', async status => {
    rows[0].status = status; rows[0].notes = 'Preserved terminal notes'; await start();
    expect(within(card()).getByText(status.toLowerCase().startsWith('cancel') ? 'CANCELLED' : 'COMPLETED')).toBeInTheDocument();
    expect(within(card()).queryByText('Job Done')).not.toBeInTheDocument(); expect(within(card()).queryByText('Heading to Job')).not.toBeInTheDocument();
    expect(within(card()).getByText(/Preserved terminal notes/)).toBeInTheDocument(); expect(calls(apptURL)).toHaveLength(0);
  });
  it('retains a failed quote transcript and ignores success after its dialog was closed', async () => {
    const pending = deferred(); handler = url => url.endsWith('draft_agent') ? pending.promise : undefined;
    await start(); fireEvent.click(screen.getByTestId('voice-quote-btn-job-1')); fireEvent.change(screen.getByTestId('voice-transcript-input'), { target: { value: 'My exact notes' } });
    fireEvent.click(screen.getByTestId('generate-quote-btn')); await waitFor(() => expect(calls('/api/v1/quotes/draft_agent')).toHaveLength(1));
    expect(bodyOf(calls('/api/v1/quotes/draft_agent')[0])).toEqual({ inquiry: 'My exact notes', customer_id: 'cust-1' });
    fireEvent.click(screen.getByLabelText('Close quote draft')); fireEvent.click(screen.getByTestId('voice-quote-btn-job-2'));
    await act(async () => pending.resolve(Response.json({ quote: { id: 'draft-1', total_amount_cents: 1000 } })));
    expect(screen.queryByTestId('draft-quote-result')).not.toBeInTheDocument(); expect(screen.queryByText('Approve & Send')).not.toBeInTheDocument();
  });
});
