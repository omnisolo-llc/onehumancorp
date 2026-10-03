import { expect, test } from './fixtures';
import fs from 'node:fs';
import path from 'node:path';
import type { Page } from '@playwright/test';
import type { ClickEffects } from './support/ui_click_audit';
import { ATTACHMENT, INVENTORY_TITLE, PROTOCOL, discoverAppRoutes as discoverSourceRoutes } from '../../scripts/ui-click-audit.cjs';
import { hasMeaningfulClickEffect, hasFragmentTarget, observeClickEffects, replaceAuditDocument, resolveAuditTarget, clickableAuditSelector, tagClickTargets } from './support/ui_click_audit';
import { authenticateRequest } from './authenticate';
import { E2E_ADMIN_USER } from './identities';
import { createAuditNavigation, type AuditNavigationReceipt } from './support/ui_audit_navigation';
import { createDashboardAuditCase } from './support/dashboard_audit_fixture';
import { assertSameClickInventory } from '../../scripts/ui-audit-fixture.cjs';
import { runFiniteClickInventory, FINITE_CLICK_CASE_BUDGET } from '../../scripts/ui-audit-inventory.cjs';

const appRoot = path.resolve(__dirname, '../ui/next/src/app');
function discoverAppRoutes(): string[] { return discoverSourceRoutes(path.resolve(__dirname, '../..')); }

const clickableCssSelector = [
  'button:not([disabled])',
  '[role="button"]:not([aria-disabled="true"])',
  '[onclick]',
  'input[type="button"]:not([disabled])',
  'input[type="submit"]:not([disabled])',
  'input[type="reset"]:not([disabled])',
  'summary',
].join(', ');

const clickableSelector = clickableAuditSelector;

const interactiveCssSelector = [
  'a[href]',
  clickableCssSelector,
  'input:not([type="hidden"])',
  'select',
  'textarea',
].join(', ');

const interactiveSelector = [
  'a[href]:visible',
  clickableSelector,
  'input:visible:not([type="hidden"])',
  'select:visible',
  'textarea:visible',
].join(', ');


const viewports = [
  { name: 'desktop', width: 1440, height: 1000 },
  { name: 'mobile', width: 390, height: 844 },
];

function normalizeInternalHref(href: string, currentUrl: string): string | null {
  if (!href || href.startsWith('#') || href.startsWith('mailto:') || href.startsWith('tel:')) return null;
  if (href.startsWith('javascript:')) return 'javascript:';

  try {
    const base = new URL(currentUrl);
    const url = new URL(href, base);
    if (!['http:', 'https:'].includes(base.protocol) || url.origin !== base.origin || url.username || url.password) return null;
    return `${url.pathname}${url.search}`;
  } catch {
    return null;
  }
}

function routeLabel(route: string) {
  return route || '/';
}

const allowedExternalHosts = [
  'facebook.com',
  'linkedin.com',
  'meet.google.com',
  'cloud.omnisolo.co',
  'omnisolo.co',
  'twitter.com',
  'wa.me',
  'www.facebook.com',
  'www.linkedin.com',
  'x.com',
];

function externalHostAllowed(hostname: string) {
  return allowedExternalHosts.some((allowedHost) => hostname === allowedHost || hostname.endsWith(`.${allowedHost}`));
}

function isFakeOmniSoloUrl(href: string) {
  try {
    const url = new URL(href, 'http://dummy.base');
    return url.protocol === 'ohc:' || url.protocol === 'omnisolo:' || url.hostname === 'ohc.store' || url.hostname.endsWith('.ohc.store') || url.hostname === 'omnisolo.store' || url.hostname.endsWith('.omnisolo.store');
  } catch {
    return href.startsWith('ohc://') || href.startsWith('omnisolo://') || href.includes('ohc.store') || href.includes('omnisolo.store');
  }
}

async function visibleText(page: Page) {
  return page.locator('body').innerText({ timeout: 3000 }).catch(() => '');
}

const auditBaseURL = process.env.BASE_URL || 'http://127.0.0.1:18789';
const gotoReady = createAuditNavigation(auditBaseURL, async (page) => {
  await authenticateRequest(page.request, {
    username: E2E_ADMIN_USER.email,
    password: E2E_ADMIN_USER.password,
    organizationId: E2E_ADMIN_USER.organizationId,
  }, new URL(auditBaseURL).origin);
});



