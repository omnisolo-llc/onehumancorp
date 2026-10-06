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
vi.mock('@/lib/sync/queueIdentity', () => ({
  readQueueOwner: async () => ({userId:'user-one',tenantId:'tenant-one'}),
  currentVerifiedQueueOwner: () => ({userId:'user-one',tenantId:'tenant-one'}),
  sameOwner: (a: {userId:string;tenantId:string},b: {userId:string;tenantId:string}) => a.userId===b.userId && a.tenantId===b.tenantId,
  subscribeQueueIdentityReadiness: () => () => {}, QUEUE_IDENTITY_EPOCH_KEY:'queue-epoch',
}));
vi.mock('../../../lib/sync/MutationService', () => ({
  MutationService: { getInstance: () => ({ executeMutationBatch: provider.executeMutationBatch }) },
}));

beforeEach(() => {
  vi.clearAllMocks(); localStorage.clear();
  vi.mocked(navigator.locks.request).mockImplementation(async (_name, options, callback) => { return callback ? callback({} as Lock) : (options as LockGrantedCallback<unknown>)({} as Lock); });
  provider.discoverReaders.mockResolvedValue({ discoveredReaders: [{ id: 'reader-local', label: 'Test reader' }] });
  provider.connectReader.mockResolvedValue({ reader: { id: 'reader-local' } });
  provider.collectPaymentMethod.mockResolvedValue({ paymentIntent: { id: 'pi_terminal' } });
  provider.processPayment.mockResolvedValue({ paymentIntent: { id: 'pi_terminal', status: 'requires_capture' } });
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

it('collects an amount-only charge, processes and waits for backend capture before reporting payment success', async () => {
  let capture!: (response: Response) => void;
  let operation = '';
  vi.stubGlobal('fetch', vi.fn(async (url: string, init?:RequestInit) => {
    if (url.endsWith('/capture')) return new Promise<Response>((resolve) => { capture = resolve; });
    if (url.endsWith('/intent')) { const input=JSON.parse(String(init?.body)); operation=input.idempotency_key; return Response.json({ client_secret:'pi_terminal_secret_fixture',payment_intent_id:'pi_terminal',operation_id:input.idempotency_key,amount_cents:5000,currency:'usd',lock_id:null }); }
    return Response.json({ id: 'checkout-one' });
  }));
  const success = vi.fn();
  const reserve = vi.fn();
  render(<StripeTerminalClient amount={5000} productId="custom-charge" tenantId="tenant-one" onSuccess={success} onOptimisticReserve={reserve} />);
  const user = await connectReader();
  await user.click(screen.getByRole('button', { name: 'Charge $50.00' }));
  expect(await screen.findByText('Status: Payment authorized. Capturing...')).toBeVisible();
  expect(success).not.toHaveBeenCalled();
  expect(reserve).not.toHaveBeenCalled();
  expect(provider.collectPaymentMethod).toHaveBeenCalledWith('pi_terminal_secret_fixture');
  expect(provider.processPayment).toHaveBeenCalledWith({ id: 'pi_terminal' });
  await act(async () => capture(Response.json({ success:true,status:'succeeded',payment_intent_id:'pi_terminal',operation_id:operation,amount_cents:5000,currency:'usd' })));
  expect(await screen.findByText('Status: Payment successful!')).toBeVisible();
  expect(success).toHaveBeenCalledOnce();
});

it('does not treat a rejected capture in an HTTP 200 envelope as a successful payment', async () => {
  vi.stubGlobal('fetch', vi.fn(async (url: string, init?:RequestInit) => {
    if (url.endsWith('/capture')) return Response.json({ success: false, status: '', error_message: 'Capture declined' });
    if (url.endsWith('/intent')) { const input=JSON.parse(String(init?.body)); return Response.json({ client_secret:'pi_terminal_secret_fixture',payment_intent_id:'pi_terminal',operation_id:input.idempotency_key,amount_cents:5000,currency:'usd',lock_id:null }); }
    return Response.json({ id: 'checkout-one' });
  }));
  const success = vi.fn();
  render(<StripeTerminalClient amount={5000} productId="custom-charge" tenantId="tenant-one" onSuccess={success} />);
  const user = await connectReader();
  await user.click(screen.getByRole('button', { name: 'Charge $50.00' }));
  expect(await screen.findByText('Status: Payment outcome is unconfirmed. Reconcile the original operation before retrying.')).toBeVisible();
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

it('persists an amount-only payment identity before transport and blocks a second click after an unknown capture', async () => {
  localStorage.clear();
  let input: Record<string, unknown> = {};
  vi.stubGlobal('fetch', vi.fn(async (url: string, init?: RequestInit) => {
    if (url.endsWith('/capture')) throw new Error('Response lost after capture');
    if (url.endsWith('/intent')) {
      input = JSON.parse(String(init?.body));
      return Response.json({ client_secret:'pi_terminal_secret_fixture', payment_intent_id:'pi_terminal', operation_id:input.idempotency_key, amount_cents:5000, currency:'usd', lock_id:null });
    }
    return Response.json({ id:'checkout-one' });
  }));
  const success = vi.fn();
  render(<StripeTerminalClient amount={5000} productId="custom-charge" tenantId="tenant-one" onSuccess={success} />);
  const user = await connectReader();
  await user.click(screen.getByRole('button', { name:'Charge $50.00' }));
  expect(input.idempotency_key).toMatch(/^[A-Za-z0-9_-]{1,128}$/);
  expect(localStorage.getItem('omnisolo_terminal_payment_v1:["user-one","tenant-one"]')).toContain(input.idempotency_key);
  expect(screen.getByRole('button', { name:'Charge $50.00' })).toBeDisabled();
  expect(success).not.toHaveBeenCalled();
});

it('rejects an unbound catalog card sale before any payment transport or reader collection', async () => {
  vi.stubGlobal('fetch',vi.fn());
  render(<StripeTerminalClient amount={5000} productId="product-one" tenantId="tenant-one" />);
  const user=await connectReader(); await user.click(screen.getByRole('button',{name:'Charge $50.00'}));
  expect(screen.getByText(/Catalog card payments need a persisted cart reservation/)).toBeVisible();
  expect(fetch).not.toHaveBeenCalled(); expect(provider.collectPaymentMethod).not.toHaveBeenCalled();
});
it('does not collect a card from a mismatched persisted amount receipt', async () => {
  vi.stubGlobal('fetch',vi.fn(async (_url:string,init?:RequestInit)=>{
    const input=JSON.parse(String(init?.body));
    return Response.json({client_secret:'pi_terminal_secret_fixture',payment_intent_id:'pi_terminal',operation_id:input.idempotency_key,amount_cents:1,currency:'usd'});
  }));
  render(<StripeTerminalClient amount={5000} productId="custom-charge" tenantId="tenant-one" />);
  const user=await connectReader(); await user.click(screen.getByRole('button',{name:'Charge $50.00'}));
  expect(provider.collectPaymentMethod).not.toHaveBeenCalled();
  expect(screen.getByRole('button',{name:'Charge $50.00'})).toBeDisabled();
});
it('restores a prior uncertain operation after remount without creating a replacement intent', async () => {
  localStorage.setItem('omnisolo_terminal_payment_v1:["user-one","tenant-one"]',JSON.stringify({operation_id:'original',amount_cents:5000,currency:'usd'}));
  vi.stubGlobal('fetch',vi.fn());
  render(<StripeTerminalClient amount={5000} productId="custom-charge" tenantId="tenant-one" />);
  expect(await screen.findByText('Status: An earlier card payment needs reconciliation. Do not start a replacement charge.')).toBeVisible();
  expect(screen.getByRole('button',{name:'Back'})).toBeDisabled();
  expect(fetch).not.toHaveBeenCalled();expect(provider.collectPaymentMethod).not.toHaveBeenCalled();
});
it('keeps recovery evidence when identity changes after provider authorization and does not capture', async () => {
  vi.stubGlobal('fetch',vi.fn(async (_url:string,init?:RequestInit)=>{
    const input=JSON.parse(String(init?.body));
    return Response.json({client_secret:'pi_terminal_secret_fixture',payment_intent_id:'pi_terminal',operation_id:input.idempotency_key,amount_cents:5000,currency:'usd'});
  }));
  provider.processPayment.mockImplementation(async()=>{
    window.dispatchEvent(new Event('omnisolo_auth_changed'));
    return {paymentIntent:{id:'pi_terminal',status:'requires_capture'}};
  });
  const success=vi.fn();render(<StripeTerminalClient amount={5000} productId="custom-charge" tenantId="tenant-one" onSuccess={success}/>);
  const user=await connectReader();await user.click(screen.getByRole('button',{name:'Charge $50.00'}));
  expect(fetch).toHaveBeenCalledTimes(1);expect(success).not.toHaveBeenCalled();
  expect(localStorage.getItem('omnisolo_terminal_payment_v1:["user-one","tenant-one"]')).not.toBeNull();
});

it.each(['tenant', 'cart', 'cart-reverted'] as const)('does not apply an in-flight capture to a changed %s view', async drift => {
  let finish!: (response:Response)=>void;
  let operation='';
  vi.stubGlobal('fetch',vi.fn(async (url:string,init?:RequestInit)=>{
    if(url.endsWith('/capture')) return new Promise<Response>(resolve=>{finish=resolve;});
    const input=JSON.parse(String(init?.body)); operation=input.idempotency_key;
    return Response.json({client_secret:'pi_terminal_secret_fixture',payment_intent_id:'pi_terminal',operation_id:operation,amount_cents:5000,currency:'usd'});
  }));
  const success=vi.fn();
  const view=render(<StripeTerminalClient amount={5000} productId="custom-charge" tenantId="tenant-one" onSuccess={success}/>);
  const user=await connectReader();await user.click(screen.getByRole('button',{name:'Charge $50.00'}));
  await screen.findByText('Status: Payment authorized. Capturing...');
  const cart=drift!=='tenant' ? [{product:{id:'new-product',name:'New cart',price_cents:5000},quantity:1}] : undefined;
  view.rerender(<StripeTerminalClient amount={5000} productId="custom-charge" tenantId={drift==='tenant'?'tenant-two':'tenant-one'} cart={cart} onSuccess={success}/>);
  if (drift==='cart-reverted') view.rerender(<StripeTerminalClient amount={5000} productId="custom-charge" tenantId="tenant-one" onSuccess={success}/>);
  await act(async()=>finish(Response.json({success:true,status:'succeeded',payment_intent_id:'pi_terminal',operation_id:operation,amount_cents:5000,currency:'usd'})));
  expect(success).not.toHaveBeenCalled();
  expect(JSON.parse(localStorage.getItem('omnisolo_terminal_payment_v1:["user-one","tenant-one"]')!)).toMatchObject({operation_id:operation,confirmed_capture:{operation_id:operation,amount_cents:5000,status:'succeeded'}});
  expect(screen.queryByText('Status: Payment successful!')).toBeNull();
});
