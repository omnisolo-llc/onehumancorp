import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import POSTerminal from './page';
import { invalidateQueueOwner, QUEUE_IDENTITY_EPOCH_KEY } from '@/lib/sync/queueIdentity';

vi.mock('./StripeTerminalClient', () => ({ default: ({ onQueued, onSuccess }: { onQueued?: (amount: number) => void; onSuccess?: (amount: number) => void }) => <div>Payment reader<button onClick={() => onQueued?.(5000)}>Queue test sale</button><button onClick={() => onSuccess?.(5000)}>Confirm test payment</button></div> }));
vi.mock('../../../components/LocalizationToggle', () => ({ LocalizationToggle: () => null }));
vi.mock('../../../lib/sync/SyncManager', () => ({
  SyncManager: { getInstance: () => ({ start: vi.fn(), enqueue: vi.fn().mockResolvedValue(undefined), getQueueLength: vi.fn().mockResolvedValue(0), getClockQueueSummary: vi.fn().mockResolvedValue({ confirmed: 0, unconfirmed: 0, legacyHeld: 0 }) }) },
}));
vi.mock('../../../lib/sync/MutationService', () => ({
  MutationService: { getInstance: () => ({ syncPendingMutations: vi.fn(), executeMutation: vi.fn() }) },
}));

const staff = { id: 'user-a', name: 'Verified Staff', role: 'OWNER', tenant_id: 'tenant-a' };
let authentication: () => Promise<Response>;
let startSession: () => Promise<Response>;
const transport = vi.fn<typeof fetch>(async (input) => {
  const url = String(input);
  if (url === '/api/v1/auth/session-identity') return Response.json({ userId: 'user-a', tenantId: staff.tenant_id, expiresAt: Date.now() + 60_000 });
  if (url === '/api/v1/pos/auth') return authentication();
  if (url === '/api/v1/payments/terminal/session/start') return startSession();
  if (url === '/api/v1/pos/inventory') return Response.json({ inventory: [] });
  return Response.json([]);
});

async function openTerminal() {
  fireEvent.click(screen.getByRole('button', { name: 'Continue with signed-in account' }));
  await act(async () => {});
}

