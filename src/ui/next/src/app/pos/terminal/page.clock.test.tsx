import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import POSTerminal from './page';
import { invalidateQueueOwner, readQueueOwner, QUEUE_IDENTITY_EPOCH_KEY } from '@/lib/sync/queueIdentity';

const queueLength = vi.hoisted(() => vi.fn<() => Promise<number>>());
const clockSummary = vi.hoisted(() => vi.fn<() => Promise<{ confirmed: number; unconfirmed: number; legacyHeld: number }>>());
const reconcileClockReceipts = vi.hoisted(() => vi.fn<() => Promise<void>>());
const enqueue = vi.hoisted(() => vi.fn<(...args: unknown[]) => Promise<void>>());
vi.mock('./StripeTerminalClient', () => ({ default: () => <div>Payment controls</div> }));
vi.mock('../../../components/LocalizationToggle', () => ({ LocalizationToggle: () => null }));
vi.mock('../../../lib/sync/SyncManager', () => ({ SyncManager: { getInstance: () => ({ enqueue, getQueueLength: queueLength, getClockQueueSummary: clockSummary, reconcileClockReceipts }) } }));
vi.mock('../../../lib/sync/MutationService', () => ({ MutationService: { getInstance: () => ({ syncPendingMutations: vi.fn(), executeMutation: vi.fn() }) } }));

let owner = 'user-a';
let identityUnavailable = false;
let ownerExpiry: number;
let inventoryRead: () => Promise<Response>;
const staff = { id: 'user-a', name: 'Recorded Staff', role: 'OWNER', tenant_id: 'tenant-a' };
const identity = () => Response.json({ userId: owner, tenantId: 'tenant-a', expiresAt: ownerExpiry });
const transport = vi.fn<typeof fetch>(async (input) => {
  const path = String(input);
  if (path === '/api/v1/auth/session-identity') {
    if (identityUnavailable) throw new Error('Identity read unavailable');
    return identity();
  }
  if (path === '/api/v1/pos/auth') return Response.json({ success: true, staff: { ...staff, id: owner } });
  if (path === '/api/v1/payments/terminal/session/start') return Response.json({ success: true, session_id: 'local-session' });
  if (path === '/api/v1/pos/inventory') return inventoryRead();
  throw new Error(`Unexpected fixture request: ${path}`);
});
async function unlock() {
  for (const digit of ['1', '2', '3', '4']) fireEvent.click(screen.getByRole('button', { name: digit }));
  await act(async () => {});
  await screen.findByText('Recorded Staff');
}
function pendingWrite() {
  let resolve!: () => void;
  let reject!: (reason: Error) => void;
  enqueue.mockImplementation(() => new Promise<void>((yes, no) => { resolve = yes; reject = no; }));
  return { resolve: () => resolve(), reject: () => reject(new Error('Local storage unavailable')) };
}
beforeEach(async () => {
  vi.clearAllMocks(); clockSummary.mockResolvedValue({ confirmed: 0, unconfirmed: 0, legacyHeld: 0 }); reconcileClockReceipts.mockResolvedValue(); enqueue.mockReset(); queueLength.mockResolvedValue(0); owner = 'user-a'; identityUnavailable = false; localStorage.clear(); invalidateQueueOwner();
  ownerExpiry = Date.now() + 60_000; inventoryRead = async () => Response.json({ inventory: [] });
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true);
  vi.stubGlobal('fetch', transport); await readQueueOwner();
});
afterEach(() => { vi.useRealTimers(); vi.restoreAllMocks(); vi.unstubAllGlobals(); invalidateQueueOwner(); });

