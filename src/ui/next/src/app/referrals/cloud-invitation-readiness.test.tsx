import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { useCallback, useState } from 'react';
import { invalidateQueueOwner, readQueueOwner } from '../../lib/sync/queueIdentity';
import { useCloudInvitation } from './useCloudInvitation';

const ownerA = { userId: 'owner-a', tenantId: 'tenant-a' };
const ownerB = { userId: 'owner-b', tenantId: 'tenant-b' };
const link = 'https://omnisolo.co/invite/real-owned-record';
const keyA = 'omnisolo_invite_creation_v1:' + encodeURIComponent(JSON.stringify([ownerA.userId, ownerA.tenantId]));
const deferred = <T,>() => { let resolve!: (value: T) => void; const promise = new Promise<T>(yes => { resolve = yes; }); return { promise, resolve }; };
let currentOwner = ownerA;
let identityReply: () => Promise<Response>;
let invitationReply: () => Promise<Response>;
const posts = () => vi.mocked(fetch).mock.calls.filter(([, options]) => options?.method === 'POST');
function Harness() {
  const [draft, setDraft] = useState('');
  const retire = useCallback(() => setDraft(''), []);
  const invitation = useCloudInvitation(retire);
  return <><input aria-label="Invitee" value={draft} onChange={event => setDraft(event.target.value)} />
    <button disabled={invitation.phase !== 'ready'} onClick={() => void invitation.create(draft)}>Create invitation</button>
    <output aria-label="Owner">{invitation.verifiedOwner?.userId ?? 'none'}</output>
    <div role="status" aria-label="Invitation status">{invitation.message}</div><output aria-label="Link">{invitation.link}</output></>;
}
beforeEach(() => {
  localStorage.clear(); invalidateQueueOwner(); currentOwner = ownerA;
  Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
  Object.defineProperty(navigator, 'locks', { value: { request: async (_name: string, _options: unknown, callback: (lock: object) => Promise<void>) => callback({}) } });
  identityReply = async () => Response.json({ ...currentOwner, expiresAt: Date.now() + 60_000 });
  invitationReply = async () => Response.json({ invite_link: link });
  vi.stubGlobal('fetch', vi.fn(async url => url === '/api/v1/auth/session-identity' ? identityReply() : invitationReply()));
});
afterEach(() => { invalidateQueueOwner(); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
async function ready() {
  render(<Harness />);
  await waitFor(() => expect(screen.getByRole('button', { name: 'Create invitation' })).toBeEnabled());
  fireEvent.change(screen.getByLabelText('Invitee'), { target: { value: 'private-a@example.test' } });
}
async function create() { await act(async () => { fireEvent.click(screen.getByRole('button', { name: 'Create invitation' })); }); }

it('retires a private draft when the actual canonical reader verifies another owner without an auth event', async () => {
  await ready();
  currentOwner = ownerB;
  await act(async () => { expect(await readQueueOwner()).toEqual(ownerB); });
  expect(screen.getByLabelText('Invitee')).toHaveValue('');
  expect(screen.getByLabelText('Owner')).toHaveTextContent('none');
  expect(screen.getByRole('button', { name: 'Create invitation' })).toBeDisabled();
  expect(posts()).toHaveLength(0);
});
it('retires a confirmed link and keeps its original owner marker after a separately verified owner change', async () => {
  await ready(); await create();
  expect(screen.getByLabelText('Link')).toHaveTextContent(link);
  const bytes = localStorage.getItem(keyA);
  currentOwner = ownerB;
  await act(async () => { await readQueueOwner(); });
  expect(screen.getByLabelText('Link')).toBeEmptyDOMElement();
  expect(screen.getByRole('status', { name: 'Invitation status' })).not.toHaveTextContent(link);
  expect(localStorage.getItem(keyA)).toBe(bytes);
  expect(posts()).toHaveLength(1);
});
it('never adopts a late initial identity after a newer canonical verification proved another owner', async () => {
  const body = deferred<{ userId: string; tenantId: string; expiresAt: number }>();
  identityReply = async () => ({ status: 200, json: () => body.promise }) as Response;
  render(<Harness />);
  await waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
  currentOwner = ownerB;
  identityReply = async () => Response.json({ ...ownerB, expiresAt: Date.now() + 60_000 });
  await act(async () => { await readQueueOwner(); });
  await act(async () => body.resolve({ ...ownerA, expiresAt: Date.now() + 60_000 }));
  expect(screen.getByLabelText('Owner')).toHaveTextContent('none');
  expect(screen.getByRole('button', { name: 'Create invitation' })).toBeDisabled();
  expect(posts()).toHaveLength(0);
});
it('does not expose a late POST body after a canonical owner replacement', async () => {
  const body = deferred<{ invite_link: string }>();
  invitationReply = async () => ({ status: 200, json: () => body.promise }) as Response;
  await ready(); await create();
  expect(JSON.parse(localStorage.getItem(keyA)!)).toMatchObject({ owner: ownerA, state: 'pending' });
  currentOwner = ownerB;
  await act(async () => { await readQueueOwner(); });
  await act(async () => body.resolve({ invite_link: link }));
  expect(screen.getByLabelText('Link')).toBeEmptyDOMElement();
  expect(screen.getByLabelText('Owner')).toHaveTextContent('none');
  expect(screen.getByLabelText('Invitee')).toHaveValue('');
  expect(JSON.parse(localStorage.getItem(keyA)!)).toMatchObject({ owner: ownerA, state: 'created' });
  expect(posts()).toHaveLength(1);
});
it('does not mistake an unresolved same-owner verification for a proved account change', async () => {
  await ready();
  const pending = deferred<Response>(); identityReply = () => pending.promise;
  let read!: Promise<unknown>;
  act(() => { read = readQueueOwner(); });
  expect(screen.getByLabelText('Invitee')).toHaveValue('private-a@example.test');
  await act(async () => { pending.resolve(Response.json({ ...ownerA, expiresAt: Date.now() + 60_000 })); await read; });
  expect(screen.getByLabelText('Invitee')).toHaveValue('private-a@example.test');
  expect(screen.getByLabelText('Owner')).toHaveTextContent(ownerA.userId);
  expect(screen.getByRole('button', { name: 'Create invitation' })).toBeEnabled();
});
