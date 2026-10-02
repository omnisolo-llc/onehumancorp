import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import Page from './page';
import { invalidateQueueOwner } from '@/lib/sync/queueIdentity';
vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: {
        children: React.ReactNode;
    }) => <main>{children}</main> }));
const owner = { userId: 'owner-a', tenantId: 'tenant-a', expiresAt: Date.now() + 60000 };
const appointment = { id: 'owned-appointment', customer_id: 'customer-a', customer_name: 'Real Customer A', job_template_id: 'template-a', job_name: 'Recorded Service A', status: 'Confirmed', scheduled_start_time: '2026-10-02T11:00:00Z', scheduled_end_time: '2026-10-02T12:00:00Z', notes: 'Owner-entered notes' };
function deferred<T>() { let resolve!: (value: T) => void; const promise = new Promise<T>(yes => { resolve = yes; }); return { promise, resolve }; }
function responses(data: unknown = { appointments: [appointment] }) { return vi.fn<typeof fetch>(async (input) => String(input) === '/api/v1/auth/session-identity' ? Response.json({ ...owner, expiresAt: Date.now() + 60000 }) : Response.json(data)); }
beforeEach(() => { act(() => invalidateQueueOwner()); vi.stubGlobal('fetch', responses()); });
afterEach(() => { act(() => invalidateQueueOwner()); vi.unstubAllGlobals(); vi.useRealTimers(); });
it('holds example/private data while the actual identity is loading', () => {
    vi.stubGlobal('fetch', vi.fn().mockReturnValue(new Promise(() => { })));
    render(<Page />);
    expect(screen.getByRole('status', { name: 'Appointment status' })).toHaveTextContent(/Loading/);
    expect(screen.queryByText(/Alice Smith|Sarah Johnson|Mike Brown|You have 4 appointments/)).not.toBeInTheDocument();
});
it('renders actual records and owner-bound request without invented payment or AI facts', async () => {
    const fetcher = responses();
    vi.stubGlobal('fetch', fetcher);
    render(<Page />);
    await screen.findByText('Recorded Service A');
    expect(screen.getByText('Real Customer A')).toBeVisible();
    expect(screen.getByText('Confirmed', { exact: true })).toBeVisible();
    expect(screen.getByText('Owner-entered notes')).toBeVisible();
    expect(screen.getByText('1 recorded appointment.')).toBeVisible();
    expect(screen.queryByText(/Deposit Required|AI Summary:|Paid/)).not.toBeInTheDocument();
    const [url, options] = fetcher.mock.calls.find(([url]) => url === '/api/v1/field-ops/appointments')!;
    expect(url).toBe('/api/v1/field-ops/appointments');
    const headers = new Headers(options!.headers);
    expect(headers.get('x-ohc-expected-user')).toBe('owner-a');
    expect(headers.get('x-ohc-expected-tenant')).toBe('tenant-a');
    expect(options).toMatchObject({ cache: 'no-store', credentials: 'same-origin', redirect: 'error' });
    expect(fetcher.mock.calls.every(([, options]) => !options?.method || options.method === 'GET')).toBe(true);
});
it('distinguishes empty appointments from unavailable storage', async () => {
    vi.stubGlobal('fetch', responses({ appointments: [] }));
    render(<Page />);
    expect(await screen.findByText('No appointments recorded for this business.')).toBeVisible();
    expect(screen.queryByRole('alert')).not.toBeInTheDocument();
});
it('shows a failed read and retries only when requested', async () => {
    let reads = 0;
    const fetcher = vi.fn(async (input) => { if (String(input) === '/api/v1/auth/session-identity')
        return Response.json({ ...owner, expiresAt: Date.now() + 60000 }); return ++reads === 1 ? Response.json({ error: 'unavailable' }, { status: 503 }) : Response.json({ appointments: [appointment] }); });
    vi.stubGlobal('fetch', fetcher);
    render(<Page />);
    expect(await screen.findByRole('alert')).toHaveTextContent(/could not be loaded/i);
    expect(screen.queryByText('No appointments recorded for this business.')).not.toBeInTheDocument();
    expect(reads).toBe(1);
    fireEvent.click(screen.getByRole('button', { name: 'Retry loading appointments' }));
    await screen.findByText('Recorded Service A');
    expect(reads).toBe(2);
});
it.each([{ appointments: [{ ...appointment, scheduled_start_time: 'not-a-date' }] }, { appointments: [appointment], success: false }, { appointments: [appointment], success: 0 }, { appointments: [appointment], success: 'true' }, { error: 'contradiction', appointments: [appointment] }, { appointments: null }])('holds malformed or contradictory appointment receipts: %j', async (body) => {
    vi.stubGlobal('fetch', responses(body));
    render(<Page />);
    await screen.findByRole('alert');
    expect(screen.queryByText('Real Customer A')).not.toBeInTheDocument();
});
it.each(['omnisolo_auth_changed', 'pagehide', 'storage'])('clears private records on canonical lifecycle invalidation: %s', async (event) => {
    render(<Page />);
    await screen.findByText('Real Customer A');
    act(() => window.dispatchEvent(event === 'storage' ? new StorageEvent('storage', { key: 'omnisolo_queue_identity_epoch_v2' }) : new Event(event)));
    expect(screen.queryByText('Real Customer A')).not.toBeInTheDocument();
    expect(screen.getByRole('status', { name: 'Appointment status' })).toHaveTextContent(/session changed/i);
});
it('cannot install a late response after identity invalidation', async () => {
    const pending = deferred<Response>();
    const fetcher = vi.fn(async (input) => String(input) === '/api/v1/auth/session-identity' ? Response.json({ ...owner, expiresAt: Date.now() + 60000 }) : pending.promise);
    vi.stubGlobal('fetch', fetcher);
    render(<Page />);
    await waitFor(() => expect(fetcher.mock.calls.some(([url]) => url === '/api/v1/field-ops/appointments')).toBe(true));
    act(() => window.dispatchEvent(new Event('omnisolo_auth_changed')));
    await act(async () => pending.resolve(Response.json({ appointments: [appointment] })));
    expect(screen.queryByText('Real Customer A')).not.toBeInTheDocument();
});
it('shows missing relationships and dates as unavailable fields without sample replacements', async () => {
    vi.stubGlobal('fetch', responses({ appointments: [{ ...appointment, customer_id: '', job_template_id: '', customer_name: '', job_name: '', scheduled_start_time: null, scheduled_end_time: null, notes: undefined }] }));
    render(<Page />);
    expect(await screen.findByText('Service details unavailable')).toBeVisible();
    expect(screen.getByText('Customer details unavailable')).toBeVisible();
    expect(screen.getByText('Time not scheduled')).toBeVisible();
});
