import { isPublicSiteDocumentPath } from './publicSitePath';
import { parseAuthRuntimeConfig } from './runtimeConfig';

export const PUBLIC_SITE_SECURITY_HEADERS = {
  'content-type': 'text/html; charset=utf-8',
  'cache-control': 'no-store',
  'x-content-type-options': 'nosniff',
  'referrer-policy': 'no-referrer',
  'content-security-policy': "default-src 'none'; script-src 'none'; style-src 'unsafe-inline'; img-src https: http:; base-uri 'none'; form-action 'none'; object-src 'none'; frame-src 'none'; connect-src 'none'",
} as const;
const MAX_DOCUMENT_BYTES = 8 * 1024 * 1024;
type Dependencies = { backendOrigin: string; fetchImpl: typeof fetch; timeoutMs: number };
function unavailable(status: number) {
  return new Response(status === 404 ? 'Publication not found.' : 'Public document unavailable.', { status, headers: { ...PUBLIC_SITE_SECURITY_HEADERS, 'content-type': 'text/plain; charset=utf-8' } });
}

/** Anonymous by construction: neither cookies nor authenticated request identity cross this boundary. */
export async function proxyPublicSiteRequest(request: Request, dependencies: Dependencies): Promise<Response> {
  const url = new URL(request.url);
  if (request.method !== 'GET') return unavailable(405);
  if (url.search || url.hash || !isPublicSiteDocumentPath(url.pathname)) return unavailable(400);
  if (!Number.isSafeInteger(dependencies.timeoutMs) || dependencies.timeoutMs <= 0 || dependencies.timeoutMs > 60_000) return unavailable(503);
  let target: URL;
  try {
    const origin = new URL(dependencies.backendOrigin);
    target = new URL(url.pathname, origin);
    if (origin.origin !== dependencies.backendOrigin || !['http:', 'https:'].includes(origin.protocol) || target.origin !== origin.origin || target.pathname !== url.pathname) return unavailable(503);
  } catch { return unavailable(503); }
  const controller = new AbortController();
  let rejectAbort!: (reason: Error) => void;
  const aborted = new Promise<never>((_, reject) => { rejectAbort = reject; });
  // Always attach a handler, including the already-aborted request case.
  void aborted.catch(() => undefined);
  const cancel = () => { controller.abort(); rejectAbort(new Error('Public document read aborted')); };
  request.signal.addEventListener('abort', cancel, { once: true });
  if (request.signal.aborted) cancel();
  const timer = setTimeout(cancel, dependencies.timeoutMs);
  let reader: ReadableStreamDefaultReader<Uint8Array> | undefined;
  try {
    if (controller.signal.aborted) return unavailable(503);
    const backend = await Promise.race([dependencies.fetchImpl(target, {
      method: 'GET', headers: { accept: 'text/html' }, credentials: 'omit', redirect: 'manual', cache: 'no-store', signal: controller.signal,
    }), aborted]);
    if (backend.status !== 200) {
      void backend.body?.cancel().catch(() => undefined);
      return unavailable(backend.status === 404 ? 404 : backend.status === 503 ? 503 : 502);
    }
    const declared = backend.headers.get('content-length');
    if (Object.entries(PUBLIC_SITE_SECURITY_HEADERS).some(([name, value]) => backend.headers.get(name) !== value)
      || declared !== null && (!/^(0|[1-9]\d*)$/.test(declared) || Number(declared) > MAX_DOCUMENT_BYTES) || !backend.body) {
      void backend.body?.cancel().catch(() => undefined); return unavailable(502);
    }
    reader = backend.body.getReader(); const chunks: Uint8Array[] = []; let total = 0;
    for (;;) {
      const next = await Promise.race([reader.read(), aborted]);
      if (next.done) break;
      total += next.value.byteLength;
      if (total > MAX_DOCUMENT_BYTES) { void reader.cancel().catch(() => undefined); return unavailable(502); }
      chunks.push(next.value);
    }
    const bytes = new Uint8Array(total); let offset = 0;
    for (const chunk of chunks) { bytes.set(chunk, offset); offset += chunk.length; }
    new TextDecoder('utf-8', { fatal: true }).decode(bytes);
    const headers = new Headers(PUBLIC_SITE_SECURITY_HEADERS);
    const etag = backend.headers.get('etag');
    if (etag && /^"[A-Za-z0-9_-]{1,128}"$/.test(etag)) headers.set('etag', etag);
    return new Response(bytes, { status: 200, headers });
  } catch {
    void reader?.cancel().catch(() => undefined);
    return unavailable(controller.signal.aborted ? 503 : 502);
  } finally { reader?.releaseLock(); clearTimeout(timer); request.signal.removeEventListener('abort', cancel); }
}
export async function proxyLivePublicSiteRequest(request: Request): Promise<Response> {
  try {
    const config = parseAuthRuntimeConfig(process.env);
    return proxyPublicSiteRequest(request, { backendOrigin: config.backendOrigin, fetchImpl: fetch, timeoutMs: 10_000 });
  } catch { return unavailable(503); }
}
