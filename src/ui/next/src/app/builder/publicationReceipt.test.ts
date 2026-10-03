import { expect, it } from 'vitest';
import { readSitePublicationReceipt, type SitePublicationBinding, type SitePublicationReceipt } from './publicationContracts';

const owner = { userId: 'publisher-a', tenantId: 'workspace-a' };
const operation = '10000000-0000-4000-8000-000000000001';
const publication = '20000000-0000-4000-8000-000000000002';
const site = '30000000-0000-4000-8000-000000000003';
const digest = 'a'.repeat(64);
const expected = { owner, operation_id: operation, site_id: null, snapshot_sha256: digest, snapshot_encoding: 'jcs-rfc8785-v1' as const };
const receipt = (status = 'pending', publicPath: string | null = null) => ({ schema_version: 1, user_id: owner.userId, organization_id: owner.tenantId,
  publication_id: publication, operation_id: operation, site_id: site, version: 1, status, snapshot_sha256: digest, snapshot_encoding: 'jcs-rfc8785-v1', public_path: publicPath });
function read(httpStatus: number, body: unknown, binding = expected as SitePublicationBinding, action: 'submit' | 'read' | 'revoke' = 'submit', previous?: unknown) {
  return readSitePublicationReceipt(httpStatus, body, binding, action, previous as SitePublicationReceipt | undefined);
}
it.each(['pending', 'processing'])('accepts acknowledged202 %s without a public link', status => {
  expect(read(202, receipt(status))).toEqual(receipt(status));
});
it('accepts a same-owner published receipt only with its exact site path', () => {
  expect(read(200, receipt('published', '/api/v1/public/sites/' + site))).toEqual(receipt('published', '/api/v1/public/sites/' + site));
});
it.each(['published', 'failed', 'revoked'])('retains a terminal %s receipt with no currently eligible public path', status => {
  expect(read(200, receipt(status))).toEqual(receipt(status));
});
it('allows a read-only200 to report pending, and a confirmed revoke only to report revoked', () => {
  expect(read(200, receipt(), expected, 'read').status).toBe('pending');
  expect(read(200, receipt('revoked'), expected, 'revoke').status).toBe('revoked');
});
it.each([
  { schema_version: 2 }, { status: ['published'] }, { version: 0 }, { version: 1.5 },
  { user_id: 'other' }, { organization_id: 'other' }, { operation_id: publication },
  { publication_id: '' }, { site_id: '' }, { snapshot_sha256: 'b'.repeat(64) },
  { public_path: '//outside.example' }, { status: 'failed', public_path: '/api/v1/public/sites/' + site },
  { status: 'published', public_path: '/api/v1/public/sites/' + publication },
  { status: 'published', public_path: '/api/v1/public/sites/' + site + '?token=unexpected' },
  { success: false }, { error: 'failed' }, { snapshot_encoding: undefined }, { snapshot_encoding: 'legacy-sorted-json' },
])('holds contradictory or unbound publication data %#', change => {
  expect(() => read(202, { ...receipt(), ...change })).toThrow();
});
it.each([[200, 'pending'], [202, 'published'], [204, 'published'], [500, 'published']])('rejects HTTP %s with contradictory status %s', (status, state) => {
  expect(() => read(Number(status), receipt(String(state)))).toThrow();
});
it('binds republishing to the already owned site identity', () => {
  expect(() => read(202, receipt(), { ...expected, site_id: publication })).toThrow();
  expect(read(202, receipt(), { ...expected, site_id: site }).site_id).toBe(site);
});
it.each([{ publication_id: operation }, { version: 2 }, { site_id: publication }])('rejects changed publication identity on status recovery %#', change => {
  expect(() => read(200, { ...receipt('published'), ...change }, expected, 'read', receipt())).toThrow();
});
it('does not treat a still-published read as a revoke acknowledgement', () => {
  expect(() => read(200, receipt('published', '/api/v1/public/sites/' + site), expected, 'revoke', receipt('published'))).toThrow();
});
it.each([
  ['revoked', 'published'], ['revoked', 'pending'], ['revoked', 'failed'],
  ['failed', 'published'], ['failed', 'processing'], ['published', 'failed'], ['published', 'pending'],
])('rejects resurrection or regression from terminal %s to %s on the same version', (previous, next) => {
  expect(() => read(200, receipt(next, next === 'published' ? '/api/v1/public/sites/' + site : null), expected, 'read', receipt(previous))).toThrow();
});
it.each(['failed', 'published', 'revoked'])('accepts an exact %s version being revoked without affecting another version', previous => {
  expect(read(200, receipt('revoked'), expected, 'read', receipt(previous)).status).toBe('revoked');
});
