import { createHash } from 'node:crypto';
import { expect, it } from 'vitest';
import { definitiveDefinitionRejection, readCatalogue, readDefinition, readDefinitionReceipt, readPublication, type Definition, type Operation } from './definitions';

const owner = { userId: 'owner-a', tenantId: 'tenant-a' };
const requestId = '10000000-0000-4000-8000-000000000001';
const publication = { name: 'Reviewed helper', description: 'A public definition', role: 'Writer', system_prompt: 'Draft text for owner review.\nDo not send it.', visibility: 'public' as const };
function definition(overrides: Partial<Definition> = {}): Definition {
  const value: Definition = { ...publication, id: '20000000-0000-4000-8000-000000000001', version: 1, source: 'community', digest: '', ...overrides };
  value.digest = createHash('sha256').update(JSON.stringify([value.name, value.description, value.role, value.system_prompt, value.visibility, value.source, value.version])).digest('hex');
  return value;
}
const publish: Operation = { kind: 'publish', request_id: requestId, publication };
const receipt = (overrides: Record<string, unknown> = {}) => ({ success: true, status: 'published', request_id: requestId, organization_id: owner.tenantId, user_id: owner.userId, replayed: false, definition: definition(), ...overrides });
it('retains the exact reviewed publication bytes within Unicode bounds', () => {
  const input = { ...publication, name: '  Reviewed helper  ', role: '🧩'.repeat(120), system_prompt: '  Keep these spaces\n' };
  expect(readPublication(input)).toEqual(input);
});
it.each([
  { ...publication, name: ' ' }, { ...publication, role: '🧩'.repeat(121) },
  { ...publication, system_prompt: 'x'.repeat(16001) }, { ...publication, description: 'x'.repeat(2001) },
  { ...publication, system_prompt: 'bad\0prompt' }, { ...publication, visibility: 'private' },
  { ...publication, tool_grants: ['network'] },
])('rejects invalid or unreviewed publication fields %#', value => { expect(() => readPublication(value)).toThrow(); });
it('verifies the immutable full definition digest', async () => {
  const saved = definition(); expect(await readDefinition(saved)).toEqual(saved);
  await expect(readDefinition({ ...saved, system_prompt: 'Replaced prompt' })).rejects.toThrow();
});
it('keeps independent pagination cursors and rejects duplicate public definitions', async () => {
  const value = { definitions: [definition()], installations: [], next_cursor: 'opaque-public', next_installation_cursor: null };
  expect(await readCatalogue(value)).toEqual(value);
  await expect(readCatalogue({ ...value, definitions: [definition(), definition()] })).rejects.toThrow();
});
it('accepts only a matching completed publication receipt', async () => {
  expect(await readDefinitionReceipt(200, receipt(), publish, owner)).toEqual(receipt());
});
it.each([
  { status: 202, value: receipt() }, { status: 200, value: receipt({ success: false }) },
  { status: 200, value: receipt({ user_id: 'owner-b' }) }, { status: 200, value: receipt({ organization_id: 'tenant-b' }) },
  { status: 200, value: receipt({ request_id: '10000000-0000-4000-8000-000000000002' }) },
  { status: 200, value: receipt({ definition: definition({ system_prompt: 'Changed reviewed prompt' }) }) },
  { status: 200, value: receipt({ reason: 'conflict' }) },
])('holds mismatched or nonterminal publication receipts %#', async ({ status, value }) => {
  await expect(readDefinitionReceipt(status, value, publish, owner)).rejects.toThrow();
});
it('binds an inactive installation receipt to the exact reviewed definition', async () => {
  const saved = definition(); const operation: Operation = { kind: 'install', request_id: requestId, definition: saved };
  const installation = { id: '30000000-0000-4000-8000-000000000001', definition_id: saved.id, version: saved.version, digest: saved.digest, role_key: 'marketplace/30000000-0000-4000-8000-000000000001/agent', name: saved.name, role: saved.role, system_prompt: saved.system_prompt, status: 'installed_inactive' };
  const value = { ...receipt(), definition: undefined, status: 'installed_inactive', installation };
  expect((await readDefinitionReceipt(200, value, operation, owner)).installation).toEqual(installation);
  await expect(readDefinitionReceipt(200, { ...value, installation: { ...installation, status: 'running' } }, operation, owner)).rejects.toThrow();
  await expect(readDefinitionReceipt(200, { ...value, installation: { ...installation, digest: 'a'.repeat(64) } }, operation, owner)).rejects.toThrow();
});
it('recognizes only the mounted pre-mutation rejection contracts', () => {
  expect(definitiveDefinitionRejection(400, { success: false, reason: 'invalid_request' })).toBe(true);
  expect(definitiveDefinitionRejection(403, { success: false, reason: 'owner_or_admin_required' })).toBe(true);
  for (const status of [200, 202, 404, 409, 500, 503]) expect(definitiveDefinitionRejection(status, { success: false, reason: 'invalid_request' })).toBe(false);
});
it('rejects a non-string source even when its malformed representation has a matching digest', async () => {
  await expect(readDefinition(definition({ source: ['community'] as unknown as Definition['source'] }))).rejects.toThrow();
});
it('holds contradictory rejection and catalogue envelopes', async () => {
  expect(definitiveDefinitionRejection(400, { success: false, reason: 'invalid_request', definition: definition() })).toBe(false);
  await expect(readCatalogue({ success: 0, definitions: [definition()], installations: [], next_cursor: null, next_installation_cursor: null })).rejects.toThrow();
});
