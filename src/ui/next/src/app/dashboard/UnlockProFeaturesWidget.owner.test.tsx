import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { UnlockProFeaturesWidget } from './UnlockProFeaturesWidget';

const owner = { userId: 'owner-a', tenantId: 'tenant-a' };
const link = 'https://omnisolo.co/invite/actual-unlock-record';
const metricsPath = '/api/v1/growth/team-invites/aggregated-metrics';
const deferred = <T,>() => { let resolve!: (value: T) => void; let reject!: (error: unknown) => void; const promise = new Promise<T>((yes, no) => { resolve = yes; reject = no; }); return { promise, resolve, reject }; };
let metrics: () => Promise<Response>;
let identity: () => Promise<Response>;
let invitation: () => Promise<Response>;
let clipboard: ReturnType<typeof vi.fn>;
const posts = () => vi.mocked(fetch).mock.calls.filter(([, init]) => init?.method === 'POST');
beforeEach(() => {
  localStorage.clear(); localStorage.setItem('business_display_name', 'foreign-local-display');
  metrics = async () => Response.json({ total_invites: 1 });
  identity = async () => Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
  invitation = async () => Response.json({ invite_link: link });
  vi.stubGlobal('fetch', vi.fn(async url => url === '/api/v1/auth/session-identity' ? identity() : url === metricsPath ? metrics() : url === '/api/v1/growth/cloud-bridge/invite' ? invitation() : Response.json({}, { status: 404 })));
  const locks = new Set<string>();
  Object.defineProperty(navigator, 'locks', { value: { request: async (name: string, _options: unknown, callback: (lock: object | null) => Promise<void>) => { if (locks.has(name)) return callback(null); locks.add(name); try { return await callback({}); } finally { locks.delete(name); } } } });
  clipboard = vi.fn().mockResolvedValue(undefined);
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: clipboard } });
});
afterEach(() => { vi.useRealTimers(); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
async function create() {
  const button = screen.getByRole('button', { name: 'Create Invite Link' });
  await waitFor(() => expect(button).toBeEnabled());
  await act(async () => { fireEvent.click(button); });
}
it('does not expose zero-progress sharing while the actual metric body is pending', async () => {
  const pending = deferred<Response>(); metrics = () => pending.promise;
  render(<UnlockProFeaturesWidget />);
  expect(screen.queryByText('0 / 3 Invites')).not.toBeInTheDocument();
  expect(screen.queryByRole('button', { name: /Share on X/ })).not.toBeInTheDocument();
  expect(screen.queryByRole('link', { name: 'Share on X' })).not.toBeInTheDocument();
  await waitFor(() => expect(fetch).toHaveBeenCalledWith(metricsPath, expect.anything()));
  expect(screen.getByTestId('unlock-pro-features-widget')).toHaveAttribute('aria-busy', 'true');
  await act(async () => pending.resolve(Response.json({ total_invites: 3 })));
  expect(await screen.findByText('Invite target reached')).toBeVisible();
  expect(screen.queryByRole('button', { name: 'Create Invite Link' })).not.toBeInTheDocument();
  expect(posts()).toHaveLength(0);
});
it('binds the metric read to the verified owner and never uses a display-name URL', async () => {
  render(<UnlockProFeaturesWidget />); await screen.findByText('1 / 3 Invites');
  const options = vi.mocked(fetch).mock.calls.find(([url]) => url === metricsPath)![1]!;
  const headers = new Headers(options.headers);
  expect(headers.get('x-ohc-expected-user')).toBe(owner.userId); expect(headers.get('x-ohc-expected-tenant')).toBe(owner.tenantId);
  expect(options).toMatchObject({ credentials: 'same-origin', cache: 'no-store', redirect: 'error' });
  expect(screen.queryByRole('link', { name: 'Share on X' })).not.toBeInTheDocument();
  await create();
  const share = screen.getByRole('link', { name: 'Share on X' });
  expect(new URL(share.getAttribute('href')!).searchParams.get('text')).toContain(link);
  expect(share.getAttribute('href')).not.toContain('foreign-local-display');
  expect(share).toHaveAttribute('rel', 'noopener noreferrer'); expect(posts()).toHaveLength(1);
});
it.each(['auth', 'storage', 'null-storage', 'pagehide'])('%s retires counts and a delayed metric body', async event => {
  const body = deferred<unknown>(); metrics = async () => ({ status: 200, json: () => body.promise }) as Response;
  render(<UnlockProFeaturesWidget />); await waitFor(() => expect(fetch).toHaveBeenCalledWith(metricsPath, expect.anything()));
  act(() => window.dispatchEvent(event === 'auth' ? new Event('omnisolo_auth_changed') : event === 'pagehide' ? new Event('pagehide') : new StorageEvent('storage', { key: event === 'storage' ? 'omnisolo_queue_identity_epoch_v2' : null })));
  await act(async () => body.resolve({ total_invites: 2 }));
  expect(screen.queryByText('2 / 3 Invites')).not.toBeInTheDocument(); expect(screen.queryByText('0 / 3 Invites')).not.toBeInTheDocument();
  expect(screen.getByRole('button', { name: 'Create Invite Link' })).toBeDisabled(); expect(posts()).toHaveLength(0);
});
it.each([{}, { total_invites: -1 }, { total_invites: 1.5 }, { total_invites: '3' }, { total_invites: 3, success: false }])('invalid metrics remain unavailable: %j', async data => {
  metrics = async () => Response.json(data); render(<UnlockProFeaturesWidget />);
  await waitFor(() => expect(screen.getByRole('status', { name: 'Referral progress status' })).toHaveTextContent('unavailable'));
  expect(screen.queryByText('Invite target reached')).not.toBeInTheDocument(); expect(screen.queryByText('0 / 3 Invites')).not.toBeInTheDocument(); expect(posts()).toHaveLength(0);
});
it('an auth denial retires immediately without waiting for the rejected body', async () => {
  const read = vi.fn(() => new Promise(() => {})); metrics = async () => ({ status: 401, json: read }) as unknown as Response;
  render(<UnlockProFeaturesWidget />); await waitFor(() => expect(screen.getByRole('status', { name: 'Referral progress status' })).toHaveTextContent('session changed'));
  expect(read).not.toHaveBeenCalled(); expect(posts()).toHaveLength(0);
});
it('failed metrics can be explicitly read again without creating an invitation', async () => {
  metrics = async () => Response.json({}, { status: 500 }); render(<UnlockProFeaturesWidget />);
  const retry = await screen.findByRole('button', { name: 'Retry Referral Progress' });
  metrics = async () => Response.json({ total_invites: 2 }); fireEvent.click(retry);
  expect(await screen.findByText('2 / 3 Invites')).toBeVisible(); expect(posts()).toHaveLength(0);
});
it('requires a genuine invitation receipt and platform copy acknowledgement', async () => {
  render(<UnlockProFeaturesWidget />); await create();
  const pending = deferred<void>(); clipboard.mockReturnValueOnce(pending.promise);
  fireEvent.click(screen.getByRole('button', { name: 'Copy Invite Link' }));
  expect(clipboard).toHaveBeenCalledWith(link); expect(screen.queryByText('Copied Link!')).not.toBeInTheDocument();
  await act(async () => pending.reject(new DOMException('Denied', 'NotAllowedError')));
  expect(screen.getByRole('alert')).toHaveTextContent('Copy failed'); expect(screen.queryByText('Copied Link!')).not.toBeInTheDocument();
  fireEvent.click(screen.getByRole('button', { name: 'Copy Invite Link' })); expect(await screen.findByText('Copied Link!')).toBeVisible();
});
it('owner retirement clears a real link and late clipboard result', async () => {
  render(<UnlockProFeaturesWidget />); await create();
  const pending = deferred<void>(); clipboard.mockReturnValueOnce(pending.promise); fireEvent.click(screen.getByRole('button', { name: 'Copy Invite Link' }));
  act(() => window.dispatchEvent(new Event('omnisolo_auth_changed'))); await act(async () => pending.resolve());
  expect(screen.queryByRole('link', { name: 'Share on X' })).not.toBeInTheDocument(); expect(screen.queryByText('Copied Link!')).not.toBeInTheDocument(); expect(screen.queryByText('1 / 3 Invites')).not.toBeInTheDocument();
});
it('unknown invitation results never produce a share link or repeat POST', async () => {
  invitation = async () => Response.json({ error: 'unavailable' }, { status: 503 }); render(<UnlockProFeaturesWidget />); await create();
  expect(screen.queryByRole('link', { name: 'Share on X' })).not.toBeInTheDocument(); fireEvent.click(screen.getByRole('button', { name: 'Create Invite Link' })); expect(posts()).toHaveLength(1);
});
it.each(['session_identity_changed', 'queued owner does not match the current session'])('retires the exact owner mismatch %s', async error => {
  metrics = async () => Response.json({ error }, { status: 409 }); render(<UnlockProFeaturesWidget />);
  await waitFor(() => expect(screen.getByRole('status', { name: 'Referral progress status' })).toHaveTextContent('session changed'));
  expect(screen.getByRole('button', { name: 'Create Invite Link' })).toBeDisabled();
  expect(screen.queryByRole('button', { name: 'Retry Referral Progress' })).not.toBeInTheDocument();
});
it('a business conflict holds metrics without falsely invalidating the owner', async () => {
  const changed = vi.fn(); window.addEventListener('omnisolo_auth_changed', changed);
  try {
    metrics = async () => Response.json({ error: 'metrics_unavailable' }, { status: 409 }); render(<UnlockProFeaturesWidget />);
    expect(await screen.findByRole('button', { name: 'Retry Referral Progress' })).toBeEnabled(); expect(changed).not.toHaveBeenCalled(); expect(posts()).toHaveLength(0);
  } finally { window.removeEventListener('omnisolo_auth_changed', changed); }
});
it('a replacement view is not overwritten by an unmounted metric body', async () => {
  const body = deferred<unknown>(); metrics = async () => ({ status: 200, json: () => body.promise }) as Response;
  const first = render(<UnlockProFeaturesWidget />); await waitFor(() => expect(fetch).toHaveBeenCalledWith(metricsPath, expect.anything())); first.unmount();
  metrics = async () => Response.json({ total_invites: 2 }); render(<UnlockProFeaturesWidget />); await screen.findByText('2 / 3 Invites');
  await act(async () => body.resolve({ total_invites: 3 }));
  expect(screen.getByText('2 / 3 Invites')).toBeVisible(); expect(screen.queryByText('Invite target reached')).not.toBeInTheDocument();
});
it('verified expiry clears private progress using the existing invitation lifetime', async () => {
  vi.useFakeTimers(); identity = async () => Response.json({ ...owner, expiresAt: Date.now() + 1_000 });
  await act(async () => { render(<UnlockProFeaturesWidget />); });
  expect(screen.getByText('1 / 3 Invites')).toBeVisible();
  await act(async () => { await vi.advanceTimersByTimeAsync(1_000); });
  expect(screen.queryByText('1 / 3 Invites')).not.toBeInTheDocument(); expect(screen.getByRole('status', { name: 'Referral progress status' })).toHaveTextContent('session changed');
});
it('missing verified identity settles unavailable without sending a metric or invitation request', async () => {
  identity = async () => Response.json({ error: 'denied' }, { status: 401 }); render(<UnlockProFeaturesWidget />);
  await waitFor(() => expect(screen.getByTestId('unlock-pro-features-widget')).toHaveAttribute('aria-busy', 'false'));
  expect(screen.getByRole('status', { name: 'Referral progress status' })).toHaveTextContent('unavailable');
  expect(vi.mocked(fetch).mock.calls.some(([url]) => url === metricsPath)).toBe(false); expect(posts()).toHaveLength(0);
});
it.each([1, 2])('refreshes recorded count after a confirmed invitation: %s plus one', async before => {
  let reads = 0; const refreshed = deferred<Response>();
  metrics = async () => ++reads === 1 ? Response.json({ total_invites: before }) : refreshed.promise;
  render(<UnlockProFeaturesWidget />); await screen.findByText(`${before} / 3 Invites`); await create();
  await waitFor(() => expect(reads).toBe(2));
  expect(screen.queryByRole('link', { name: 'Share on X' })).not.toBeInTheDocument();
  expect(screen.queryByText(`${before + 1} / 3 Invites`)).not.toBeInTheDocument();
  await act(async () => refreshed.resolve(Response.json({ total_invites: before + 1 })));
  expect(await screen.findByText(`${before + 1} / 3 Invites`)).toBeVisible();
  if (before === 2) { expect(screen.getByText('Invite target reached')).toBeVisible(); expect(screen.queryByRole('link', { name: 'Share on X' })).not.toBeInTheDocument(); }
  else expect(screen.getByRole('link', { name: 'Share on X' })).toBeVisible();
});
it('an unknown invitation cannot refresh metrics or claim progress', async () => {
  let reads = 0; metrics = async () => { reads++; return Response.json({ total_invites: reads }); };
  invitation = async () => { throw new Error('reply lost'); };
  render(<UnlockProFeaturesWidget />); await create();
  expect(reads).toBe(1); expect(screen.getByText('1 / 3 Invites')).toBeVisible(); expect(screen.queryByRole('link', { name: 'Share on X' })).not.toBeInTheDocument(); expect(posts()).toHaveLength(1);
});
