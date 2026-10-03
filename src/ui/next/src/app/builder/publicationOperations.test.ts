import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange, readQueueOwner } from '@/lib/sync/queueIdentity';
import { onboardingOwner } from '../onboarding/draftSession';
import { openBuilderScope, builderDraftKey, type BuilderScope } from './ownedDraft';
import { installBuilderLocks } from './testLocks';
import * as api from './publicationOperations';

type Review = api.PublicationReview;
const ROOT = '/api/v1/builder/publications';
const SITE = '30000000-0000-4000-8000-000000000003';
const PUBLICATION = '20000000-0000-4000-8000-000000000002';
let owner = { userId: 'publisher-a', tenantId: 'workspace-a' };
const draft = () => ({ domain: null, pages: [{ path: '/', title: 'Owner reviewed', seo_metadata: {}, blocks: [{ block_type: 'HeroBlock', content: { headline: 'Private review' }, sort_order: 0 }] }] });
const key = (scope: BuilderScope) => builderDraftKey('site-publication-operation:builder', scope);
const responseReceipt = (review: Review, status = 'pending') => ({ schema_version: 1, user_id: owner.userId, organization_id: owner.tenantId,
  operation_id: review.operation_id, publication_id: PUBLICATION, site_id: SITE, version: 1, status,
  snapshot_sha256: review.snapshot_sha256, snapshot_encoding: 'jcs-rfc8785-v1', public_path: status === 'published' ? '/api/v1/public/sites/' + SITE : null });
let handle: (url: string, options?: RequestInit) => Promise<Response>;
beforeEach(() => {
  const values = new Map<string, string>();
  vi.stubGlobal('localStorage', { getItem: (key: string) => values.get(key) ?? null,
    setItem: (key: string, value: string) => { values.set(key, String(value)); },
    removeItem: (key: string) => { values.delete(key); }, clear: () => values.clear(),
    key: (index: number) => Array.from(values.keys())[index] ?? null, get length() { return values.size; } });
  localStorage.clear(); notifyQueueIdentityChange(); installBuilderLocks(); owner = { userId: 'publisher-a', tenantId: 'workspace-a' };
  handle = async () => Response.json({ error: 'Unexpected request' }, { status: 500 });
  vi.stubGlobal('fetch', vi.fn(async (url, options) => String(url).endsWith('/session-identity')
    ? Response.json({ ...owner, expiresAt: Date.now() + 60_000 }) : handle(String(url), options)));
});
afterEach(() => { vi.unstubAllGlobals(); vi.restoreAllMocks(); });
async function review(scope: BuilderScope) { expect(typeof api.preparePublicationReview).toBe('function'); return api.preparePublicationReview(scope, 'builder', draft()); }
const mutations = () => vi.mocked(fetch).mock.calls.filter(([url, options]) => String(url).startsWith(ROOT) && ['POST', 'DELETE'].includes(String(options?.method)));

