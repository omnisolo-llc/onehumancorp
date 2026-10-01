import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import type { ReactNode } from 'react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import KitchenView from './page';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { ownedOnboardingKey } from '../onboarding/draftSession';
const owner = { userId: 'kitchen-user', tenantId: 'kitchen-tenant' };
vi.mock('@/lib/sync/queueIdentity', async original => ({ ...await original<typeof import('@/lib/sync/queueIdentity')>(), readQueueOwner: vi.fn(async () => ({ userId: 'kitchen-user', tenantId: 'kitchen-tenant' })) }));

vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: { children: ReactNode }) => <main>{children}</main> }));
const sync = vi.hoisted(() => ({ enqueue: vi.fn(), getQueueLength: vi.fn(async () => 0) }));
vi.mock('../../lib/sync/SyncManager', () => ({ SyncManager: { getInstance: () => sync } }));
beforeEach(() => { localStorage.clear(); notifyQueueIdentityChange(); vi.clearAllMocks(); sync.enqueue.mockReset().mockResolvedValue(undefined); });
afterEach(() => vi.unstubAllGlobals());

it.each([false, true])('keeps an authoritative empty kitchen empty without invented or stale records (cache: %s)', async (withCache) => {
  if (withCache) localStorage.setItem('kds_orders_cache', JSON.stringify([{ id: 'old', customer_name: 'Stale customer', status: 'pending' }]));
  if (withCache) localStorage.setItem('kds_menu_cache', JSON.stringify([{ id: 'old-menu', title: 'Stale product' }]));
  const legacyOrders = localStorage.getItem('kds_orders_cache'); const legacyMenu = localStorage.getItem('kds_menu_cache');
  vi.stubGlobal('fetch', vi.fn(async () => new Response('[]', { status: 200 })));
  render(<KitchenView />);
  await waitFor(() => expect(localStorage.getItem(ownedOnboardingKey('kitchen-inventory-v1')!)).toBe(JSON.stringify({ format: 1, records: [] })));
  expect(localStorage.getItem('kds_orders_cache')).toBe(legacyOrders); expect(localStorage.getItem('kds_menu_cache')).toBe(legacyMenu);
  expect(screen.getByText('No active orders')).toBeVisible();
  expect(screen.queryByRole('button', { name: 'Mark Ready & Notify' })).toBeNull();
  expect(screen.queryByRole('button', { name: 'Mark Sold Out' })).toBeNull();
  expect(screen.queryByText(/Alice|Bob|Stale customer|Stale product/)).toBeNull();
});

it('renders recorded translations and queues the real order and menu identities', async () => {
  vi.stubGlobal('fetch', vi.fn(async (url: string) => new Response(JSON.stringify(
    url.endsWith('/orders') ? { orders: [{ id: 'order-1', customer_name: 'Alice', status: 'pending', notes: 'No onions', translated_notes: 'بدون بصل', updated_at: '2026-10-01T00:00:00Z' }] }
      : { inventory: [{ id: 'menu-1', name: 'Falafel Wrap', stock: 10, is_sold_out: false, updated_at: '2026-10-01T00:00:00Z' }] },
  ), { status: 200 })));
  render(<KitchenView />);
  expect(await screen.findByText('بدون بصل')).toBeVisible();
  const soldOut = await screen.findByRole('button', { name: 'Mark Sold Out' });
  fireEvent.click(soldOut);
  expect(soldOut).toHaveTextContent('Sold Out');
  expect(sync.enqueue).toHaveBeenCalledWith(expect.objectContaining({ type: 'TOGGLE_SOLD_OUT', payload: { item_id: 'menu-1', is_sold_out: true, expected_is_sold_out: false, expected_updated_at: '2026-10-01T00:00:00Z' } }), owner);
  fireEvent.click(screen.getByRole('button', { name: 'Mark Ready & Notify' }));
  await waitFor(() => expect(screen.getByText('No active orders')).toBeVisible());
  expect(sync.enqueue).toHaveBeenCalledWith(expect.objectContaining({ type: 'UPDATE_ORDER_STATUS', payload: { order_id: 'order-1', status: 'ready', expected_status: 'pending', expected_updated_at: '2026-10-01T00:00:00Z' } }), owner);
});

it('restores an optimistic order change when durable enqueue fails', async () => {
  sync.enqueue.mockRejectedValueOnce(new Error('Storage unavailable'));
  vi.stubGlobal('fetch', vi.fn(async (url: string) => Response.json(url.endsWith('/orders') ? { orders: [{ id: 'order-1', customer_name: 'Alice', status: 'pending' }] } : { inventory: [] })));
  render(<KitchenView />);
  fireEvent.click(await screen.findByRole('button', { name: 'Mark Ready & Notify' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('could not be saved');
  expect(screen.getByRole('button', { name: 'Mark Ready & Notify' })).toBeVisible();
});
