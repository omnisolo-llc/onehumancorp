export const HYDRATION_FAILURE_PATTERN = /Text content does not match server-rendered HTML|Text content did not match|Hydration failed|error occurred during hydration|server HTML (?:was )?replaced|initial UI does not match/i;

const RESOURCE_FAILURE_PATTERN = /^Failed to load resource: the server responded with a status of (?:404|500|502)\b/i;

const EXPECTED_ISOLATED_RESOURCE_PATHS = new Set([
  '/api/ui/swagger-ui.css',
  '/api/ui/swagger-ui-bundle.js',
  '/api/mesh/v2/collective',
  '/api/ui/dashboard/analytics/briefing',
  '/api/ui/triage',
  '/api/finance/safe-to-spend',
  '/api/v1/billing/department-tier-usage',
  '/api/v1/growth/milestone',
  '/api/v1/growth/wrapped',
  '/api/v1/growth/campaign/abandoned-carts-count',
  '/api/v1/growth/team-invites/aggregated-metrics',
  '/api/v1/growth/affiliate/stats',
  '/api/v1/growth/referrals/milestones/status',
  '/api/ui/dashboard/unified-feed',
  '/api/walkthrough/dashboard',
  '/api/onboarding/state',
  '/api/ledger/accounts',
  '/api/user/usage',
  '/api/v1/growth/milestone/card',
  '/api/assistant/tasks',
  '/api/walkthrough/assistant',
  '/api/ui/orders',
  '/api/ui/inventory',
  '/api/agents/approvals',
  '/api/v1/auth/powersync_token',
  '/api/agents/approvals/activity',
  '/api/settings/delivery',
  '/api/settings/voice',
  '/api/settings/telemetry',
  '/api/local_seo/discovery_report',
  '/api/ui/dashboard/metrics',
  '/api/integrations',
  '/api/ui/bookings',
  '/api/v1/health',
  '/api/agents/marketplace',
  '/api/onboarding/draft',
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

function isExpectedWebSocketFailure({ message, locationUrl }) {
  if (!message.includes("WebSocket connection to 'ws://127.0.0.1:18789/api/v1/feed/ws' failed")) return false;
  const location = parsedUrl(locationUrl);
  return location?.hostname === '127.0.0.1'
    && location.port === '3000'
    && /^\/_next\/static\/chunks\/app\/(?:dashboard|agents)\//.test(location.pathname);
}

function isExpectedApplicationFailure({ message, locationUrl, pageUrl }) {
  const location = parsedUrl(locationUrl);
  const page = parsedUrl(pageUrl);
  if (location?.hostname !== '127.0.0.1' || location.port !== '3000') return false;

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

function isExpectedResourceFailure({ message, locationUrl }) {
  if (!RESOURCE_FAILURE_PATTERN.test(message)) return false;
  const location = parsedUrl(locationUrl);
  return location?.hostname === '127.0.0.1'
    && location.port === '3000'
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
