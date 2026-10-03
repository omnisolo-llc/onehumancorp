import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import KDSPage from './page';
const enqueue = vi.hoisted(() => vi.fn());
vi.mock('../../../lib/sync/SyncManager', () => ({ SyncManager: { getInstance: () => ({ enqueue }) } }));
beforeEach(() => {
  enqueue.mockReset(); localStorage.clear();
  vi.stubGlobal('fetch', vi.fn(async (url: string) => Response.json(url.endsWith('/orders') ? { orders: [{ id: '1', customer_name: 'Alice', status: 'Received', items: [] }] } : { inventory: [{ id: 'p', name: 'Wrap', is_sold_out: false }] })));
});
afterEach(() => vi.unstubAllGlobals());
it('captures actual pre-edit order and inventory state in the durable intent', async () => {
  render(<KDSPage />);
  fireEvent.click(await screen.findByTestId('btn-prepare-1'));
  expect(enqueue).toHaveBeenCalledWith(expect.objectContaining({ payload: { order_id: '1', status: 'Preparing', expected_status: 'Received' } }));
  fireEvent.click(await screen.findByTestId('toggle-soldout-p'));
  expect(enqueue).toHaveBeenCalledWith(expect.objectContaining({ payload: { item_id: 'p', is_sold_out: true, expected_is_sold_out: false } }));
});
it('restores the order and cache when queue persistence fails', async () => {
  enqueue.mockRejectedValueOnce(new Error('Storage unavailable'));
  render(<KDSPage />);
  fireEvent.click(await screen.findByTestId('btn-prepare-1'));
  expect(await screen.findByRole('alert')).toHaveTextContent('could not be saved');
  expect(screen.getByTestId('btn-prepare-1')).toBeVisible();
  await waitFor(() => expect(JSON.parse(localStorage.getItem('omnisolo_pos_kds_orders')!)[0].status).toBe('Received'));
});
