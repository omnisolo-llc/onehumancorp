import { act, fireEvent, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import StripeTerminalClient from './StripeTerminalClient';

const provider = vi.hoisted(() => ({
  discoverReaders: vi.fn(), connectReader: vi.fn(), collectPaymentMethod: vi.fn(), processPayment: vi.fn(),
  executeMutationBatch: vi.fn(),
}));
vi.mock('@stripe/terminal-js', () => ({ loadStripeTerminal: async () => ({ create: () => provider }) }));
vi.mock('../../../lib/sync/SyncManager', () => ({}));
vi.mock('../../../lib/sync/MutationService', () => ({
  MutationService: { getInstance: () => ({ executeMutationBatch: provider.executeMutationBatch }) },
}));

beforeEach(() => {
  vi.clearAllMocks();
  provider.discoverReaders.mockResolvedValue({ discoveredReaders: [{ id: 'reader-local', label: 'Test reader' }] });
  provider.connectReader.mockResolvedValue({ reader: { id: 'reader-local' } });
  provider.collectPaymentMethod.mockResolvedValue({ paymentIntent: { id: 'pi-terminal' } });
  provider.processPayment.mockResolvedValue({ paymentIntent: { id: 'pi-terminal', status: 'requires_capture' } });
});
afterEach(() => { vi.useRealTimers(); vi.unstubAllGlobals(); vi.restoreAllMocks(); });

async function connectReader() {
  const user = userEvent.setup();
  await screen.findByText('Status: Terminal initialized. Ready to discover readers.');
  await user.click(screen.getByRole('button', { name: 'Discover Readers' }));
  await user.click(await screen.findByRole('button', { name: 'Connect' }));
  await screen.findByRole('button', { name: 'Charge $50.00' });
  return user;
}

it('reserves, collects, processes and waits for backend capture before reporting payment success', async () => {
  let capture!: (response: Response) => void;
  vi.stubGlobal('fetch', vi.fn(async (url: string) => {
    if (url.endsWith('/capture')) return new Promise<Response>((resolve) => { capture = resolve; });
    if (url.endsWith('/intent')) return Response.json({ client_secret: 'synthetic-terminal-secret', lock_id: 'lock-one' });
    return Response.json({ id: 'checkout-one' });
  }));
  const success = vi.fn();
  const reserve = vi.fn();
  render(<StripeTerminalClient amount={5000} productId="product-one" tenantId="tenant-one" onSuccess={success} onOptimisticReserve={reserve} />);
  const user = await connectReader();
  await user.click(screen.getByRole('button', { name: 'Charge $50.00' }));
  expect(await screen.findByText('Status: Payment authorized. Capturing...')).toBeVisible();
  expect(success).not.toHaveBeenCalled();
  expect(reserve).toHaveBeenCalledOnce();
  expect(provider.collectPaymentMethod).toHaveBeenCalledWith('synthetic-terminal-secret');
  expect(provider.processPayment).toHaveBeenCalledWith({ id: 'pi-terminal' });
  await act(async () => capture(Response.json({ success: true, status: 'succeeded' })));
  expect(await screen.findByText('Status: Payment successful!')).toBeVisible();
  expect(success).toHaveBeenCalledOnce();
});

it('does not treat a rejected capture in an HTTP 200 envelope as a successful payment', async () => {
  vi.stubGlobal('fetch', vi.fn(async (url: string) => {
    if (url.endsWith('/capture')) return Response.json({ success: false, status: '', error_message: 'Capture declined' });
    if (url.endsWith('/intent')) return Response.json({ client_secret: 'synthetic-terminal-secret', lock_id: 'lock-one' });
    return Response.json({ id: 'checkout-one' });
  }));
  const success = vi.fn();
  render(<StripeTerminalClient amount={5000} productId="product-one" tenantId="tenant-one" onSuccess={success} />);
  const user = await connectReader();
  await user.click(screen.getByRole('button', { name: 'Charge $50.00' }));
  expect(await screen.findByText('Status: Failed to capture intent')).toBeVisible();
  expect(success).not.toHaveBeenCalled();
  expect(screen.queryByText('Status: Payment successful!')).toBeNull();
});

it.each(['tap_to_pay', 'cash_sale'])('never overwrites a rejected offline %s enqueue with timer-based success', async (method) => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({})));
  const reserve = vi.fn();
  const rollback = vi.fn();
  const success = vi.fn();
  provider.executeMutationBatch.mockImplementation((_type, _payload, optimistic, rejectOptimistic) => {
    optimistic();
    const result = new Promise<void>((_resolve, reject) => setTimeout(() => {
      rejectOptimistic();
      reject(new Error('Offline storage full'));
    }, 100));
    // Observe the test dependency's rejection even before the caller awaits it.
    void result.catch(() => {});
    return result;
  });
  render(<StripeTerminalClient amount={5000} productId="product-one" tenantId="tenant-one" onSuccess={success} onOptimisticReserve={reserve} onOptimisticRollback={rollback} />);
  const user = await connectReader();
  if (method === 'cash_sale') {
    await user.click(screen.getByRole('button', { name: 'Back' }));
    await user.click(screen.getByRole('button', { name: 'Cash' }));
  }
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  vi.useFakeTimers();
  fireEvent.click(screen.getByRole('button', { name: method === 'cash_sale' ? 'Record Offline Cash Sale 50.00' : 'Charge $50.00' }));
  expect(provider.executeMutationBatch).toHaveBeenCalledWith(method, [{ amount_cents: 5000, product_id: 'product-one', quantity: 1 }], expect.any(Function), expect.any(Function));
  expect(provider.collectPaymentMethod).not.toHaveBeenCalled();
  expect(reserve).toHaveBeenCalledOnce();
  await act(async () => { await vi.advanceTimersByTimeAsync(2000); });
  expect(rollback).toHaveBeenCalledOnce();
  expect(screen.getByText(method === 'cash_sale' ? 'Status: Failed to save offline cash sale.' : 'Status: Failed to save offline payment.')).toBeVisible();
  expect(success).not.toHaveBeenCalled();
});

