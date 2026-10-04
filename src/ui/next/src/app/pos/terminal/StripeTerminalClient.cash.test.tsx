import { act, cleanup, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import StripeTerminalClient from './StripeTerminalClient';
import { invalidateQueueOwner, notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { MutationService } from '@/lib/sync/MutationService';

vi.mock('@stripe/terminal-js', () => ({ loadStripeTerminal: async () => ({ create: () => ({}) }) }));
vi.mock('../../../lib/sync/SyncManager', () => ({}));

const reservePath = '/api/v1/payments/terminal/reserve';
const commitPath = '/api/v1/payments/terminal/commit';
let requests: { path: string; init?: RequestInit }[];
let commit: (body: Record<string, unknown>) => Promise<Response>;
const owner = { userId: 'cash-owner', tenantId: 'tenant-one' };
const completed = (body: Record<string, unknown>) => Response.json({ success: true, status: 'completed', receipt: { ...body, order_id: 'recorded-order', customer_id: null, items: ((body.items ?? [{ product_id: body.product_id, quantity: body.quantity, amount_cents: body.amount_cents }]) as Record<string, unknown>[]).map(item => ({ ...item, lock_id: '' })) } });

beforeEach(() => {
  requests = [];
  localStorage.clear(); invalidateQueueOwner();
  const held = new Set<string>();
  Object.defineProperty(navigator, 'locks', { value: {
    async request<T>(name: string, _options: unknown, callback: (lock: Lock | null) => Promise<T>) {
      if (held.has(name)) return callback(null);
      held.add(name);
      try { return await callback({ name, mode: 'exclusive' }); } finally { held.delete(name); }
    },
  } });
  commit = async () => Response.json({ success: false, error_message: 'Inventory commit rejected' });
  vi.stubGlobal('fetch', vi.fn<typeof fetch>(async (input, init) => {
    const path = String(input);
    requests.push({ path, init });
    if (path === reservePath) return Response.json({ success: true, lock_id: 'owned-reservation', error_message: '' });
    if (path === '/api/v1/auth/session-identity') return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
    if (path === commitPath) return commit(JSON.parse(String(init?.body)));
    if (path === '/api/v1/checkout/session') return Response.json({ success: true, session_id: 'unused-session' });
    throw new Error(`Unexpected cash transport request: ${path}`);
  }));
});
afterEach(() => { cleanup(); invalidateQueueOwner(); vi.unstubAllGlobals(); vi.restoreAllMocks(); });

async function cashClient() {
  const success = vi.fn();
  const view = render(<StripeTerminalClient amount={5000} productId="product-one" tenantId="tenant-one"
    cart={[{ product: { id: 'product-one', price_cents: 5000 }, quantity: 1 }]} onSuccess={success} />);
  const user = userEvent.setup();
  await user.click(screen.getByRole('button', { name: 'Back' }));
  await user.click(screen.getByRole('button', { name: 'Cash' }));
  return { ...view, user, success, button: screen.getByRole('button', { name: 'Record Offline Cash Sale 50.00' }) };
}

it('does not report a rejected cash commit inside HTTP 200 as a recorded sale', async () => {
  const sale = await cashClient();
  await sale.user.click(sale.button);
  await waitFor(() => expect(requests.some(request => request.path === commitPath)).toBe(true));
  expect(sale.success).not.toHaveBeenCalled();
  expect(screen.queryByText('Status: Cash sale recorded.')).not.toBeInTheDocument();
});

it('uses one reservation lifecycle instead of pre-reserving the same cart in an unused checkout session', async () => {
  const sale = await cashClient();
  await sale.user.click(sale.button);
  await waitFor(() => expect(requests.some(request => request.path === commitPath)).toBe(true));
  expect(requests.filter(request => request.init?.method === 'POST').map(request => request.path)).toEqual([commitPath]);
});

it('waits for the complete cash acknowledgment body before reporting a sale', async () => {
  let finish!: (body: Uint8Array) => void;
  const response = new Response(new ReadableStream({ start(controller) { finish = bytes => { controller.enqueue(bytes); controller.close(); }; } }), {
    status: 200, headers: { 'content-type': 'application/json' },
  });
  commit = async () => response;
  const sale = await cashClient();
  await sale.user.click(sale.button);
  await waitFor(() => expect(requests.some(request => request.path === commitPath)).toBe(true));
  expect(sale.success).not.toHaveBeenCalled();
  expect(sale.button).toBeDisabled();
  await act(async () => { finish(new TextEncoder().encode('{"success":false,"error_message":"Commit rejected"}')); });
  expect(sale.success).not.toHaveBeenCalled();
});

it.each(['transport', 'malformed', 'server-error'] as const)('holds an ambiguous %s cash commit without replay or invented success', async failure => {
  commit = async () => {
    if (failure === 'transport') throw new Error('Connection lost after submitting commit');
    if (failure === 'malformed') return new Response('{', { status: 200 });
    return Response.json({ error: 'unavailable' }, { status: 502 });
  };
  const sale = await cashClient();
  await sale.user.click(sale.button);
  await waitFor(() => expect(requests.some(request => request.path === commitPath)).toBe(true));
  expect(sale.success).not.toHaveBeenCalled();
  expect(sale.button).toBeDisabled();
  expect(screen.queryByText('Status: Cash sale recorded.')).not.toBeInTheDocument();
  await sale.user.click(sale.button);
  expect(requests.filter(request => request.path === commitPath)).toHaveLength(1);
});

it('admits only one atomic cash commit when two clicks arrive before React updates the button', async () => {
  const sale = await cashClient();
  await act(async () => { sale.button.click(); sale.button.click(); });
  expect(requests.filter(request => request.path === reservePath)).toHaveLength(0);
  expect(requests.filter(request => request.path === commitPath)).toHaveLength(1);
});


it('reports cash success only from a complete receipt bound to the exact atomic request', async () => {
  commit = async body => completed(body);
  const sale = await cashClient();
  await sale.user.click(sale.button);
  await waitFor(() => expect(sale.success).toHaveBeenCalledExactlyOnceWith(5000));
  const writes = requests.filter(request => request.init?.method === 'POST');
  expect(writes).toHaveLength(1);
  expect(writes[0].path).toBe(commitPath);
  const body = JSON.parse(String(writes[0].init?.body));
  expect(body).toEqual({ operation_id: expect.any(String), tenant_id: owner.tenantId, amount_cents: 5000,
    items: [{ product_id: 'product-one', quantity: 1, amount_cents: 5000 }] });
  expect(body.operation_id).toMatch(/^[a-f0-9-]{36}$/);
  expect(writes[0].init?.headers).toMatchObject({ 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId });
});

it.each(['operation', 'tenant', 'amount', 'item', 'order'] as const)('holds a mismatched %s receipt instead of recording cash success', async mismatch => {
  commit = async body => {
    const receipt = (await completed(body).json()).receipt;
    if (mismatch === 'operation') receipt.operation_id = 'different-operation';
    if (mismatch === 'tenant') receipt.tenant_id = 'another-tenant';
    if (mismatch === 'amount') receipt.amount_cents = 1;
    if (mismatch === 'item') receipt.items[0].quantity = 2;
    if (mismatch === 'order') receipt.order_id = '';
    return Response.json({ success: true, status: 'completed', receipt });
  };
  const sale = await cashClient();
  await sale.user.click(sale.button);
  expect(sale.success).not.toHaveBeenCalled();
  expect(sale.button).toBeDisabled();
});

it('uses readback after a lost response and never posts the unknown operation again', async () => {
  let submitted!: Record<string, unknown>;
  commit = async body => { submitted = body; throw new Error('Response lost after commit'); };
  const sale = await cashClient();
  await sale.user.click(sale.button);
  expect(sale.success).not.toHaveBeenCalled();
  const transport = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation(async (input, init) => String(input) === `${commitPath}/${submitted.operation_id}`
    ? completed(submitted) : transport(input, init));
  await sale.user.click(screen.getByRole('button', { name: 'Check recorded cash sale' }));
  expect(sale.success).toHaveBeenCalledExactlyOnceWith(5000);
  expect(requests.filter(request => request.init?.method === 'POST')).toHaveLength(1);
});

it('does not submit a replacement cash sale after remount while an earlier result is unknown', async () => {
  commit = async () => { throw new Error('Response lost'); };
  const first = await cashClient();
  await first.user.click(first.button);
  first.unmount();
  const secondSuccess = vi.fn();
  render(<StripeTerminalClient amount={5000} productId="product-one" tenantId="tenant-one" onSuccess={secondSuccess} />);
  expect(screen.getByRole('button', { name: 'Record Offline Cash Sale 50.00' })).toBeDisabled();
  expect(screen.getByRole('button', { name: 'Back' })).toBeDisabled();
  expect(secondSuccess).not.toHaveBeenCalled();
  expect(requests.filter(request => request.path === commitPath)).toHaveLength(1);
  expect(screen.getByRole('button', { name: 'Check recorded cash sale' })).toBeEnabled();
});

it.each([5000, 6000])('does not apply a recovered earlier sale to a remounted %s-cent cart', async currentAmount => {
  let previous!: Record<string, unknown>;
  commit = async body => { previous = body; throw new Error('Receipt response lost'); };
  const first = await cashClient();
  await first.user.click(first.button);
  first.unmount();
  const success = vi.fn();
  const reserve = vi.fn();
  const id = currentAmount === 5000 ? 'product-one' : 'product-two';
  render(<StripeTerminalClient amount={currentAmount} productId={id} tenantId="tenant-one"
    cart={[{ product: { id, price_cents: currentAmount }, quantity: 1 }]} onSuccess={success} onOptimisticReserve={reserve} />);
  const transport = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation(async (input, init) => String(input).startsWith(commitPath + '/') ? completed(previous) : transport(input, init));
  await userEvent.setup().click(screen.getByRole('button', { name: 'Check recorded cash sale' }));
  expect(success).not.toHaveBeenCalled();
  expect(reserve).not.toHaveBeenCalled();
  expect(screen.getByText('Status: The earlier cash sale is recorded. This cart has not been recorded.')).toBeVisible();
  expect(screen.getByRole('button', { name: `Record Offline Cash Sale ${(currentAmount / 100).toFixed(2)}` })).toBeEnabled();
});

it('fences a late cash acknowledgment after the signed owner changes', async () => {
  let finish!: () => void;
  commit = body => new Promise(resolve => { finish = () => resolve(completed(body)); });
  const sale = await cashClient();
  await sale.user.click(sale.button);
  await waitFor(() => expect(finish).toBeDefined());
  act(() => notifyQueueIdentityChange());
  await act(async () => finish());
  expect(sale.success).not.toHaveBeenCalled();
});

it('does not send cash when the recovery journal cannot be stored', async () => {
  const sale = await cashClient();
  const original = localStorage.setItem;
  localStorage.setItem = () => { throw new Error('Storage unavailable'); };
  try {
    await sale.user.click(sale.button);
    expect(requests.filter(request => request.init?.method === 'POST')).toHaveLength(0);
    expect(sale.success).not.toHaveBeenCalled();
    expect(sale.button).toBeEnabled();
  } finally { localStorage.setItem = original; }
});

it('does not leave cash pending or send a sale when recovery storage cannot be read', async () => {
  const sale = await cashClient();
  const original = localStorage.getItem;
  localStorage.getItem = () => { throw new Error('Storage unavailable'); };
  try {
    await sale.user.click(sale.button);
    expect(requests.filter(request => request.init?.method === 'POST')).toHaveLength(0);
    expect(sale.success).not.toHaveBeenCalled();
    expect(sale.button).toBeEnabled();
    expect(screen.getByText(/Status: .*Storage unavailable/)).toBeVisible();
  } finally { localStorage.getItem = original; }
});

it('keeps an unknown cash operation held when readback has not found a committed receipt', async () => {
  commit = async () => { throw new Error('Response lost'); };
  const sale = await cashClient();
  await sale.user.click(sale.button);
  const transport = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation(async (input, init) => String(input).startsWith(commitPath + '/')
    ? Response.json({ success: false, status: 'not_found' }, { status: 404 }) : transport(input, init));
  await sale.user.click(screen.getByRole('button', { name: 'Check recorded cash sale' }));
  expect(sale.button).toBeDisabled();
  expect(sale.success).not.toHaveBeenCalled();
  expect(requests.filter(request => request.init?.method === 'POST')).toHaveLength(1);
});

it('allows an explicit identical-operation retry only after readback, never a replacement sale', async () => {
  let calls = 0;
  commit = async body => { if (++calls === 1) throw new Error('Response lost'); return completed(body); };
  const sale = await cashClient();
  await sale.user.click(sale.button);
  expect(screen.queryByRole('button', { name: 'Retry same cash sale' })).not.toBeInTheDocument();
  const original = String(requests.find(request => request.path === commitPath)?.init?.body);
  const transport = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation(async (input, init) => String(input).startsWith(commitPath + '/')
    ? Response.json({ success: false, status: 'not_found' }, { status: 404 }) : transport(input, init));
  await sale.user.click(screen.getByRole('button', { name: 'Check recorded cash sale' }));
  expect(requests.filter(request => request.path === commitPath)).toHaveLength(1);
  await sale.user.click(screen.getByRole('button', { name: 'Retry same cash sale' }));
  expect(requests.filter(request => request.path === commitPath).map(request => String(request.init?.body))).toEqual([original, original]);
  expect(sale.success).toHaveBeenCalledExactlyOnceWith(5000);
});

it('never converts an unknown online cash operation into a new offline sale', async () => {
  commit = async () => { throw new Error('Response lost'); };
  const sale = await cashClient();
  await sale.user.click(sale.button);
  const transport = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation(async (input, init) => String(input).startsWith(commitPath + '/')
    ? Response.json({ success: false, status: 'not_found' }, { status: 404 }) : transport(input, init));
  await sale.user.click(screen.getByRole('button', { name: 'Check recorded cash sale' }));
  const enqueue = vi.spyOn(MutationService.getInstance(), 'executeMutationBatch').mockResolvedValue(undefined);
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  await sale.user.click(screen.getByRole('button', { name: 'Retry same cash sale' }));
  expect(enqueue).not.toHaveBeenCalled();
  expect(sale.success).not.toHaveBeenCalled();
  expect(requests.filter(request => request.path === commitPath)).toHaveLength(1);
  expect(screen.getByText('Status: Reconnect to check the original cash sale. No replacement sale was queued.')).toBeVisible();
});

it('shows an acknowledged stock rejection without inventing a cash sale', async () => {
  commit = async () => Response.json({ success: false, status: 'rejected', error_message: 'Insufficient available stock' }, { status: 409 });
  const sale = await cashClient();
  await sale.user.click(sale.button);
  expect(screen.getByText('Status: Cash sale rejected. Review the cart and available inventory before trying again.')).toBeVisible();
  expect(sale.success).not.toHaveBeenCalled();
  expect(sale.button).toBeEnabled();
});

it('submits every cart line together and requires the whole order receipt', async () => {
  commit = async body => completed(body);
  const success = vi.fn();
  render(<StripeTerminalClient amount={12000} productId="product-two" tenantId="tenant-one" onSuccess={success}
    cart={[{ product: { id: 'product-two', price_cents: 3500 }, quantity: 2 }, { product: { id: 'product-one', price_cents: 5000 }, quantity: 1 }]} />);
  const user = userEvent.setup();
  await user.click(screen.getByRole('button', { name: 'Back' }));
  await user.click(screen.getByRole('button', { name: 'Cash' }));
  await user.click(screen.getByRole('button', { name: 'Record Offline Cash Sale 120.00' }));
  expect(success).toHaveBeenCalledExactlyOnceWith(12000);
  const writes = requests.filter(request => request.init?.method === 'POST');
  expect(writes).toHaveLength(1);
  expect(JSON.parse(String(writes[0].init?.body)).items).toEqual([
    { product_id: 'product-one', quantity: 1, amount_cents: 5000 },
    { product_id: 'product-two', quantity: 2, amount_cents: 7000 },
  ]);
});
