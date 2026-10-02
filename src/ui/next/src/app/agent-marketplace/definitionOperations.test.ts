import { createHash } from 'node:crypto';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { openBuilderScope, builderDraftKey } from '../builder/ownedDraft';
import { installBuilderLocks } from '../builder/testLocks';
import { executeDefinitionOperation, readSavedDefinitionOperation, recoverDefinitionOperation } from './definitionOperations';
import type { Operation } from './definitions';

let owner = { userId: 'owner-a', tenantId: 'tenant-a' };
const root = '/api/v1/agents/definitions';
const operation: Operation = { kind: 'publish', request_id: '10000000-0000-4000-8000-000000000001', publication: { name: 'Reviewed helper', description: 'Public text', role: 'Writer', system_prompt: 'Write a draft for review.', visibility: 'public' } };
function receipt(op = operation, replayed = false) {
  if (op.kind !== 'publish') throw new Error('Fixture requires a publication');
  const definition = { ...op.publication, id: '20000000-0000-4000-8000-000000000001', version: 1, source: 'community', digest: '' };
  definition.digest = createHash('sha256').update(JSON.stringify([definition.name, definition.description, definition.role, definition.system_prompt, definition.visibility, definition.source, definition.version])).digest('hex');
  return { success: true, status: 'published', request_id: op.request_id, organization_id: owner.tenantId, user_id: owner.userId, replayed, definition };
}
const posts = () => vi.mocked(fetch).mock.calls.filter(([url, options]) => String(url).startsWith(root) && options?.method === 'POST');
let api: (url: string, options?: RequestInit) => Promise<Response>;
beforeEach(() => {
  localStorage.clear(); notifyQueueIdentityChange(); installBuilderLocks(); owner = { userId: 'owner-a', tenantId: 'tenant-a' };
  api = async () => Response.json(receipt());
  vi.stubGlobal('fetch', vi.fn(async (url: string, options?: RequestInit) => url.endsWith('/session-identity') ? Response.json({ ...owner, expiresAt: Date.now() + 60_000 }) : api(url, options)));
});
afterEach(() => { vi.unstubAllGlobals(); vi.restoreAllMocks(); });
it('persists the immutable owner operation before the actual POST and records only its verified receipt', async () => {
  const scope = await openBuilderScope();
  api = async () => {
    const saved = JSON.parse(localStorage.getItem(builderDraftKey('agent-definition-operation', scope))!);
    expect(saved).toMatchObject({ format: 1, state: 'pending', operation });
    return Response.json(receipt());
  };
  expect((await executeDefinitionOperation(scope, operation)).status).toBe('published');
  expect((await readSavedDefinitionOperation(scope))?.state).toBe('confirmed');
  expect(posts()).toHaveLength(1);
});
it('holds lost replies across reopening and recovers by GET without another publication', async () => {
  const scope = await openBuilderScope(); api = async () => { throw new Error('Lost response'); };
  await expect(executeDefinitionOperation(scope, operation)).rejects.toThrow();
  expect((await readSavedDefinitionOperation(scope))?.state).toBe('pending');
  notifyQueueIdentityChange(); const reopened = await openBuilderScope();
  await expect(executeDefinitionOperation(reopened, operation)).rejects.toThrow(); expect(posts()).toHaveLength(1);
  api = async url => { expect(url).toBe(root + '/operations/' + operation.request_id); return Response.json(receipt(operation, true)); };
  expect((await recoverDefinitionOperation(reopened))?.state).toBe('confirmed'); expect(posts()).toHaveLength(1);
});
it('keeps a404 recovery held and replays only the same ID and bytes after an explicit retry', async () => {
  const scope = await openBuilderScope(); api = async () => { throw new Error('Lost response'); };
  await expect(executeDefinitionOperation(scope, operation)).rejects.toThrow();
  api = async () => Response.json({ success: false, reason: 'operation_not_found' }, { status: 404 });
  expect((await recoverDefinitionOperation(scope))?.state).toBe('pending');
  const changed: Operation = { ...operation, request_id: '10000000-0000-4000-8000-000000000002' };
  await expect(executeDefinitionOperation(scope, changed, true)).rejects.toThrow();
  api = async () => Response.json(receipt(operation, true));
  await executeDefinitionOperation(scope, operation, true);
  expect(posts()).toHaveLength(2);
  expect(posts()[0][1]?.body).toBe(posts()[1][1]?.body);
});
it('allows a newly reviewed request after the exact pre-mutation validation rejection', async () => {
  const scope = await openBuilderScope(); api = async () => Response.json({ success: false, reason: 'invalid_request' }, { status: 400 });
  await expect(executeDefinitionOperation(scope, operation)).rejects.toThrow();
  expect((await readSavedDefinitionOperation(scope))?.state).toBe('rejected');
  const changed: Operation = { kind: 'publish', request_id: '10000000-0000-4000-8000-000000000002', publication: { ...operation.publication, name: 'Corrected helper' } };
  api = async () => Response.json(receipt(changed));
  expect((await executeDefinitionOperation(scope, changed)).status).toBe('published');
});
it('coordinates two same-owner views without dispatching the second request concurrently', async () => {
  const first = await openBuilderScope(); const second = await openBuilderScope();
  let finish!: (response: Response) => void; api = () => new Promise(resolve => { finish = resolve; });
  const pending = executeDefinitionOperation(first, operation);
  await vi.waitFor(() => expect(posts()).toHaveLength(1));
  await expect(executeDefinitionOperation(second, operation)).rejects.toThrow();
  expect(posts()).toHaveLength(1); finish(Response.json(receipt())); await pending;
});
it('does not dispatch when the preflight marker cannot be stored', async () => {
  const scope = await openBuilderScope(); const key = builderDraftKey('agent-definition-operation', scope);
  const storage = vi.mocked(localStorage.setItem); const original = storage.getMockImplementation()!;
  storage.mockImplementation((name, value) => { if (name === key) throw new Error('Storage unavailable'); original(name, value); });
  try { await expect(executeDefinitionOperation(scope, operation)).rejects.toThrow(); expect(posts()).toHaveLength(0); }
  finally { storage.mockImplementation(original); }
});
it('holds operations when origin locks are unavailable', async () => {
  const scope = await openBuilderScope(); Object.defineProperty(navigator, 'locks', { value: undefined });
  await expect(executeDefinitionOperation(scope, operation)).rejects.toThrow(); expect(posts()).toHaveLength(0);
});
it('does not expose or apply a late prior-owner receipt after switching accounts', async () => {
  const first = await openBuilderScope(); const key = builderDraftKey('agent-definition-operation', first);
  let finish!: (response: Response) => void; const reply = receipt(); api = () => new Promise(resolve => { finish = resolve; });
  const pending = executeDefinitionOperation(first, operation);
  await vi.waitFor(() => expect(posts()).toHaveLength(1));
  owner = { userId: 'owner-b', tenantId: 'tenant-b' }; notifyQueueIdentityChange(); const next = await openBuilderScope();
  finish(Response.json(reply)); await expect(pending).rejects.toThrow();
  expect(await readSavedDefinitionOperation(next)).toBeNull();
  expect(JSON.parse(localStorage.getItem(key)!).state).toBe('pending'); expect(posts()).toHaveLength(1);
});

