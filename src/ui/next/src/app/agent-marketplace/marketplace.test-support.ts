import { createHash, randomUUID } from 'node:crypto';
import { vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installBuilderLocks } from '../builder/testLocks';
import type { Definition, Installation, Publication, Receipt } from './definitions';

export const fields: Publication = { name: 'Reviewed Writer', description: 'Drafts for owner review', role: 'Writer', system_prompt: 'Write a draft.\nWait for owner approval before sending.', visibility: 'public' };
export function definition(value: Partial<Definition> = {}): Definition {
  const saved: Definition = { ...fields, id: randomUUID(), version: 1, source: 'community', digest: '', ...value };
  saved.digest = createHash('sha256').update(JSON.stringify([saved.name, saved.description, saved.role, saved.system_prompt, saved.visibility, saved.source, saved.version])).digest('hex');
  return saved;
}
/** Explicit unit-test backend; native tests separately prove the actual DB contract. */
export function marketplaceBackend() {
  localStorage.clear(); notifyQueueIdentityChange(); installBuilderLocks();
  const state = {
    owner: { userId: 'owner-a', tenantId: 'tenant-a' }, roleAllowed: true, pageSize: 50,
    definitions: [definition({ name: 'Senior Rust Developer', role: 'Rust Developer', source: 'first_party' }), definition({ name: 'Technical Writer', source: 'first_party' })],
    installations: new Map<string, Installation[]>(), receipts: new Map<string, Receipt>(),
  };
  const ownerKey = () => JSON.stringify(state.owner);
  const route = async (url: string, options: RequestInit = {}): Promise<Response> => {
    if (url === '/api/v1/auth/session-identity') return Response.json({ ...state.owner, expiresAt: Date.now() + 60000 });
    const parsed = new URL(url, 'https://workspace.test'); const method = options.method ?? 'GET';
    const headers = new Headers(options.headers);
    if (headers.get('x-ohc-expected-user') !== state.owner.userId || headers.get('x-ohc-expected-tenant') !== state.owner.tenantId) return Response.json({ success: false, reason: 'session_identity_changed' }, { status: 409 });
    if (parsed.pathname === '/api/v1/agents/definitions' && method === 'GET') {
      const q = (parsed.searchParams.get('q') || '').toLowerCase();
      const matches = state.definitions.filter(item => (item.name + ' ' + item.description + ' ' + item.role).toLowerCase().includes(q));
      const own = state.installations.get(ownerKey()) ?? [];
      const offset = Number(parsed.searchParams.get('cursor')?.split(':')[1] ?? 0);
      const installationOffset = Number(parsed.searchParams.get('installation_cursor')?.split(':')[1] ?? 0);
      return Response.json({ definitions: matches.slice(offset, offset + state.pageSize), installations: own.slice(installationOffset, installationOffset + state.pageSize),
        next_cursor: offset + state.pageSize < matches.length ? `public:${offset + state.pageSize}` : null,
        next_installation_cursor: installationOffset + state.pageSize < own.length ? `installed:${installationOffset + state.pageSize}` : null });
    }
    if (parsed.pathname.startsWith('/api/v1/agents/definitions/operations/') && method === 'GET') {
      const saved = state.receipts.get(ownerKey() + ':' + parsed.pathname.split('/').pop());
      return saved ? Response.json({ ...saved, replayed: true }) : Response.json({ success: false, reason: 'operation_not_found' }, { status: 404 });
    }
    if (method !== 'POST' || !parsed.pathname.startsWith('/api/v1/agents/definitions')) throw new Error(`Unexpected unit-test request ${method} ${url}`);
    if (!state.roleAllowed) return Response.json({ success: false, reason: 'owner_or_admin_required' }, { status: 403 });
    const body = JSON.parse(String(options.body)); const receiptKey = ownerKey() + ':' + body.request_id;
    const previous = state.receipts.get(receiptKey); if (previous) return Response.json({ ...previous, replayed: true });
    const common = { success: true as const, request_id: body.request_id, organization_id: state.owner.tenantId, user_id: state.owner.userId, replayed: false };
    let saved: Receipt;
    if (parsed.pathname.endsWith('/install')) {
      const id = parsed.pathname.split('/').at(-2); const selected = state.definitions.find(item => item.id === id && item.version === body.version && item.digest === body.digest);
      if (!selected) return Response.json({ success: false, reason: 'request_or_definition_conflict' }, { status: 409 });
      const own = state.installations.get(ownerKey()) ?? [];
      let installation = own.find(item => item.definition_id === selected.id && item.version === selected.version);
      if (!installation) {
        const installationId = randomUUID();
        installation = { id: installationId, definition_id: selected.id, version: selected.version, digest: selected.digest, role_key: `marketplace/${installationId}/agent`, name: selected.name, role: selected.role, system_prompt: selected.system_prompt, status: 'installed_inactive' };
        state.installations.set(ownerKey(), [...own, installation]);
      }
      saved = { ...common, status: 'installed_inactive', installation };
    } else {
      const published = definition({ name: body.name, description: body.description, role: body.role, system_prompt: body.system_prompt, visibility: body.visibility });
      state.definitions.push(published); saved = { ...common, status: 'published', definition: published };
    }
    state.receipts.set(receiptKey, saved); return Response.json(saved);
  };
  const fetch = vi.fn(route); vi.stubGlobal('fetch', fetch);
  return { state, route, fetch, posts: () => fetch.mock.calls.filter(([url, options]) => url.startsWith('/api/v1/agents/definitions') && options?.method === 'POST') };
}
