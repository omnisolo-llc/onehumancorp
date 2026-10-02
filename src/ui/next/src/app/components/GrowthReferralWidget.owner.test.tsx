import { act, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import GrowthReferralWidget from './GrowthReferralWidget';
import { useCloudInvitation } from '../referrals/useCloudInvitation';

const owner = { userId: 'verified-owner', tenantId: 'verified-tenant' };
const receipt = 'https://cloud.omnisolo.co/invite/persisted-record';
const key = 'omnisolo_invite_creation_v1:' + encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId]));
let identityReply: () => Promise<Response>;
let inviteReply: () => Promise<Response>;
const deferred = <T,>() => { let resolve!: (value: T) => void; const promise = new Promise<T>(yes => { resolve = yes; }); return { promise, resolve }; };
const posts = () => vi.mocked(fetch).mock.calls.filter(([url, init]) => url === '/api/v1/growth/cloud-bridge/invite' && init?.method === 'POST');
const inviteButton = () => screen.getByRole('button', { name: 'Unlock Cloud Collaboration' });
const noRetirementDraft = () => {};
function OtherInvitationView() {
  const invitation = useCloudInvitation(noRetirementDraft);
  return <section aria-label="Other invitation view"><button disabled={invitation.phase !== 'ready'} onClick={() => void invitation.create('other@example.test')}>Other invite</button><p role="status">{invitation.message}</p></section>;
}

beforeEach(() => {
  localStorage.clear();
  localStorage.setItem('business_display_name', 'Display name is not authority');
  localStorage.setItem('user_display_name', 'Unverified local name');
  identityReply = async () => Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
  inviteReply = async () => Response.json({ invite_link: receipt });
  const locks = new Set<string>();
  Object.defineProperty(navigator, 'locks', { value: { request: async (name: string, _options: unknown, callback: (lock: object | null) => Promise<void>) => {
    if (locks.has(name)) return callback(null);
    locks.add(name); try { return await callback({}); } finally { locks.delete(name); }
  } } });
  vi.stubGlobal('fetch', vi.fn(async url => {
    if (url === '/api/v1/auth/session-identity') return identityReply();
    if (url === '/api/v1/growth/cloud-bridge/invite') return inviteReply();
    throw new Error(`Unexpected local request ${url}`);
  }));
});
afterEach(() => { vi.unstubAllGlobals(); vi.restoreAllMocks(); });
async function create() {
  await waitFor(() => expect(inviteButton()).toBeEnabled());
  await act(async () => { fireEvent.click(inviteButton()); });
}

it('waits for a verified identity before permitting a team invitation', async () => {
  const pending = deferred<Response>(); identityReply = () => pending.promise;
  const view = render(<GrowthReferralWidget />);
  expect(inviteButton()).toBeDisabled(); expect(posts()).toHaveLength(0);
  view.unmount(); await act(async () => pending.resolve(Response.json({ ...owner, expiresAt: Date.now() + 60_000 })));
});
it('binds the genuine receipt to the verified owner without local display-name authority', async () => {
  render(<GrowthReferralWidget />); await create();
  expect(posts()).toHaveLength(1); const options = posts()[0][1]!; const headers = new Headers(options.headers);
  expect(headers.get('x-ohc-expected-user')).toBe(owner.userId); expect(headers.get('x-ohc-expected-tenant')).toBe(owner.tenantId);
  expect(JSON.parse(options.body as string)).toEqual({ invitee_id: 'pending-invite' });
  expect(screen.getByDisplayValue(receipt)).toBeVisible(); expect(JSON.parse(localStorage.getItem(key)!).state).toBe('created');
});
it.each([
  { status: 200, body: { invite_link: receipt, error: 'denied' } },
  { status: 202, body: { invite_link: receipt } },
  { status: 200, body: { invite_url: receipt } },
])('holds a contradictory or unconfirmed receipt: %j', async fixture => {
  inviteReply = async () => Response.json(fixture.body, { status: fixture.status });
  render(<GrowthReferralWidget />); await create();
  expect(screen.queryByDisplayValue(receipt)).not.toBeInTheDocument(); expect(inviteButton()).toBeDisabled();
  expect(JSON.parse(localStorage.getItem(key)!).state).toBe('pending'); expect(posts()).toHaveLength(1);
});
it.each(['auth', 'storage', 'pagehide'])('%s retirement removes a late private invitation receipt', async event => {
  const pending = deferred<Response>(); inviteReply = () => pending.promise;
  render(<GrowthReferralWidget />); await create();
  act(() => window.dispatchEvent(event === 'auth' ? new Event('omnisolo_auth_changed') : event === 'pagehide' ? new Event('pagehide') : new StorageEvent('storage', { key: 'omnisolo_queue_identity_epoch_v2' })));
  await act(async () => pending.resolve(Response.json({ invite_link: receipt })));
  expect(screen.queryByDisplayValue(receipt)).not.toBeInTheDocument(); expect(inviteButton()).toBeDisabled();
  expect(JSON.parse(localStorage.getItem(key)!).state).toBe('created');
});
it('an uncertain POST remains held after the team view is reopened', async () => {
  inviteReply = async () => { throw new Error('connection closed after dispatch'); };
  const view = render(<GrowthReferralWidget />); await create(); view.unmount();
  render(<GrowthReferralWidget />); await act(async () => {});
  expect(inviteButton()).toBeDisabled(); expect(posts()).toHaveLength(1);
  expect(screen.getByRole('status', { name: 'Team invitation status' })).toHaveTextContent(/previous invitation request is unconfirmed/);
});
it('shares the existing invitation lock and marker with another maintained view', async () => {
  const pending = deferred<Response>(); inviteReply = () => pending.promise;
  render(<><GrowthReferralWidget /><OtherInvitationView /></>);
  const other = within(screen.getByRole('region', { name: 'Other invitation view' }));
  await waitFor(() => expect(other.getByRole('button')).toBeEnabled()); await create();
  await act(async () => fireEvent.click(other.getByRole('button')));
  expect(posts()).toHaveLength(1); expect(other.getByRole('status')).toHaveTextContent(/creation is held/);
  await act(async () => pending.resolve(Response.json({ invite_link: receipt })));
});
it('does not invent an order milestone, a default-team capability, or a reward', async () => {
  render(<GrowthReferralWidget />); await act(async () => {});
  expect(screen.queryByText(/10th Order! Share your success/)).not.toBeInTheDocument();
  expect(screen.queryByRole('img', { name: '10th Order Milestone' })).not.toBeInTheDocument();
  expect(document.querySelector('a[href*="default-team"]')).toBeNull();
  expect(screen.getByRole('heading', { name: 'Order milestones unavailable' })).toBeVisible();
  expect(screen.queryByRole('link', { name: /Share to WhatsApp/ })).not.toBeInTheDocument();
});
it('does not promise referral rewards from an unverified embed preview', async () => {
  render(<GrowthReferralWidget />); await act(async () => {});
  expect(screen.queryByText(/built-in referral loop to reward you/)).not.toBeInTheDocument();
  expect(screen.getByText(/Business identity and referral rewards have not been verified/)).toBeVisible();
});
