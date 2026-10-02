import { act, cleanup, fireEvent, render, screen } from '@testing-library/react';
import { renderToString } from 'react-dom/server';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { Omnibox } from './Omnibox';
import { QUEUE_IDENTITY_EPOCH_KEY } from '@/lib/sync/queueIdentity';

const push = vi.hoisted(() => vi.fn());
vi.mock('next/navigation', () => ({ useRouter: () => ({ push }) }));
beforeEach(() => { vi.useFakeTimers(); push.mockReset(); });
afterEach(() => { cleanup(); vi.useRealTimers(); vi.unstubAllGlobals(); });
function open() { render(<Omnibox />); fireEvent.keyDown(window, { key: 'k', ctrlKey: true }); }
async function query(value: string) {
  fireEvent.change(screen.getByPlaceholderText(/Search customers/), { target: { value } });
  await act(async () => { await vi.advanceTimersByTimeAsync(300); });
}
const result = (title: string) => ({ id: title, entity_type: 'customer', title, subtitle: '', route: '/customers' });

it('only signals keyboard readiness after the listener is mounted', () => {
  expect(renderToString(<Omnibox />)).toContain('data-ready="false"');
  render(<Omnibox />);
  expect(screen.getByTestId('omnibox-readiness')).toHaveAttribute('data-ready', 'true');
  fireEvent.keyDown(window, { key: 'K', ctrlKey: true });
  expect(screen.getByPlaceholderText(/Search customers/)).toHaveFocus();
  fireEvent.keyDown(window, { key: 'Escape' });
  expect(screen.queryByPlaceholderText(/Search customers/)).toBeNull();
});
it('does not report empty results before the request finishes', async () => {
  vi.stubGlobal('fetch', vi.fn(() => new Promise<Response>(() => {})));
  open(); fireEvent.change(screen.getByPlaceholderText(/Search customers/), { target: { value: 'John' } });
  expect(screen.queryByText(/No results found/)).toBeNull();
  expect(screen.getByRole('status')).toHaveTextContent('Searching');
  await act(async () => { await vi.advanceTimersByTimeAsync(300); });
  expect(screen.queryByText(/No results found/)).toBeNull();
});
it('waits for the mounted search contract minimum before dispatch', async () => {
  vi.stubGlobal('fetch', vi.fn()); open(); await query('J');
  expect(fetch).not.toHaveBeenCalled();
  expect(screen.getByText('Enter at least 2 characters to search.')).toBeVisible();
});
it.each([Response.json({ error: 'unavailable' }, { status: 503 }), Response.json({ success: false, results: [] }), ...['//outside.example', '/\n/outside.example', '/\\outside.example'].map(route => Response.json({ success: true, results: [result('bad'), { ...result('unsafe'), route }] }))])('shows a failed or invalid search as unavailable %#', async response => {
  vi.stubGlobal('fetch', vi.fn(async () => response)); open(); await query('John');
  expect(screen.getByRole('alert')).toHaveTextContent('Search is unavailable');
  expect(screen.queryByText(/No results found/)).toBeNull();
});
it('ignores an older query body arriving after the latest results', async () => {
  let finish!: (value: unknown) => void;
  vi.stubGlobal('fetch', vi.fn(async (url: string) => url.endsWith('old')
    ? { ok: true, json: () => new Promise(resolve => { finish = resolve; }) }
    : Response.json({ success: true, results: [result('New result')] })));
  open(); await query('old'); await query('new');
  expect(screen.getByText('New result')).toBeVisible();
  await act(async () => finish({ success: true, results: [result('Old result')] }));
  expect(screen.getByText('New result')).toBeVisible();
  expect(screen.queryByText('Old result')).toBeNull();
});
it('aborts a pending query on close and does not restore its private results on reopen', async () => {
  let finish!: (value: unknown) => void; let signal: AbortSignal | undefined;
  vi.stubGlobal('fetch', vi.fn(async (_url: string, options: RequestInit) => {
    signal = options.signal as AbortSignal;
    return { ok: true, json: () => new Promise(resolve => { finish = resolve; }) };
  }));
  open(); await query('private'); fireEvent.keyDown(window, { key: 'Escape' });
  expect(signal?.aborted).toBe(true);
  await act(async () => finish({ success: true, results: [result('Old private result')] }));
  fireEvent.keyDown(window, { key: 'k', ctrlKey: true });
  expect(screen.queryByText('Old private result')).toBeNull();
});
it.each(['same-tab', 'other-tab', 'cleared-storage'])('clears private results and ignores late responses after %s identity invalidation', async mode => {
  let finish!: (value: unknown) => void;
  vi.stubGlobal('fetch', vi.fn(async (url: string) => url.endsWith('pending')
    ? { ok: true, json: () => new Promise(resolve => { finish = resolve; }) }
    : Response.json({ success: true, results: [result('Private result')] })));
  open(); await query('saved'); expect(screen.getByText('Private result')).toBeVisible();
  await query('pending');
  act(() => window.dispatchEvent(mode === 'same-tab' ? new Event('omnisolo_auth_changed')
    : new StorageEvent('storage', { key: mode === 'other-tab' ? QUEUE_IDENTITY_EPOCH_KEY : null })));
  expect(screen.queryByPlaceholderText(/Search customers/)).toBeNull();
  await act(async () => finish({ success: true, results: [result('Late private result')] }));
  fireEvent.keyDown(window, { key: 'k', ctrlKey: true });
  expect(screen.getByPlaceholderText(/Search customers/)).toHaveValue('');
  expect(screen.queryByText('Late private result')).toBeNull();
});
it('shows acknowledged empty results and navigates through a returned local destination', async () => {
  vi.stubGlobal('fetch', vi.fn(async (url: string) => Response.json({ success: true, results: url.endsWith('none') ? [] : [result('John')] })));
  open(); await query('none'); expect(screen.getByText('No results found for "none".')).toBeVisible();
  await query('John'); fireEvent.click(screen.getByText('John'));
  expect(push).toHaveBeenCalledWith('/customers');
  expect(screen.queryByPlaceholderText(/Search customers/)).toBeNull();
});