it('acknowledges clock-in only after the owner-bound durable queue commit and blocks duplicates', async () => {
  const pending = pendingWrite(); render(<POSTerminal />); await unlock();
  const clock = screen.getByRole('button', { name: 'Clock In' });
  fireEvent.click(clock); fireEvent.click(clock);
  await waitFor(() => expect(enqueue).toHaveBeenCalledTimes(1));
  expect(screen.getByRole('heading', { name: 'Not Clocked In' })).toBeVisible();
  expect(screen.queryByText('Your time is being tracked locally.')).toBeNull();
  expect(clock).toBeDisabled();
  await act(async () => pending.resolve());
  expect(await screen.findByRole('heading', { name: 'Clocked In' })).toBeVisible();
  expect(enqueue.mock.calls[0][1]).toEqual({ userId: 'user-a', tenantId: 'tenant-a' });
});

it('keeps clock-in unconfirmed and reports a failed local queue commit', async () => {
  const pending = pendingWrite(); render(<POSTerminal />); await unlock();
  fireEvent.click(screen.getByRole('button', { name: 'Clock In' }));
  await waitFor(() => expect(enqueue).toHaveBeenCalledTimes(1));
  await act(async () => pending.reject());
  expect(await screen.findByRole('alert')).toHaveTextContent('Clock change could not be saved');
  expect(screen.getByRole('heading', { name: 'Not Clocked In' })).toBeVisible();
  expect(screen.getByRole('button', { name: 'Clock In' })).toBeEnabled();
});

it.each(['lock', 'owner', 'epoch'] as const)('rejects a late clock receipt after %s changes the terminal lease', async (change) => {
  const pending = pendingWrite(); render(<POSTerminal />); await unlock();
  fireEvent.click(screen.getByRole('button', { name: 'Clock In' }));
  await waitFor(() => expect(enqueue).toHaveBeenCalledTimes(1));
  if (change === 'lock') fireEvent.click(screen.getByRole('button', { name: 'Lock' }));
  else if (change === 'owner') { owner = 'user-b'; await act(async () => { await readQueueOwner(); }); }
  else localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY, 'replacement-session');
  await act(async () => pending.resolve());
  expect(screen.getByText('Terminal Locked')).toBeVisible();
  expect(screen.queryByRole('heading', { name: 'Clocked In' })).toBeNull();
  if (change === 'lock') {
    await unlock();
    expect(screen.getByRole('heading', { name: 'Not Clocked In' })).toBeVisible();
  }
});

it('exposes offline readiness only while the same canonical owner is currently verified', async () => {
  render(<POSTerminal />); await unlock();
  expect(await screen.findByText('Offline queue ready for this session.')).toBeVisible();
  let release!: (response: Response) => void;
  transport.mockImplementationOnce(() => new Promise<Response>(resolve => { release = resolve; }));
  let request!: ReturnType<typeof readQueueOwner>;
  await act(async () => { request = readQueueOwner(); });
  expect(screen.queryByText('Offline queue ready for this session.')).toBeNull();
  expect(screen.getByRole('button', { name: 'Clock In', hidden: true })).toBeDisabled();
  await act(async () => { release(identity()); await request; });
  expect(screen.getByText('Offline queue ready for this session.')).toBeVisible();
});

it('preserves confirmed clock-in if the clock-out queue write fails', async () => {
  enqueue.mockResolvedValue(); render(<POSTerminal />); await unlock();
  fireEvent.click(screen.getByRole('button', { name: 'Clock In' }));
  await screen.findByRole('heading', { name: 'Clocked In' });
  const pending = pendingWrite();
  fireEvent.click(screen.getByRole('button', { name: 'Clock Out' }));
  await waitFor(() => expect(enqueue).toHaveBeenCalledTimes(2));
  await act(async () => pending.reject());
  expect(await screen.findByRole('alert')).toHaveTextContent('Clock change could not be saved');
  expect(screen.getByRole('heading', { name: 'Clocked In' })).toBeVisible();
});

it('does not perform receipt follow-up after the terminal unmounts', async () => {
  const pending = pendingWrite(); const view = render(<POSTerminal />); await unlock();
  fireEvent.click(screen.getByRole('button', { name: 'Clock In' }));
  await waitFor(() => expect(enqueue).toHaveBeenCalledTimes(1));
  view.unmount(); const count = transport.mock.calls.length;
  await act(async () => pending.resolve());
  expect(transport.mock.calls).toHaveLength(count);
});

