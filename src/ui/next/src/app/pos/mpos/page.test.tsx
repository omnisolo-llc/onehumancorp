import React from 'react';
import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import POSTerminalMobile from './page';
import userEvent from '@testing-library/user-event';
import { cashItems } from '../terminal/cashReceipt';
import type { CartItem } from '@/lib/business-records';
import { notifyQueueIdentityChange, readQueueOwner, currentVerifiedQueueOwner, subscribeQueueIdentityReadiness } from '@/lib/sync/queueIdentity';

const { useSearchParamsMock } = vi.hoisted(() => ({
  useSearchParamsMock: vi.fn(),
}));

vi.mock('next/navigation', () => ({
  useSearchParams: useSearchParamsMock,
}));

vi.mock('../terminal/StripeTerminalClient', () => ({
  default: ({ amount, cart, tenantId }: { amount: number; cart: CartItem[]; tenantId: string }) => <div data-testid="stripe-terminal-client" data-amount={amount} data-cart={JSON.stringify(cart)} data-tenant={tenantId} />,
}));

describe('POSTerminalMobile', () => {
  let products: { id: string; title: string; price_cents: number; image_url?: string }[];
  let owner: { userId: string; tenantId: string };
  afterEach(() => { cleanup(); vi.useRealTimers(); });
  beforeEach(() => {
    useSearchParamsMock.mockReturnValue(new URLSearchParams('tenantId=forged-query-tenant'));
    localStorage.clear(); notifyQueueIdentityChange();
    owner = { userId: 'mobile-owner', tenantId: 'e2e-tenant' };
    products = [{ id: 'e2e-product-cake', title: 'Vegan Celebration Cake', price_cents: 3999, image_url: '/dashboard_with_charts.png' }];
    global.fetch = vi.fn(async input => {
      const url = String(input);
      if (url.endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
      if (url === '/api/v1/catalog/products') return Response.json(products);
      if (url === '/api/v1/pos/inventory') return Response.json({ inventory: products });
      throw new Error(`Unexpected mobile read: ${url}`);
    });
  });

  it.each([0, 1])('waits for overlapping canonical identity reads before loading its owner catalog (first reply %s)', async firstReply => {
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    const replies: ((response: Response) => void)[] = [];
    const transport = global.fetch;
    vi.mocked(global.fetch).mockImplementation(input => {
      if (String(input).endsWith('/session-identity') && replies.length < 2) {
        return new Promise<Response>(resolve => { replies.push(resolve); });
      }
      if (String(input).endsWith('/session-identity')) return Promise.resolve(Response.json({ ...owner, expiresAt: Date.now() + 60_000 }));
      if (String(input) === '/api/v1/pos/inventory') return Promise.resolve(Response.json({ inventory: products }));
      throw new Error(`Unexpected mobile read: ${String(input)}`);
    });
    render(<POSTerminalMobile />);
    let concurrent!: ReturnType<typeof readQueueOwner>;
    act(() => { concurrent = readQueueOwner(); });
    expect(replies).toHaveLength(2);
    await act(async () => { replies[firstReply](Response.json({ ...owner, expiresAt: Date.now() + 60_000 })); });
    expect(screen.queryByRole('alert')).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Quick Charge' })).toBeDisabled();
    expect(screen.queryByText('Vegan Celebration Cake')).not.toBeInTheDocument();
    await act(async () => {
      replies[1 - firstReply](Response.json({ ...owner, expiresAt: Date.now() + 60_000 }));
      await concurrent;
    });
    expect(await screen.findByText('Vegan Celebration Cake')).toBeVisible();
    const inventory = vi.mocked(transport).mock.calls.find(([url]) => String(url) === '/api/v1/pos/inventory');
    expect(inventory).toBeDefined();
    expect(new Headers(inventory![1]?.headers).get('x-ohc-expected-tenant')).toBe(owner.tenantId);
    expect(new Headers(inventory![1]?.headers).get('x-ohc-expected-user')).toBe(owner.userId);
  });

  it('waits for a concurrent identity check when the catalog response body finishes', async () => {
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    let finishBody!: (value: unknown) => void;
    vi.mocked(global.fetch).mockImplementation(async input => {
      if (String(input).endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
      if (String(input) === '/api/v1/pos/inventory') return { status: 200, json: () => new Promise(resolve => { finishBody = resolve; }) } as Response;
      throw new Error('Unexpected mobile request');
    });
    await act(async () => { render(<POSTerminalMobile />); });
    let finishVerification!: (response: Response) => void;
    vi.mocked(global.fetch).mockImplementationOnce(() => new Promise<Response>(resolve => { finishVerification = resolve; }));
    let concurrent!: ReturnType<typeof readQueueOwner>;
    act(() => { concurrent = readQueueOwner(); });
    await act(async () => { finishBody({ inventory: products }); });
    expect(screen.queryByRole('alert')).not.toBeInTheDocument();
    expect(screen.queryByText('Vegan Celebration Cake')).not.toBeInTheDocument();
    await act(async () => { finishVerification(Response.json({ ...owner, expiresAt: Date.now() + 60_000 })); await concurrent; });
    expect(await screen.findByText('Vegan Celebration Cake')).toBeVisible();
  });

  it('retains its catalog across consecutive same-owner readiness checks without replacing the original lease', async () => {
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    let finishBody!: () => void;
    vi.mocked(global.fetch).mockImplementation(async input => {
      if (String(input).endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
      if (String(input) === '/api/v1/pos/inventory') return new Response(new ReadableStream<Uint8Array>({ start(controller) {
        finishBody = () => { controller.enqueue(new TextEncoder().encode(JSON.stringify({ inventory: products }))); controller.close(); };
      } }));
      throw new Error('Unexpected mobile request');
    });
    await act(async () => { render(<POSTerminalMobile />); });
    let finishFirst!: (response: Response) => void;
    vi.mocked(global.fetch).mockImplementationOnce(() => new Promise<Response>(resolve => { finishFirst = resolve; }));
    let first!: ReturnType<typeof readQueueOwner>;
    act(() => { first = readQueueOwner(); });
    await act(async () => finishBody());
    let finishSecond!: (response: Response) => void;
    let second!: ReturnType<typeof readQueueOwner>;
    vi.mocked(global.fetch).mockImplementationOnce(() => new Promise<Response>(resolve => { finishSecond = resolve; }));
    let startSecond = true;
    const unsubscribe = subscribeQueueIdentityReadiness(() => {
      if (startSecond && currentVerifiedQueueOwner()) { startSecond = false; second = readQueueOwner(); }
    });
    try {
      await act(async () => { finishFirst(Response.json({ ...owner, expiresAt: Date.now() + 60_000 })); await first; });
      expect(finishSecond).toBeDefined();
      expect(screen.queryByText('Vegan Celebration Cake')).toBeNull();
      expect(screen.getByRole('button', { name: 'Quick Charge' })).toBeDisabled();
      await act(async () => { finishSecond(Response.json({ ...owner, expiresAt: Date.now() + 60_000 })); await second; });
      expect(await screen.findByText('Vegan Celebration Cake')).toBeVisible();
      expect(screen.queryByRole('alert')).toBeNull();
      expect(vi.mocked(global.fetch).mock.calls.filter(([url]) => String(url) === '/api/v1/pos/inventory')).toHaveLength(1);
    } finally { unsubscribe(); }
  });

  it.each(['rejected', 'different owner', 'expired'])('does not load inventory when the remaining initial verification is %s', async outcome => {
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    const replies: ((response: Response) => void)[] = [];
    vi.mocked(global.fetch).mockImplementation(() => new Promise<Response>(resolve => { replies.push(resolve); }));
    render(<POSTerminalMobile />);
    let concurrent!: Promise<unknown>;
    act(() => { concurrent = readQueueOwner().catch(error => error); });
    await act(async () => { replies[0](Response.json({ ...owner, expiresAt: Date.now() + 60_000 })); });
    await act(async () => {
      replies[1](outcome === 'rejected' ? new Response('{}', { status: 401 }) : Response.json({
        ...owner, tenantId: outcome === 'different owner' ? 'other-tenant' : owner.tenantId,
        expiresAt: Date.now() + (outcome === 'expired' ? -1 : 60_000),
      }));
      await concurrent;
    });
    expect(await screen.findByRole('alert')).toHaveTextContent('could not be verified');
    expect(screen.getByRole('button', { name: 'Quick Charge' })).toBeDisabled();
    expect(screen.queryByText('Vegan Celebration Cake')).not.toBeInTheDocument();
    expect(vi.mocked(global.fetch).mock.calls.every(([url]) => String(url).endsWith('/session-identity'))).toBe(true);
  });

  it('cancels an initial lease wait on unmount without dispatching a late inventory read', async () => {
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    const replies: ((response: Response) => void)[] = [];
    vi.mocked(global.fetch).mockImplementation(() => new Promise<Response>(resolve => { replies.push(resolve); }));
    const view = render(<POSTerminalMobile />);
    let concurrent!: ReturnType<typeof readQueueOwner>;
    act(() => { concurrent = readQueueOwner(); });
    await act(async () => { replies[0](Response.json({ ...owner, expiresAt: Date.now() + 60_000 })); });
    view.unmount();
    await act(async () => { replies[1](Response.json({ ...owner, expiresAt: Date.now() + 60_000 })); await concurrent; });
    expect(vi.mocked(global.fetch).mock.calls).toHaveLength(2);
  });

  it('retires the original lease if it expires while a catalog body waits for another identity read', async () => {
    vi.useFakeTimers();
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    let finishBody!: (value: unknown) => void;
    vi.mocked(global.fetch).mockImplementation(async input => {
      if (String(input).endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 1000 });
      if (String(input) === '/api/v1/pos/inventory') return { status: 200, json: () => new Promise(resolve => { finishBody = resolve; }) } as Response;
      throw new Error('Unexpected mobile request');
    });
    await act(async () => { render(<POSTerminalMobile />); });
    let finishVerification!: (response: Response) => void;
    vi.mocked(global.fetch).mockImplementationOnce(() => new Promise<Response>(resolve => { finishVerification = resolve; }));
    let concurrent!: ReturnType<typeof readQueueOwner>;
    act(() => { concurrent = readQueueOwner(); });
    await act(async () => { finishBody({ inventory: products }); });
    await act(async () => { await vi.advanceTimersByTimeAsync(1001); });
    expect(screen.getByRole('alert')).toHaveTextContent('could not be verified');
    await act(async () => { finishVerification(Response.json({ ...owner, expiresAt: Date.now() + 60_000 })); await concurrent; });
    expect(screen.queryByText('Vegan Celebration Cake')).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Quick Charge' })).toBeDisabled();
  });

  it('normalizes the backend catalog contract before rendering prices', async () => {
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    render(<POSTerminalMobile />);

    await waitFor(() => expect(screen.getByText('Vegan Celebration Cake')).toBeInTheDocument());
    expect(screen.getByText('$39.99')).toBeInTheDocument();
    expect(global.fetch).toHaveBeenCalledWith('/api/v1/pos/inventory', expect.objectContaining({ headers: expect.any(Headers) }));
    expect(screen.getByRole('heading', { name: 'mPOS' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Quick Charge' })).toBeDisabled();
  });

  it('passes exact integer cents and every quantity from the real mobile cart to cash checkout', async () => {
    products = [{ id: 'cent-product', title: 'Tea', price_cents: 255 }];
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    render(<POSTerminalMobile />);
    const product = await screen.findByText('Tea');
    const user = userEvent.setup();
    await user.click(product); await user.click(product); await user.click(product);
    await user.click(screen.getByRole('button', { name: 'Quick Charge' }));
    const terminal = screen.getByTestId('stripe-terminal-client');
    const amount = Number(terminal.getAttribute('data-amount'));
    const cart = JSON.parse(terminal.getAttribute('data-cart') || 'null') as CartItem[];
    expect(amount).toBe(765);
    expect(cart[0].product.price_cents).toBe(255);
    expect(cashItems(cart, 'mpos_cart', amount)).toEqual([{ product_id: 'cent-product', quantity: 3, amount_cents: 765 }]);
  });

  it.each(['', 'tenantId=another-tenant'])('uses verified owner authority rather than the destination query %s', async query => {
    useSearchParamsMock.mockReturnValue(new URLSearchParams(query));
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    render(<POSTerminalMobile />);
    const user = userEvent.setup();
    await user.click(await screen.findByText('Vegan Celebration Cake'));
    await user.click(screen.getByRole('button', { name: 'Quick Charge' }));
    expect(screen.getByTestId('stripe-terminal-client')).toHaveAttribute('data-tenant', owner.tenantId);
  });

  it('clears the old owner cart and catalog when signed identity changes', async () => {
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    render(<POSTerminalMobile />);
    const user = userEvent.setup();
    await user.click(await screen.findByText('Vegan Celebration Cake'));
    await user.click(screen.getByRole('button', { name: 'Quick Charge' }));
    owner = { userId: 'new-mobile-owner', tenantId: 'new-mobile-tenant' };
    products = [{ id: 'new-product', title: 'New owner product', price_cents: 1234 }];
    act(() => notifyQueueIdentityChange());
    expect(screen.queryByText('Vegan Celebration Cake')).not.toBeInTheDocument();
    expect(screen.queryByTestId('stripe-terminal-client')).not.toBeInTheDocument();
    expect(await screen.findByText('New owner product')).toBeVisible();
    expect(screen.getByRole('button', { name: 'Quick Charge' })).toBeDisabled();
  });

  it('never restores an unscoped legacy catalog before owner verification', async () => {
    localStorage.setItem('omnisolo_catalog_cache', JSON.stringify([{ id: 'old', name: 'Other owner private catalog', price: 99 }]));
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: false });
    render(<POSTerminalMobile />);
    expect(screen.queryByText('Other owner private catalog')).not.toBeInTheDocument();
    expect(await screen.findByRole('alert')).toHaveTextContent('verified');
  });

  it('retires private catalog, cart and payment view when the signed lease expires', async () => {
    vi.useFakeTimers();
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    await act(async () => { render(<POSTerminalMobile />); });
    fireEvent.click(screen.getByText('Vegan Celebration Cake'));
    fireEvent.click(screen.getByRole('button', { name: 'Quick Charge' }));
    expect(screen.getByTestId('stripe-terminal-client')).toBeVisible();
    await act(async () => { await vi.advanceTimersByTimeAsync(60_001); });
    expect(screen.queryByText('Vegan Celebration Cake')).not.toBeInTheDocument();
    expect(screen.queryByTestId('stripe-terminal-client')).not.toBeInTheDocument();
    expect(screen.getByRole('alert')).toHaveTextContent('verified');
    expect(screen.getByRole('button', { name: 'Quick Charge' })).toBeDisabled();
  });

  it('hides the private owner view while verification is pending and retires it on failure', async () => {
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    render(<POSTerminalMobile />);
    const product = await screen.findByText('Vegan Celebration Cake');
    fireEvent.click(product);
    fireEvent.click(screen.getByRole('button', { name: 'Quick Charge' }));
    let rejectVerification!: (error: Error) => void;
    vi.mocked(global.fetch).mockImplementationOnce(() => new Promise<Response>((_resolve, reject) => { rejectVerification = reject; }));
    let verification!: Promise<void>;
    act(() => { verification = readQueueOwner().then(() => undefined, () => undefined); });
    expect(product).not.toBeVisible();
    expect(screen.getByTestId('stripe-terminal-client')).not.toBeVisible();
    await act(async () => { rejectVerification(new Error('Owned session unavailable')); await verification; });
    expect(screen.queryByText('Vegan Celebration Cake')).not.toBeInTheDocument();
    expect(screen.queryByTestId('stripe-terminal-client')).not.toBeInTheDocument();
    expect(screen.getByRole('alert')).toHaveTextContent('verified');
  });

  it('does not apply a catalog body that finishes after the signed lease expires', async () => {
    vi.useFakeTimers();
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    let finishBody!: (value: unknown) => void;
    vi.mocked(global.fetch).mockImplementation(async input => {
      if (String(input).endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 1000 });
      if (String(input) === '/api/v1/pos/inventory') return { status: 200, json: () => new Promise(resolve => { finishBody = resolve; }) } as Response;
      throw new Error('Unexpected mobile request');
    });
    await act(async () => { render(<POSTerminalMobile />); });
    expect(finishBody).toBeTypeOf('function');
    await act(async () => { await vi.advanceTimersByTimeAsync(1001); });
    await act(async () => { finishBody({ inventory: products }); });
    expect(screen.queryByText('Vegan Celebration Cake')).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Quick Charge' })).toBeDisabled();
    expect(screen.getByRole('alert')).toHaveTextContent('verified');
  });

  it('preserves the current cart and mounted payment flow after same-owner verification succeeds', async () => {
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    render(<POSTerminalMobile />);
    fireEvent.click(await screen.findByText('Vegan Celebration Cake'));
    fireEvent.click(screen.getByRole('button', { name: 'Quick Charge' }));
    const terminal = screen.getByTestId('stripe-terminal-client');
    let finishVerification!: (response: Response) => void;
    vi.mocked(global.fetch).mockImplementationOnce(() => new Promise<Response>(resolve => { finishVerification = resolve; }));
    let verification!: ReturnType<typeof readQueueOwner>;
    act(() => { verification = readQueueOwner(); });
    expect(terminal).not.toBeVisible();
    await act(async () => { finishVerification(Response.json({ ...owner, expiresAt: Date.now() + 60_000 })); await verification; });
    expect(screen.getByTestId('stripe-terminal-client')).toBe(terminal);
    expect(terminal).toBeVisible();
    expect(terminal).toHaveAttribute('data-amount', '3999');
    expect(terminal).toHaveAttribute('data-tenant', owner.tenantId);
  });

  it('shows a loading fallback while the query string is being resolved', () => {
    const pendingSearchParams = new Promise<never>(() => {});
    useSearchParamsMock.mockImplementation(() => {
      throw pendingSearchParams;
    });

    render(<POSTerminalMobile />);

    expect(screen.getByText('Loading mPOS...')).toBeInTheDocument();
  });
});
