import type { BrowserContext, Page, Response } from '@playwright/test';
import { expectedAuditPath } from '../../../scripts/ui-click-audit.cjs';

export type AuditNavigationReceipt = {
  requestedUrl: string; finalUrl: string; redirected: boolean;
  sourceRoute?: string;
  quoteFixture?: { namespace?: string; tenantId: string; quoteId: string; customerId: string; detailUrl: string; method: 'GET'; status: 200 };
};

function observeApiDocumentation(page: Page, origin: string) {
  let receive!: (response: Response) => void;
  const source = new Promise<Response>(resolve => { receive = resolve; });
  const listener = (response: Response) => {
    const url = new URL(response.url());
    if (url.origin === origin && url.pathname === '/api/v1/api-docs-spec' && response.request().method() === 'GET') receive(response);
  };
  page.on('response', listener);
  return {
    dispose: () => page.off('response', listener),
    ready: async () => {
      let timer: ReturnType<typeof setTimeout> | undefined;
      try {
        const response = await Promise.race([source, new Promise<never>((_resolve, reject) => {
          timer = setTimeout(() => reject(new Error('API documentation source did not load')), 10_000);
        })]);
        if (response.status() !== 200) throw new Error(`API documentation source returned HTTP ${response.status()}`);
        const spec: unknown = await response.json();
        const record = (value: unknown): value is Record<string, unknown> => !!value && typeof value === 'object' && !Array.isArray(value);
        if (!record(spec) || !record(spec.paths)) throw new Error('API documentation operation source is malformed');
        const methods = new Set(['get', 'put', 'post', 'delete', 'options', 'head', 'patch', 'trace']);
        const expected: string[] = [];
        for (const [path, item] of Object.entries(spec.paths)) {
          if (!record(item) || '$ref' in item) throw new Error('API documentation path must declare its operations');
          for (const [method, operation] of Object.entries(item)) {
            if (!methods.has(method)) continue;
            if (!record(operation)) throw new Error('API documentation operation is malformed');
            const tags = operation.tags === undefined ? [] : operation.tags;
            if (!Array.isArray(tags) || tags.some(tag => typeof tag !== 'string' || !tag)) throw new Error('API documentation operation tags are malformed');
            for (const tag of new Set(tags.length ? tags : ['default'])) expected.push(JSON.stringify([tag, method.toUpperCase(), path]));
          }
        }
        if (!expected.length) throw new Error('API documentation source declares no operations');
        expected.sort();
        // The shell can appear before Swagger parses its spec. Bind readiness
        // to every source operation, including operations shown under each tag.
        const rendered = await page.waitForFunction(expected => {
          const frame = document.querySelector<HTMLIFrameElement>('iframe[data-ohc-api-docs-viewer]');
          let owner = document;
          if (frame) {
            const source = new URL(frame.src, location.href);
            if (source.origin !== location.origin || source.pathname !== '/api-docs/viewer' || !frame.contentDocument || frame.contentDocument.location.pathname !== '/api-docs/viewer') return false;
            owner = frame.contentDocument;
          }
          const actual = Array.from(owner.querySelectorAll('.swagger-ui .opblock-summary-control')).map(element => JSON.stringify([
            element.closest('.opblock-tag-section')?.querySelector('.opblock-tag')?.getAttribute('data-tag'),
            element.querySelector('.opblock-summary-method')?.textContent?.trim(),
            element.querySelector('[data-path]')?.getAttribute('data-path'),
          ])).sort();
          return JSON.stringify(actual) === JSON.stringify(expected);
        }, expected, { timeout: 10_000 });
        await rendered.dispose();
      } finally { clearTimeout(timer); }
    },
  };
}

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
      const documentation = destination.pathname === '/api-docs' ? observeApiDocumentation(page, origin) : undefined;
      try {
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
        }), undefined, { timeout: destination.pathname === '/api-docs' ? 15_000 : 5000 });
        await ready.dispose();
        await documentation?.ready();
        const final = new URL(page.url());
        if (final.origin !== origin || final.username || final.password || final.pathname !== expectedPath) throw new Error('Audit navigation reached an unclassified destination route');
        return { requestedUrl: destination.href, finalUrl: final.href, redirected: destination.href !== final.href };
      } finally { documentation?.dispose(); }
    }
    throw new Error('Audit navigation did not reach a verified destination');
  };
}