async function auditInteractivePurposeForRoute(page: Page, route: string) {
  await gotoReady(page, route);
  const results = await page.locator(interactiveSelector).evaluateAll((elements) =>
    elements.filter((element) => {
      const style = window.getComputedStyle(element);
      const rect = element.getBoundingClientRect();
      if (element.closest('[aria-hidden="true"]')) return false;
      if (element.closest('nextjs-portal')) return false;
      return style.visibility !== 'hidden' && style.display !== 'none' && style.opacity !== '0' && rect.width > 0 && rect.height > 0;
    }).map((element, index) => {
      const tag = element.tagName.toLowerCase();
      const type = element.getAttribute('type') || '';
      const href = element.getAttribute('href') || '';
      const role = element.getAttribute('role') || '';
      const purpose =
        element.getAttribute('aria-label') ||
        element.getAttribute('title') ||
        element.getAttribute('placeholder') ||
        element.getAttribute('name') ||
        element.getAttribute('value') ||
        Array.from((element as HTMLInputElement).labels || []).map((labelElement) => labelElement.textContent || '').join(' ').trim() ||
        (element.textContent || '').trim().replace(/\s+/g, ' ');
      const disabled = element.hasAttribute('disabled') || element.getAttribute('aria-disabled') === 'true';
      return { index, tag, type, href, role, purpose: purpose.trim(), disabled };
    }),
  );

  const failures: string[] = [];
  for (const result of results) {
    const target = `${route}: ${result.tag}${result.type ? `[type=${result.type}]` : ''} #${result.index + 1}`;
    if (!result.disabled && !result.purpose) {
      failures.push(`${target} has no visible or accessible designed purpose`);
    }
    if (result.tag === 'a') {
      if (!result.href.trim()) failures.push(`${target} has no href`);
      if (result.href.startsWith('#') && !await page.evaluate(hasFragmentTarget, result.href)) failures.push(`${target} uses a missing or placeholder fragment href`);
      if (result.href.startsWith('javascript:')) failures.push(`${target} uses a javascript: href`);
      if (isFakeOmniSoloUrl(result.href)) failures.push(`${target} uses fake OmniSolo destination ${result.href}`);
    }
    if ((result.tag === 'button' || result.role === 'button') && /^button$/i.test(result.purpose)) {
      failures.push(`${target} exposes only a generic button purpose`);
    }
  }

  return { auditedElements: results.length, failures };
}

type RouteClickAudit = {
  protocol: number; kind: 'route'; route: string; discoveredKeys: string[];
  observations: { key: string; completed: boolean; effect: ClickEffects | null; error: string | null }[];
  navigations: AuditNavigationReceipt[];
  exhausted: boolean; failures: string[]; assertionsPassed: boolean;
  isolation?: { kind: 'case-owned-postgres'; seedDigest: string; cases: { tenantId: string; userId: string; keys: string[] }[] };
  finiteInventory?: { caseBudgetMs: number; targetCount: number };
  timings?: { phase: string; target?: string; elapsedMs: number }[];
};

