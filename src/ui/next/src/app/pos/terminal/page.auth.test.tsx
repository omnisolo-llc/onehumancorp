import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import POSTerminal from './page';
import { invalidateQueueOwner } from '@/lib/sync/queueIdentity';

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
const transport = vi.fn<typeof fetch>(async (input) => {
  const url = String(input);
  if (url === '/api/v1/auth/session-identity') return Response.json({ userId: 'user-a', tenantId: staff.tenant_id, expiresAt: Date.now() + 60_000 });
  if (url === '/api/v1/pos/auth') return authentication();
  if (url === '/api/v1/payments/terminal/session/start') return Response.json({ success: true, session_id: 'session-a' });
  if (url === '/api/v1/pos/inventory') return Response.json({ inventory: [] });
  return Response.json([]);
});

async function enterPin() {
  for (const digit of ['1', '2', '3', '4']) fireEvent.click(screen.getByRole('button', { name: digit }));
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
  });
  afterEach(() => { vi.restoreAllMocks(); invalidateQueueOwner(); });

  it('does not invent an offline manager or authorize payment while disconnected', async () => {
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
    render(<POSTerminal />);
    await enterPin();
    expect(screen.getByText('Terminal Locked')).toBeVisible();
    expect(screen.queryByText('Offline Manager')).not.toBeInTheDocument();
    expect(transport.mock.calls.some(([url]) => String(url) === '/api/v1/payments/terminal/session/start')).toBe(false);
  });

  it('keeps the terminal locked after a network error', async () => {
    authentication = async () => { throw new TypeError('network unavailable'); };
    render(<POSTerminal />);
    await enterPin();
    expect(screen.getByText('Terminal Locked')).toBeVisible();
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
    await enterPin();
    expect(screen.getByText('Terminal Locked')).toBeVisible();
    expect(transport.mock.calls.some(([url]) => String(url) === '/api/v1/payments/terminal/session/start')).toBe(false);
  });

  it('uses the confirmed staff identity and never elevates its role', async () => {
    render(<POSTerminal />);
    await enterPin();
    expect(await screen.findByText('Verified Staff')).toBeVisible();
    expect(screen.getByText('OWNER')).toBeVisible();
    await waitFor(() => expect(transport.mock.calls.filter(([url]) => String(url) === '/api/v1/payments/terminal/session/start')).toHaveLength(1));
    expect(screen.queryByText('Manager')).not.toBeInTheDocument();
  });

  it('shows an offline queue receipt without saying a payment was charged', async () => {
    render(<POSTerminal />);
    await enterPin();
    fireEvent.click(await screen.findByRole('button', { name: 'Clock In' }));
    fireEvent.click(await screen.findByRole('button', { name: 'Queue test sale' }));
    expect(await screen.findByRole('heading', { name: 'Sale queued offline' })).toBeVisible();
    expect(screen.getByText('The $50.00 sale is saved on this device and still needs to sync.')).toBeVisible();
    expect(screen.queryByText('Payment Successful!')).toBeNull();
    expect(screen.queryByText(/was charged/)).toBeNull();
  });
});
