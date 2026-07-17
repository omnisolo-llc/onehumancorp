export const HYDRATION_FAILURE_PATTERN = /Text content does not match server-rendered HTML|Text content did not match|Hydration failed|error occurred during hydration|server HTML (?:was )?replaced|initial UI does not match/i;

const RESOURCE_FAILURE_PATTERN = /^Failed to load resource: the server responded with a status of (?:401|403|404|500|502)\b/i;

const EXPECTED_ISOLATED_RESOURCE_PATHS = new Set([
  '/api/v1/help',
  '/api/v1/videos',
  '/api/v1/tooltips',
  '/api/v1/mesh/v2/collective',
  '/api/v1/ui/dashboard/analytics/briefing',
  '/api/v1/ui/triage',
  '/api/v1/payments/ledger/safe-to-spend',
  '/api/v1/billing/department-tier-usage',
  '/api/v1/growth/milestone',
  '/api/v1/growth/wrapped',
  '/api/v1/growth/campaign/abandoned-carts-count',
  '/api/v1/growth/team-invites/aggregated-metrics',
  '/api/v1/growth/affiliate/stats',
  '/api/v1/growth/referrals/milestones/status',
  '/api/v1/ui/dashboard/unified-feed',
  '/api/v1/walkthrough/dashboard',
  '/api/v1/onboarding/state',
  '/api/v1/ledger/accounts',
  '/api/v1/ledger/entries',
  '/api/v1/user/usage',
  '/api/v1/growth/milestone/card',
  '/api/v1/assistant/tasks',
  '/api/v1/walkthrough/assistant',
  '/api/v1/ui/orders',
  '/api/v1/ui/omni_inbox',
  '/api/v1/ui/inventory',
  '/api/v1/agents/approvals',
  '/api/v1/auth/powersync_token',
  '/api/v1/agents/approvals/activity',
  '/api/v1/settings/delivery',
  '/api/v1/settings/voice',
  '/api/v1/settings/telemetry',
  '/api/v1/local_seo/discovery_report',
  '/api/v1/seo/discovery_report',
  '/api/v1/ui/dashboard/metrics',
  '/api/v1/dashboard/metrics',
  '/api/v1/integrations',
  '/api/v1/ui/bookings',
  '/api/v1/health',
  '/api/v1/agents/marketplace',
  '/api/v1/onboarding/draft',
  '/api/v1/billing/my-plan',
  '/api/v1/growth/birthday-club/embed',
  '/api/v1/growth/referrals/generate',
  '/api/v1/invoices',
  '/api/v1/staff/escalations',
  '/api/v1/staff/shifts',
  '/api/v1/agent-debug-trace',
  '/api/v1/agent-feed',
  '/api/v1/agents/goose',
  '/api/v1/agents/protocol',
  '/api/v1/api-docs-spec',
  '/api/v1/billing/cost-dashboard',
  '/api/v1/campaign/proposals',
  '/api/v1/catalog/products',
  '/api/v1/changelog',
  '/api/v1/chaos/report',
  '/api/v1/fulfillment',
  '/api/v1/growth/link-in-bio/my-store',
  '/api/v1/growth/link-in-bio/visual-audit-business',
  '/api/v1/growth/loyalty',
  '/api/v1/growth/milestones/check',
  '/api/v1/growth/viral-leaderboard/embed',
  '/api/v1/inbox/summary/default-tenant-id/default-customer-id',
  '/api/v1/kairos/memory',
  '/api/v1/kairos/mesh',
  '/api/v1/kairos/tasks',
  '/api/v1/location/dashboard',
  '/api/v1/memory',
  '/api/v1/ohc_job_queue',
  '/api/v1/payments/ledger/balance',
  '/api/v1/payments/terminal/backend',
  '/api/v1/pos/inventory',
  '/api/v1/pos/orders',
  '/api/v1/proposals/visual-audit-id',
  '/api/v1/quotes/visual-audit-id',
  '/api/v1/sona',
  '/api/v1/staff/tasks',
  '/api/v1/subscriptions',
  '/api/v1/subscriptions/visual-audit-id',
  '/api/v1/triage/pending',
  '/api/v1/ui/dashboard/daily-work',
  '/api/v1/ui/opportunities',
  '/api/v1/ui/prep-forecast',
  '/api/v1/walkthrough/pos',
  '/api/v1/walkthrough/store-setup',
  '/api/v1/help/visual-audit-article',
  '/agent-audit-dashboard',
  '/favicon.ico',
]);

function parsedUrl(value) {
  try {
    return new URL(value);
  } catch {
    return null;
  }
}

function isPrivateAuditOrigin(url) {
  if (!url || url.port !== '3000') return false;
  const hostname = url.hostname.toLowerCase();
  if (hostname === 'localhost' || hostname === '127.0.0.1' || hostname === '[::1]') return true;

  const octets = hostname.split('.').map(Number);
  if (octets.length === 4 && octets.every((octet) => Number.isInteger(octet) && octet >= 0 && octet <= 255)) {
    return octets[0] === 10
      || (octets[0] === 172 && octets[1] >= 16 && octets[1] <= 31)
      || (octets[0] === 192 && octets[1] === 168);
  }

  const bareIpv6 = hostname.startsWith('[') && hostname.endsWith(']')
    ? hostname.slice(1, -1)
    : hostname;
  const firstGroup = Number.parseInt(bareIpv6.split(':', 1)[0], 16);
  return bareIpv6.includes(':') && Number.isFinite(firstGroup)
    && ((firstGroup & 0xfe00) === 0xfc00 || (firstGroup & 0xffc0) === 0xfe80);
}

