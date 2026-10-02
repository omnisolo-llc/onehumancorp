import type { QueueOwner } from '@/lib/sync/queueIdentity';

export type Definition = { id: string; version: number; digest: string; name: string; description: string; role: string; system_prompt: string; visibility: 'public'; source: 'first_party' | 'community' };
export type Installation = { id: string; definition_id: string; version: number; digest: string; role_key: string; name: string; role: string; system_prompt: string; status: 'installed_inactive' };
export type Publication = Pick<Definition, 'name' | 'description' | 'role' | 'system_prompt' | 'visibility'>;
export type Operation = { kind: 'publish'; request_id: string; publication: Publication } | { kind: 'install'; request_id: string; definition: Definition };
export type Receipt = { success: true; status: 'published' | 'installed_inactive'; request_id: string; organization_id: string; user_id: string; replayed: boolean; definition?: Definition; installation?: Installation };
const UUID = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
const DIGEST = /^[0-9a-f]{64}$/;
function record(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('The saved agent response is invalid');
  return value as Record<string, unknown>;
}
function text(value: unknown, max: number, required = true): value is string {
  return typeof value === 'string' && (!required || !!value.trim()) && Array.from(value).length <= max
    && !Array.from(value).some(character => character.charCodeAt(0) === 0 || (character.codePointAt(0)! >= 0xd800 && character.codePointAt(0)! <= 0xdfff));
}
export function isDefinitionId(value: unknown): value is string { return typeof value === 'string' && UUID.test(value); }
export function readPublication(value: unknown): Publication {
  const data = record(value);
  if (Object.keys(data).some(key => !['name', 'description', 'role', 'system_prompt', 'visibility'].includes(key))
    || !text(data.name, 120) || !text(data.description, 2000, false) || !text(data.role, 120)
    || !text(data.system_prompt, 16000) || data.visibility !== 'public') throw new Error('Review the required name, role and prompt within the published limits');
  return { name: data.name, description: data.description, role: data.role, system_prompt: data.system_prompt, visibility: 'public' };
}
export async function readDefinition(value: unknown): Promise<Definition> {
  const data = record(value);
  const fields = readPublication({ name: data.name, description: data.description, role: data.role, system_prompt: data.system_prompt, visibility: data.visibility });
  if (!isDefinitionId(data.id) || !Number.isSafeInteger(data.version) || Number(data.version) < 1
    || typeof data.digest !== 'string' || !DIGEST.test(data.digest) || typeof data.source !== 'string' || !['first_party', 'community'].includes(data.source)) throw new Error('The saved agent definition is invalid');
  const bytes = new TextEncoder().encode(JSON.stringify([fields.name, fields.description, fields.role, fields.system_prompt, fields.visibility, data.source, data.version]));
  const digest = Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256', bytes)), byte => byte.toString(16).padStart(2, '0')).join('');
  if (digest !== data.digest) throw new Error('The saved agent definition changed');
  return { ...fields, id: data.id, version: Number(data.version), digest, source: data.source as Definition['source'] };
}
function readInstallation(value: unknown): Installation {
  const data = record(value);
  if (!isDefinitionId(data.id) || !isDefinitionId(data.definition_id) || !Number.isSafeInteger(data.version) || Number(data.version) < 1
    || typeof data.digest !== 'string' || !DIGEST.test(data.digest) || data.role_key !== `marketplace/${data.id}/agent`
    || !text(data.name, 120) || !text(data.role, 120) || !text(data.system_prompt, 16000)
    || data.status !== 'installed_inactive') throw new Error('The inactive installation could not be verified');
  return { id: data.id, definition_id: data.definition_id, version: Number(data.version), digest: data.digest,
    role_key: data.role_key as string, name: data.name, role: data.role, system_prompt: data.system_prompt, status: 'installed_inactive' };
}
function cursor(value: unknown): string | null {
  if (value === null) return null;
  if (!text(value, 2048)) throw new Error('The catalogue page cursor is invalid');
  return value;
}
export async function readCatalogue(value: unknown) {
  const data = record(value);
  if (('success' in data && data.success !== true) || data.error != null || data.reason != null || !Array.isArray(data.definitions) || !Array.isArray(data.installations)
    || data.definitions.length > 100 || data.installations.length > 100) throw new Error('The agent catalogue could not be verified');
  const definitions = await Promise.all(data.definitions.map(readDefinition));
  const installations = data.installations.map(readInstallation);
  if (new Set(definitions.map(item => item.id + ':' + item.version)).size !== definitions.length
    || new Set(installations.map(item => item.id)).size !== installations.length) throw new Error('The catalogue contains contradictory records');
  return { definitions, installations, next_cursor: cursor(data.next_cursor), next_installation_cursor: cursor(data.next_installation_cursor) };
}
export async function readDefinitionReceipt(status: number, value: unknown, operation: Operation, owner: QueueOwner): Promise<Receipt> {
  const data = record(value);
  if (status !== 200 || data.success !== true || data.error != null || data.reason != null
    || data.request_id !== operation.request_id || data.organization_id !== owner.tenantId || data.user_id !== owner.userId
    || typeof data.replayed !== 'boolean') throw new Error('The saved agent operation could not be confirmed');
  const common = { success: true as const, request_id: operation.request_id, organization_id: owner.tenantId, user_id: owner.userId, replayed: data.replayed };
  if (operation.kind === 'publish') {
    if (data.status !== 'published' || data.installation != null) throw new Error('The publication receipt is contradictory');
    const definition = await readDefinition(data.definition);
    if (definition.source !== 'community' || Object.entries(operation.publication).some(([key, value]) => definition[key as keyof Definition] !== value)) throw new Error('The published definition differs from the reviewed fields');
    return { ...common, status: 'published', definition };
  }
  if (data.status !== 'installed_inactive' || data.definition != null) throw new Error('The installation receipt is contradictory');
  const installation = readInstallation(data.installation);
  const expected = operation.definition;
  if (installation.definition_id !== expected.id || installation.version !== expected.version || installation.digest !== expected.digest
    || installation.name !== expected.name || installation.role !== expected.role || installation.system_prompt !== expected.system_prompt) throw new Error('The installed definition differs from the reviewed version');
  return { ...common, status: 'installed_inactive', installation };
}
export function definitiveDefinitionRejection(status: number, value: unknown): boolean {
  if (!value || typeof value !== 'object') return false;
  const data = value as Record<string, unknown>;
  return Object.keys(data).every(key => key === 'success' || key === 'reason') && data.success === false
    && ((status === 400 && data.reason === 'invalid_request') || (status === 403 && data.reason === 'owner_or_admin_required'));
}
