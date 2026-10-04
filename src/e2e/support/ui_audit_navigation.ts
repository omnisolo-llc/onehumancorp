import type { BrowserContext, Page } from '@playwright/test';
import { expectedAuditPath } from '../../../scripts/ui-click-audit.cjs';

export type AuditNavigationReceipt = {
  requestedUrl: string; finalUrl: string; redirected: boolean;
  sourceRoute?: string;
  quoteFixture?: { namespace?: string; tenantId: string; quoteId: string; customerId: string; detailUrl: string; method: 'GET'; status: 200 };
};

// A test session is isolated from the suite's shared JWT because the crawler
// also exercises Log out. All authentication still uses the real login endpoint.
export function createAuditNavigation(baseURL: string, authenticate: (page: Page) => Promise<void>) {
  const base = new URL(baseURL);
  if (!['http:', 'https:'].includes(base.protocol) || base.username || base.password) throw new Error('Invalid audit base destination');
  const origin = base.origin;
  const sessions = new WeakMap<BrowserContext, { valid: boolean }>();
  const sameOriginPath = (value: string) => {
    try {
      const url = new URL(value);
      return url.origin === origin ? url.pathname : undefined;
    } catch { return undefined; }
  };
  const sessionFor = (context: BrowserContext) => {
    let session = sessions.get(context);
    if (!session) {
      session = { valid: false };
      sessions.set(context, session);
      const tracked = session;
      context.on('request', (request) => {
        if (request.method() === 'POST' && sameOriginPath(request.url()) === '/api/v1/auth/logout') tracked.valid = false;
      });
      context.on('response', (response) => {
        const path = sameOriginPath(response.url());
        // Include a late logout response, which can clear the session cookie.
        if (path && (response.status() === 401 || path === '/api/v1/auth/logout')) tracked.valid = false;
      });
    }
    return session;
  };
  return async (page: Page, route: string): Promise<AuditNavigationReceipt> => {
    const destination = new URL(route, baseURL);
    if (!route.startsWith('/') || route.startsWith('//') || destination.origin !== origin || destination.username || destination.password) throw new Error('Audit destination must be an app route on the configured origin');
    if (destination.pathname === '/share-card' && (destination.search || destination.hash)) throw new Error('Only the default share-card redirect is classified by this route audit');
    const expectedPath = expectedAuditPath(destination.pathname);
    const session = sessionFor(page.context());
    for (let attempt = 0; attempt < 2; attempt += 1) {
      if (!session.valid) {
        await authenticate(page);
        session.valid = true;
      }
      // Only a GET navigation may be repeated after observed authentication loss.
      // No target preparation, form fill or user action occurs before this gate.
      const response = await page.goto(destination.href, { waitUntil: 'load' });
      await page.waitForLoadState('networkidle', { timeout: 100 }).catch(() => undefined);
      await page.waitForTimeout(100);
      if (destination.pathname === '/share-card') {
        // This page intentionally replaces its initial document. Do not discover
        // transient layout controls, and never retry a target click after it moves.
        await page.waitForURL(url => url.origin === origin && [expectedPath, '/login'].includes(url.pathname), { waitUntil: 'load', timeout: 5000 });
      }
      const redirectedToLogin = destination.pathname !== '/login' && sameOriginPath(page.url()) === '/login';
      if (response?.status() === 401 || redirectedToLogin || !session.valid) {
        session.valid = false;
        if (attempt === 0) continue;
        throw new Error(`Audit authentication remained unavailable for ${destination.pathname}`);
      }
      // Initialization may replace its entire shell (including voice controls).
      // This read-only gate runs only before discovery, never after a click.
      const ready = await page.waitForFunction(() => !Array.from(document.querySelectorAll('[aria-busy="true"]')).some(element => {
        const style = getComputedStyle(element);
        const bounds = element.getBoundingClientRect();
        return !element.closest('[hidden], [aria-hidden="true"]') && style.visibility !== 'hidden'
          && style.display !== 'none' && bounds.width > 0 && bounds.height > 0;
      }), undefined, { timeout: 5000 });
      await ready.dispose();
      const final = new URL(page.url());
      if (final.origin !== origin || final.username || final.password || final.pathname !== expectedPath) throw new Error('Audit navigation reached an unclassified destination route');
      return { requestedUrl: destination.href, finalUrl: final.href, redirected: destination.href !== final.href };
    }
    throw new Error('Audit navigation did not reach a verified destination');
  };
}