async function auditClickEffectsForRoute(sourcePage: Page, route: string, audit: RouteClickAudit) {
  const failures = audit.failures;
  const audited = new Set<string>();
  const startedAt = Date.now();
  const timed = async <T>(phase: string, operation: () => Promise<T>, target?: string): Promise<T> => {
    const started = Date.now();
    try { return await operation(); }
    finally { (audit.timings ??= []).push({ phase, target, elapsedMs: Date.now() - started }); }
  };
  if (route === '/dashboard' || route === '/') {
    const browser = sourcePage.context().browser();
    if (!browser) throw new Error('Dashboard click isolation requires the real test browser');
    let owned = await timed('seed', () => createDashboardAuditCase(browser, auditBaseURL, sourcePage.viewportSize(), test.info().outputPath('dashboard-audit-videos')));
    try {
      await timed('navigate', async () => { audit.navigations.push(await owned.navigate(route)); });
      const baseline = await timed('discover', () => tagClickTargets(owned.page, owned.actor.namespace, owned.actor.canonicalIds));
      audit.discoveredKeys = baseline.map(target => target.key);
      audit.isolation = { kind: 'case-owned-postgres', seedDigest: owned.actor.sourceDigest, cases: [] };
      // This is a finite frozen inventory, not an open-ended enumeration loop.
      // Budget each independently seeded/authenticated case and its existing
      // bounded lookup and gestures, rather than raising a flat route timeout.
      const caseBudget = 30_000;
      test.setTimeout(Math.max(120_000, (baseline.length + 2) * caseBudget));
      for (const candidate of baseline) {
        await test.step(`isolated dashboard control: ${candidate.label}`, async () => {
          const current = await timed('discover', () => tagClickTargets(owned.page, owned.actor.namespace, owned.actor.canonicalIds));
          assertSameClickInventory(audit.discoveredKeys, current.map(target => target.key));
          audit.isolation!.cases.push({ tenantId: owned.actor.tenantId, userId: owned.actor.userId, keys: current.map(target => target.key) });
          const target = await timed('resolve', () => resolveAuditTarget(owned.page, candidate.key, () => tagClickTargets(owned.page, owned.actor.namespace, owned.actor.canonicalIds)), candidate.label);
          audited.add(candidate.key);
          try {
            const observed = await timed('observe', () => observeClickEffects(owned.page, target), candidate.label);
            audit.observations.push({ key: candidate.key, completed: true, effect: observed, error: null });
            if (!hasMeaningfulClickEffect(observed)) {
              if (observed.dialogSeen) failures.push(`${route}: "${candidate.label}" only opened a browser dialog`);
              failures.push(`${route}: "${candidate.label}" produced no observable user effect`);
            }
          } catch (error) {
            audit.observations.push({ key: candidate.key, completed: false, effect: null, error: String(error).split('\n')[0] });
            failures.push(`${route}: "${candidate.label}" click failed: ${String(error).split('\n')[0]}`);
          }
          await timed('retire', () => owned.close());
          owned = await timed('seed', () => createDashboardAuditCase(browser, auditBaseURL, sourcePage.viewportSize(), test.info().outputPath('dashboard-audit-videos')));
          await timed('navigate', async () => { audit.navigations.push(await owned.navigate(route)); });
        }, { timeout: caseBudget });
      }
      // Even the final reset must preserve the full initial inventory. A
      // destructive click cannot erase another expected target from coverage.
      const final = await timed('discover', () => tagClickTargets(owned.page, owned.actor.namespace, owned.actor.canonicalIds));
      assertSameClickInventory(audit.discoveredKeys, final.map(target => target.key));
      audit.isolation.cases.push({ tenantId: owned.actor.tenantId, userId: owned.actor.userId, keys: final.map(target => target.key) });
      audit.exhausted = true;
      return { auditedTargets: audited.size, failures };
    } catch (error) {
      if (!owned.page.isClosed()) await test.info().attach('dashboard-isolation-failure', { body: await owned.page.screenshot({ fullPage: true }), contentType: 'image/png' });
      throw error;
    } finally { await owned.close(); }
  }
  // Restore with a committed blank document between clicks, retaining one
  // page/video for the route and destroying delayed callbacks from the old realm.
  let page = await sourcePage.context().newPage();
  try {
    await timed('navigate', async () => { audit.navigations.push(await gotoReady(page, route)); });
    if (route === '/api-docs') {
      const baseline = await timed('discover', () => tagClickTargets(page));
      audit.discoveredKeys = baseline.map(target => target.key);
      audit.finiteInventory = { caseBudgetMs: FINITE_CLICK_CASE_BUDGET, targetCount: baseline.length };
      test.setTimeout(Math.max(120_000, (baseline.length + 2) * FINITE_CLICK_CASE_BUDGET));
      await runFiniteClickInventory(baseline, {
        discover: () => timed('discover', () => tagClickTargets(page)),
        visit: async (candidate: { key: string; label: string }) => {
          const target = await timed('resolve', () => resolveAuditTarget(page, candidate.key, () => tagClickTargets(page)), candidate.label);
          audited.add(candidate.key);
          try {
            const observed = await timed('observe', () => observeClickEffects(page, target), candidate.label);
            audit.observations.push({ key: candidate.key, completed: true, effect: observed, error: null });
            if (!hasMeaningfulClickEffect(observed)) failures.push(`${route}: "${candidate.label}" produced no observable user effect`);
          } catch (error) {
            audit.observations.push({ key: candidate.key, completed: false, effect: null, error: String(error).split('\n')[0] });
            failures.push(`${route}: "${candidate.label}" click failed: ${String(error).split('\n')[0]}`);
          }
        },
        reset: async () => {
          page = await timed('retire', () => replaceAuditDocument(page));
          await timed('navigate', async () => { audit.navigations.push(await gotoReady(page, route)); });
        },
        step: (candidate: { label: string }, operation: () => Promise<void>, timeout: number) => test.step(`finite API documentation control: ${candidate.label}`, operation, { timeout }),
      });
      audit.exhausted = true;
      return { auditedTargets: audited.size, failures };
    }
    while (true) {
      const candidates = await timed('discover', () => tagClickTargets(page));
      for (const target of candidates) if (!audit.discoveredKeys.includes(target.key)) audit.discoveredKeys.push(target.key);
      const candidate = candidates.find((target) => !audited.has(target.key));
      if (!candidate) { audit.exhausted = true; break; }
      if (Date.now() - startedAt > 90_000) {
        throw new Error(`${route}: click target enumeration did not converge after ${audited.size} targets; next=${candidate.label}. No remaining coverage was silently skipped.`);
      }
      const target = await timed('resolve', () => resolveAuditTarget(page, candidate.key, () => tagClickTargets(page)), candidate.label);
      audited.add(candidate.key);
      try {
        const observed = await timed('observe', () => observeClickEffects(page, target), candidate.label);
        audit.observations.push({ key: candidate.key, completed: true, effect: observed, error: null });
        if (!hasMeaningfulClickEffect(observed)) {
          if (observed.dialogSeen) failures.push(`${route}: "${candidate.label}" only opened a browser dialog`);
          failures.push(`${route}: "${candidate.label}" produced no observable user effect`);
        }
      } catch (error) {
        audit.observations.push({ key: candidate.key, completed: false, effect: null, error: String(error).split('\n')[0] });
        failures.push(`${route}: "${candidate.label}" click failed: ${String(error).split('\n')[0]}`);
      }
      page = await timed('retire', () => replaceAuditDocument(page));
      await timed('navigate', async () => { audit.navigations.push(await gotoReady(page, route)); });
    }
    return { auditedTargets: audited.size, failures };
  } finally {
    await page.close().catch(() => undefined);
  }
}

