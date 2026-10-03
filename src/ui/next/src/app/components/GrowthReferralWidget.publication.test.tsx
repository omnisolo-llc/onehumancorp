import { act, cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner, readQueueOwner, QUEUE_IDENTITY_EPOCH_KEY } from '@/lib/sync/queueIdentity';
import { openBuilderScope } from '../builder/ownedDraft';
import { preparePublicationReview, publicationOperationKey, type PublicationChannel, type PublicationReview } from '../builder/publicationOperations';
import { installBuilderLocks } from '../builder/testLocks';
import { invalidateOnboardingSession } from '../onboarding/draftSession';
import GrowthReferralWidget from './GrowthReferralWidget';
const OWNER = { userId: 'published-owner', tenantId: 'published-tenant' };
const SITE = '30000000-0000-4000-8000-000000000003';
const PUBLICATION = '20000000-0000-4000-8000-000000000002';
const ROOT = '/api/v1/builder/publications/operations/';
let owner = OWNER;
let leaseMs = 60_000;
let identityReply: () => Promise<Response>;
let reply: (url: string) => Promise<Response>;
const card = () => within(screen.getByRole('region', { name: 'Published storefront embed' }));
const deferred = <T,>() => { let resolve!: (value: T) => void; const promise = new Promise<T>(done => { resolve = done; }); return { promise, resolve }; };
const snapshot = { domain: null, pages: [{ path: '/', title: 'Published shop', seo_metadata: {}, blocks: [{ block_type: 'HeroBlock', content: { headline: 'Reviewed storefront' }, sort_order: 0 }] }] };
const receipt = (review: PublicationReview, status = 'published') => ({ schema_version: 1, user_id: review.owner.userId, organization_id: review.owner.tenantId, operation_id: review.operation_id, publication_id: PUBLICATION, site_id: SITE, version: 1, status, snapshot_sha256: review.snapshot_sha256, snapshot_encoding: 'jcs-rfc8785-v1', public_path: status === 'published' ? '/api/v1/public/sites/' + SITE : null });
async function saved(channel: PublicationChannel = 'storefront-builder', status = 'published') {
  const scope = await openBuilderScope(); const review = await preparePublicationReview(scope, channel, snapshot);
  const { previous_sha256, ...operation } = review; expect(previous_sha256).toBeNull();
  localStorage.setItem(publicationOperationKey(scope, channel), JSON.stringify({ format: 1, phase: 'acknowledged', operation, receipt: receipt(review, status) }));
  return review;
}
beforeEach(() => {
  localStorage.clear(); act(() => { invalidateQueueOwner(); invalidateOnboardingSession(); }); owner = OWNER; leaseMs = 60_000;
  localStorage.setItem('business_display_name', 'Forged store & tenant'); installBuilderLocks();
  identityReply = async () => Response.json({ ...owner, expiresAt: Date.now() + leaseMs });
  reply = async () => Response.json({ error: 'unavailable' }, { status: 503 });
  vi.stubGlobal('fetch', vi.fn(async (url, init) => {
    if (url === '/api/v1/auth/session-identity') return identityReply();
    if (String(url).startsWith(ROOT)) {
      expect(init?.method).toBe('GET');
      expect(new Headers(init?.headers).get('x-ohc-expected-user')).toBe(OWNER.userId);
      expect(new Headers(init?.headers).get('x-ohc-expected-tenant')).toBe(OWNER.tenantId);
      return reply(String(url));
    }
    if (url === '/api/v1/growth/milestone') return Response.json({ error: 'unavailable' }, { status: 503 });
    throw new Error(`Unexpected fixture request: ${url}`);
  }));
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: vi.fn().mockResolvedValue(undefined) } });
});
afterEach(() => { cleanup(); act(() => { invalidateQueueOwner(); invalidateOnboardingSession(); }); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
async function loaded() { await waitFor(() => expect(card().getByRole('button', { name: 'Check publication' })).toBeEnabled()); }
async function check() { await loaded(); fireEvent.click(card().getByRole('button', { name: 'Check publication' })); }
it('has an honest empty state instead of a local-name tenant or invented referral link', async () => {
  render(<GrowthReferralWidget />);
  await waitFor(() => expect(card().getByText(/No saved publication/)).toBeVisible());
  expect(card().getByRole('link', { name: 'Open storefront builder' })).toHaveAttribute('href', '/storefront-builder');
  expect(card().getByRole('button', { name: 'Copy Embed Code' })).toBeDisabled();
  expect(card().queryByRole('textbox')).not.toBeInTheDocument();
  expect(navigator.clipboard.writeText).not.toHaveBeenCalled();
});
it('refreshes an actual matching published operation before displaying and again before copying its path', async () => {
  const review = await saved(); reply = async () => Response.json(receipt(review));
  render(<GrowthReferralWidget />); await loaded();
  expect(card().getByRole('button', { name: 'Copy Embed Code' })).toBeDisabled();
  await check(); await waitFor(() => expect(card().getByRole('button', { name: 'Copy Embed Code' })).toBeEnabled());
  const code = (card().getByRole('textbox', { name: 'Published embed code' }) as HTMLTextAreaElement).value;
  expect(code).toContain(window.location.origin + '/api/v1/public/sites/' + SITE);
  expect(code).not.toContain('Forged'); expect(code).not.toContain('referrals/click');
  fireEvent.click(card().getByRole('button', { name: 'Copy Embed Code' }));
  await waitFor(() => expect(navigator.clipboard.writeText).toHaveBeenCalledExactlyOnceWith(code));
  expect(vi.mocked(fetch).mock.calls.filter(([url]) => String(url) === ROOT + review.operation_id)).toHaveLength(2);
  await waitFor(() => expect(card().getByRole('status')).toHaveTextContent(/Copied/));
});
it.each(['revoked', 'pending', 'server503', 'denied403', 'accepted202', 'wrong-owner', 'malformed'])('never copies a locally claimed publication with a real %s receipt', async status => {
  const review = await saved('storefront-builder', status === 'pending' ? 'pending' : 'published');
  reply = async () => Response.json(status === 'server503' ? { error: 'unavailable' } : status === 'malformed' ? {} : { ...receipt(review, status === 'revoked' || status === 'pending' ? status : 'published'), ...(status === 'wrong-owner' ? { user_id: 'foreign' } : {}) }, { status: status === 'server503' ? 503 : status === 'denied403' ? 403 : status === 'accepted202' ? 202 : 200 });
  render(<GrowthReferralWidget />); await check();
  await waitFor(() => expect(card().getByRole('status')).not.toHaveTextContent('Checking current publication status…'));
  expect(vi.mocked(fetch).mock.calls.filter(([url]) => String(url).startsWith(ROOT))).toHaveLength(1);
  expect(card().getByRole('button', { name: 'Copy Embed Code' })).toBeDisabled();
  expect(card().queryByRole('textbox')).not.toBeInTheDocument(); expect(navigator.clipboard.writeText).not.toHaveBeenCalled();
});
it('rechecks revocation instead of copying a previously displayed path', async () => {
  const review = await saved(); reply = async () => Response.json(receipt(review));
  render(<GrowthReferralWidget />); await check(); await waitFor(() => expect(card().getByRole('button', { name: 'Copy Embed Code' })).toBeEnabled());
  reply = async () => Response.json(receipt(review, 'revoked'));
  fireEvent.click(card().getByRole('button', { name: 'Copy Embed Code' }));
  await waitFor(() => expect(card().queryByRole('textbox')).not.toBeInTheDocument());
  expect(navigator.clipboard.writeText).not.toHaveBeenCalled();
});
it.each(['auth', 'storage', 'pagehide', 'unmount'])('%s retirement fences a delayed published body', async event => {
  const review = await saved(); const pending = deferred<Response>(); reply = () => pending.promise;
  const view = render(<GrowthReferralWidget />); await check();
  await waitFor(() => expect(vi.mocked(fetch).mock.calls.some(([url]) => String(url).startsWith(ROOT))).toBe(true));
  if (event === 'unmount') view.unmount();
  else act(() => window.dispatchEvent(event === 'storage' ? new StorageEvent('storage', { key: QUEUE_IDENTITY_EPOCH_KEY }) : new Event(event === 'auth' ? 'omnisolo_auth_changed' : event)));
  await act(async () => pending.resolve(Response.json(receipt(review))));
  expect(screen.queryByRole('textbox', { name: 'Published embed code' })).not.toBeInTheDocument();
  expect(navigator.clipboard.writeText).not.toHaveBeenCalled();
});
it('does not select another account’s saved publication', async () => {
  await saved(); owner = { userId: 'other-owner', tenantId: 'other-tenant' }; act(() => invalidateOnboardingSession());
  render(<GrowthReferralWidget />);
  await waitFor(() => expect(card().getByText(/No saved publication/)).toBeVisible());
  expect(vi.mocked(fetch).mock.calls.some(([url]) => String(url).startsWith(ROOT))).toBe(false);
});

it('expires a verified preview and does not restore it merely on back navigation', async () => {
  leaseMs = 1_000;
  const review = await saved(); reply = async () => Response.json(receipt(review));
  render(<GrowthReferralWidget />); await check(); await waitFor(() => expect(card().getByRole('button', { name: 'Copy Embed Code' })).toBeEnabled());
  await waitFor(() => expect(card().queryByRole('textbox')).not.toBeInTheDocument(), { timeout: 2_500 });
  expect(card().getByRole('button', { name: 'Copy Embed Code' })).toBeDisabled();
  act(() => window.dispatchEvent(new Event('pageshow')));
  expect(card().queryByRole('textbox')).not.toBeInTheDocument();
});
it('does not copy after a silent identity epoch change', async () => {
  const review = await saved(); reply = async () => Response.json(receipt(review));
  render(<GrowthReferralWidget />); await check(); await waitFor(() => expect(card().getByRole('button', { name: 'Copy Embed Code' })).toBeEnabled());
  localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY, 'replaced-owner');
  fireEvent.click(card().getByRole('button', { name: 'Copy Embed Code' }));
  expect(navigator.clipboard.writeText).not.toHaveBeenCalled();
  expect(card().queryByRole('textbox')).not.toBeInTheDocument();
});
it('reports a genuine clipboard rejection while retaining manually selectable published code', async () => {
  const review = await saved(); reply = async () => Response.json(receipt(review));
  vi.mocked(navigator.clipboard.writeText).mockRejectedValue(new DOMException('Denied', 'NotAllowedError'));
  render(<GrowthReferralWidget />); await check(); await waitFor(() => expect(card().getByRole('button', { name: 'Copy Embed Code' })).toBeEnabled());
  fireEvent.click(card().getByRole('button', { name: 'Copy Embed Code' }));
  await waitFor(() => expect(card().getByRole('alert')).toHaveTextContent(/Copy failed/));
  expect(card().getByRole('textbox', { name: 'Published embed code' })).toBeVisible();
  expect(card().queryByText(/Copied to clipboard/)).not.toBeInTheDocument();
});
it('retires clipboard completion feedback after navigation closes the original view', async () => {
  const review = await saved(); reply = async () => Response.json(receipt(review));
  const pending = deferred<void>(); vi.mocked(navigator.clipboard.writeText).mockImplementation(() => pending.promise);
  render(<GrowthReferralWidget />); await check(); await waitFor(() => expect(card().getByRole('button', { name: 'Copy Embed Code' })).toBeEnabled());
  fireEvent.click(card().getByRole('button', { name: 'Copy Embed Code' }));
  await waitFor(() => expect(navigator.clipboard.writeText).toHaveBeenCalledOnce());
  act(() => window.dispatchEvent(new Event('pagehide')));
  await act(async () => pending.resolve());
  expect(card().queryByRole('textbox')).not.toBeInTheDocument();
  expect(card().queryByText(/Copied to clipboard/)).not.toBeInTheDocument();
});

it('does not revive an expired publication view after a late same-owner verification', async () => {
  leaseMs = 1_000;
  const review = await saved(); reply = async () => Response.json(receipt(review));
  render(<GrowthReferralWidget />); await check(); await waitFor(() => expect(card().getByRole('button', { name: 'Copy Embed Code' })).toBeEnabled());
  const pending = deferred<Response>(); identityReply = () => pending.promise;
  let verification!: Promise<unknown>;
  act(() => { verification = readQueueOwner().catch(error => error); });
  expect(card().queryByRole('textbox')).not.toBeInTheDocument();
  await act(async () => new Promise(resolve => setTimeout(resolve, 1_100)));
  await act(async () => { pending.resolve(Response.json({ ...owner, expiresAt: Date.now() + 60_000 })); await verification; });
  expect(card().queryByRole('textbox')).not.toBeInTheDocument();
  expect(card().getByRole('button', { name: 'Copy Embed Code' })).toBeDisabled();
  expect(navigator.clipboard.writeText).not.toHaveBeenCalled();
});
