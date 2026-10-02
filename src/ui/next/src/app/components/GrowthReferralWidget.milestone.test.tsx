import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner, readQueueOwner } from '@/lib/sync/queueIdentity';
import GrowthReferralWidget from './GrowthReferralWidget';

const a = { userId: 'recorded-owner-a', tenantId: 'recorded-tenant-a' };
const b = { userId: 'recorded-owner-b', tenantId: 'recorded-tenant-b' };
const link = 'https://cloud.omnisolo.co/invite/actual-recorded-invite';
let owner = a;
let metricReply: () => Promise<Response>;
let inviteReply: () => Promise<Response>;
const metric = (count = 11) => ({ success: true, metric: 'recorded_orders', included_statuses: 'all_recorded_statuses', user_id: a.userId, tenant_id: a.tenantId,
  recorded_orders: count, reached_thresholds: [1,10,50,100,1000].filter(n => count >= n), highest_threshold: [...[1,10,50,100,1000].filter(n => count >= n)].pop() ?? null, observed_at: '2026-10-02T00:00:00Z' });
function deferred<T>() { let resolve!: (value: T) => void; const promise = new Promise<T>(yes => { resolve = yes; }); return { promise, resolve }; }
beforeEach(() => {
  localStorage.clear(); act(() => invalidateQueueOwner()); owner = a;
  Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
  localStorage.setItem('business_display_name', 'Unverified display name');
  const locks = new Set<string>();
  Object.defineProperty(navigator, 'locks', { value: { request: async (name: string, _options: unknown, callback: (lock: object | null) => Promise<void>) => {
    if (locks.has(name)) return callback(null); locks.add(name);
    try { return await callback({}); } finally { locks.delete(name); }
  } } });
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: vi.fn().mockResolvedValue(undefined) } });
  metricReply = async () => Response.json(metric());
  inviteReply = async () => Response.json({ invite_link: link });
  vi.stubGlobal('fetch', vi.fn(async url => {
    if (url === '/api/v1/auth/session-identity') return Response.json({ ...owner, expiresAt: Date.now() + 60000 });
    if (url === '/api/v1/growth/milestone') return metricReply();
    if (url === '/api/v1/growth/cloud-bridge/invite') return inviteReply();
    throw new Error(`Unexpected request ${String(url)}`);
  }));
});
afterEach(() => { act(() => invalidateQueueOwner()); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
async function ready() { await screen.findByText('11 recorded orders'); }
async function invite() {
  await waitFor(() => expect(screen.getByRole('button', { name: 'Unlock Cloud Collaboration' })).toBeEnabled());
  await act(async () => fireEvent.click(screen.getByRole('button', { name: 'Unlock Cloud Collaboration' })));
}
it('reads the actual recorded-order receipt for the verified raw owner without reward or delivery claims', async () => {
  render(<GrowthReferralWidget />); await ready();
  const [, init] = vi.mocked(fetch).mock.calls.find(([url]) => url === '/api/v1/growth/milestone')!;
  const headers = new Headers(init?.headers);
  expect(headers.get('x-ohc-expected-user')).toBe(a.userId); expect(headers.get('x-ohc-expected-tenant')).toBe(a.tenantId);
  expect(screen.getByText(/Counts all recorded statuses/)).toBeVisible();
  expect(screen.queryByText(/Available reward|Order Delivered|unlock.*credit/i)).not.toBeInTheDocument();
  expect(screen.queryByRole('link', { name: 'Share to WhatsApp' })).not.toBeInTheDocument();
});
it('shows a genuine zero without an invented milestone', async () => {
  metricReply = async () => Response.json(metric(0)); render(<GrowthReferralWidget />);
  expect(await screen.findByText('No recorded orders yet.')).toBeVisible();
  expect(screen.queryByRole('link', { name: 'Share to WhatsApp' })).not.toBeInTheDocument();
});
it.each([
  { ...metric(), success: false }, { ...metric(), recorded_orders: -1 }, { ...metric(), tenant_id: b.tenantId },
  { ...metric(), highest_threshold: 100 }, { title: '100th Order Delivered!', reward: '$50 Credit' },
])('rejects a contradictory, foreign or fabricated metric: %j', async value => {
  metricReply = async () => Response.json(value); render(<GrowthReferralWidget />);
  expect(await screen.findByText('Recorded order data is unavailable.')).toBeVisible();
  expect(screen.queryByText('11 recorded orders')).not.toBeInTheDocument();
});
it('does not accept a202 metric acknowledgement as completed data', async () => {
  metricReply = async () => Response.json(metric(), { status: 202 }); render(<GrowthReferralWidget />);
  expect(await screen.findByText('Recorded order data is unavailable.')).toBeVisible();
});
it('retries only the read after an unavailable response without creating an invitation', async () => {
  metricReply = async () => Response.json({ error: 'milestone_unavailable' }, { status: 503 }); render(<GrowthReferralWidget />);
  await screen.findByText('Recorded order data is unavailable.');
  metricReply = async () => Response.json(metric()); fireEvent.click(screen.getByRole('button', { name: 'Refresh recorded orders' }));
  await ready();
  expect(vi.mocked(fetch).mock.calls.filter(([url]) => url === '/api/v1/growth/milestone')).toHaveLength(2);
  expect(vi.mocked(fetch).mock.calls.some(([, init]) => init?.method === 'POST')).toBe(false);
});
it('offers only the same-owner confirmed invitation as a reviewed WhatsApp intent', async () => {
  render(<GrowthReferralWidget />); await ready(); await invite();
  const anchor = await screen.findByRole('link', { name: 'Share to WhatsApp' });
  const destination = new URL(anchor.getAttribute('href')!);
  expect(destination.origin).toBe('https://wa.me');
  expect(destination.searchParams.get('text')).toBe(`We've recorded 11 orders in OmniSolo. ${link}`);
  expect(screen.getByLabelText('Milestone share preview')).toHaveValue(`We've recorded 11 orders in OmniSolo. ${link}`);
  expect(screen.getByText(/Opening a share intent does not send a message/)).toBeVisible();
});
it('never invents a milestone share link after an unknown invitation outcome', async () => {
  inviteReply = async () => { throw new Error('lost after dispatch'); };
  render(<GrowthReferralWidget />); await ready(); await invite();
  expect(screen.queryByRole('link', { name: 'Share to WhatsApp' })).not.toBeInTheDocument();
});
it('retires private counts and a delayed body when the authenticated session changes', async () => {
  const body = deferred<unknown>(); metricReply = async () => ({ status: 200, json: () => body.promise }) as Response;
  render(<GrowthReferralWidget />);
  await waitFor(() => expect(vi.mocked(fetch).mock.calls.some(([url]) => url === '/api/v1/growth/milestone')).toBe(true));
  act(() => window.dispatchEvent(new Event('omnisolo_auth_changed')));
  await act(async () => body.resolve(metric()));
  expect(screen.queryByText('11 recorded orders')).not.toBeInTheDocument();
  expect(screen.queryByRole('link', { name: 'Share to WhatsApp' })).not.toBeInTheDocument();
});
it('retires both aggregate and confirmed link after another canonical read verifies a different owner', async () => {
  render(<GrowthReferralWidget />); await ready(); await invite();
  expect(await screen.findByRole('link', { name: 'Share to WhatsApp' })).toBeVisible();
  owner = b; await act(async () => { await readQueueOwner(); });
  expect(screen.queryByText('11 recorded orders')).not.toBeInTheDocument();
  expect(screen.queryByRole('link', { name: 'Share to WhatsApp' })).not.toBeInTheDocument();
});
it('holds recorded data during a pending same-owner verification and restores the same receipt afterward', async () => {
  render(<GrowthReferralWidget />); await ready(); await invite();
  const pending = deferred<Response>(); const previous = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((url, init) => url === '/api/v1/auth/session-identity' ? pending.promise : previous(url, init));
  let checking!: Promise<unknown>; act(() => { checking = readQueueOwner(); });
  expect(screen.queryByText('11 recorded orders')).not.toBeInTheDocument();
  expect(screen.queryByRole('link', { name: 'Share to WhatsApp' })).not.toBeInTheDocument();
  await act(async () => { pending.resolve(Response.json({ ...a, expiresAt: Date.now() + 60000 })); await checking; });
  expect(screen.getByText('11 recorded orders')).toBeVisible();
  expect(screen.getByRole('link', { name: 'Share to WhatsApp' })).toBeVisible();
});
it('retires a401 metric denial before consuming a stalled response body', async () => {
  const json = vi.fn(() => new Promise(() => {})); metricReply = async () => ({ status: 401, json }) as unknown as Response;
  render(<GrowthReferralWidget />);
  await waitFor(() => expect(screen.getByRole('status', { name: 'Team invitation status' })).toHaveTextContent(/session changed/));
  expect(json).not.toHaveBeenCalled();
  expect(screen.queryByText('11 recorded orders')).not.toBeInTheDocument();
});
it.each(['session_identity_changed', 'queued owner does not match the current session'])('retires a proved owner mismatch: %s', async error => {
  metricReply = async () => Response.json({ error }, { status: 409 }); render(<GrowthReferralWidget />);
  await waitFor(() => expect(screen.getByRole('status', { name: 'Team invitation status' })).toHaveTextContent(/session changed/));
  expect(screen.getByRole('button', { name: 'Unlock Cloud Collaboration' })).toBeDisabled();
});
it('waits for actual clipboard completion and surfaces a denied copy without success', async () => {
  const copying = deferred<void>(); vi.mocked(navigator.clipboard.writeText).mockImplementation(() => copying.promise);
  render(<GrowthReferralWidget />); await ready(); await invite();
  const button = await screen.findByRole('button', { name: 'Copy milestone share text' });
  fireEvent.click(button); expect(button).toBeDisabled();
  expect(screen.queryByRole('status', { name: 'Milestone clipboard' })).not.toHaveTextContent('Copied');
  await act(async () => copying.resolve());
  expect(await screen.findByRole('status', { name: 'Milestone clipboard' })).toHaveTextContent(/copied/i);
  vi.mocked(navigator.clipboard.writeText).mockRejectedValueOnce(new Error('clipboard denied'));
  fireEvent.click(button);
  expect(await screen.findByRole('alert', { name: 'Milestone clipboard' })).toHaveTextContent('Copy failed. Select the content and copy it manually.');
});
