import { assertBuilderScope, builderDraftKey, type BuilderScope } from '../builder/ownedDraft';
import { fetchForOwnedDefinition } from '../onboarding/draftSession';
import { canonicalRequest } from '../onboarding/contracts';
import { sameOwner, type QueueOwner } from '@/lib/sync/queueIdentity';
import { definitiveDefinitionRejection, isDefinitionId, readDefinition, readDefinitionReceipt, readPublication, type Operation, type Receipt } from './definitions';

export type SavedOperation = { format: 1; owner: QueueOwner; operation: Operation; state: 'pending' | 'confirmed' | 'rejected'; receipt?: Receipt; rejection?: string };
const ROOT = '/api/v1/agents/definitions';
const KEY = 'agent-definition-operation';
function record(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('The previous agent operation remains held because its saved data is invalid');
  return value as Record<string, unknown>;
}
async function checkedOperation(value: unknown): Promise<Operation> {
  const data = record(JSON.parse(JSON.stringify(value)));
  if (!isDefinitionId(data.request_id)) throw new Error('The agent request identity is invalid');
  if (data.kind === 'publish' && Object.keys(data).every(key => ['kind', 'request_id', 'publication'].includes(key))) {
    return { kind: 'publish', request_id: data.request_id, publication: readPublication(data.publication) };
  }
  if (data.kind === 'install' && Object.keys(data).every(key => ['kind', 'request_id', 'definition'].includes(key))) {
    return { kind: 'install', request_id: data.request_id, definition: await readDefinition(data.definition) };
  }
  throw new Error('The agent operation is invalid');
}
export async function readSavedDefinitionOperation(scope: BuilderScope): Promise<SavedOperation | null> {
  assertBuilderScope(scope);
  const key = builderDraftKey(KEY, scope); const raw = localStorage.getItem(key);
  if (raw === null) return null;
  if (raw.length > 262144) throw new Error('The saved agent operation is too large and remains held');
  const data = record(JSON.parse(raw)); const storedOwner = record(data.owner);
  if (data.format !== 1 || typeof storedOwner.userId !== 'string' || typeof storedOwner.tenantId !== 'string'
    || !sameOwner(scope.owner, { userId: storedOwner.userId, tenantId: storedOwner.tenantId })
    || typeof data.state !== 'string' || !['pending', 'confirmed', 'rejected'].includes(data.state)) throw new Error('The previous agent operation belongs to a different or invalid session and remains held');
  const operation = await checkedOperation(data.operation);
  const saved: SavedOperation = { format: 1, owner: { ...scope.owner }, operation, state: data.state as SavedOperation['state'] };
  if (saved.state === 'confirmed') saved.receipt = await readDefinitionReceipt(200, data.receipt, operation, scope.owner);
  if (saved.state === 'rejected') {
    if (typeof data.rejection !== 'string' || !['invalid_request', 'owner_or_admin_required'].includes(data.rejection)) throw new Error('The previous rejection remains unverified');
    saved.rejection = data.rejection;
  }
  assertBuilderScope(scope);
  if (localStorage.getItem(key) !== raw) throw new Error('The agent operation changed in another view. Check its status again.');
  return saved;
}
async function locked<T>(scope: BuilderScope, work: () => Promise<T>): Promise<T> {
  assertBuilderScope(scope);
  if (!navigator.locks?.request) throw new Error('This browser cannot coordinate safe agent writes. Your request remains held.');
  return navigator.locks.request(builderDraftKey(KEY, scope) + ':request', { mode: 'exclusive', ifAvailable: true }, async lock => {
    if (!lock) throw new Error('Another agent operation is active. Check its status before trying again.');
    assertBuilderScope(scope); return work();
  });
}
function saveCompared(scope: BuilderScope, expected: string, value: SavedOperation) {
  assertBuilderScope(scope); const key = builderDraftKey(KEY, scope);
  if (localStorage.getItem(key) !== expected) throw new Error('The agent operation changed. Its outcome remains held.');
  localStorage.setItem(key, JSON.stringify(value));
}
export async function executeDefinitionOperation(scope: BuilderScope, input: Operation, replay = false, active: () => boolean = () => true): Promise<Receipt> {
  assertBuilderScope(scope);
  if (!active()) throw new Error('This view was closed before the agent request was sent');
  const operation = await checkedOperation(input);
  return locked(scope, async () => {
    if (!active()) throw new Error('This view was closed before the agent request was sent');
    const previousRaw = localStorage.getItem(builderDraftKey(KEY, scope));
    const previous = await readSavedDefinitionOperation(scope);
    if (previous && previous.operation.request_id === operation.request_id && previous.state !== 'pending') throw new Error('This request already has a recorded outcome. Check its receipt, or review a new request.');
    if (previous?.state === 'pending' && (!replay || canonicalRequest(previous.operation) !== canonicalRequest(operation))) throw new Error('A previous agent request is unconfirmed. Check its status or explicitly retry those exact reviewed fields.');
    if (replay && (previous?.state !== 'pending' || canonicalRequest(previous.operation) !== canonicalRequest(operation))) throw new Error('Only the exact pending request can be retried');
    const pending: SavedOperation = { format: 1, owner: { ...scope.owner }, operation, state: 'pending' };
    const raw = JSON.stringify(pending);
    const body = operation.kind === 'publish'
      ? { request_id: operation.request_id, ...operation.publication }
      : { request_id: operation.request_id, version: operation.definition.version, digest: operation.definition.digest };
    const url = operation.kind === 'publish' ? ROOT : ROOT + '/' + operation.definition.id + '/install';
    const response = await fetchForOwnedDefinition(url, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) }, scope.owner, () => {
      assertBuilderScope(scope);
      if (!active()) throw new Error('This view was closed before the agent request was sent');
      const key = builderDraftKey(KEY, scope);
      if (localStorage.getItem(key) !== previousRaw) throw new Error('The saved agent operation changed before dispatch and remains held');
      localStorage.setItem(key, raw);
      if (!active()) throw new Error('This view was closed before the agent request was sent');
    });
    const value: unknown = await response.json(); assertBuilderScope(scope);
    if (definitiveDefinitionRejection(response.status, value)) {
      if (previous?.state === 'pending') throw new Error('The retry was rejected, but the earlier request remains unconfirmed. Check its saved receipt.');
      const rejection = String(record(value).reason);
      saveCompared(scope, raw, { ...pending, state: 'rejected', rejection });
      throw new Error(rejection === 'owner_or_admin_required' ? 'Only an owner or administrator may publish or install definitions.' : 'The request was rejected before saving. Review the fields before submitting a new request.');
    }
    const receipt = await readDefinitionReceipt(response.status, value, operation, scope.owner);
    saveCompared(scope, raw, { ...pending, state: 'confirmed', receipt });
    return receipt;
  });
}
export async function recoverDefinitionOperation(scope: BuilderScope): Promise<SavedOperation | null> {
  return locked(scope, async () => {
    const saved = await readSavedDefinitionOperation(scope);
    if (!saved || saved.state === 'rejected') return saved;
    const raw = localStorage.getItem(builderDraftKey(KEY, scope))!;
    const response = await fetchForOwnedDefinition(ROOT + '/operations/' + saved.operation.request_id, { method: 'GET' }, scope.owner);
    const value: unknown = await response.json(); assertBuilderScope(scope);
    if (response.status === 404) {
      const pending: SavedOperation = { ...saved, state: 'pending' };
      saveCompared(scope, raw, pending);
      return { ...pending, receipt: undefined };
    }
    const receipt = await readDefinitionReceipt(response.status, value, saved.operation, scope.owner);
    const confirmed: SavedOperation = { ...saved, state: 'confirmed', receipt };
    saveCompared(scope, raw, confirmed); return confirmed;
  });
}
