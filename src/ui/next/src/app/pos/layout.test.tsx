import { act, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import POSLayout from './layout';

beforeEach(() => {
  // Match browser Storage semantics, including a stored empty string. The
  // shared test setup's getItem returns null for empty strings.
  const values = new Map<string, string>();
  vi.stubGlobal('localStorage', {
    get length() { return values.size; },
    key: (index: number) => [...values.keys()][index] ?? null,
    getItem: (key: string) => values.get(key) ?? null,
    setItem: (key: string, value: string) => { values.set(key, String(value)); },
    removeItem: (key: string) => { values.delete(key); },
    clear: () => values.clear(),
  });
  vi.stubGlobal('fetch', vi.fn());
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });

it.each([null, '[]', ' [ \n ] '])('keeps the maintained terminal visible without a historical warning for %j', raw => {
  if (raw !== null) localStorage.setItem('pos_offline_queue', raw);
  render(<POSLayout><h1>Open POS terminal</h1></POSLayout>);
  expect(screen.getByRole('heading', { name: 'Open POS terminal' })).toBeVisible();
  expect(screen.queryByRole('status')).toBeNull();
  expect(localStorage.getItem('pos_offline_queue')).toBe(raw);
  expect(fetch).not.toHaveBeenCalled();
});

it.each([
  '[ { "offline_id": "unknown-outcome", "total": 70, "customer": "another-owner@example.test" } ]',
  '[{"total":100},null,{"items":[]}]',
  '{"unowned":"record"}',
  '[truncated',
  'null',
  '',
])('preserves historical data byte-for-byte across reconnect and remount: %j', raw => {
  localStorage.setItem('pos_offline_queue', raw);
  const view = render(<POSLayout><h1>Open POS terminal</h1></POSLayout>);
  expect(screen.getByRole('status')).toHaveTextContent('Historical POS data needs review');
  expect(screen.getByRole('status')).toHaveTextContent('Payment status and account ownership are unverified');
  expect(screen.getByRole('status')).toHaveTextContent('Nothing in this legacy queue will be sent or removed');
  expect(screen.getByRole('status')).toHaveTextContent('account owner compare provider transactions and receipts');
  expect(screen.queryByText(/another-owner@example.test|unknown-outcome/)).toBeNull();
  act(() => {
    window.dispatchEvent(new Event('offline'));
    window.dispatchEvent(new Event('online'));
    window.dispatchEvent(new StorageEvent('storage', { key: 'pos_offline_queue' }));
  });
  view.unmount();
  render(<POSLayout><h1>Open POS terminal</h1></POSLayout>);
  expect(screen.getByRole('status')).toHaveTextContent('Historical POS data needs review');
  expect(localStorage.getItem('pos_offline_queue')).toBe(raw);
  expect(fetch).not.toHaveBeenCalled();
});

it('reports unavailable storage without claiming there are no old payments or blocking the canonical screen', () => {
  vi.spyOn(localStorage, 'getItem').mockImplementation(() => { throw new DOMException('Storage blocked', 'SecurityError'); });
  render(<POSLayout><h1>Open POS terminal</h1></POSLayout>);
  expect(screen.getByRole('status')).toHaveTextContent('Historical POS data could not be checked');
  expect(screen.getByRole('status')).toHaveTextContent('Keep this browser’s stored data');
  expect(screen.getByRole('heading', { name: 'Open POS terminal' })).toBeVisible();
  expect(fetch).not.toHaveBeenCalled();
});

it('rechecks historical data when another tab saves a record, without claiming or replaying it', () => {
  render(<POSLayout><h1>Open POS terminal</h1></POSLayout>);
  expect(screen.queryByRole('status')).toBeNull();
  const raw = '[{"offline_id":"saved-by-old-tab"}]';
  localStorage.setItem('pos_offline_queue', raw);
  act(() => window.dispatchEvent(new StorageEvent('storage', { key: 'pos_offline_queue' })));
  expect(screen.getByRole('status')).toHaveTextContent('Historical POS data needs review');
  expect(localStorage.getItem('pos_offline_queue')).toBe(raw);
  expect(fetch).not.toHaveBeenCalled();
});
