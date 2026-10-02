import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner } from '@/lib/sync/queueIdentity';
import { SuccessMilestoneWidget } from './SuccessMilestoneWidget';

const owner = { userId: 'dashboard-owner', tenantId: 'dashboard-tenant' };
const link = 'https://cloud.omnisolo.co/invite/dashboard-record';
let metricReply: () => Promise<Response>;
beforeEach(() => {
  localStorage.clear(); act(() => invalidateQueueOwner());
  Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
  Object.defineProperty(navigator, 'locks', { value: { request: async (_name: string, _options: unknown, callback: (lock: object) => Promise<void>) => callback({}) } });
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: vi.fn().mockResolvedValue(undefined) } });
  metricReply = async () => Response.json({ success: true, user_id: owner.userId, tenant_id: owner.tenantId, metric: 'recorded_orders', included_statuses: 'all_recorded_statuses', recorded_orders: 101, reached_thresholds: [1,10,50,100], highest_threshold: 100, observed_at: '2026-10-02T00:00:00Z' });
  vi.stubGlobal('fetch', vi.fn(async url => {
    if (url === '/api/v1/auth/session-identity') return Response.json({ ...owner, expiresAt: Date.now() + 60000 });
    if (url === '/api/v1/growth/milestone') return metricReply();
    if (url === '/api/v1/growth/cloud-bridge/invite') return Response.json({ invite_link: link });
    throw new Error(`Unexpected request: ${String(url)}`);
  }));
});
afterEach(() => { act(() => invalidateQueueOwner()); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
async function ready() { render(<SuccessMilestoneWidget />); await screen.findByText('101 recorded orders'); }
async function create() { await ready(); const button = screen.getByRole('button', { name: 'Create milestone invitation' }); await waitFor(() => expect(button).toBeEnabled()); await act(async () => fireEvent.click(button)); }
it('renders only recorded aggregates and threshold labels without sales or rewards', async () => {
  await ready(); expect(screen.getByText('Recorded-order milestone: 100')).toBeVisible();
  expect(screen.getByText(/Counts all recorded statuses/)).toBeVisible();
  expect(screen.queryByText(/100th Order Delivered|Available reward|\$50 Credit/)).not.toBeInTheDocument();
});
it('copies the actual same-owner share text only after a confirmed invitation', async () => {
  await create(); fireEvent.click(await screen.findByRole('button', { name: 'Copy milestone share text' }));
  await waitFor(() => expect(screen.getByRole('status', { name: 'Milestone clipboard' })).toHaveTextContent(/copied/i));
  expect(navigator.clipboard.writeText).toHaveBeenCalledWith(`We've recorded 101 orders in OmniSolo. ${link}`);
});
it('shows an unavailable state when the backend fails', async () => {
  metricReply = async () => Response.json({ error: 'milestone_unavailable' }, { status: 503 }); render(<SuccessMilestoneWidget />);
  expect(await screen.findByText('Recorded order data is unavailable.')).toBeVisible();
  expect(screen.queryByRole('link', { name: 'Share on X' })).not.toBeInTheDocument();
});
it('keeps both actual social intent links bound to reviewed aggregate text and the confirmed link', async () => {
  await create();
  const twitter = new URL((await screen.findByRole('link', { name: 'Share on X' })).getAttribute('href')!);
  const whatsapp = new URL(screen.getByRole('link', { name: 'Share to WhatsApp' }).getAttribute('href')!);
  expect(twitter.origin + twitter.pathname).toBe('https://twitter.com/intent/tweet'); expect(whatsapp.origin).toBe('https://wa.me');
  expect(twitter.searchParams.get('text')).toBe(`We've recorded 101 orders in OmniSolo. ${link}`);
  expect(whatsapp.searchParams.get('text')).toBe(twitter.searchParams.get('text'));
});