it('recovers an already committed clock receipt without writing a second event after identity read failure', async () => {
  const pending = pendingWrite(); render(<POSTerminal />); await unlock();
  fireEvent.click(screen.getByRole('button', { name: 'Clock In' }));
  await waitFor(() => expect(enqueue).toHaveBeenCalledTimes(1));
  identityUnavailable = true;
  await act(async () => pending.resolve());
  expect(await screen.findByRole('alert')).toHaveTextContent('Clock change was saved locally');
  expect(screen.getByRole('button', { name: 'Clock In', hidden: true })).toBeDisabled();
  identityUnavailable = false;
  await act(async () => { await readQueueOwner(); });
  expect(screen.getByRole('heading', { name: 'Clocked In' })).toBeVisible();
  expect(enqueue).toHaveBeenCalledTimes(1);
});

it('does not apply an old inventory body after locking and verifying a different owner', async () => {
  let release!: (response: Response) => void;
  inventoryRead = () => new Promise<Response>(resolve => { release = resolve; });
  render(<POSTerminal />); await unlock();
  await waitFor(() => expect(release).toBeDefined());
  fireEvent.click(screen.getByRole('button', { name: 'Lock' }));
  owner = 'user-b'; await act(async () => { await readQueueOwner(); });
  inventoryRead = async () => Response.json({ inventory: [{ id: 'b', name: 'Current inventory', description: '', price_cents: 100, stock: 2 }] });
  await unlock(); await screen.findByText('Current inventory');
  await act(async () => release(Response.json({ inventory: [{ id: 'a', name: 'Previous private inventory', description: '', price_cents: 100, stock: 2 }] })));
  expect(screen.queryByText('Previous private inventory')).toBeNull();
  expect(screen.getByText('Current inventory')).toBeVisible();
});

it('hides private staff and retires the original lease at expiry during pending revalidation', async () => {
  ownerExpiry = Date.now() + 2000;
  await readQueueOwner(); render(<POSTerminal />); await unlock();
  let release!: (response: Response) => void;
  const response = new Promise<Response>(resolve => { release = resolve; });
  transport.mockImplementationOnce(() => response);
  let request!: ReturnType<typeof readQueueOwner>;
  await act(async () => { request = readQueueOwner(); });
  try { expect(screen.getByText('Recorded Staff')).not.toBeVisible(); }
  finally {
    await waitFor(() => expect(screen.getByText('Terminal Locked')).toBeVisible(), { timeout: 3000 });
    ownerExpiry = Date.now() + 60_000;
    await act(async () => { release(identity()); await request; });
  }
  expect(screen.getByText('Terminal Locked')).toBeVisible();
  expect(screen.queryByText('Recorded Staff')).toBeNull();
});

it('keeps saved clock work visible without blocking product selection', async () => {
  queueLength.mockResolvedValue(1); enqueue.mockResolvedValue();
  inventoryRead = async () => Response.json({ inventory: [{ id: 'owned-last-unit', name: 'Owned last unit', price_cents: 1999, stock: 1 }] });
  render(<POSTerminal />); await unlock();
  fireEvent.click(screen.getByRole('button', { name: 'Clock In' }));
  await screen.findByRole('heading', { name: 'Clocked In' });
  const saved = screen.getByText('Saved transactions awaiting confirmation');
  expect(saved).toBeVisible();
  expect(getComputedStyle(saved.closest('div')!).pointerEvents).toBe('none');
  await act(async () => { fireEvent.click(screen.getByRole('button', { name: /Owned last unit/ })); });
  expect(screen.getByRole('button', { name: /1 item\s*Charge \$19\.99/ })).toBeEnabled();
  expect(saved).toBeVisible();
  expect(screen.queryByText('Synced')).toBeNull();
  queueLength.mockResolvedValue(0);
  await act(async () => window.dispatchEvent(new Event('omnisolo_queue_updated')));
  const synced = screen.getByText('Synced');
  expect(synced).toBeVisible();
  expect(getComputedStyle(synced.closest('div')!).pointerEvents).toBe('none');
});


