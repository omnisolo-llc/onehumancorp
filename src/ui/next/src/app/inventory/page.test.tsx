import React from 'react';
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import InventoryDashboard from './page';

// Supply has separate integration tests; keep these stock-receipt safety cases isolated.
vi.mock('./SupplyRecords', () => ({ SupplyRecords: () => null }));
vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: { children: React.ReactNode }) => <main>{children}</main> }));
const version = 'a'.repeat(64);
const product = { id: 'owned-product', name: 'Owner-made item', description: null, price_cents: 1250, currency: 'USD', stock: 3, inventory_version: version };
let currentOwner: { userId: string; tenantId: string };
let identityStatus: number;
const identity = () => identityStatus === 200 ? Response.json({ ...currentOwner, expiresAt: Date.now() + 60000 }) : new Response('expired identity', { status: identityStatus });
let inventory: () => Promise<Response>;
let mutate: (init: RequestInit) => Promise<Response>;
let posts: RequestInit[];
let checks: string[];
let lookup: (id: string) => Promise<Response>;
beforeEach(() => {
  localStorage.clear(); sessionStorage.clear(); posts = []; checks = [];
  currentOwner = { userId: 'owner', tenantId: 'tenant-a' }; identityStatus = 200;
  lookup = async id => Response.json({ success: false, receipt_status: 'not_found', id }, { status: 404 });
  inventory = async () => Response.json({ inventory: [] });
  mutate = async () => Response.json({});
  vi.stubGlobal('fetch', vi.fn(async (url: string, init?: RequestInit) => {
    if (url === '/api/v1/auth/session-identity') return identity();
    if (String(url).startsWith('/api/v1/ui/inventory')) {
      if (init?.method === 'POST') { posts.push(init); return mutate(init); }
      const id = new URL(String(url), 'https://app.test').searchParams.get('adjustment_id');
      if (id) { checks.push(id); return lookup(id); }
      return inventory();
    }
    throw new Error(`Unexpected route ${url}`);
  }));
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });

describe('Inventory persistence truth', () => {
  it('does not expose backend API route names in the inventory UI', async () => {
    render(<InventoryDashboard />);
    await screen.findByText('No products found in the inventory ledger.');
    expect(screen.getByText('Products & Variants')).toBeInTheDocument();
    expect(screen.queryByText(/\/api\/(?:v1\/)?ui\/inventory/)).not.toBeInTheDocument();
  });
  it('shows an actually empty inventory without fixture products', async () => {
    render(<InventoryDashboard />);
    expect(await screen.findByText('No products found in the inventory ledger.')).toBeInTheDocument();
    expect(screen.queryByText('Chocolate Cake')).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Increase stock' })).not.toBeInTheDocument();
  });
  it.each([{}, { inventory: null }, { inventory: [product, product] }, { inventory: [{ ...product, stock: -1 }] }])('does not turn a malformed payload into empty or fabricated stock: %j', async body => {
    inventory = async () => Response.json(body);
    render(<InventoryDashboard />);
    expect(await screen.findByRole('alert')).toHaveTextContent(/inventory.*unavailable|could not be loaded/i);
    expect(screen.queryByText('Chocolate Cake')).not.toBeInTheDocument();
    expect(screen.queryByTestId('stock-count-owned-product')).not.toBeInTheDocument();
  });
  it('does not optimistically change stock before a matching committed receipt', async () => {
    inventory = async () => Response.json({ inventory: [product] });
    let finish!: (r: Response) => void;
    mutate = () => new Promise(r => { finish = r; });
    render(<InventoryDashboard />);
    expect(await screen.findByTestId('stock-count-owned-product')).toHaveTextContent('3');
    fireEvent.click(screen.getByRole('button', { name: 'Increase stock' }));
    await waitFor(() => expect(posts).toHaveLength(1));
    expect(screen.getByTestId('stock-count-owned-product')).toHaveTextContent('3');
    expect(screen.getByRole('button', { name: 'Increase stock' })).toBeDisabled();
    const request = JSON.parse(posts[0].body as string)[0];
    expect(request.payload.expected_version).toBe(version);
    finish(Response.json({ status: 'ok', success: true, outcomes: [{ id: request.id, item_id: product.id, status: 'acknowledged', quantity_change: 1, previous_version: version, inventory_version: 'b'.repeat(64), stock: 4 }] }));
    expect(await screen.findByText('Stock adjustment saved.')).toBeInTheDocument();
    expect(screen.getByTestId('stock-count-owned-product')).toHaveTextContent('4');
  });
  it('checks unknown outcomes with GET and only retries the same operation on an explicit retry', async () => {
    inventory = async () => Response.json({ inventory: [product] });
    mutate = async () => Response.json({ status: 'ok' });
    render(<InventoryDashboard />);
    await screen.findByTestId('stock-count-owned-product');
    fireEvent.click(screen.getByRole('button', { name: 'Increase stock' }));
    const retry = await screen.findByRole('button', { name: 'Check saved adjustment' });
    expect(screen.getByTestId('stock-count-owned-product')).toHaveTextContent('3');
    fireEvent.click(retry);
    fireEvent.click(await screen.findByRole('button', { name: 'Retry same adjustment' }));
    expect(checks).toHaveLength(1);
    expect(checks[0]).toBe(JSON.parse(posts[0].body as string)[0].id);
    await waitFor(() => expect(posts).toHaveLength(2));
    expect(posts[1].body).toEqual(posts[0].body);
  });
  it('ignores a late inventory response after account change', async () => {
    let finish!: (r: Response) => void;
    inventory = () => new Promise(r => { finish = r; });
    render(<InventoryDashboard />);
    await waitFor(() => expect(finish).toBeDefined());
    window.dispatchEvent(new Event('omnisolo_auth_changed'));
    finish(Response.json({ inventory: [product] }));
    await screen.findByRole('alert');
    expect(screen.queryByTestId('stock-count-owned-product')).not.toBeInTheDocument();
  });
  it('keeps zero stock and unknown price truthful without preventing replenishment', async () => {
    inventory = async () => Response.json({ inventory: [{ ...product, stock: 0, price_cents: null, currency: null }] });
    render(<InventoryDashboard />);
    expect(await screen.findByTestId('stock-count-owned-product')).toHaveTextContent('0');
    expect(screen.getByText('Price unavailable')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Decrease stock' })).toBeDisabled();
    expect(screen.getByRole('button', { name: 'Increase stock' })).toBeEnabled();
  });
  it('retains the exact pending identity across remount before reconciling a lost response', async () => {
    inventory = async () => Response.json({ inventory: [product] });
    mutate = async () => { throw new Error('response lost'); };
    const first = render(<InventoryDashboard />);
    await screen.findByTestId('stock-count-owned-product');
    fireEvent.click(screen.getByRole('button', { name: 'Increase stock' }));
    await screen.findByRole('button', { name: 'Check saved adjustment' });
    first.unmount();
    lookup = async id => Response.json({ success: true, outcomes: [{ id, item_id: product.id, status: 'acknowledged', quantity_change: 1, previous_version: version, inventory_version: 'b'.repeat(64), stock: 4 }] });
    render(<InventoryDashboard />);
    fireEvent.click(await screen.findByRole('button', { name: 'Check saved adjustment' }));
    await screen.findByText('Earlier stock adjustment was saved. Reload inventory for current stock.');
    expect(posts).toHaveLength(1);
    expect(checks).toHaveLength(1);
    expect(screen.getByTestId('stock-count-owned-product')).toHaveTextContent('3');
  });
  it.each(['wrong-id', 'wrong-product', 'wrong-version', 'negative-stock', 'http-error'])('never accepts a contradictory receipt: %s', async fault => {
    inventory = async () => Response.json({ inventory: [product] });
    mutate = async init => {
      const request = JSON.parse(init.body as string)[0];
      return Response.json({ success: true, outcomes: [{
        id: fault === 'wrong-id' ? 'other' : request.id,
        item_id: fault === 'wrong-product' ? 'other' : product.id,
        status: fault === 'http-error' ? 'blocked' : 'acknowledged',
        quantity_change: 1, previous_version: fault === 'wrong-version' ? 'c'.repeat(64) : version,
        inventory_version: 'b'.repeat(64), stock: fault === 'negative-stock' ? -1 : 4,
      }] }, { status: fault === 'http-error' ? 503 : 200 });
    };
    render(<InventoryDashboard />); await screen.findByTestId('stock-count-owned-product');
    fireEvent.click(screen.getByRole('button', { name: 'Increase stock' }));
    await screen.findByText(/Stock adjustment is unconfirmed/);
    expect(screen.getByTestId('stock-count-owned-product')).toHaveTextContent('3');
    expect(screen.queryByText('Stock adjustment saved.')).not.toBeInTheDocument();
  });

  it('a no-record receipt GET keeps the held operation without submitting or declaring no change', async () => {
    inventory = async () => Response.json({ inventory: [product] });
    mutate = async () => { throw new Error('unknown transport'); };
    render(<InventoryDashboard />); await screen.findByTestId('stock-count-owned-product');
    fireEvent.click(screen.getByRole('button', { name: 'Increase stock' }));
    fireEvent.click(await screen.findByRole('button', { name: 'Check saved adjustment' }));
    await screen.findByText(/No saved receipt was found/);
    expect(posts).toHaveLength(1); expect(checks).toHaveLength(1);
    expect(screen.getByRole('button', { name: 'Increase stock' })).toBeDisabled();
    expect(screen.queryByText(/Stock was not changed/)).not.toBeInTheDocument();
  });

  const recoveryKey = 'inventory-adjustment-v1:' + encodeURIComponent(JSON.stringify(['owner', 'tenant-a']));
  const acknowledged = (request: { id: string }) => Response.json({ success: true, outcomes: [{ id: request.id, item_id: product.id, status: 'acknowledged', quantity_change: 1, previous_version: version, inventory_version: 'b'.repeat(64), stock: 4 }] });
  it.each(['different-owner', 'revoked-identity'])('retires old inventory before POST on silent identity retirement: %s', async fault => {
    inventory = async () => Response.json({ inventory: [product] });
    render(<InventoryDashboard />); await screen.findByTestId('stock-count-owned-product');
    if (fault === 'different-owner') currentOwner = { userId: 'owner-b', tenantId: 'tenant-b' };
    else identityStatus = 401;
    fireEvent.click(screen.getByRole('button', { name: 'Increase stock' }));
    await screen.findByRole('alert');
    expect(screen.queryByTestId('stock-count-owned-product')).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Increase stock' })).not.toBeInTheDocument();
    expect(posts).toHaveLength(0);
    const saved = sessionStorage.getItem(recoveryKey);
    expect(saved).not.toBeNull();
    currentOwner = { userId: 'owner', tenantId: 'tenant-a' }; identityStatus = 200;
    fireEvent.click(screen.getByRole('button', { name: 'Reload inventory' }));
    await screen.findByRole('button', { name: 'Check saved adjustment' });
    expect(sessionStorage.getItem(recoveryKey)).toBe(saved);
    expect(posts).toHaveLength(0);
  });
  it.each(['different-owner', 'revoked-identity'])('retires an old-owner delayed POST result without clearing recovery: %s', async fault => {
    inventory = async () => Response.json({ inventory: [product] });
    let finish!: (response: Response) => void;
    mutate = () => new Promise(resolve => { finish = resolve; });
    render(<InventoryDashboard />); await screen.findByTestId('stock-count-owned-product');
    fireEvent.click(screen.getByRole('button', { name: 'Increase stock' }));
    await waitFor(() => expect(posts).toHaveLength(1));
    const saved = sessionStorage.getItem(recoveryKey);
    if (fault === 'different-owner') currentOwner = { userId: 'owner-b', tenantId: 'tenant-b' };
    else identityStatus = 401;
    finish(acknowledged(JSON.parse(posts[0].body as string)[0]));
    await screen.findByRole('alert');
    expect(screen.queryByTestId('stock-count-owned-product')).not.toBeInTheDocument();
    expect(screen.queryByText('Stock adjustment saved.')).not.toBeInTheDocument();
    expect(sessionStorage.getItem(recoveryKey)).toBe(saved);
  });
  it.each([401, 403])('retires the private view on an empty/non-JSON HTTP %s write rejection', async status => {
    inventory = async () => Response.json({ inventory: [product] });
    mutate = async () => new Response(status === 401 ? '' : 'forbidden', { status });
    render(<InventoryDashboard />); await screen.findByTestId('stock-count-owned-product');
    fireEvent.click(screen.getByRole('button', { name: 'Increase stock' }));
    await screen.findByRole('alert');
    expect(screen.queryByTestId('stock-count-owned-product')).not.toBeInTheDocument();
    expect(screen.queryByText('Stock adjustment saved.')).not.toBeInTheDocument();
    expect(sessionStorage.getItem(recoveryKey)).not.toBeNull();
  });
  it.each(['different-owner', 'revoked-identity', 'empty-401', 'nonjson-403'])('retires delayed receipt GET on identity/auth rejection: %s', async fault => {
    inventory = async () => Response.json({ inventory: [product] });
    mutate = async () => { throw new Error('unknown'); };
    let finish!: (response: Response) => void;
    lookup = () => new Promise(resolve => { finish = resolve; });
    render(<InventoryDashboard />); await screen.findByTestId('stock-count-owned-product');
    fireEvent.click(screen.getByRole('button', { name: 'Increase stock' }));
    const check = await screen.findByRole('button', { name: 'Check saved adjustment' });
    await waitFor(() => expect(check).toBeEnabled()); fireEvent.click(check);
    await waitFor(() => expect(checks).toHaveLength(1));
    const saved = sessionStorage.getItem(recoveryKey);
    if (fault === 'different-owner') currentOwner = { userId: 'owner-b', tenantId: 'tenant-b' };
    if (fault === 'revoked-identity') identityStatus = 401;
    finish(fault === 'empty-401' ? new Response('', { status: 401 }) : fault === 'nonjson-403' ? new Response('forbidden', { status: 403 }) : acknowledged({ id: checks[0] }));
    await screen.findByRole('alert');
    expect(screen.queryByTestId('stock-count-owned-product')).not.toBeInTheDocument();
    expect(screen.queryByText('Earlier stock adjustment was saved. Reload inventory for current stock.')).not.toBeInTheDocument();
    expect(posts).toHaveLength(1); expect(sessionStorage.getItem(recoveryKey)).toBe(saved);
  });

  it('reports unresolved PN shortage without claiming stock or clearing its business debt', async () => {
    inventory = async () => Response.json({ inventory: [{ ...product, stock: 0 }] });
    mutate = async init => {
      const request = JSON.parse(init.body as string)[0];
      return Response.json({ success: false, outcomes: [{ id: request.id, item_id: product.id, quantity_change: 1, previous_version: version, status: 'blocked', reason: 'inventory_debt_requires_reconciliation' }] });
    };
    render(<InventoryDashboard />); await screen.findByTestId('stock-count-owned-product');
    fireEvent.click(screen.getByRole('button', { name: 'Increase stock' }));
    await screen.findByText('Stock adjustment is blocked by an unresolved inventory shortage. Reconcile the shortage before adjusting stock.');
    expect(screen.getByTestId('stock-count-owned-product')).toHaveTextContent('0');
    expect(screen.queryByText('Stock adjustment saved.')).not.toBeInTheDocument();
  });

});