describe('POS terminal identity', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.stubGlobal('fetch', transport);
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true);
    localStorage.clear();
    invalidateQueueOwner();
    authentication = async () => Response.json({ success: true, staff });
    startSession = async () => Response.json({ success: true, session_id: 'session-a' });
  });
  afterEach(() => { vi.restoreAllMocks(); invalidateQueueOwner(); });

  it('does not invent an offline manager or authorize payment while disconnected', async () => {
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
    render(<POSTerminal />);
    await openTerminal();
    expect(screen.getByText('Open POS terminal')).toBeVisible();
    expect(screen.queryByText('Offline Manager')).not.toBeInTheDocument();
    expect(transport.mock.calls.some(([url]) => String(url) === '/api/v1/payments/terminal/session/start')).toBe(false);
  });

  it('keeps the terminal closed after a network error', async () => {
    authentication = async () => { throw new TypeError('network unavailable'); };
    render(<POSTerminal />);
    await openTerminal();
    expect(screen.getByText('Open POS terminal')).toBeVisible();
    expect(screen.queryByText('Offline Manager (Fallback)')).not.toBeInTheDocument();
    expect(transport.mock.calls.some(([url]) => String(url) === '/api/v1/payments/terminal/session/start')).toBe(false);
  });

  it.each([
    { status: 401, body: { success: true, staff } },
    { status: 200, body: { success: true } },
    { status: 200, body: { success: true, staff: { ...staff, tenant_id: '' } } },
    { status: 200, body: { success: true, staff: { ...staff, id: 'unverified-staff' } } },
    { status: 200, body: { success: true, staff: { ...staff, role: {} } } },
  ])('rejects unsuccessful or malformed identity responses: %j', async ({ status, body }) => {
    authentication = async () => Response.json(body, { status });
    render(<POSTerminal />);
    await openTerminal();
    expect(screen.getByText('Open POS terminal')).toBeVisible();
    expect(transport.mock.calls.some(([url]) => String(url) === '/api/v1/payments/terminal/session/start')).toBe(false);
  });

  it('uses the confirmed staff identity and never elevates its role', async () => {
    render(<POSTerminal />);
    await openTerminal();
    expect(await screen.findByText('Verified Staff')).toBeVisible();
    expect(screen.getByText('OWNER')).toBeVisible();
    await waitFor(() => expect(transport.mock.calls.filter(([url]) => String(url) === '/api/v1/payments/terminal/session/start')).toHaveLength(1));
    expect(screen.queryByText('Manager')).not.toBeInTheDocument();
  });

  it('shows an offline queue receipt without saying a payment was charged', async () => {
    render(<POSTerminal />);
    await openTerminal();
    fireEvent.click(await screen.findByRole('button', { name: 'Clock In' }));
    fireEvent.click(await screen.findByRole('button', { name: 'Queue test sale' }));
    expect(await screen.findByRole('heading', { name: 'Sale queued offline' })).toBeVisible();
    expect(screen.getByText('The $50.00 sale is saved on this device and still needs to sync.')).toBeVisible();
    expect(screen.queryByText('Payment Successful!')).toBeNull();
    expect(screen.queryByText(/was charged/)).toBeNull();
  });

  it('offers verified account access without pretending a PIN protects the terminal', async () => {
    const { container } = render(<POSTerminal />);
    expect(screen.getByRole('heading', { name: 'Open POS terminal' })).toBeVisible();
    expect(screen.getByText('Access uses your signed-in account. Close the terminal to hide its contents; sign out of your account before leaving a shared device.')).toBeVisible();
    expect(screen.queryByText(/enter.*pin/i)).toBeNull();
    expect(container.querySelector('#pos-keypad')).toBeNull();
    expect(screen.queryByRole('button', { name: '1' })).toBeNull();
    await openTerminal();
    const request = transport.mock.calls.find(([url]) => String(url) === '/api/v1/pos/auth')?.[1];
    expect(request).toMatchObject({ method: 'POST', body: '{}', headers: {
      'x-ohc-expected-user': staff.id, 'x-ohc-expected-tenant': staff.tenant_id,
    } });
    fireEvent.click(await screen.findByRole('button', { name: 'Close terminal' }));
    expect(screen.getByRole('heading', { name: 'Open POS terminal' })).toBeVisible();
    expect(screen.queryByText(staff.name)).toBeNull();
  });

  it('prevents repeated account-open attempts from sending duplicate auth or registration requests', async () => {
    let release!: (response: Response) => void;
    authentication = () => new Promise(resolve => { release = resolve; });
    render(<POSTerminal />);
    const open = screen.getByRole('button', { name: 'Continue with signed-in account' });
    fireEvent.click(open); fireEvent.click(open);
    await waitFor(() => expect(release).toBeDefined());
    expect(open).toBeDisabled();
    expect(transport.mock.calls.filter(([url]) => String(url) === '/api/v1/pos/auth')).toHaveLength(1);
    await act(async () => release(Response.json({ success: true, staff })));
    expect(await screen.findByText(staff.name)).toBeVisible();
    expect(transport.mock.calls.filter(([url]) => String(url) === '/api/v1/payments/terminal/session/start')).toHaveLength(1);
  });

  it.each(['epoch', 'pagehide', 'unmount'] as const)('does not open from a stale authentication response after %s', async boundary => {
    let release!: (response: Response) => void;
    authentication = () => new Promise(resolve => { release = resolve; });
    const view = render(<POSTerminal />);
    await openTerminal();
    await waitFor(() => expect(release).toBeDefined());
    if (boundary === 'unmount') view.unmount();
    else if (boundary === 'pagehide') act(() => window.dispatchEvent(new Event('pagehide')));
    else {
      localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY, 'replacement-account');
      act(() => window.dispatchEvent(new StorageEvent('storage', { key: QUEUE_IDENTITY_EPOCH_KEY })));
    }
    await act(async () => release(Response.json({ success: true, staff })));
    expect(screen.queryByText(staff.name)).toBeNull();
    if (boundary !== 'unmount') expect(screen.getByRole('heading', { name: 'Open POS terminal' })).toBeVisible();
    expect(transport.mock.calls.some(([url]) => String(url) === '/api/v1/payments/terminal/session/start')).toBe(false);
  });

  it.each([
    { status: 503, body: { success: true, session_id: 'not-confirmed' } },
    { status: 200, body: { success: false, error_message: 'private database detail' } },
    { status: 200, body: { success: true, session_id: '' } },
    { status: 200, body: { success: true, session_id: 42 } },
  ])('does not treat an unconfirmed terminal registration as ready: %j', async ({ status, body }) => {
    startSession = async () => Response.json(body, { status });
    render(<POSTerminal />); await openTerminal();
    expect(await screen.findByText(staff.name)).toBeVisible();
    expect(await screen.findByRole('status', { name: 'Terminal session status' })).toHaveTextContent('Terminal session registration could not be confirmed. Signing in does not confirm payment readiness.');
    expect(screen.queryByText('private database detail')).toBeNull();
    expect(screen.queryByText(/Payment Successful/)).toBeNull();
  });

  it('reports unavailable session registration without retrying a potentially completed write', async () => {
    startSession = async () => { throw new TypeError('lost response'); };
    render(<POSTerminal />); await openTerminal();
    expect(await screen.findByRole('status', { name: 'Terminal session status' })).toHaveTextContent('Terminal session registration could not be confirmed');
    expect(transport.mock.calls.filter(([url]) => String(url) === '/api/v1/payments/terminal/session/start')).toHaveLength(1);
  });

  it('discards a late session-registration failure after closing and reopening the terminal', async () => {
    let release!: (response: Response) => void;
    startSession = () => new Promise(resolve => { release = resolve; });
    render(<POSTerminal />); await openTerminal();
    await waitFor(() => expect(release).toBeDefined());
    expect(screen.getByRole('status', { name: 'Terminal session status' })).toHaveTextContent('Registering this terminal session');
    fireEvent.click(screen.getByRole('button', { name: 'Close terminal' }));
    startSession = async () => Response.json({ success: true, session_id: 'current-session' });
    await openTerminal();
    expect(await screen.findByText(staff.name)).toBeVisible();
    await act(async () => release(Response.json({ success: false })));
    expect(screen.queryByRole('status', { name: 'Terminal session status' })).toBeNull();
  });

});