it('persists exact reviewed bytes before POST and reports202 as pending until a real published GET', async () => {
  const scope = await openBuilderScope(); const approved = await review(scope);
  handle = async (url, options) => {
    const saved = JSON.parse(localStorage.getItem(key(scope))!);
    expect(saved.operation.snapshot).toEqual(draft()); expect(saved.operation.operation_id).toBe(approved.operation_id);
    if (options?.method === 'POST') { expect(JSON.parse(String(options.body))).toEqual({ operation_id: approved.operation_id, site_id: null, snapshot_encoding: 'jcs-rfc8785-v1', snapshot: draft() }); return Response.json(responseReceipt(approved), { status: 202 }); }
    expect(url).toBe(ROOT + '/operations/' + approved.operation_id); return Response.json(responseReceipt(approved, 'published'));
  };
  const pending = await api.submitPublicationReview(scope, 'builder', approved);
  expect(pending.receipt?.status).toBe('pending'); expect(pending.receipt?.public_path).toBeNull();
  expect((await api.refreshPublicationOperation(scope, 'builder'))?.receipt?.status).toBe('published');
  expect(mutations()).toHaveLength(1);
});
it.each(['network', 'body', 'malformed', 'server503', 'wrong-owner'])('holds unknown %s across reload with no new mutation', async mode => {
  const scope = await openBuilderScope(); const approved = await review(scope);
  handle = async () => {
    if (mode === 'network') throw new Error('Lost network');
    if (mode === 'body') return new Response(new ReadableStream({ start(controller) { controller.error(new Error('Lost body')); } }), { headers: { 'content-type': 'application/json' } });
    return Response.json(mode === 'malformed' ? {} : mode === 'wrong-owner' ? { ...responseReceipt(approved), user_id: 'another' } : { schema_version: 1, error: 'publication_unavailable', effect: 'unknown', message: 'Unavailable' }, { status: mode === 'server503' ? 503 : 202 });
  };
  await expect(api.submitPublicationReview(scope, 'builder', approved)).rejects.toThrow();
  const original = localStorage.getItem(key(scope)); expect(original).not.toBeNull();
  notifyQueueIdentityChange(); const reopened = await openBuilderScope();
  await expect(review(reopened)).rejects.toThrow(); await expect(api.submitPublicationReview(reopened, 'builder', approved)).rejects.toThrow();
  expect(localStorage.getItem(key(reopened))).toBe(original); expect(mutations()).toHaveLength(1);
});
it('keeps404 recovery unknown and only clears it after a matching committed receipt', async () => {
  const scope = await openBuilderScope(); const approved = await review(scope); handle = async () => { throw new Error('Lost response'); };
  await expect(api.submitPublicationReview(scope, 'builder', approved)).rejects.toThrow();
  handle = async () => Response.json({ schema_version: 1, error: 'publication_not_found', effect: 'none', message: 'Missing' }, { status: 404 });
  await expect(api.refreshPublicationOperation(scope, 'builder')).rejects.toThrow(); await expect(review(scope)).rejects.toThrow();
  handle = async () => Response.json(responseReceipt(approved, 'published'));
  expect((await api.refreshPublicationOperation(scope, 'builder'))?.receipt?.status).toBe('published');
  const revised = await review(scope); expect(revised.operation_id).not.toBe(approved.operation_id); expect(revised.site_id).toBe(SITE); expect(mutations()).toHaveLength(1);
});
it.each([[400, 'publication_invalid'], [403, 'publication_forbidden']])('allows a new explicit review only after a verified fresh pre-effect%s', async (status, error) => {
  const scope = await openBuilderScope(); const approved = await review(scope);
  handle = async () => Response.json({ schema_version: 1, error, effect: 'none', message: 'Review fields' }, { status: Number(status) });
  await expect(api.submitPublicationReview(scope, 'builder', approved)).rejects.toThrow();
  expect((await api.readSavedPublicationOperation(scope, 'builder'))?.phase).toBe('rejected');
  expect((await review(scope)).operation_id).not.toBe(approved.operation_id); expect(mutations()).toHaveLength(1);
});
it('holds a lost revoke until the original receipt proves revoked, including a still-published read', async () => {
  const scope = await openBuilderScope(); const approved = await review(scope); handle = async () => Response.json(responseReceipt(approved, 'published'));
  await api.submitPublicationReview(scope, 'builder', approved);
  handle = async (url, options) => { expect(url).toBe(ROOT + '/' + PUBLICATION); expect(options?.method).toBe('DELETE'); throw new Error('Lost revoke'); };
  await expect(api.revokePublication(scope, 'builder')).rejects.toThrow();
  handle = async url => { expect(url).toBe(ROOT + '/operations/' + approved.operation_id); return Response.json(responseReceipt(approved, 'published')); };
  expect((await api.refreshPublicationOperation(scope, 'builder'))?.phase).toBe('revoke_unknown');
  expect(api.publishedSitePath(await api.readSavedPublicationOperation(scope, 'builder'))).toBeNull();
  await expect(api.revokePublication(scope, 'builder')).rejects.toThrow(); await expect(review(scope)).rejects.toThrow(); expect(mutations()).toHaveLength(2);
  handle = async () => Response.json(responseReceipt(approved, 'revoked'));
  expect((await api.refreshPublicationOperation(scope, 'builder'))?.receipt?.status).toBe('revoked');
});
it('does not send a second-tab stale approval after another review has submitted', async () => {
  const scope = await openBuilderScope(); const first = await review(scope); const stale = await review(scope);
  handle = async () => Response.json(responseReceipt(first, 'published'));
  await api.submitPublicationReview(scope, 'builder', first);
  await expect(api.submitPublicationReview(scope, 'builder', stale)).rejects.toThrow(); expect(mutations()).toHaveLength(1);
});
it('fails closed when Web Locks or durable marker storage are unavailable', async () => {
  const scope = await openBuilderScope(); const approved = await review(scope);
  Object.defineProperty(navigator, 'locks', { value: undefined });
  await expect(api.submitPublicationReview(scope, 'builder', approved)).rejects.toThrow(); expect(mutations()).toHaveLength(0);
  installBuilderLocks(); const original = localStorage.setItem.bind(localStorage);
  vi.spyOn(localStorage, 'setItem').mockImplementation((name, value) => { if (name === key(scope)) throw new Error('Storage full'); original(name, value); });
  await expect(api.submitPublicationReview(scope, 'builder', approved)).rejects.toThrow('Storage full'); expect(mutations()).toHaveLength(0);
});
it('retains the pending operation when acknowledgement storage fails', async () => {
  const scope = await openBuilderScope(); const approved = await review(scope); handle = async () => Response.json(responseReceipt(approved, 'published'));
  const original = localStorage.setItem.bind(localStorage); let writes = 0;
  vi.spyOn(localStorage, 'setItem').mockImplementation((name, value) => { if (name === key(scope) && ++writes > 1) throw new Error('Acknowledgement storage full'); original(name, value); });
  await expect(api.submitPublicationReview(scope, 'builder', approved)).rejects.toThrow();
  expect((await api.readSavedPublicationOperation(scope, 'builder'))?.phase).toBe('publish_unknown'); expect(mutations()).toHaveLength(1);
});
it('holds corrupt persisted state rather than coercing an enum and overwriting the original', async () => {
  const scope = await openBuilderScope(); const approved = await review(scope); handle = async () => { throw new Error('Lost response'); };
  await expect(api.submitPublicationReview(scope, 'builder', approved)).rejects.toThrow();
  const saved = JSON.parse(localStorage.getItem(key(scope))!); saved.phase = ['publish_unknown']; const corrupt = JSON.stringify(saved); localStorage.setItem(key(scope), corrupt);
  await expect(review(scope)).rejects.toThrow(); expect(localStorage.getItem(key(scope))).toBe(corrupt); expect(mutations()).toHaveLength(1);
});
it('holds a duplicate-key operation record instead of interpreting its last phase as a rejection', async () => {
  const scope = await openBuilderScope(); const approved = await review(scope); handle = async () => { throw new Error('Lost response'); };
  await expect(api.submitPublicationReview(scope, 'builder', approved)).rejects.toThrow();
  const corrupt = localStorage.getItem(key(scope))!.replace('"phase":"publish_unknown"', '"phase":"publish_unknown","phase":"rejected","rejection":"publication_invalid"');
  localStorage.setItem(key(scope), corrupt);
  await expect(review(scope)).rejects.toThrow(); expect(localStorage.getItem(key(scope))).toBe(corrupt); expect(mutations()).toHaveLength(1);
});
it('keeps a prior owner operation private and ignores its delayed response after account replacement', async () => {
  const a = await openBuilderScope(); const approved = await review(a); let finish!: (response: Response) => void;
  handle = async () => new Promise<Response>(resolve => { finish = resolve; });
  const pending = api.submitPublicationReview(a, 'builder', approved).catch(error => error); await vi.waitFor(() => expect(finish).toBeDefined());
  const original = localStorage.getItem(key(a)); const aKey = key(a);
  owner = { userId: 'publisher-b', tenantId: 'workspace-b' }; notifyQueueIdentityChange(); const b = await openBuilderScope();
  expect(await api.readSavedPublicationOperation(b, 'builder')).toBeNull();
  finish(Response.json({ ...responseReceipt(approved, 'published'), user_id: 'publisher-a', organization_id: 'workspace-a' }));
  expect(await pending).toBeInstanceOf(Error); expect(localStorage.getItem(aKey)).toBe(original); expect(localStorage.getItem(key(b))).toBeNull();
});
it('does not consume a stalled denied response body after401 retires the owner', async () => {
  const scope = await openBuilderScope(); const approved = await review(scope); let release!: () => void;
  const response = new Response(new ReadableStream({ start(controller) { release = () => controller.close(); } }), { status: 401, headers: { 'content-type': 'application/json' } });
  const body = vi.spyOn(response.body!, 'getReader');
  handle = async () => response;
  const attempt = api.submitPublicationReview(scope, 'builder', approved).catch(error => error);
  await vi.waitFor(() => expect(onboardingOwner()).toBeNull());
  try { expect(body).not.toHaveBeenCalled(); } finally { release?.(); await attempt; }
});
it('does not overwrite a publication when another same-owner tab is in flight', async () => {
  const scope = await openBuilderScope(); const first = await review(scope); const second = await review(scope); let finish!: () => void;
  handle = async () => new Promise<Response>(resolve => { finish = () => resolve(Response.json(responseReceipt(first), { status: 202 })); });
  const firstRequest = api.submitPublicationReview(scope, 'builder', first);
  await vi.waitFor(() => expect(finish).toBeDefined());
  const original = localStorage.getItem(key(scope));
  await expect(api.submitPublicationReview(scope, 'builder', second)).rejects.toThrow();
  expect(localStorage.getItem(key(scope))).toBe(original); expect(mutations()).toHaveLength(1); finish(); await firstRequest;
  await expect(review(scope)).rejects.toThrow();
});
it('retains unknown publication state after a later definitive role denial during recovery', async () => {
  const scope = await openBuilderScope(); const approved = await review(scope); handle = async () => { throw new Error('Lost response'); };
  await expect(api.submitPublicationReview(scope, 'builder', approved)).rejects.toThrow();
  const original = localStorage.getItem(key(scope));
  handle = async () => Response.json({ schema_version: 1, error: 'publication_forbidden', effect: 'none', message: 'Role revoked' }, { status: 403 });
  await expect(api.refreshPublicationOperation(scope, 'builder')).rejects.toThrow();
  expect(localStorage.getItem(key(scope))).toBe(original); await expect(review(scope)).rejects.toThrow(); expect(mutations()).toHaveLength(1);
});
it('holds an old view after a canonical owner contradiction without a separate auth event', async () => {
  const scope = await openBuilderScope(); const approved = await review(scope);
  owner = { userId: 'publisher-b', tenantId: 'workspace-b' }; await readQueueOwner();
  await expect(api.submitPublicationReview(scope, 'builder', approved)).rejects.toThrow(); expect(mutations()).toHaveLength(0);
});
it('retains the durable marker when its same-owner view closes before a late receipt', async () => {
  const scope = await openBuilderScope(); const approved = await review(scope); let active = true; let finish!: () => void;
  handle = async () => new Promise<Response>(resolve => { finish = () => resolve(Response.json(responseReceipt(approved, 'published'))); });
  const attempt = api.submitPublicationReview(scope, 'builder', approved, () => active).catch(error => error);
  await vi.waitFor(() => expect(finish).toBeDefined()); const original = localStorage.getItem(key(scope)); active = false; finish();
  expect(await attempt).toBeInstanceOf(Error); expect(localStorage.getItem(key(scope))).toBe(original);
  expect((await api.readSavedPublicationOperation(scope, 'builder'))?.phase).toBe('publish_unknown');
});
it.each(['submit', 'read', 'revoke'])('holds duplicate status keys in the actual %s response bytes', async action => {
  const scope = await openBuilderScope(); const approved = await review(scope);
  handle = async () => Response.json(responseReceipt(approved), { status: 202 });
  if (action !== 'submit') await api.submitPublicationReview(scope, 'builder', approved);
  const terminal = action === 'revoke' ? 'revoked' : 'published';
  const raw = JSON.stringify(responseReceipt(approved, terminal)).replace('"status":', '"status":"failed","status":');
  handle = async () => new Response(raw, { status: 200, headers: { 'content-type': 'application/json' } });
  const attempt = action === 'submit' ? api.submitPublicationReview(scope, 'builder', approved) : action === 'read' ? api.refreshPublicationOperation(scope, 'builder') : api.revokePublication(scope, 'builder');
  await expect(attempt).rejects.toThrow();
  const saved = await api.readSavedPublicationOperation(scope, 'builder'); expect(api.publishedSitePath(saved)).toBeNull();
  expect(saved?.phase).toBe(action === 'revoke' ? 'revoke_unknown' : action === 'submit' ? 'publish_unknown' : 'acknowledged');
});
it.each([[400, 'publication_invalid'], [403, 'publication_forbidden']])('does not release a marker after contradictory effect keys in HTTP%s', async (status, error) => {
  const scope = await openBuilderScope(); const approved = await review(scope); const ownerKey = key(scope);
  const raw = JSON.stringify({ schema_version: 1, error, effect: 'none', message: 'Rejected' }).replace('"effect":', '"effect":"unknown","effect":');
  handle = async () => new Response(raw, { status: Number(status), headers: { 'content-type': 'application/json' } });
  await expect(api.submitPublicationReview(scope, 'builder', approved)).rejects.toThrow();
  expect(JSON.parse(localStorage.getItem(ownerKey)!).phase).toBe('publish_unknown'); expect(mutations()).toHaveLength(1);
});
it('can restore a maximum-depth wire-valid snapshot from its deeper local operation envelope', async () => {
  const scope = await openBuilderScope();
  let metadata: Record<string, unknown> = { value: 'deep reviewed metadata' };
  // Snapshot root, pages array and page object add three containers; metadata adds28.
  for (let i = 0; i < 27; i++) metadata = { nested: metadata };
  const snapshot = draft();
  const approved = await api.preparePublicationReview(scope, 'builder', { ...snapshot, pages: [{ ...snapshot.pages[0], seo_metadata: metadata }] });
  handle = async () => Response.json(responseReceipt(approved), { status: 202 });
  await api.submitPublicationReview(scope, 'builder', approved);
  expect((await api.readSavedPublicationOperation(scope, 'builder'))?.operation.snapshot_sha256).toBe(approved.snapshot_sha256);
});
