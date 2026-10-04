import { proxyBackendRequest } from '@/lib/auth/backendTransport';
import { canonicalRawPath } from '@/lib/auth/url';

function reject(status: number): Response {
  return Response.json({ error: status === 405 ? 'method not allowed' : 'invalid clock receipt path' }, {
    status, headers: { 'cache-control': 'private, no-store', pragma: 'no-cache', 'x-content-type-options': 'nosniff' },
  });
}

export async function GET(request: Request): Promise<Response> {
  if (request.method !== 'GET') return reject(405);
  // Validate raw spelling before URL parsing can normalize dot segments. Do not
  // decode receipt IDs: the exact single safe segment is the backend identity.
  const rawPath = request.url.match(/^https?:\/\/[^/?#]+(\/[^?#]*)/)?.[1];
  if (!rawPath) return reject(400);
  try { canonicalRawPath(rawPath); } catch { return reject(400); }
  if (!/^\/api\/v1\/staff\/timecard\/receipts\/[A-Za-z0-9_-]{1,128}$/.test(rawPath)) return reject(400);
  return proxyBackendRequest(request, rawPath, { forwardQuery: false });
}
