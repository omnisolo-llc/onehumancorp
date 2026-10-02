import { act, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner, readQueueOwner } from '@/lib/sync/queueIdentity';
import Page from './page';
vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: {
        children: React.ReactNode;
    }) => <main>{children}</main> }));
const profile = { id: 'appointment-a', customer_id: 'customer-a', customer_name: 'Private customer A', job_template_id: 'template-a', job_name: 'Private service A', status: 'Confirmed', scheduled_start_time: null, scheduled_end_time: null };
const deferred = <T,>() => { let resolve!: (value: T) => void; const promise = new Promise<T>(yes => { resolve = yes; }); return { promise, resolve }; };
let identity: () => Promise<Response>;
let records: () => Promise<Response>;
let fetcher: ReturnType<typeof vi.fn>;
const owner = (tenant = 'a') => ({ userId: `owner-${tenant}`, tenantId: `tenant-${tenant}`, expiresAt: Date.now() + 60000 });
beforeEach(() => { localStorage.clear(); act(() => invalidateQueueOwner()); identity = async () => Response.json(owner()); records = async () => Response.json({ appointments: [profile] }); fetcher = vi.fn(async (input) => String(input) === '/api/v1/auth/session-identity' ? identity() : records()); vi.stubGlobal('fetch', fetcher); });
afterEach(() => { act(() => invalidateQueueOwner()); vi.unstubAllGlobals(); });
it('retires loaded private records when a canonical read verifies another owner without an auth event', async () => {
    render(<Page />);
    await screen.findByText('Private customer A');
    identity = async () => Response.json(owner('b'));
    await act(async () => { await readQueueOwner(); });
    expect(screen.queryByText('Private customer A')).not.toBeInTheDocument();
    expect(screen.getByRole('status', { name: 'Appointment status' })).toHaveTextContent(/session changed/i);
});
it('a late appointment body cannot install after a no-event canonical owner change', async () => {
    const pending = deferred<Response>();
    records = () => pending.promise;
    render(<Page />);
    await waitFor(() => expect(fetcher.mock.calls.some(([input]) => input === '/api/v1/field-ops/appointments')).toBe(true));
    identity = async () => Response.json(owner('b'));
    await act(async () => { await readQueueOwner(); });
    await act(async () => pending.resolve(Response.json({ appointments: [profile] })));
    expect(screen.queryByText('Private customer A')).not.toBeInTheDocument();
});
it('a pending same-owner verification holds output then restores the unchanged records', async () => {
    render(<Page />);
    await screen.findByText('Private customer A');
    const pending = deferred<Response>();
    identity = () => pending.promise;
    let verifying!: Promise<unknown>;
    act(() => { verifying = readQueueOwner(); });
    expect(screen.queryByText('Private customer A')).not.toBeInTheDocument();
    await act(async () => { pending.resolve(Response.json(owner())); await verifying; });
    expect(screen.getByText('Private customer A')).toBeVisible();
    expect(fetcher.mock.calls.filter(([input]) => input === '/api/v1/field-ops/appointments')).toHaveLength(1);
});
it('an initial empty canonical cache does not permanently hold a freshly verified owner', async () => {
    render(<Page />);
    expect(await screen.findByText('Private customer A')).toBeVisible();
});
it('retains strict identity HTTP status before consulting the shared readiness cache', async () => {
    identity = async () => Response.json(owner(), { status: 202 });
    render(<Page />);
    await screen.findByRole('alert');
    expect(fetcher.mock.calls.filter(([input]) => input === '/api/v1/field-ops/appointments')).toHaveLength(0);
});
