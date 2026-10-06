import React from 'react';
import { cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import InventoryDashboard from './page';

vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: { children: React.ReactNode }) => <main>{children}</main> }));
const material = { id: 'flour', name: 'Recorded flour', current_quantity: 2, reorder_threshold: 5 };
const vendor = { id: 'mill', name: 'Recorded mill', contact_info: 'orders@example.test' };
const product = { id: 'item', name: 'Recorded product', description: null, price_cents: 100, currency: 'USD', stock: 3, inventory_version: 'a'.repeat(64) };
const recorded = { raw_materials: [material], vendors: [vendor], bom_items: [] };
let supply: () => Promise<Response>;
let owner: { userId: string; tenantId: string };
let identityStatus: number;
let expiresAt: number;
let requests: RequestInit[];
beforeEach(() => {
  localStorage.clear(); sessionStorage.clear(); requests = [];
  owner = { userId: 'owner', tenantId: 'tenant' }; identityStatus = 200; expiresAt = Date.now() + 60000;
  supply = async () => Response.json(recorded);
  vi.stubGlobal('fetch', vi.fn(async (url: string, init?: RequestInit) => {
    if (url === '/api/v1/auth/session-identity') return identityStatus === 200 ? Response.json({ ...owner, expiresAt }) : new Response('', { status: identityStatus });
    if (url === '/api/v1/ui/inventory') return Response.json({ inventory: [product] });
    if (url === '/api/v1/ui/supply') { requests.push(init ?? {}); return supply(); }
    throw new Error(`Unexpected request ${url}`);
  }));
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
const region = () => within(screen.getByRole('region', { name: 'Supply records' }));

describe('Recorded supply alongside the product ledger', () => {
  it('renders actual material thresholds and vendor details beside independently loaded products', async () => {
    render(<InventoryDashboard />);
    expect(await screen.findByText(material.name)).toBeInTheDocument();
    expect(region().getByRole('heading', { name: 'Raw Materials' })).toBeInTheDocument();
    expect(region().getByRole('heading', { name: 'Vendors' })).toBeInTheDocument();
    expect(region().getByText(vendor.name)).toBeInTheDocument();
    expect(region().getByText(vendor.contact_info)).toBeInTheDocument();
    expect(region().getByTestId('material-quantity-flour')).toHaveTextContent('2');
    expect(region().getByTestId('material-threshold-flour')).toHaveTextContent('5');
    expect(region().getByText('Low Stock')).toBeInTheDocument();
    expect(await screen.findByTestId('stock-count-item')).toHaveTextContent('3');
    expect(screen.queryByText(/\/api\/v1\/ui\/supply/)).not.toBeInTheDocument();
    expect(requests[0]).toMatchObject({ credentials: 'same-origin', cache: 'no-store', redirect: 'error', headers: { 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId } });
  });
  it('shows loading rather than invented empty rows, then distinguishes an empty result', async () => {
    let finish!: (response: Response) => void;
    supply = () => new Promise(resolve => { finish = resolve; });
    render(<InventoryDashboard />);
    await waitFor(() => expect(finish).toBeDefined());
    expect(region().getByRole('status')).toHaveTextContent('Loading supply records');
    expect(region().queryByText(/No .* recorded/)).not.toBeInTheDocument();
    finish(Response.json({ vendors: [], raw_materials: [], bom_items: [] }));
    expect(await screen.findByText('No raw materials recorded for this business.')).toBeInTheDocument();
    expect(region().getByText('No vendors recorded for this business.')).toBeInTheDocument();
  });
  it.each([{}, { raw_materials: [], vendors: null }, { ...recorded, error: 'failed' }, { ...recorded, success: 'false' },
    { ...recorded, raw_materials: [material, material] }, { ...recorded, vendors: [vendor, vendor] },
    { ...recorded, raw_materials: [{ ...material, current_quantity: null }] },
    { ...recorded, raw_materials: [{ ...material, current_quantity: -1 }] },
    { ...recorded, raw_materials: [{ ...material, reorder_threshold: '5' }] },
    { ...recorded, vendors: [{ ...vendor, contact_info: {} }] },
  ])('rejects invalid supply without fake empty or zero: %j', async payload => {
    supply = async () => Response.json(payload);
    render(<InventoryDashboard />);
    await waitFor(() => expect(region().getByRole('alert')).toHaveTextContent('Supply records are unavailable'));
    expect(region().queryByText(/No .* recorded/)).not.toBeInTheDocument();
    expect(region().queryByTestId('material-quantity-flour')).not.toBeInTheDocument();
    expect(await screen.findByTestId('stock-count-item')).toHaveTextContent('3');
  });
  it('clears stale records on a failed refresh while keeping the healthy product ledger', async () => {
    render(<InventoryDashboard />); await screen.findByText(material.name);
    supply = async () => new Response('database unavailable', { status: 503 });
    fireEvent.click(region().getByRole('button', { name: 'Reload supply records' }));
    await waitFor(() => expect(region().getByRole('alert')).toHaveTextContent('Supply records are unavailable'));
    expect(region().queryByText(material.name)).not.toBeInTheDocument();
    expect(region().queryByText(/No .* recorded/)).not.toBeInTheDocument();
    expect(screen.getByTestId('stock-count-item')).toHaveTextContent('3');
    supply = async () => Response.json({ ...recorded, raw_materials: [{ ...material, current_quantity: 9 }] });
    fireEvent.click(region().getByRole('button', { name: 'Reload supply records' }));
    await waitFor(() => expect(region().getByTestId('material-quantity-flour')).toHaveTextContent('9'));
    expect(region().getByText('Healthy')).toBeInTheDocument();
  });
  it.each(['auth-event', 'storage-event', 'pagehide', 'silent-owner-change', 'revoked'])('rejects late supply after %s', async change => {
    let finish!: (response: Response) => void;
    supply = () => new Promise(resolve => { finish = resolve; });
    render(<InventoryDashboard />); await waitFor(() => expect(finish).toBeDefined());
    await screen.findByTestId('stock-count-item');
    if (change === 'auth-event') window.dispatchEvent(new Event('omnisolo_auth_changed'));
    if (change === 'storage-event') window.dispatchEvent(new StorageEvent('storage', { key: 'omnisolo_queue_identity_epoch_v2' }));
    if (change === 'pagehide') window.dispatchEvent(new Event('pagehide'));
    if (change === 'silent-owner-change') owner = { userId: 'other', tenantId: 'foreign' };
    if (change === 'revoked') identityStatus = 401;
    finish(Response.json(recorded));
    await waitFor(() => expect(region().getByRole('alert')).toBeInTheDocument());
    expect(region().queryByText(material.name)).not.toBeInTheDocument();
    expect(region().queryByText(vendor.name)).not.toBeInTheDocument();
    if (change === 'silent-owner-change' || change === 'revoked') fireEvent.click(screen.getByRole('button', { name: 'Increase stock' }));
    await waitFor(() => expect(screen.queryByTestId('stock-count-item')).not.toBeInTheDocument());
  });
  it.each([401, 403, 409])('removes supply after HTTP %s without parsing a success body', async status => {
    supply = async () => Response.json(recorded, { status });
    render(<InventoryDashboard />);
    await waitFor(() => expect(region().getByRole('alert')).toBeInTheDocument());
    expect(region().queryByText(material.name)).not.toBeInTheDocument();
  });
  it('retires visible supply when its verified identity expires', async () => {
    expiresAt = Date.now() + 400;
    render(<InventoryDashboard />); await screen.findByText(material.name);
    await waitFor(() => expect(region().queryByText(material.name)).not.toBeInTheDocument());
    expect(region().getByRole('alert')).toHaveTextContent(/session expired/i);
  });
});