it('holds a formerly confirmed request when authoritative recovery no longer finds its receipt', async () => {
  const scope = await openBuilderScope(); await executeDefinitionOperation(scope, operation);
  api = async () => Response.json({ success: false, reason: 'operation_not_found' }, { status: 404 });
  expect((await recoverDefinitionOperation(scope))?.state).toBe('pending');
  expect((await readSavedDefinitionOperation(scope))?.state).toBe('pending');
  const next: Operation = { ...operation, request_id: '10000000-0000-4000-8000-000000000002' };
  await expect(executeDefinitionOperation(scope, next)).rejects.toThrow();
  expect(posts()).toHaveLength(1);
});
it('does not overwrite a changed saved operation during identity preflight', async () => {
  const scope = await openBuilderScope(); const key = builderDraftKey('agent-definition-operation', scope);
  let release!: (response: Response) => void;
  vi.mocked(fetch).mockImplementation(async (url: string) => url.endsWith('/session-identity') ? new Promise(resolve => { release = resolve; }) : Response.json(receipt()));
  const pending = executeDefinitionOperation(scope, operation); const rejected = expect(pending).rejects.toThrow();
  await vi.waitFor(() => expect(release).toBeTypeOf('function'));
  localStorage.setItem(key, 'another saved operation remains held');
  release(Response.json({ ...owner, expiresAt: Date.now() + 60000 }));
  await rejected;
  expect(localStorage.getItem(key)).toBe('another saved operation remains held'); expect(posts()).toHaveLength(0);
});
it('does not dispatch a delayed preflight after its view is retired', async () => {
  const scope = await openBuilderScope(); let active = true; let release!: (response: Response) => void;
  vi.mocked(fetch).mockImplementation(async (url: string) => url.endsWith('/session-identity') ? new Promise(resolve => { release = resolve; }) : Response.json(receipt()));
  const pending = executeDefinitionOperation(scope, operation, false, () => active);
  const rejected = expect(pending).rejects.toThrow();
  await vi.waitFor(() => expect(release).toBeTypeOf('function')); active = false;
  release(Response.json({ ...owner, expiresAt: Date.now() + 60000 })); await rejected;
  expect(posts()).toHaveLength(0); expect(await readSavedDefinitionOperation(scope)).toBeNull();
});
it.each([[400, 'invalid_request'], [403, 'owner_or_admin_required']] as const)('keeps an earlier unknown operation held after a replay gets%s', async (status, reason) => {
  const scope = await openBuilderScope(); api = async () => { throw new Error('Original reply was lost'); };
  await expect(executeDefinitionOperation(scope, operation)).rejects.toThrow();
  api = async () => Response.json({ success: false, reason }, { status });
  await expect(executeDefinitionOperation(scope, operation, true)).rejects.toThrow();
  expect((await readSavedDefinitionOperation(scope))?.state).toBe('pending');
  await expect(executeDefinitionOperation(scope, { ...operation, request_id: '10000000-0000-4000-8000-000000000002' })).rejects.toThrow();
  expect(posts()).toHaveLength(2);
  api = async () => Response.json(receipt(operation, true));
  expect((await recoverDefinitionOperation(scope))?.state).toBe('confirmed');
});
it('never replaces a confirmed operation with a fresh mutation using the same ID', async () => {
  const scope = await openBuilderScope(); await executeDefinitionOperation(scope, operation);
  api = async () => Response.json({ success: false, reason: 'invalid_request' }, { status: 400 });
  await expect(executeDefinitionOperation(scope, operation)).rejects.toThrow();
  expect((await readSavedDefinitionOperation(scope))?.state).toBe('confirmed'); expect(posts()).toHaveLength(1);
});
it.each([['pending'], ['rejected']])('holds a non-string saved state without overwriting it: %j', async state => {
  const scope = await openBuilderScope(); const key = builderDraftKey('agent-definition-operation', scope);
  const original = JSON.stringify({ format: 1, owner, operation, state: [state], rejection: 'invalid_request' });
  localStorage.setItem(key, original);
  const next: Operation = { ...operation, request_id: '10000000-0000-4000-8000-000000000002' };
  await expect(executeDefinitionOperation(scope, next)).rejects.toThrow();
  expect(posts()).toHaveLength(0); expect(localStorage.getItem(key)).toBe(original);
});
it('holds a malformed rejection enum rather than treating it as permission for a new request', async () => {
  const scope = await openBuilderScope(); const key = builderDraftKey('agent-definition-operation', scope);
  const original = JSON.stringify({ format: 1, owner, operation, state: 'rejected', rejection: ['invalid_request'] });
  localStorage.setItem(key, original);
  await expect(executeDefinitionOperation(scope, { ...operation, request_id: '10000000-0000-4000-8000-000000000002' })).rejects.toThrow();
  expect(posts()).toHaveLength(0); expect(localStorage.getItem(key)).toBe(original);
});
