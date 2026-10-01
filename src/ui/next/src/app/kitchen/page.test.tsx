import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import type { ReactNode } from 'react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import KitchenView from './page';

vi.mock('../components/AppShell', () => ({ AppShell: ({ children }: { children: ReactNode }) => <main>{children}</main> }));
const sync = vi.hoisted(() => ({ enqueue: vi.fn(), getQueueLength: vi.fn(async () => 0) }));
vi.mock('../../lib/sync/SyncManager', () => ({ SyncManager: { getInstance: () => sync } }));
beforeEach(() => { localStorage.clear(); vi.clearAllMocks(); });
afterEach(() => vi.unstubAllGlobals());

it.each([false, true])('keeps an authoritative empty kitchen empty without invented or stale records (cache: %s)', async (withCache) => {
  if (withCache) localStorage.setItem('kds_orders_cache', JSON.stringify([{ id: 'old', customer_name: 'Stale customer', status: 'pending' }]));
  if (withCache) localStorage.setItem('kds_menu_cache', JSON.stringify([{ id: 'old-menu', title: 'Stale product' }]));
  vi.stubGlobal('fetch', vi.fn(async () => new Response('[]', { status: 200 })));
  render(<KitchenView />);
  await waitFor(() => expect(localStorage.getItem('kds_menu_cache')).toBe('[]'));
  expect(screen.getByText('No active orders')).toBeVisible();
  expect(screen.queryByRole('button', { name: 'Mark Ready & Notify' })).toBeNull();
  expect(screen.queryByRole('button', { name: 'Mark Sold Out' })).toBeNull();
  expect(screen.queryByText(/Alice|Bob|Stale customer|Stale product/)).toBeNull();
});

it('renders recorded translations and queues the real order and menu identities', async () => {
  vi.stubGlobal('fetch', vi.fn(async (url: string) => new Response(JSON.stringify(
    url.endsWith('/orders') ? { orders: [{ id: 'order-1', customer_name: 'Alice', status: 'pending', notes: 'No onions', translated_notes: 'بدون بصل' }] }
      : { inventory: [{ id: 'menu-1', name: 'Falafel Wrap', stock: 10, is_sold_out: false }] },
  ), { status: 200 })));
  render(<KitchenView />);
  expect(await screen.findByText('بدون بصل')).toBeVisible();
  const soldOut = await screen.findByRole('button', { name: 'Mark Sold Out' });
  fireEvent.click(soldOut);
  expect(soldOut).toHaveTextContent('Sold Out');
  expect(sync.enqueue).toHaveBeenCalledWith(expect.objectContaining({ type: 'TOGGLE_SOLD_OUT', payload: { item_id: 'menu-1', is_sold_out: true, expected_is_sold_out: false } }));
  fireEvent.click(screen.getByRole('button', { name: 'Mark Ready & Notify' }));
  await waitFor(() => expect(screen.getByText('No active orders')).toBeVisible());
  expect(sync.enqueue).toHaveBeenCalledWith(expect.objectContaining({ type: 'UPDATE_ORDER_STATUS', payload: { order_id: 'order-1', status: 'ready', expected_status: 'pending' } }));
});

it('restores an optimistic order change when durable enqueue fails', async () => {
  sync.enqueue.mockRejectedValueOnce(new Error('Storage unavailable'));
  vi.stubGlobal('fetch', vi.fn(async (url: string) => Response.json(url.endsWith('/orders') ? { orders: [{ id: 'order-1', customer_name: 'Alice', status: 'pending' }] } : { inventory: [] })));
  render(<KitchenView />);
  fireEvent.click(await screen.findByRole('button', { name: 'Mark Ready & Notify' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('could not be saved');
  expect(screen.getByRole('button', { name: 'Mark Ready & Notify' })).toBeVisible();
});
