import type { BrowserContext, Page } from '@playwright/test';
import { fillEmptyAuditControls } from './ui_click_audit';

// A test session is isolated from the suite's shared JWT because the crawler
// also exercises Log out. All authentication still uses the real login endpoint.
export function createAuditNavigation(baseURL: string, authenticate: (page: Page) => Promise<void>) {
  const origin = new URL(baseURL).origin;
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
  return async (page: Page, route: string) => {
    const session = sessionFor(page.context());
    const destination = new URL(route, baseURL);
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
      const redirectedToLogin = destination.pathname !== '/login' && sameOriginPath(page.url()) === '/login';
      if (response?.status() === 401 || redirectedToLogin || !session.valid) {
        session.valid = false;
        if (attempt === 0) continue;
        throw new Error(`Audit authentication remained unavailable for ${destination.pathname}`);
      }
      await page.evaluate(fillEmptyAuditControls);
      return;
    }
  };
}