it('produces a versioned clock event with the signed actor and no duplicate payload timestamp', async () => {
  enqueue.mockResolvedValue(); render(<POSTerminal />); await unlock();
  fireEvent.click(screen.getByRole('button', { name: 'Clock In' }));
  await screen.findByRole('heading', { name: 'Clocked In' });
  expect(enqueue.mock.calls[0][0]).toEqual({ type: 'staff_clock_event_v1', payload: { staff_id: 'user-a', event_type: 'CLOCK_IN' } });
});
it('checks saved clock receipts explicitly and distinguishes held historical and unconfirmed local work', async () => {
  clockSummary.mockResolvedValue({ confirmed: 0, unconfirmed: 1, legacyHeld: 1 });
  enqueue.mockResolvedValue();
  let finish!: () => void;
  reconcileClockReceipts.mockImplementation(() => new Promise<void>(resolve => { finish = resolve; }));
  render(<POSTerminal />); await unlock();
  expect(screen.getByText('1 saved clock change awaiting server confirmation.')).toBeVisible();
  expect(screen.getByText('1 historical clock change requires review. Original records are preserved.')).toBeVisible();
  const button = screen.getByRole('button', { name: 'Check saved clock status' });
  fireEvent.click(button); fireEvent.click(button);
  expect(reconcileClockReceipts).toHaveBeenCalledTimes(1); expect(button).toBeDisabled();
  clockSummary.mockResolvedValue({ confirmed: 1, unconfirmed: 0, legacyHeld: 1 });
  await act(async () => finish());
  expect(screen.getByText('1 saved clock change confirmed by the server.')).toBeVisible();
  expect(screen.queryByText('1 saved clock change awaiting server confirmation.')).toBeNull();
  expect(enqueue).not.toHaveBeenCalled();
});
it('keeps missing receipt checks honest instead of confirming a locally saved event', async () => {
  clockSummary.mockResolvedValue({ confirmed: 0, unconfirmed: 1, legacyHeld: 0 });
  render(<POSTerminal />); await unlock();
  fireEvent.click(screen.getByRole('button', { name: 'Check saved clock status' }));
  await waitFor(() => expect(reconcileClockReceipts).toHaveBeenCalledOnce());
  expect(screen.getByText('1 saved clock change awaiting server confirmation.')).toBeVisible();
  expect(screen.queryByText(/saved clock change confirmed by the server/)).toBeNull();
});


it.each(['lock', 'owner', 'epoch'] as const)('never reports an old status check as confirmed after the terminal %s changes', async change => {
  clockSummary.mockResolvedValue({ confirmed: 0, unconfirmed: 1, legacyHeld: 0 });
  let finish!: () => void;
  reconcileClockReceipts.mockImplementation(() => new Promise<void>(resolve => { finish = resolve; }));
  render(<POSTerminal />); await unlock();
  fireEvent.click(screen.getByRole('button', { name: 'Check saved clock status' }));
  await waitFor(() => expect(reconcileClockReceipts).toHaveBeenCalledOnce());
  if (change === 'owner') { owner = 'user-b'; await act(async () => { await readQueueOwner(); }); }
  else if (change === 'epoch') {
    localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY, 'new-session');
    await act(async () => window.dispatchEvent(new StorageEvent('storage', { key: QUEUE_IDENTITY_EPOCH_KEY })));
  } else fireEvent.click(screen.getByRole('button', { name: 'Lock' }));
  await unlock();
  // Even an old read completing with confirmation cannot update the new lease.
  clockSummary.mockResolvedValue({ confirmed: 1, unconfirmed: 0, legacyHeld: 0 });
  await act(async () => finish());
  expect(screen.queryByText('1 saved clock change confirmed by the server.')).toBeNull();
  expect(screen.getByRole('heading', { name: 'Not Clocked In' })).toBeVisible();
});