const generatedContractRoutes = discoverAppRoutes();

test.describe('comprehensive UI contract', () => {
  test.describe.configure({ timeout: 300000 });

  test('per-route exhaustive UI element contracts cover at least 100 additional checks', async () => {
    expect(generatedContractRoutes.length, 'Route discovery must find enough pages for 100+ generated UI contracts.').toBeGreaterThanOrEqual(50);
    expect(generatedContractRoutes.length * 2).toBeGreaterThanOrEqual(100);
  });

  for (const route of generatedContractRoutes) {
    test(`all visible interactive elements declare their designed purpose on ${routeLabel(route)}`, async ({ page }) => {
      test.setTimeout(120000);
      const audit = await auditInteractivePurposeForRoute(page, route);
      console.info(`Audited ${audit.auditedElements} interactive elements on ${routeLabel(route)}.`);
      expect(audit.failures).toEqual([]);
    });

    test(`all visible enabled buttons and click targets have an effect on ${routeLabel(route)}`, async ({ page }) => {
      test.setTimeout(120000);
      const audit: RouteClickAudit = { protocol: PROTOCOL, kind: 'route', route, discoveredKeys: [], observations: [], navigations: [], exhausted: false, failures: [], assertionsPassed: false };
      try {
        const result = await auditClickEffectsForRoute(page, route, audit);
        console.info(`Audited ${result.auditedTargets} click targets on ${routeLabel(route)}.`);
        expect(audit.failures).toEqual([]);
        audit.assertionsPassed = true;
      } finally {
        await test.info().attach(ATTACHMENT, { body: Buffer.from(JSON.stringify(audit)), contentType: 'application/json' });
      }
    });
  }

  test('every app page loads without visible crash output', async ({ page }) => {
    expect(fs.existsSync(appRoot), 'Next UI source/routes are not available in this Playwright runfiles tree.').toBeTruthy();
    test.setTimeout(180000);
    const failures: string[] = [];
    const appRoutes = discoverAppRoutes();
    console.info(`Discovered ${appRoutes.length} app routes for load audit.`);
    expect(appRoutes.length, 'App route discovery must include at least one page.').toBeGreaterThan(0);

    page.on('pageerror', (error) => {
      failures.push(`uncaught page error: ${error.message}`);
    });

    for (const route of appRoutes) {
      const navigation = await gotoReady(page, route);
      // Check the verified final document with the same authenticated context.
      // The share-card's initial shell is not the document being audited.
      const response = await page.request.get(navigation.finalUrl, { failOnStatusCode: false });
      const status = response?.status() ?? 0;
      if (status >= 400) {
        failures.push(`${routeLabel(route)}: HTTP ${status}`);
        continue;
      }

      if (route === '/orders/e2e-seeded-record') {
        // The dynamic example is a persisted order, not a tolerated missing
        // record. Wait for its actual read instead of accepting a loading shell.
        await expect(page.getByRole('heading', { name: 'Order Summary', exact: true })).toBeVisible();
        await expect(page.getByText('e2e-seeded-record', { exact: true })).toBeVisible();
      }
      const bodyText = await visibleText(page);
      if (/404|not found|application error|failed to load/i.test(bodyText)) {
        failures.push(`${routeLabel(route)}: visible error text found`);
      }
    }

    expect(failures).toEqual([]);
  });

  test('visible internal links resolve to real pages', async ({ page }) => {
    test.setTimeout(180000);
    const failures: string[] = [];
    const checked = new Set<string>();
    const appRoutes = discoverAppRoutes();
    console.info(`Discovered ${appRoutes.length} app routes for internal link audit.`);
    expect(appRoutes.length, 'App route discovery must include at least one page.').toBeGreaterThan(0);

    for (const route of appRoutes) {
      await gotoReady(page, route);
      const hrefs = await page.locator('a[href]').evaluateAll((anchors) =>
        anchors
          .filter((anchor) => {
            const style = window.getComputedStyle(anchor);
            const rect = anchor.getBoundingClientRect();
            if (anchor.closest('[aria-hidden="true"]')) return false;
            return style.visibility !== 'hidden' && style.display !== 'none' && style.opacity !== '0' && rect.width > 0 && rect.height > 0;
          })
          .map((anchor) => (anchor as HTMLAnchorElement).getAttribute('href') || ''),
      );

      for (const rawHref of hrefs) {
        const href = normalizeInternalHref(rawHref, page.url());
        if (!href) continue;
        if (href === 'javascript:') {
          failures.push(`${routeLabel(route)}: javascript: link`);
          continue;
        }
        if (checked.has(href)) continue;
        checked.add(href);

        const response = await page.request.get(href, { failOnStatusCode: false });
        if (response.status() >= 400) {
          failures.push(`${routeLabel(route)}: ${href} resolved with HTTP ${response.status()}`);
        }
      }
    }

    expect(failures).toEqual([]);
  });

  test('visible external and protocol links use expected destinations', async ({ page }) => {
    test.setTimeout(180000);
    const failures: string[] = [];
    const appRoutes = discoverAppRoutes();
    console.info(`Discovered ${appRoutes.length} app routes for external/protocol link audit.`);
    expect(appRoutes.length, 'App route discovery must include at least one page.').toBeGreaterThan(0);

    for (const route of appRoutes) {
      await gotoReady(page, route);
      const hrefs = await page.locator('a[href]').evaluateAll((anchors) =>
        anchors
          .filter((anchor) => {
            const style = window.getComputedStyle(anchor);
            const rect = anchor.getBoundingClientRect();
            if (anchor.closest('[aria-hidden="true"]')) return false;
            return style.visibility !== 'hidden' && style.display !== 'none' && style.opacity !== '0' && rect.width > 0 && rect.height > 0;
          })
          .map((anchor, index) => ({
            index,
            href: (anchor as HTMLAnchorElement).getAttribute('href') || '',
            target: anchor.getAttribute('target') || '',
            rel: anchor.getAttribute('rel') || '',
            text: (anchor.textContent || '').trim().replace(/\s+/g, ' '),
          })),
      );

      for (const link of hrefs) {
        const target = `${routeLabel(route)}: link #${link.index + 1}${link.text ? ` "${link.text}"` : ''}`;
        if (!link.href.trim()) {
          failures.push(`${target} has an empty href`);
          continue;
        }
        if (link.href.startsWith('#')) {
          if (!await page.evaluate(hasFragmentTarget, link.href)) failures.push(`${target} uses a missing or placeholder fragment href`);
          continue;
        }
        if (link.href.startsWith('javascript:')) {
          failures.push(`${target} uses a javascript: href`);
          continue;
        }
        if (isFakeOmniSoloUrl(link.href)) {
          failures.push(`${target} uses fake OmniSolo destination ${link.href}`);
          continue;
        }
        if (link.href.startsWith('mailto:') || link.href.startsWith('tel:')) {
          continue;
        }

        const url = new URL(link.href, page.url());
        if (normalizeInternalHref(link.href, page.url()) !== null) continue;

        if (!['http:', 'https:'].includes(url.protocol)) {
          failures.push(`${target} uses unexpected protocol ${url.protocol}`);
        }
        if (url.username || url.password) failures.push(`${target} embeds credentials in its destination`);
        if (!externalHostAllowed(url.hostname)) {
          failures.push(`${target} points at unexpected external host ${url.hostname}`);
        }
        if (link.target === '_blank' && (!link.rel.includes('noopener') || !link.rel.includes('noreferrer'))) {
          failures.push(`${target} opens a new tab without rel="noopener noreferrer"`);
        }
      }
    }

    expect(failures).toEqual([]);
  });

  test(INVENTORY_TITLE, async () => {
    expect(process.env.OHC_CLICK_AUDIT_CONTEXT, 'Complete coverage requires the native runner and mandatory post-run receipt verification.').toBeTruthy();
    const routes = discoverAppRoutes();
    expect(routes.length, 'App route discovery must include at least one page.').toBeGreaterThan(0);
    // The required post-run gate evaluates completed per-route click assertions
    // for this exact inventory, without executing each business action twice.
    await test.info().attach(ATTACHMENT, {
      body: Buffer.from(JSON.stringify({ protocol: PROTOCOL, kind: 'inventory', routes, assertionsPassed: true })),
      contentType: 'application/json',
    });
  });

  test('all visible interactive elements are usable and named', async ({ page }) => {
    test.setTimeout(180000);
    const failures: string[] = [];
    const appRoutes = discoverAppRoutes();
    let auditedElements = 0;
    console.info(`Discovered ${appRoutes.length} app routes for interactive element audit.`);
    expect(appRoutes.length, 'App route discovery must include at least one page.').toBeGreaterThan(0);

    for (const route of appRoutes) {
      await gotoReady(page, route);
      const results = await page.locator(interactiveSelector).evaluateAll((elements) =>
        elements.filter((element) => {
          const style = window.getComputedStyle(element);
          if (element.closest('[aria-hidden="true"]')) return false;
          return style.opacity !== '0';
        }).map((element, index) => {
          const rect = element.getBoundingClientRect();
          const style = window.getComputedStyle(element);
          const tag = element.tagName.toLowerCase();
          const type = element.getAttribute('type') || '';
          const label =
            element.getAttribute('aria-label') ||
            element.getAttribute('title') ||
            element.getAttribute('placeholder') ||
            Array.from((element as HTMLInputElement).labels || []).map((labelElement) => labelElement.textContent || '').join(' ').trim() ||
            (element.textContent || '').trim();

          let hiddenAncestor: string | null = null;
          if (rect.width < 1 || rect.height < 1) {
            for (let ancestor: Element | null = element; ancestor; ancestor = ancestor.parentElement) {
              const ancestorStyle = window.getComputedStyle(ancestor);
              if (ancestor.hasAttribute('hidden') || ancestorStyle.display === 'none' || ancestorStyle.visibility === 'hidden') {
                hiddenAncestor = `${ancestor.tagName.toLowerCase()}#${ancestor.id} ${ancestor.getAttribute('class') || ''}`.slice(0, 240);
                break;
              }
            }
          }

          return {
            index,
            tag,
            type,
            label: label.trim(),
            width: rect.width,
            height: rect.height,
            pointerEvents: style.pointerEvents,
            disabled: element.hasAttribute('disabled') || element.getAttribute('aria-disabled') === 'true',
            id: element.id.slice(0, 120),
            classes: (element.getAttribute('class') || '').slice(0, 240),
            display: style.display,
            visibility: style.visibility,
            hiddenAncestor,
          };
        }),
      );
      auditedElements += results.length;

      for (const result of results) {
        const target = `${route}: ${result.tag}${result.type ? `[type=${result.type}]` : ''} #${result.index + 1}`;
        if (result.width < 1 || result.height < 1) {
          failures.push(`${target} has no rendered hit area: ${JSON.stringify({ label: result.label.slice(0, 200), id: result.id, classes: result.classes, width: result.width, height: result.height, display: result.display, visibility: result.visibility, hiddenAncestor: result.hiddenAncestor })}`);
        }
        if (!result.disabled && result.pointerEvents === 'none') failures.push(`${target} has pointer-events disabled`);
        if (!result.disabled && !result.label) failures.push(`${target} has no accessible label/text/placeholder/title`);
      }
    }

    console.info(`Audited ${auditedElements} visible interactive elements.`);
    expect(failures).toEqual([]);
  });

  test('layouts do not overflow or overlap click targets on desktop and mobile', async ({ page }) => {
    test.setTimeout(240000);
    const failures: string[] = [];
    const appRoutes = discoverAppRoutes();
    let auditedLayouts = 0;
    console.info(`Discovered ${appRoutes.length} app routes for layout audit across ${viewports.length} viewports.`);
    expect(appRoutes.length, 'App route discovery must include at least one page.').toBeGreaterThan(0);

    for (const viewport of viewports) {
      await page.setViewportSize({ width: viewport.width, height: viewport.height });

      for (const route of appRoutes) {
        await gotoReady(page, route);
        auditedLayouts += 1;
        const layout = await page.evaluate((selector) => {
          const documentElement = document.documentElement;
          const body = document.body;
          const horizontalOverflow = Math.max(documentElement.scrollWidth, body.scrollWidth) - window.innerWidth;
          const verticalOverflow = Math.max(documentElement.scrollHeight, body.scrollHeight) - window.innerHeight;

          const elements = Array.from(document.querySelectorAll(selector))
            .filter((element) => {
              const rect = element.getBoundingClientRect();
              const style = window.getComputedStyle(element);
              if (element.closest('[data-ui-overlay="true"]')) return false;
              if (element.closest('[aria-hidden="true"]')) return false;
              if (element.closest('nextjs-portal')) return false;
              return style.visibility !== 'hidden' && style.display !== 'none' && style.opacity !== '0' && rect.width > 0 && rect.height > 0;
            })
            .map((element, index) => {
              const rect = element.getBoundingClientRect();
              return {
                index,
                label: element.getAttribute('aria-label') || (element.textContent || '').trim().replace(/\s+/g, ' ').slice(0, 80) || element.tagName.toLowerCase(),
                left: rect.left,
                top: rect.top,
                right: rect.right,
                bottom: rect.bottom,
                width: rect.width,
                height: rect.height,
              };
            });

          const overlaps: string[] = [];
          for (let i = 0; i < elements.length; i += 1) {
            for (let j = i + 1; j < elements.length; j += 1) {
              const a = elements[i];
              const b = elements[j];
              const overlapX = Math.max(0, Math.min(a.right, b.right) - Math.max(a.left, b.left));
              const overlapY = Math.max(0, Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top));
              const overlapArea = overlapX * overlapY;
              if (overlapArea === 0) continue;
              const smallerArea = Math.min(a.width * a.height, b.width * b.height);
              if (smallerArea > 0 && overlapArea / smallerArea > 0.35) {
                overlaps.push(`"${a.label}" overlaps "${b.label}"`);
              }
            }
          }

          return { horizontalOverflow, verticalOverflow, overlaps };
        }, interactiveCssSelector);

        if (layout.horizontalOverflow > 2) {
          failures.push(`${route} (${viewport.name}) horizontal overflow ${Math.round(layout.horizontalOverflow)}px`);
        }
        if (layout.verticalOverflow < -2) {
          failures.push(`${route} (${viewport.name}) invalid vertical layout measurement`);
        }
        for (const overlap of layout.overlaps) {
          failures.push(`${route} (${viewport.name}) ${overlap}`);
        }
      }
    }

    console.info(`Audited ${auditedLayouts} route/viewport layout combinations.`);
    expect(failures).toEqual([]);
  });
});
