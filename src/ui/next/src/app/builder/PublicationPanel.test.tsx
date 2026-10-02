import { useLayoutEffect } from 'react';
import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange, readQueueOwner } from '@/lib/sync/queueIdentity';
import { installBuilderLocks } from './testLocks';
import * as contracts from './publicationContracts';
import type { QueueOwner } from '@/lib/sync/queueIdentity';
import { PublicationPanel as Panel } from './PublicationPanel';

type Props = { channel: 'public-bio'; expectedOwner: QueueOwner | null; getSnapshot: () => unknown; isEditorCurrent: () => boolean; onRetired: (owner: QueueOwner, reason: string) => void };
const ROOT = '/api/v1/builder/publications';
const SITE = '30000000-0000-4000-8000-000000000003';
const PUBLICATION = '20000000-0000-4000-8000-000000000002';
let owner: QueueOwner; let snapshot: ReturnType<typeof draft>; let status: string; let lastReceipt: Record<string, unknown> | null;
let handle: (url: string, options?: RequestInit) => Promise<Response>;
const retired = vi.fn();
function draft() { return { domain: null, pages: [{ path: '/', title: 'Private profile', seo_metadata: {}, blocks: [{ block_type: 'HeroBlock', content: { headline: 'Owner A profile', subtitle: 'Reviewed biography' }, sort_order: 0 }] }] }; }
beforeEach(() => {
  const values = new Map<string, string>();
  vi.stubGlobal('localStorage', { getItem: (key: string) => values.get(key) ?? null, setItem: (key: string, value: string) => { values.set(key, String(value)); },
    removeItem: (key: string) => { values.delete(key); }, clear: () => values.clear(), key: (index: number) => Array.from(values.keys())[index] ?? null, get length() { return values.size; } });
  notifyQueueIdentityChange(); installBuilderLocks(); owner = { userId: 'owner-a', tenantId: 'tenant-a' }; snapshot = draft(); status = 'pending'; lastReceipt = null; retired.mockReset();
  handle = async (_url, options) => {
    if (options?.method === 'POST') {
      const data = JSON.parse(String(options.body)); const prepared = await contracts.prepareSiteSnapshot(data.snapshot);
      lastReceipt = { schema_version: 1, user_id: owner.userId, organization_id: owner.tenantId, operation_id: data.operation_id, site_id: SITE,
        publication_id: PUBLICATION, version: 1, snapshot_sha256: prepared.snapshot_sha256, snapshot_encoding: prepared.snapshot_encoding };
    }
    if (options?.method === 'DELETE') status = 'revoked';
    return Response.json({ ...lastReceipt, status, public_path: status === 'published' ? '/api/v1/public/sites/' + SITE : null }, { status: options?.method === 'POST' && status === 'pending' ? 202 : 200 });
  };
  vi.stubGlobal('fetch', vi.fn(async (url, options) => String(url).endsWith('/session-identity') ? Response.json({ ...owner, expiresAt: Date.now() + 60_000 }) : handle(String(url), options)));
});
afterEach(() => { cleanup(); vi.useRealTimers(); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
const props = (): Props => ({ channel: 'public-bio', expectedOwner: { ...owner }, getSnapshot: () => snapshot, isEditorCurrent: () => true, onRetired: retired });
const mutations = () => vi.mocked(fetch).mock.calls.filter(([url, options]) => String(url).startsWith(ROOT) && ['POST', 'DELETE'].includes(String(options?.method)));
async function reviewed() { fireEvent.click(await screen.findByRole('button', { name: 'Review public version' })); return screen.findByRole('button', { name: 'Publish reviewed version' }); }

it('shows every selected field and the public audience before any publication mutation', async () => {
  render(<Panel {...props()} />); const submit = await reviewed();
  expect(screen.getByLabelText('Public website snapshot')).toHaveTextContent('Owner A profile');
  expect(screen.getByLabelText('Public website snapshot')).toHaveTextContent('Reviewed biography');
  expect(screen.getByText(/Anyone with the link can read these reviewed pages/)).toBeVisible(); expect(mutations()).toHaveLength(0);
  fireEvent.click(submit);
  expect(await screen.findByText(/Publication is queued/)).toBeVisible(); expect(screen.queryByRole('link', { name: 'Open published website' })).toBeNull();
});
it('exposes only a current published receipt path after status recovery and revokes that exact version', async () => {
  render(<Panel {...props()} />); fireEvent.click(await reviewed()); await screen.findByText(/Publication is queued/);
  status = 'published'; fireEvent.click(screen.getByRole('button', { name: 'Check publication status' }));
  expect(await screen.findByRole('link', { name: 'Open published website' })).toHaveAttribute('href', '/api/v1/public/sites/' + SITE);
  fireEvent.click(screen.getByRole('button', { name: 'Revoke publication version' }));
  await waitFor(() => expect(screen.queryByRole('link', { name: 'Open published website' })).toBeNull());
  expect(await screen.findByText(/This publication version is revoked/)).toBeVisible();
  expect(mutations().map(([url, options]) => [url, options?.method])).toEqual([[ROOT, 'POST'], [ROOT + '/' + PUBLICATION, 'DELETE']]);
});
it('requires fresh review when private edits change after approval', async () => {
  render(<Panel {...props()} />); const submit = await reviewed(); snapshot = { ...snapshot, pages: [{ ...snapshot.pages[0], title: 'New private title' }] };
  fireEvent.click(submit); expect(await screen.findByText(/changed.*review.*again/i)).toBeVisible(); expect(mutations()).toHaveLength(0);
  expect(screen.queryByRole('button', { name: 'Publish reviewed version' })).toBeNull();
});
it('cancels an unsubmitted review without discarding the private editor draft', async () => {
  render(<Panel {...props()} />); await reviewed(); fireEvent.click(screen.getByRole('button', { name: 'Cancel publication review' }));
  expect(screen.queryByLabelText('Public website snapshot')).toBeNull(); expect(snapshot).toEqual(draft()); expect(mutations()).toHaveLength(0);
});
it('holds a lost POST across remount and performs only read-only recovery', async () => {
  const original = handle; let unknown = true;
  handle = async (url, options) => { if (options?.method === 'POST') { await original(url, options); throw new Error('Lost response'); } if (unknown) return Response.json({ error: 'Missing' }, { status: 404 }); return original(url, options); };
  const view = render(<Panel {...props()} />); fireEvent.click(await reviewed());
  expect(await screen.findByText(/could not be confirmed|Lost response/)).toBeVisible(); expect(mutations()).toHaveLength(1);
  view.unmount(); render(<Panel {...props()} />);
  await screen.findByRole('button', { name: 'Check publication status' }); expect(mutations()).toHaveLength(1);
  expect(screen.getByRole('button', { name: 'Review public version' })).toBeDisabled();
  unknown = false; status = 'published'; fireEvent.click(screen.getByRole('button', { name: 'Check publication status' }));
  expect(await screen.findByRole('link', { name: 'Open published website' })).toBeVisible(); expect(mutations()).toHaveLength(1);
});
it('retires the full review immediately on canonical owner contradiction and names the retired owner', async () => {
  const originalOwner = { ...owner }; render(<Panel {...props()} />); await reviewed();
  owner = { userId: 'owner-b', tenantId: 'tenant-b' }; await act(async () => { await readQueueOwner(); });
  expect(screen.queryByLabelText('Public website snapshot')).toBeNull(); expect(screen.queryByText('Owner A profile')).toBeNull();
  expect(retired).toHaveBeenCalledWith(originalOwner, expect.any(String)); expect(mutations()).toHaveLength(0);
});
it('never hydrates an operation for an expected editor owner that differs from the verified session', async () => {
  const expected = { ...owner }; owner = { userId: 'owner-b', tenantId: 'tenant-b' };
  render(<Panel {...props()} expectedOwner={expected} />);
  expect(await screen.findByText(/session.*changed|owner.*changed/i)).toBeVisible(); expect(retired).toHaveBeenCalledWith(expected, expect.any(String));
  expect(vi.mocked(fetch).mock.calls.every(([url]) => String(url).endsWith('/session-identity'))).toBe(true);
});
it('does not permit a duplicate submission after a published acknowledgement remains mounted', async () => {
  status = 'published'; render(<Panel {...props()} />); fireEvent.click(await reviewed());
  expect(await screen.findByRole('link', { name: 'Open published website' })).toBeVisible();
  expect(screen.queryByRole('button', { name: 'Publish reviewed version' })).toBeNull(); expect(mutations()).toHaveLength(1);
});
it('rechecks reviewed content after dispatch-time identity verification before the POST', async () => {
  render(<Panel {...props()} />); const submit = await reviewed();
  let resolve!: (response: Response) => void;
  const identity = new Promise<Response>(yes => { resolve = yes; });
  const previousFetch = fetch;
  vi.stubGlobal('fetch', vi.fn(async (url, options) => String(url).endsWith('/session-identity') ? identity : previousFetch(url, options)));
  fireEvent.click(submit);
  await waitFor(() => expect(fetch).toHaveBeenCalled());
  snapshot.pages[0].title = 'Changed while verifying session';
  await act(async () => resolve(Response.json({ ...owner, expiresAt: Date.now() + 60_000 })));
  expect(await screen.findByText(/changed.*review.*again/i)).toBeVisible();
  expect(mutations()).toHaveLength(0);
});
it('does not show a cached published link on remount before a fresh status receipt', async () => {
  status = 'published'; const view = render(<Panel {...props()} />); fireEvent.click(await reviewed());
  await screen.findByRole('link', { name: 'Open published website' });
  view.unmount(); render(<Panel {...props()} />);
  await screen.findByRole('button', { name: 'Check publication status' });
  expect(screen.queryByRole('link', { name: 'Open published website' })).toBeNull();
  status = 'revoked'; fireEvent.click(screen.getByRole('button', { name: 'Check publication status' }));
  expect(await screen.findByText(/This publication version is revoked/)).toBeVisible();
  expect(screen.queryByRole('link', { name: 'Open published website' })).toBeNull();
});

it('hides an old owner review in the render that receives a different editor owner', async () => {
  let observed: string | null = null;
  function Observed({ expected }: { expected: QueueOwner }) {
    useLayoutEffect(() => { observed = document.querySelector('[aria-label="Public website snapshot"]')?.textContent ?? null; }, [expected]);
    return <Panel {...props()} expectedOwner={expected} />;
  }
  const a = { ...owner }; const view = render(<Observed expected={a} />); await reviewed();
  view.rerender(<Observed expected={{ userId: 'new-editor', tenantId: 'new-workspace' }} />);
  expect(observed).toBeNull();
});
