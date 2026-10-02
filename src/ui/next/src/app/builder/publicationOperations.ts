import { assertBuilderScope, builderDraftKey, type BuilderScope } from './ownedDraft';
import { fetchForOwnedPublication } from '../onboarding/draftSession';
import { hasVerifiedOfflineQueueOwner, sameOwner } from '@/lib/sync/queueIdentity';
import { isPublicationId, parseSavedPublicationJson, prepareSiteSnapshot, readPublicationResponse, readSitePublicationReceipt, SITE_SNAPSHOT_ENCODING, type SiteSnapshot, type SitePublicationBinding, type SitePublicationReceipt } from './publicationContracts';

export type PublicationChannel = 'builder' | 'website-builder' | 'storefront-builder' | 'brand-studio' | 'public-bio';
export type SitePublicationOperation = SitePublicationBinding & { snapshot: SiteSnapshot };
export type PublicationReview = SitePublicationOperation & { previous_sha256: string | null };
export type SavedPublicationOperation = {
  format: 1; phase: 'publish_unknown' | 'acknowledged' | 'rejected' | 'revoke_unknown';
  operation: SitePublicationOperation; receipt?: SitePublicationReceipt; rejection?: string;
};
const ROOT = '/api/v1/builder/publications';
function assertActive(scope: BuilderScope, active = () => true) {
  assertBuilderScope(scope);
  if (!active()) throw new Error('This publication view was closed. Check the saved operation when you return.');
  if (!hasVerifiedOfflineQueueOwner(scope.owner)) throw new Error('Publication access is awaiting current owner verification.');
}
export function publicationOperationKey(scope: BuilderScope, channel: PublicationChannel): string {
  if (!['builder', 'website-builder', 'storefront-builder', 'brand-studio', 'public-bio'].includes(channel)) throw new Error('Invalid publication channel.');
  return builderDraftKey('site-publication-operation:' + channel, scope);
}
function record(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('The saved publication is unreadable and remains held.');
  return value as Record<string, unknown>;
}
async function digest(raw: string | null): Promise<string | null> {
  if (raw === null) return null;
  return Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256', new TextEncoder().encode(raw))), byte => byte.toString(16).padStart(2, '0')).join('');
}
async function checkedOperation(value: unknown, scope: BuilderScope): Promise<SitePublicationOperation> {
  const data = record(value); const owner = record(data.owner);
  if (Object.keys(data).some(key => !['owner', 'operation_id', 'site_id', 'snapshot_sha256', 'snapshot_encoding', 'snapshot'].includes(key))
    || typeof owner.userId !== 'string' || typeof owner.tenantId !== 'string' || !sameOwner(scope.owner, { userId: owner.userId, tenantId: owner.tenantId })
    || !isPublicationId(data.operation_id) || (data.site_id !== null && !isPublicationId(data.site_id))
    || data.snapshot_encoding !== SITE_SNAPSHOT_ENCODING) throw new Error('The saved publication identity is invalid and remains held.');
  const prepared = await prepareSiteSnapshot(data.snapshot);
  if (data.snapshot_sha256 !== prepared.snapshot_sha256) throw new Error('The saved publication content changed and remains held.');
  return { owner: { ...scope.owner }, operation_id: data.operation_id, site_id: isPublicationId(data.site_id) ? data.site_id : null,
    snapshot: prepared.snapshot, snapshot_encoding: prepared.snapshot_encoding, snapshot_sha256: prepared.snapshot_sha256 };
}
export async function readSavedPublicationOperation(scope: BuilderScope, channel: PublicationChannel): Promise<SavedPublicationOperation | null> {
  assertActive(scope); const key = publicationOperationKey(scope, channel); const raw = localStorage.getItem(key);
  if (raw === null) return null;
  if (raw.length > 2 * 1024 * 1024) throw new Error('The saved publication is too large and remains held.');
  const data = record(parseSavedPublicationJson(raw));
  if (data.format !== 1 || typeof data.phase !== 'string' || !['publish_unknown', 'acknowledged', 'rejected', 'revoke_unknown'].includes(data.phase)
    || Object.keys(data).some(key => !['format', 'phase', 'operation', 'receipt', 'rejection'].includes(key))) throw new Error('The saved publication state is invalid and remains held.');
  const operation = await checkedOperation(data.operation, scope);
  const saved: SavedPublicationOperation = { format: 1, phase: data.phase as SavedPublicationOperation['phase'], operation };
  if (data.receipt !== undefined) saved.receipt = readSitePublicationReceipt(200, data.receipt, operation, 'read');
  if (['acknowledged', 'revoke_unknown'].includes(saved.phase) && !saved.receipt) throw new Error('The saved publication receipt is missing and remains held.');
  if (saved.phase === 'rejected') {
    if (typeof data.rejection !== 'string' || !['publication_invalid', 'publication_forbidden', 'publication_not_found'].includes(data.rejection)) throw new Error('The prior rejection could not be verified.');
    saved.rejection = data.rejection;
  }
  assertActive(scope);
  if (localStorage.getItem(key) !== raw) throw new Error('The saved publication changed in another view. Check its status again.');
  return saved;
}
function blocked(saved: SavedPublicationOperation | null): boolean {
  return !!saved && (saved.phase === 'publish_unknown' || saved.phase === 'revoke_unknown'
    || saved.receipt?.status === 'pending' || saved.receipt?.status === 'processing');
}
export function publishedSitePath(saved: SavedPublicationOperation | null): string | null {
  return saved?.phase === 'acknowledged' && saved.receipt?.status === 'published' ? saved.receipt.public_path : null;
}
export async function preparePublicationReview(scope: BuilderScope, channel: PublicationChannel, snapshot: unknown): Promise<PublicationReview> {
  assertActive(scope); const key = publicationOperationKey(scope, channel); const raw = localStorage.getItem(key);
  const previous = await readSavedPublicationOperation(scope, channel);
  if (blocked(previous)) throw new Error('A previous publication is pending or unconfirmed. Check its saved status before reviewing another.');
  const prepared = await prepareSiteSnapshot(snapshot); const previous_sha256 = await digest(raw);
  assertActive(scope);
  if (localStorage.getItem(key) !== raw) throw new Error('The saved publication changed. Review the current version again.');
  return Object.freeze({ owner: Object.freeze({ ...scope.owner }), operation_id: crypto.randomUUID(),
    site_id: previous?.receipt?.site_id ?? previous?.operation.site_id ?? null,
    snapshot: prepared.snapshot, snapshot_encoding: prepared.snapshot_encoding, snapshot_sha256: prepared.snapshot_sha256, previous_sha256 });
}
async function locked<T>(scope: BuilderScope, channel: PublicationChannel, work: () => Promise<T>, active = () => true): Promise<T> {
  assertActive(scope, active);
  if (!navigator.locks?.request) throw new Error('This browser cannot coordinate safe publication requests. The saved operation remains held.');
  return navigator.locks.request(publicationOperationKey(scope, channel) + ':request', { mode: 'exclusive', ifAvailable: true }, async lock => {
    if (!lock) throw new Error('Another publication request is active. Check its saved status.');
    assertActive(scope, active); return work();
  });
}
function saveCompared(scope: BuilderScope, channel: PublicationChannel, previous: string | null, value: SavedPublicationOperation, active = () => true): string {
  assertActive(scope, active); const key = publicationOperationKey(scope, channel);
  if (localStorage.getItem(key) !== previous) throw new Error('The saved publication changed. Its outcome remains held.');
  const raw = JSON.stringify(value); localStorage.setItem(key, raw);
  if (localStorage.getItem(key) !== raw) throw new Error('The publication could not be saved on this device. Its outcome remains held.');
  return raw;
}
function rejection(status: number, value: unknown): string | null {
  if (!value || typeof value !== 'object' || Array.isArray(value)) return null;
  const data = value as Record<string, unknown>;
  if (Object.keys(data).length !== 4 || Object.keys(data).some(key => !['schema_version', 'error', 'effect', 'message'].includes(key))
    || data.schema_version !== 1 || data.effect !== 'none' || typeof data.message !== 'string' || data.message.length > 2000) return null;
  return status === 400 && data.error === 'publication_invalid' || status === 403 && data.error === 'publication_forbidden'
    || status === 404 && data.error === 'publication_not_found' ? data.error as string : null;
}
export async function submitPublicationReview(scope: BuilderScope, channel: PublicationChannel, review: PublicationReview, active = () => true, reviewCurrent = () => true): Promise<SavedPublicationOperation> {
  assertActive(scope, active);
  const { previous_sha256, ...input } = review; const operation = await checkedOperation(input, scope);
  return locked(scope, channel, async () => {
    const raw = localStorage.getItem(publicationOperationKey(scope, channel)); const previous = await readSavedPublicationOperation(scope, channel);
    if (blocked(previous) || previous?.operation.operation_id === operation.operation_id || await digest(raw) !== previous_sha256) throw new Error('The approved publication is stale or an earlier operation remains held. Review its saved status.');
    const pending: SavedPublicationOperation = { format: 1, phase: 'publish_unknown', operation }; let marker: string | undefined;
    const response = await fetchForOwnedPublication(ROOT, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ operation_id: operation.operation_id, site_id: operation.site_id, snapshot_encoding: operation.snapshot_encoding, snapshot: operation.snapshot }) }, scope.owner,
      () => {
        if (!reviewCurrent()) throw new Error('Your private draft changed. Review the public version again.');
        marker = saveCompared(scope, channel, raw, pending, active);
      });
    const body = await readPublicationResponse(response, () => assertActive(scope, active));
    if (marker === undefined) throw new Error('The publication dispatch was not recorded.');
    const reason = rejection(response.status, body);
    if (reason) { saveCompared(scope, channel, marker, { ...pending, phase: 'rejected', rejection: reason }, active); throw new Error('The publication was rejected before saving. Review its fields and current authority.'); }
    const receipt = readSitePublicationReceipt(response.status, body, operation, 'submit');
    const saved: SavedPublicationOperation = { ...pending, phase: 'acknowledged', receipt };
    saveCompared(scope, channel, marker, saved, active); return saved;
  }, active);
}
export async function refreshPublicationOperation(scope: BuilderScope, channel: PublicationChannel, active = () => true): Promise<SavedPublicationOperation | null> {
  return locked(scope, channel, async () => {
    const saved = await readSavedPublicationOperation(scope, channel); if (!saved || saved.phase === 'rejected') return saved;
    const raw = localStorage.getItem(publicationOperationKey(scope, channel));
    const response = await fetchForOwnedPublication(ROOT + '/operations/' + saved.operation.operation_id, { method: 'GET' }, scope.owner);
    const body = await readPublicationResponse(response, () => assertActive(scope, active));
    const receipt = readSitePublicationReceipt(response.status, body, saved.operation, 'read', saved.receipt);
    const next: SavedPublicationOperation = { ...saved, phase: saved.phase === 'revoke_unknown' && receipt.status !== 'revoked' ? 'revoke_unknown' : 'acknowledged', receipt };
    saveCompared(scope, channel, raw, next, active); return next;
  }, active);
}
export async function revokePublication(scope: BuilderScope, channel: PublicationChannel, active = () => true): Promise<SavedPublicationOperation> {
  return locked(scope, channel, async () => {
    const saved = await readSavedPublicationOperation(scope, channel);
    if (!saved?.receipt || saved.phase !== 'acknowledged') throw new Error('The publication version is unconfirmed. Check its saved status before revoking.');
    if (saved.receipt.status === 'revoked') return saved;
    const raw = localStorage.getItem(publicationOperationKey(scope, channel)); const pending: SavedPublicationOperation = { ...saved, phase: 'revoke_unknown' }; let marker: string | undefined;
    const response = await fetchForOwnedPublication(ROOT + '/' + saved.receipt.publication_id, { method: 'DELETE' }, scope.owner,
      () => { marker = saveCompared(scope, channel, raw, pending, active); });
    const body = await readPublicationResponse(response, () => assertActive(scope, active));
    const receipt = readSitePublicationReceipt(response.status, body, saved.operation, 'revoke', saved.receipt);
    if (marker === undefined) throw new Error('The revoke dispatch was not recorded.');
    const next: SavedPublicationOperation = { ...saved, phase: 'acknowledged', receipt };
    saveCompared(scope, channel, marker, next, active); return next;
  }, active);
}
