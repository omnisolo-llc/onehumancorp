import React from 'react';
import { act, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, it, expect, vi } from 'vitest';
import CustomerMemoryGraph from './page';
import { invalidateQueueOwner, QUEUE_IDENTITY_EPOCH_KEY, readQueueOwner } from '@/lib/sync/queueIdentity';

const navigation = vi.hoisted(() => ({ query: 'customerId=test-customer&tenantId=test-tenant' }));

const ownerA = { userId: 'memory-user-a', tenantId: 'memory-tenant-a' };
let currentOwner = ownerA;
function stubHistory(history: typeof fetch) {
  vi.stubGlobal('fetch', vi.fn<typeof fetch>((input, options) => String(input) === '/api/v1/auth/session-identity'
    ? Promise.resolve(Response.json({ ...currentOwner, expiresAt: Date.now() + 60_000 }))
    : history(input, options)));
}
beforeEach(() => {
  navigation.query = 'customerId=test-customer&tenantId=test-tenant';
  localStorage.clear();
  currentOwner = ownerA;
  act(() => invalidateQueueOwner());
  Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
});
afterEach(() => { act(() => invalidateQueueOwner()); vi.unstubAllGlobals(); });

// Mock the next/navigation hooks
vi.mock('next/navigation', () => ({
  useSearchParams: () => new URLSearchParams(navigation.query),
}));

// Mock the PoweredByOmniSolo component since we're focused on CustomerMemoryGraph
vi.mock('@/app/components/PoweredByOmniSolo', () => ({
  PoweredByOmniSolo: ({ tenantId }: { tenantId: string }) => <div data-testid="powered-by-omnisolo">{tenantId}</div>,
}));