it('reports a persisted offline intent as queued, never as a captured payment', async () => {
  let persist!: () => void;
  provider.executeMutationBatch.mockImplementation(() => new Promise<void>((resolve) => { persist = resolve; }));
  const success = vi.fn();
  const queued = vi.fn();
  render(<StripeTerminalClient amount={5000} productId="product-one" tenantId="tenant-one" onSuccess={success} onQueued={queued} />);
  const user = await connectReader();
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  await user.click(screen.getByRole('button', { name: 'Charge $50.00' }));
  expect(queued).not.toHaveBeenCalled();
  expect(success).not.toHaveBeenCalled();
  await act(async () => persist());
  expect(screen.getByText('Status: Payment queued offline. Will sync when network is restored.')).toBeVisible();
  expect(queued).toHaveBeenCalledWith(5000);
  expect(screen.getByRole('button', { name: 'Charge $50.00' })).toBeDisabled();
  expect(success).not.toHaveBeenCalled();
  expect(provider.collectPaymentMethod).not.toHaveBeenCalled();
});

it('ignores completion of an offline attempt after unmount', async () => {
  let persist!: () => void;
  provider.executeMutationBatch.mockImplementation(() => new Promise<void>((resolve) => { persist = resolve; }));
  const success = vi.fn();
  const queued = vi.fn();
  const view = render(<StripeTerminalClient amount={5000} productId="product-one" tenantId="tenant-one" onSuccess={success} onQueued={queued} />);
  await connectReader();
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  vi.useFakeTimers();
  fireEvent.click(screen.getByRole('button', { name: 'Charge $50.00' }));
  view.unmount();
  await act(async () => { persist(); await vi.advanceTimersByTimeAsync(2000); });
  expect(success).not.toHaveBeenCalled();
  expect(queued).not.toHaveBeenCalled();
});
