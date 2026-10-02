// This is a negative HTTP proof against the real freshly built server in E2E.
// These requests are never used to seed application state.
export const retiredFixturePaths = [
  '/api/v1/dev/seed', '/api/v1/dev/mock-omni-inbox',
  '/api/v1/dev/simulate-invoice-followup', '/api/v1/dev/simulate-agent-feed-item',
  '/api/v1/dev/simulate-triage-item',
  ...['smart-pricing', 'quote-draft', 'stockout-reorder', 'ambassador-draft',
    'promoter-draft', 'dispute-resolution', 'newsletter-draft', 'autonomous-booking-quote',
    'invoice-draft', 'invoice-followup', 'lead-recovery']
    .map(name => `/api/v1/agents/approvals/simulate-${name}`),
  '/api/v1/growth/reputation/simulate-event',
  '/api/v1/growth/reputation/simulate-referral-checkout',
];

export async function verifyProductionFixtureBoundary(origin, token, signal) {
  const url = new URL(origin);
  if (url.protocol !== 'http:' || url.hostname !== '127.0.0.1' || !url.port || !token) {
    throw new Error('The fixture-boundary check requires the isolated local backend and its authenticated owner');
  }
  for (const route of retiredFixturePaths) {
    const response = await fetch(new URL(route, url), {
      method: 'POST', redirect: 'manual', signal: signal
        ? AbortSignal.any([signal, AbortSignal.timeout(5000)]) : AbortSignal.timeout(5000),
      headers: { authorization: `Bearer ${token}`, 'content-type': 'application/json' },
      body: JSON.stringify({ scenario: 'launch-readiness', approved: false }),
    });
    await response.body?.cancel();
    if (response.status !== 404) throw new Error(`Production exposes retired fixture route ${route}: HTTP ${response.status}`);
  }
}
