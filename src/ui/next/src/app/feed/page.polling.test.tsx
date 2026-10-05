import React from 'react';
import { act, cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { QUEUE_IDENTITY_EPOCH_KEY } from '@/lib/sync/queueIdentity';
import FeedPage from './page';

vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <main>{children}</main>,
}));

const item = {
  id: 'pending-1', tenant_id: 'tenant-1', event_source: 'booking_request',
  context_payload: { description: 'Owner proposal' },
  proposed_action: { action_type: 'approve_booking' },
  lifecycle_state: 'PENDING_APPROVAL', created_at: '2026-10-05T00:00:00Z',
};
const feed = () => Response.json({ items: [item] });
function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>(done => { resolve = done; });
  return { promise, resolve };
}

beforeEach(() => { vi.useFakeTimers(); });
afterEach(() => { cleanup(); vi.useRealTimers(); vi.unstubAllGlobals(); });

describe('feed read lifecycle', () => {
  it('does not overlap the initial load with interval reads', async () => {
    const pending = deferred<Response>();
    const fetchMock = vi.fn().mockReturnValue(pending.promise);
    vi.stubGlobal('fetch', fetchMock);
    await act(async () => { render(<FeedPage />); });
    await act(async () => { vi.advanceTimersByTime(15_000); });
    expect(fetchMock).toHaveBeenCalledTimes(1);
    await act(async () => { pending.resolve(feed()); });
    expect(screen.getByText('Owner proposal')).toBeInTheDocument();
  });

  it.each(['Approve', 'Dismiss'])('never restores an item from a read started before %s', async action => {
    const pending = deferred<Response>();
    let reads = 0;
    vi.stubGlobal('fetch', vi.fn((url: string) => {
      if (url.endsWith('/state')) return Promise.resolve(Response.json({ item: { ...item, lifecycle_state: action === 'Approve' ? 'APPROVED' : 'DISMISSED' } }));
      return ++reads === 1 ? Promise.resolve(feed()) : pending.promise;
    }));
    await act(async () => { render(<FeedPage />); });
    await act(async () => { vi.advanceTimersByTime(5_000); });
    await act(async () => { fireEvent.click(screen.getByRole('button', { name: action })); });
    expect(screen.queryByText('Owner proposal')).not.toBeInTheDocument();
    await act(async () => { pending.resolve(feed()); });
    expect(screen.queryByText('Owner proposal')).not.toBeInTheDocument();
  });

  it.each(['logout', 'tenant change'])('aborts and retires pending data after %s', async reason => {
    const pending = deferred<Response>();
    const fetchMock = vi.fn().mockReturnValue(pending.promise);
    vi.stubGlobal('fetch', fetchMock);
    await act(async () => { render(<FeedPage />); });
    const signal = fetchMock.mock.calls[0][1]?.signal as AbortSignal | undefined;
    await act(async () => {
      if (reason === 'logout') window.dispatchEvent(new Event('omnisolo_auth_changed'));
      else window.dispatchEvent(new StorageEvent('storage', { key: QUEUE_IDENTITY_EPOCH_KEY }));
    });
    expect(signal?.aborted).toBe(true);
    await act(async () => { pending.resolve(feed()); vi.advanceTimersByTime(10_000); });
    expect(screen.queryByText('Owner proposal')).not.toBeInTheDocument();
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  it('reloads a restored back-forward cache document through session verification', async () => {
    const reload = vi.fn();
    vi.stubGlobal('location', { reload });
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(feed()));
    await act(async () => { render(<FeedPage />); });
    expect(screen.getByText('Owner proposal')).toBeInTheDocument();
    await act(async () => { window.dispatchEvent(new PageTransitionEvent('pageshow', { persisted: false })); });
    expect(reload).not.toHaveBeenCalled();
    await act(async () => { window.dispatchEvent(new PageTransitionEvent('pagehide', { persisted: true })); });
    expect(screen.queryByText('Owner proposal')).not.toBeInTheDocument();
    await act(async () => { window.dispatchEvent(new PageTransitionEvent('pageshow', { persisted: true })); });
    expect(reload).toHaveBeenCalledTimes(1);
    expect(screen.queryByText('Owner proposal')).not.toBeInTheDocument();
  });

  it('aborts an initial read when the page unmounts', async () => {
    const pending = deferred<Response>();
    const fetchMock = vi.fn().mockReturnValue(pending.promise);
    vi.stubGlobal('fetch', fetchMock);
    const view = render(<FeedPage />);
    const signal = fetchMock.mock.calls[0][1]?.signal as AbortSignal | undefined;
    view.unmount();
    expect(signal?.aborted).toBe(true);
    await act(async () => { pending.resolve(feed()); });
  });

  it('retires loaded private data and stops polling on an authentication rejection', async () => {
    const fetchMock = vi.fn().mockResolvedValueOnce(feed()).mockResolvedValue(new Response('', { status: 401 }));
    vi.stubGlobal('fetch', fetchMock);
    await act(async () => { render(<FeedPage />); });
    expect(screen.getByText('Owner proposal')).toBeInTheDocument();
    await act(async () => { vi.advanceTimersByTime(5_000); });
    expect(screen.queryByText('Owner proposal')).not.toBeInTheDocument();
    await act(async () => { vi.advanceTimersByTime(15_000); });
    expect(fetchMock).toHaveBeenCalledTimes(2);
  });

  it('reports a malformed feed instead of an invented empty success', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(Response.json({ success: false })));
    await act(async () => { render(<FeedPage />); });
    expect(screen.getByText("We couldn't load your feed.")).toBeInTheDocument();
    expect(screen.queryByTestId('agent-feed-empty')).not.toBeInTheDocument();
  });

  it('aborts a stalled read at the deadline and ignores a late response', async () => {
    const pending = deferred<Response>();
    const fetchMock = vi.fn().mockReturnValue(pending.promise);
    vi.stubGlobal('fetch', fetchMock);
    await act(async () => { render(<FeedPage />); });
    const signal = fetchMock.mock.calls[0][1]?.signal as AbortSignal | undefined;
    await act(async () => { vi.advanceTimersByTime(30_000); });
    expect(signal?.aborted).toBe(true);
    await act(async () => { pending.resolve(feed()); });
    expect(screen.queryByText('Owner proposal')).not.toBeInTheDocument();
  });

  it('clears an earlier read error when a later poll succeeds', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValueOnce(new Response('', { status: 500 })).mockResolvedValueOnce(feed()));
    await act(async () => { render(<FeedPage />); });
    expect(screen.getByText("We couldn't load your feed.")).toBeInTheDocument();
    await act(async () => { vi.advanceTimersByTime(5_000); });
    expect(screen.getByText('Owner proposal')).toBeInTheDocument();
    expect(screen.queryByText("We couldn't load your feed.")).not.toBeInTheDocument();
  });
});