describe('CustomerMemoryGraph Component', () => {
  it.each(['', 'customerId=', 'customerId=%20%20'])('asks for a customer without requesting invented history: %s', async (query) => {
    navigation.query = query;
    const fetchHistory = vi.fn<typeof fetch>(async () => Response.json({ summary: 'Customer not found.' }));
    stubHistory(fetchHistory);
    render(<CustomerMemoryGraph />);
    expect(await screen.findByRole('heading', { name: 'Choose a customer' })).toBeVisible();
    expect(screen.getByRole('link', { name: 'Open inbox' })).toHaveAttribute('href', '/inbox');
    expect(fetchHistory).not.toHaveBeenCalled();
    expect(screen.queryByText('Unknown Customer')).not.toBeInTheDocument();
  });

  it('retires a pending history request when the customer selection clears', async () => {
    let resolveHistory!: (response: Response) => void;
    const fetchHistory = vi.fn<typeof fetch>(() => new Promise<Response>(resolve => { resolveHistory = resolve; }));
    stubHistory(fetchHistory);
    const view = render(<CustomerMemoryGraph />);
    await waitFor(() => expect(fetchHistory).toHaveBeenCalledOnce());
    navigation.query = '';
    view.rerender(<CustomerMemoryGraph />);
    expect(screen.getByRole('heading', { name: 'Choose a customer' })).toBeVisible();
    expect(fetchHistory.mock.calls[0][1]?.signal?.aborted).toBe(true);
    resolveHistory(Response.json({ summary: 'Previous customer history' }));
    await waitFor(() => expect(screen.queryByText('Previous customer history')).not.toBeInTheDocument());
    expect(fetchHistory).toHaveBeenCalledOnce();
  });

  it('binds the read and branding to the verified owner instead of a query tenant', async () => {
    navigation.query = 'customerId=test-customer&tenantId=foreign-tenant';
    const history = vi.fn<typeof fetch>(async () => Response.json({ summary: 'Recorded customer context' }));
    stubHistory(history);
    render(<CustomerMemoryGraph />);
    expect(await screen.findByText('Recorded customer context')).toBeVisible();
    const [url, options] = history.mock.calls[0];
    expect(url).toBe('/api/v1/memory/summary/test-customer');
    const headers = new Headers(options?.headers);
    expect(headers.get('x-ohc-expected-user')).toBe(ownerA.userId);
    expect(headers.get('x-ohc-expected-tenant')).toBe(ownerA.tenantId);
    expect(options).toMatchObject({ credentials: 'same-origin', cache: 'no-store', redirect: 'error' });
    expect(screen.getByTestId('powered-by-omnisolo')).toHaveTextContent(ownerA.tenantId);
    expect(screen.queryByText('foreign-tenant')).not.toBeInTheDocument();
  });

  it.each(['omnisolo_auth_changed', 'pagehide', 'storage'])('retires loaded private history on %s', async (event) => {
    stubHistory(vi.fn<typeof fetch>(async () => Response.json({ summary: 'Private old customer' })));
    render(<CustomerMemoryGraph />);
    expect(await screen.findByText('Private old customer')).toBeVisible();
    act(() => window.dispatchEvent(event === 'storage'
      ? new StorageEvent('storage', { key: QUEUE_IDENTITY_EPOCH_KEY }) : new Event(event)));
    expect(screen.queryByText('Private old customer')).not.toBeInTheDocument();
  });

  it('rejects a late history body after a canonical owner replacement without an event', async () => {
    let resolveBody!: (value: unknown) => void;
    const history = vi.fn<typeof fetch>(async () => ({ ok: true, status: 200,
      json: () => new Promise(resolve => { resolveBody = resolve; }),
    }) as Response);
    stubHistory(history);
    render(<CustomerMemoryGraph />);
    await waitFor(() => expect(resolveBody).toBeTypeOf('function'));
    currentOwner = { userId: 'memory-user-b', tenantId: 'memory-tenant-b' };
    await act(async () => { await readQueueOwner(); });
    await act(async () => { resolveBody({ summary: 'Late private customer' }); });
    expect(screen.queryByText('Late private customer')).not.toBeInTheDocument();
    expect(history.mock.calls[0][1]?.signal?.aborted).toBe(true);
  });

  it('does not commit the previous customer under a new customer selection', async () => {
    const commits: string[] = [];
    function ObservedPage({ record }: { record: boolean }) {
      React.useLayoutEffect(() => { if (record) commits.push(document.body.textContent || ''); });
      return <CustomerMemoryGraph />;
    }
    stubHistory(vi.fn<typeof fetch>(async () => Response.json({ summary: 'Customer A private summary' })));
    const view = render(<ObservedPage record={false} />);
    expect(await screen.findByText('Customer A private summary')).toBeVisible();
    stubHistory(vi.fn<typeof fetch>(() => new Promise<Response>(() => {})));
    navigation.query = 'customerId=customer-b';
    view.rerender(<ObservedPage record />);
    expect(commits.length).toBeGreaterThan(0);
    expect(commits.every(text => !text.includes('Customer A private summary'))).toBe(true);
  });

  it.each([null, { events: {} }, { customer_name: 42 }, { events: [{ raw_content: {} }] }])(
    'rejects malformed history without inventing customer facts: %j', async (payload) => {
      stubHistory(vi.fn<typeof fetch>(async () => Response.json(payload)));
      render(<CustomerMemoryGraph />);
      expect(await screen.findByText('Failed to fetch customer history.')).toBeVisible();
      expect(screen.queryByText('High Intent')).not.toBeInTheDocument();
    },
  );

  it('renders loading state initially', () => {
    stubHistory(vi.fn<typeof fetch>(() => new Promise<Response>(() => {})));
    const view = render(<CustomerMemoryGraph />);
    expect(screen.getByText('Loading customer history...')).toBeInTheDocument();
    view.unmount();
  });

  it('does not render an accepted but nonterminal customer-history response', async () => {
    stubHistory(vi.fn<typeof fetch>(async () => Response.json({ summary: 'Unconfirmed customer history' }, { status: 202 })));
    render(<CustomerMemoryGraph />);
    expect(await screen.findByText('Failed to fetch customer history.')).toBeVisible();
    expect(screen.queryByText('Unconfirmed customer history')).not.toBeInTheDocument();
  });

  it('renders error state on fetch failure', async () => {
    stubHistory(vi.fn<typeof fetch>(async () => new Response(null, { status: 503 })));

    render(<CustomerMemoryGraph />);

    await waitFor(() => {
      expect(screen.getByText('Failed to fetch customer history.')).toBeInTheDocument();
    });
  });

  it('renders correctly with interaction events data', async () => {
    const mockData = {
      total_interactions: 2,
      segments: ['VIP', 'Frequent Buyer'],
      events: [
        {
          id: '1',
          channel: 'pos',
          raw_content: 'Bought in store: Summer Dress',
          created_at: new Date('2023-01-01T10:00:00Z').toISOString(),
        },
        {
          id: '2',
          channel: 'instagram',
          raw_content: 'Sent DM: Do you have vegan cakes?',
          created_at: new Date('2023-01-02T15:30:00Z').toISOString(),
        },
      ],
    };

    stubHistory(vi.fn<typeof fetch>(async () => Response.json(mockData)));

    render(<CustomerMemoryGraph />);

    await waitFor(() => {
      // Check for main headers
      expect(screen.getByText('Customer Context')).toBeInTheDocument();
      expect(screen.getByText('Timeline')).toBeInTheDocument();

      // Check for AI insights segments
      expect(screen.getByText('VIP')).toBeInTheDocument();
      expect(screen.getByText('Frequent Buyer')).toBeInTheDocument();
      expect(screen.getByText('2 total interactions recorded.')).toBeInTheDocument();

      // Check for specific events
      expect(screen.getByText('Bought in store: Summer Dress')).toBeInTheDocument();
      expect(screen.getByText('Sent DM: Do you have vegan cakes?')).toBeInTheDocument();

      // Check for channels
      expect(screen.getByText('pos')).toBeInTheDocument();
      expect(screen.getByText('instagram')).toBeInTheDocument();

      // Check for action buttons
      expect(screen.getByRole('button', { name: 'Draft Reply' })).toBeDisabled();
      expect(screen.getByRole('button', { name: 'Issue Refund' })).toBeDisabled();
      expect(screen.queryByText('High Intent')).not.toBeInTheDocument();
    });
  });

  it('renders empty state when no events exist', async () => {
    const mockData = {
      total_interactions: 0,
      segments: [],
      events: [],
    };

    stubHistory(vi.fn<typeof fetch>(async () => Response.json(mockData)));

    render(<CustomerMemoryGraph />);

    await waitFor(() => {
      expect(screen.getByText('No interaction history found.')).toBeInTheDocument();
    });
  });
});
