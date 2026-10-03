import { act, cleanup, render, screen } from '@testing-library/react';
import { afterEach, expect, test, vi } from 'vitest';
import LedgerPage from './page';

const entry = (extra = {}) => ({
  id: 'entry-a', transaction_id: 'transaction-a', account_id: 'cash-eur',
  amount: 12.34, currency: 'EUR', direction: 'credit', entry_type: 'credit',
  created_at: '2026-10-01T00:00:00Z', ...extra,
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });
const reply = (data: unknown, status = 200) => vi.stubGlobal('fetch', vi.fn(async () => Response.json(data, { status })));

test('renders actual mounted entry_type response in its recorded currency units', async () => {
  reply({ entries: [entry()] });
  render(<LedgerPage />);
  expect(await screen.findByText('EUR 12.34')).toBeVisible();
  expect(screen.getByRole('heading', { name: 'Ledger Statement' })).toBeVisible();
  expect(screen.queryByText('No recent activity.')).toBeNull();
});

test('keeps distinct currencies and zero-decimal units separate without a total', async () => {
  reply({ entries: [entry({ id: 'usd', currency: 'USD', amount: 5.25 }), entry({ id: 'jpy', currency: 'JPY', amount: 500 })] });
  render(<LedgerPage />);
  expect(await screen.findByText('USD 5.25')).toBeVisible();
  expect(screen.getByText('JPY 500')).toBeVisible();
  expect(screen.queryByText(/Total Balance|1,500/)).toBeNull();
});

test('only a successful empty entries array means no recent activity', async () => {
  reply({ entries: [] });
  render(<LedgerPage />);
  expect(await screen.findByText('No recent activity.')).toBeVisible();
});

test('retains sub-cent recorded units and accepts currency code casing', async () => {
  reply({ entries: [entry({ id: 'token', currency: 'usdc', amount: 0.000001 }), entry({ id: 'eur', currency: 'eur' })] });
  render(<LedgerPage />);
  expect(await screen.findByText('USDC 0.000001')).toBeVisible();
  expect(screen.getByText('EUR 12.34')).toBeVisible();
});

test.each([
  {}, { entries: null }, { entries: [entry({ entry_type: undefined })] },
  { entries: [entry({ currency: undefined })] }, { entries: [entry({ amount: '12.34' })] },
])('malformed ledger data cannot become an empty or invented statement: %j', async data => {
  reply(data);
  render(<LedgerPage />);
  expect(await screen.findByText('Ledger unavailable')).toBeVisible();
  expect(screen.queryByText('No recent activity.')).toBeNull();
  expect(screen.queryByRole('table')).toBeNull();
});

test.each([401, 503])('failed authoritative read stays unavailable for HTTP %s', async status => {
  reply({ error: 'unavailable' }, status);
  render(<LedgerPage />);
  expect(await screen.findByText('Ledger unavailable')).toBeVisible();
  expect(screen.queryByText('No recent activity.')).toBeNull();
});

test('departure aborts the actual pending read and ignores its later completion', async () => {
  let finish!: (response: Response) => void;
  const pending = new Promise<Response>(resolve => { finish = resolve; });
  const fetcher = vi.fn<typeof fetch>(() => pending);
  vi.stubGlobal('fetch', fetcher);
  const mounted = render(<LedgerPage />);
  expect(screen.getByText('Loading ledger entries...')).toBeVisible();
  mounted.unmount();
  expect(fetcher.mock.calls[0][1]?.signal?.aborted).toBe(true);
  await act(async () => { finish(Response.json({ entries: [entry()] })); });
  expect(screen.queryByRole('table')).toBeNull();
});
