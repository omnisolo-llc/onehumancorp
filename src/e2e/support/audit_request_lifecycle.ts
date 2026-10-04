import type { BrowserContext, Request } from '@playwright/test';

type RequestRecord = {
  id: number; method: string; path: string; documentPath: string | null;
  resourceType: string; hasQuery: boolean; rsc: boolean; startedAt: string;
  startedMs: number; responseReceived: boolean; status?: number; contentType?: string;
  state: 'awaiting-response' | 'awaiting-completion' | 'finished' | 'failed'; failure?: string;
};

// Explicit static paths used by the isolated views and their startup requests.
// Unknown suffixes are parameters, even when they resemble a short ordinary word.
const knownPaths = new Set([
  '/', '/share-card', '/dashboard', '/assistant', '/ai-workspace', '/onboarding',
  '/triage', '/orders', '/products', '/inbox', '/inventory', '/kairos', '/agents',
  '/business-analytics', '/dashboard/campaigns', '/lead-magnet-generator',
  '/settings', '/ai-usage-paywall', '/changelog', '/calendar', '/langgraph',
  '/visual-workflow', '/agent-protocol', '/integrations', '/cost-dashboard',
  '/diagnostics', '/client-portal', '/help', '/login', '/unified-feed',
  '/dashboard/unified-feed', '/feed', '/action-center', '/builder', '/website-builder',
  '/api/v1/help', '/api/v1/videos', '/api/v1/tooltips', '/api/v1/auth/session-identity',
  '/api/v1/onboarding/draft', '/api/v1/onboarding/state',
  '/api/v1/ui/dashboard/unified-feed', '/api/v1/agent-feed',
  '/api/v1/agents/approvals', '/api/v1/agents/approvals/activity',
]);
const knownPrefixes = [...knownPaths, '/api/v1/agents/definitions', '/api/v1/auth', '/api/v1/onboarding', '/api/v1/ui', '/api/v1']
  .filter(path => path !== '/').sort((a, b) => b.length - a.length);

// Never retain raw unknown route segments, credentials, query values or hashes.
function safePath(url: URL): string {
  if (knownPaths.has(url.pathname)) return url.pathname;
  const prefix = knownPrefixes.find(path => url.pathname.startsWith(path + '/'));
  return prefix ? prefix + '/[redacted]' : '/[redacted]';
}

/** Observe the existing settling boundary without reading or canceling bodies. */
export function observeAuditRequests(context: BrowserContext, baseURL: string) {
  const origin = new URL(baseURL).origin;
  const pending = new Map<Request, RequestRecord>();
  const recent: RequestRecord[] = [];
  let sequence = 0;
  context.on('request', request => {
    const url = new URL(request.url());
    if (url.origin !== origin || !['fetch', 'xhr'].includes(request.resourceType())) return;
    let documentPath: string | null = null;
    try { const document = new URL(request.frame().url()); if (document.origin === origin) documentPath = safePath(document); } catch { /* A detached frame can have no URL. */ }
    const startedMs = Date.now();
    pending.set(request, { id: ++sequence, method: request.method(), path: safePath(url), documentPath,
      resourceType: request.resourceType(), hasQuery: !!url.search, rsc: url.searchParams.has('_rsc'),
      startedAt: new Date(startedMs).toISOString(), startedMs, responseReceived: false, state: 'awaiting-response' });
  });
  context.on('response', response => {
    const record = pending.get(response.request());
    if (!record) return;
    record.responseReceived = true; record.status = response.status(); record.state = 'awaiting-completion';
    const mime = response.headers()['content-type']?.split(';')[0].trim();
    if (mime && /^[a-z0-9.+-]+\/[a-z0-9.+-]+$/i.test(mime) && mime.length <= 80) record.contentType = mime;
  });
  const retire = (request: Request, state: 'finished' | 'failed') => {
    const record = pending.get(request);
    pending.delete(request);
    if (!record) return;
    record.state = state;
    if (state === 'failed') {
      const failure = request.failure()?.errorText;
      record.failure = failure && /^net::ERR_[A-Z_]+$/.test(failure) ? failure : 'transport failure';
    }
    recent.push(record);
    if (recent.length > 20) recent.shift();
  };
  context.on('requestfinished', request => retire(request, 'finished'));
  context.on('requestfailed', request => retire(request, 'failed'));
  return {
    get size() { return pending.size; },
    snapshot() {
      const now = Date.now();
      const describe = ({ startedMs, ...record }: RequestRecord) => ({ ...record, elapsedMs: Math.max(0, now - startedMs) });
      // Evidence is bounded; the settling count always includes every request.
      return { pendingCount: pending.size, omittedPending: Math.max(0, pending.size - 20),
        pending: [...pending.values()].slice(0, 20).map(describe), recent: recent.map(describe) };
    },
  };
}