function isSamePrivateAuditOrigin(location, page) {
  return isPrivateAuditOrigin(location)
    && isPrivateAuditOrigin(page)
    && location.origin === page.origin;
}

function isExpectedWebSocketFailure({ message, locationUrl }) {
  if (!message.includes("WebSocket connection to 'ws://127.0.0.1:18789/api/v1/feed/ws' failed")) return false;
  const location = parsedUrl(locationUrl);
  return isPrivateAuditOrigin(location)
    && /^\/_next\/static\/chunks\/app\/(?:dashboard|agents)\//.test(location.pathname);
}

function isExpectedApplicationFailure({ message, locationUrl, pageUrl }) {
  const location = parsedUrl(locationUrl);
  const page = parsedUrl(pageUrl);
  if (message.startsWith('Failed to load tooltips Error: Failed to load tooltips, status:')) {
    return isPrivateAuditOrigin(page)
      && location?.protocol === 'webpack-internal:'
      && location.pathname.endsWith('/next-devtools/userspace/app/errors/intercept-console-error.js');
  }
  if (!isSamePrivateAuditOrigin(location, page)) return false;

  if (message === 'Websocket error: Event') {
    return /^\/_next\/static\/chunks\//.test(location.pathname)
      && ['/dashboard', '/agents'].includes(page?.pathname || '');
  }
  if (message.startsWith('Failed to fetch usage SyntaxError:')) {
    return /^\/_next\/static\/chunks\//.test(location.pathname) && page?.pathname === '/dashboard';
  }
  if (message.startsWith('Failed to load seo reports SyntaxError:')) {
    return /^\/_next\/static\/chunks\//.test(location.pathname) && page?.pathname === '/settings';
  }
  if (message.startsWith('Error: Failed to load bookings')) {
    return /^\/_next\/static\/chunks\//.test(location.pathname)
      && page?.pathname === '/calendar'
      && message.includes('/_next/static/chunks/app/calendar/');
  }
  return false;
}

function isExpectedResourceFailure({ message, locationUrl, pageUrl }) {
  if (!RESOURCE_FAILURE_PATTERN.test(message)) return false;
  const location = parsedUrl(locationUrl);
  const page = parsedUrl(pageUrl);
  return isSamePrivateAuditOrigin(location, page)
    && EXPECTED_ISOLATED_RESOURCE_PATHS.has(location.pathname);
}

export function classifyConsoleError(diagnostic) {
  if (HYDRATION_FAILURE_PATTERN.test(diagnostic.message)) return 'hydration';
  if (isExpectedWebSocketFailure(diagnostic)
    || isExpectedResourceFailure(diagnostic)
    || isExpectedApplicationFailure(diagnostic)) return 'expected-service';
  return 'unexpected';
}

export function failureReasons(result) {
  const reasons = [];
  if (result.navigationError) reasons.push('navigation error');
  if (result.captureError) reasons.push('capture error');
  if (result.screenshotError || !result.screenshotWritten) reasons.push('screenshot error');
  if (result.status !== null && result.status >= 400) reasons.push(`HTTP ${result.status}`);
  const expectedRedirect = result.route === '/share-card' && result.finalPathname === '/onboarding';
  if (result.finalPathname && result.finalPathname !== result.route && !expectedRedirect) {
    reasons.push(`unexpected redirect to ${result.finalPathname}`);
  }
  if (result.pageErrors?.length > 0) reasons.push('uncaught page error');
  if (result.hydrationErrors?.length > 0) reasons.push('hydration error');
  if (result.unexpectedConsoleErrors?.length > 0) reasons.push('unexpected console error');
  for (const [shell, count] of Object.entries(result.shellCounts)) {
    if (count !== 1) reasons.push(`${shell} count ${count}`);
  }
  if (result.horizontalOverflow) {
    reasons.push(`horizontal overflow ${result.documentWidth - result.viewportWidth}px`);
  }
  return reasons;
}

export function isCoverageComplete(results, expectedCases) {
  if (results.length !== expectedCases.length) return false;
  const expectedKeys = new Set(expectedCases.map((item) => `${item.viewportName}:${item.route}`));
  const resultKeys = new Set(results.map((item) => `${item.viewport}:${item.route}`));
  if (resultKeys.size !== expectedKeys.size) return false;
  if ([...expectedKeys].some((key) => !resultKeys.has(key))) return false;
  return results.every((result) => result.attempted
    && result.completed
    && result.screenshotWritten
    && !result.screenshotError);
}

export function shouldFailAudit({ results, expectedCases, fatalError, outputReady }) {
  return Boolean(fatalError)
    || !outputReady
    || !isCoverageComplete(results, expectedCases)
    || results.some((result) => failureReasons(result).length > 0);
}
