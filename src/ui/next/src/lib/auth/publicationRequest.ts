import { isPublicationId, parsePublicationJson, prepareSiteSnapshot, SITE_SNAPSHOT_ENCODING } from '@/app/builder/publicationContracts';
import { proxyBackendRequest } from './backendTransport';

const ROOT = '/api/v1/builder/publications';
const UUID = '[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}';
function invalid(status: number): Response {
  return Response.json({ error: 'invalid publication request' }, { status, headers: { 'cache-control': 'private, no-store', 'x-content-type-options': 'nosniff' } });
}
export async function validatePublicationRequest(body: Uint8Array<ArrayBuffer>): Promise<Uint8Array<ArrayBuffer>> {
  const raw = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(body);
  const value = parsePublicationJson(raw);
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Invalid publication envelope');
  const data = value as Record<string, unknown>;
  if (Object.keys(data).length !== 4 || Object.keys(data).some(key => !['operation_id', 'site_id', 'snapshot_encoding', 'snapshot'].includes(key))
    || !isPublicationId(data.operation_id) || (data.site_id !== null && !isPublicationId(data.site_id))
    || data.snapshot_encoding !== SITE_SNAPSHOT_ENCODING) throw new Error('Invalid publication envelope');
  await prepareSiteSnapshot(data.snapshot);
  // The original bytes contain only an approved envelope and snapshot. Content
  // labels named "user" or "tenant" never become transport identity.
  return body;
}
export function isPublicationOwnerPath(path: string): boolean { return path === ROOT || path.startsWith(ROOT + '/'); }
export async function proxyPublicationOwnerRequest(request: Request, path: string): Promise<Response> {
  const method = request.method.toUpperCase();
  const submit = path === ROOT && method === 'POST';
  const recover = new RegExp('^' + ROOT + '/operations/' + UUID + '$').test(path) && method === 'GET';
  const revoke = new RegExp('^' + ROOT + '/' + UUID + '$').test(path) && method === 'DELETE';
  if (!submit && !recover && !revoke) return invalid(405);
  if (new URL(request.url).search) return invalid(400);
  if (submit && request.headers.get('content-type')?.split(';', 1)[0].trim().toLowerCase() !== 'application/json') return invalid(415);
  return proxyBackendRequest(request, path, {
    forwardQuery: false,
    ...(submit ? { requestLimitBytes: 2 * 1024 * 1024, transformRequestBody: validatePublicationRequest } : {
      transformRequestBody: (body: Uint8Array<ArrayBuffer>) => { if (body.length) throw new Error('This publication operation has no body'); return body; },
    }),
  });
}
