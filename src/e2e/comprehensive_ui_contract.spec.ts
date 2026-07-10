import { expect, test } from './fixtures';
import fs from 'node:fs';
import path from 'node:path';
import type { Page } from '@playwright/test';

const appRoot = path.resolve(__dirname, '../ui/next/src/app');
const ignoredRouteSegments = new Set(['api']);
const dynamicRouteExamples: Record<string, string> = {
  '[articleId]': 'getting-started-1',
  '[tenant]': 'default',
  '[id]': 'e2e-id',
};

function walkFiles(dir: string): string[] {
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  return entries.flatMap((entry) => {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) return walkFiles(fullPath);
    return entry.isFile() || entry.isSymbolicLink() ? [fullPath] : [];
  });
}

function routeFromPageFile(file: string): string | null {
  const relativeDir = path.relative(appRoot, path.dirname(file));
  const segments = relativeDir === '' ? [] : relativeDir.split(path.sep);
  if (segments.some((segment) => ignoredRouteSegments.has(segment) || segment.startsWith('('))) {
    return null;
  }

  const routeSegments = segments
    .filter((segment) => !segment.startsWith('_'))
    .map((segment) => dynamicRouteExamples[segment] || segment);

  return `/${routeSegments.join('/')}`.replace(/\/$/, '') || '/';
}

function discoverAppRoutes() {
  return Array.from(new Set(
    walkFiles(appRoot)
      .filter((file) => file.endsWith(`${path.sep}page.tsx`))
      .map(routeFromPageFile)
      .filter((route): route is string => Boolean(route)),
  )).sort();
}

const clickableCssSelector = [
  'button:not([disabled])',
  '[role="button"]:not([aria-disabled="true"])',
  '[onclick]',
  'input[type="button"]:not([disabled])',
  'input[type="submit"]:not([disabled])',
  'input[type="reset"]:not([disabled])',
  'summary',
].join(', ');

const clickableSelector = [
  'button:visible:not([disabled])',
  '[role="button"]:visible:not([aria-disabled="true"])',
  '[onclick]:visible',
  'input[type="button"]:visible:not([disabled])',
  'input[type="submit"]:visible:not([disabled])',
  'input[type="reset"]:visible:not([disabled])',
  'summary:visible',
].join(', ');

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

function normalizeInternalHref(href: string): string | null {
  if (!href || href.startsWith('#') || href.startsWith('mailto:') || href.startsWith('tel:')) return null;
  if (href.startsWith('javascript:')) return 'javascript:';

  try {
    const url = new URL(href, 'http://dummy.base');
    if (url.origin !== 'http://dummy.base') return null;
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
  'meet.google.com',
  'ohc.app',
  'onehumancorp.com',
  'twitter.com',
  'wa.me',
  'www.facebook.com',
  'x.com',
];

function externalHostAllowed(hostname: string) {
  return allowedExternalHosts.some((allowedHost) => hostname === allowedHost || hostname.endsWith(`.${allowedHost}`));
}

function isFakeOHCUrl(href: string) {
  try {
    const url = new URL(href, 'http://dummy.base');
    return url.protocol === 'ohc:' || url.hostname === 'ohc.store' || url.hostname.endsWith('.ohc.store');
  } catch {
    return href.startsWith('ohc://') || href.includes('ohc.store');
  }
}

async function visibleText(page: Page) {
  return page.locator('body').innerText({ timeout: 3000 }).catch(() => '');
}

async function pageSignature(page: Page) {
  return page.evaluate(() => {
    const body = document.body;
    const text = body?.textContent || '';
    const html = body?.innerHTML || '';
    const elementCount = document.querySelectorAll('*').length;
    const checksum = (value: string) => {
      let hash = 0;
      for (let index = 0; index < value.length; index += 1) {
        hash = ((hash << 5) - hash + value.charCodeAt(index)) | 0;
      }
      return hash;
    };
    return `${location.href}|${checksum(text)}|${checksum(html)}|${elementCount}`;
  }).catch(() => page.url());
}

async function waitForClickEffect(page: Page, beforeUrl: string, beforeSignature: string) {
  for (let attempt = 0; attempt < 6; attempt += 1) {
    await page.waitForTimeout(50);
    const afterUrl = page.url();
    const afterSignature = await pageSignature(page);
    if (afterUrl !== beforeUrl || afterSignature !== beforeSignature) {
      return { afterUrl, afterSignature, changed: true };
    }
  }

  return { afterUrl: page.url(), afterSignature: await pageSignature(page), changed: false };
}

async function gotoReady(page: Page, route: string) {
  await page.goto(process.env.BASE_URL ? `${process.env.BASE_URL}${route}` : `http://127.0.0.1:18789${route}`, { waitUntil: 'domcontentloaded' });
  await page.waitForLoadState('networkidle', { timeout: 1000 }).catch(() => undefined);
  await page.waitForTimeout(100);
  await page.evaluate(() => {
    const controls = Array.from(document.querySelectorAll('input, textarea')) as Array<HTMLInputElement | HTMLTextAreaElement>;
    for (const control of controls) {
      const style = window.getComputedStyle(control);
      const rect = control.getBoundingClientRect();
      if (style.visibility === 'hidden' || style.display === 'none' || rect.width === 0 || rect.height === 0) continue;
      if (control.disabled || control.readOnly || control.value) continue;
      if (control instanceof HTMLInputElement) {
        if (['button', 'checkbox', 'file', 'hidden', 'image', 'radio', 'range', 'reset', 'submit'].includes(control.type)) continue;
        control.value = control.type === 'url' ? 'https://ohc.app' : control.type === 'number' ? '1' : 'Audit value';
      } else {
        control.value = 'Audit value';
      }
      control.dispatchEvent(new Event('input', { bubbles: true }));
      control.dispatchEvent(new Event('change', { bubbles: true }));
    }
  }).catch(() => undefined);
}

async function tagClickTargets(page: Page) {
  return page.locator(clickableSelector).evaluateAll((elements) => {
    const visibleTargets = elements.filter((element) => {
      const style = window.getComputedStyle(element);
      if (element.closest('[aria-hidden="true"]')) return false;
      if (element.closest('nextjs-portal')) return false;
      return style.pointerEvents !== 'none' && style.opacity !== '0';
    });
    visibleTargets.forEach((element, index) => {
      element.setAttribute('data-ui-audit-click-index', String(index));
    });
    return visibleTargets.length;
  });
}

async function describeTaggedTarget(page: Page, index: number) {
  return page.locator(`[data-ui-audit-click-index="${index}"]`).evaluate((element, fallbackIndex) => {
    const aria = element.getAttribute('aria-label');
    const text = (element.textContent || '').trim().replace(/\s+/g, ' ');
    const id = element.id ? `#${element.id}` : '';
    const role = element.getAttribute('role');
    return aria || text || role || `${element.tagName.toLowerCase()}${id} #${Number(fallbackIndex) + 1}`;
  }, index).catch(() => `click target #${index + 1}`);
}

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
      if (result.href === '#' || result.href.startsWith('#')) failures.push(`${target} uses a placeholder hash href`);
      if (result.href.startsWith('javascript:')) failures.push(`${target} uses a javascript: href`);
      if (isFakeOHCUrl(result.href)) failures.push(`${target} uses fake OHC destination ${result.href}`);
    }
    if ((result.tag === 'button' || result.role === 'button') && /^button$/i.test(result.purpose)) {
      failures.push(`${target} exposes only a generic button purpose`);
    }
  }

  return { auditedElements: results.length, failures };
}

async function auditClickEffectsForRoute(page: Page, route: string) {
  await gotoReady(page, route);
  const failures: string[] = [];
  let auditedTargets = 0;
  const targetCount = await tagClickTargets(page);
  auditedTargets += targetCount;

  for (let index = 0; index < targetCount; index += 1) {
    let target = page.locator(`[data-ui-audit-click-index="${index}"]`);
    if (await target.count() === 0) {
      await gotoReady(page, route);
      await tagClickTargets(page);
      target = page.locator(`[data-ui-audit-click-index="${index}"]`);
    }

    if (await target.count() === 0) {
      break;
    }

    const isCurrentSelection = await target.evaluate((element) =>
      element.getAttribute('aria-pressed') === 'true' ||
      element.getAttribute('aria-current') === 'page' ||
      element.getAttribute('aria-selected') === 'true',
    ).catch(() => false);
    if (isCurrentSelection) continue;

    const label = await describeTaggedTarget(page, index);
    const beforeUrl = page.url();
    const beforeSignature = await pageSignature(page);
    let dialogSeen = false;
    let requestSeen = false;

    const dialogPromise = page.waitForEvent('dialog', { timeout: 75 })
      .then(async (dialog) => {
        dialogSeen = true;
        await dialog.dismiss().catch(() => undefined);
      })
      .catch(() => undefined);
    const requestPromise = page.waitForEvent('request', { timeout: 75 })
      .then(() => { requestSeen = true; })
      .catch(() => undefined);

    await target.evaluate((element) => {
      (element as HTMLElement).click();
    }, undefined, { timeout: 500 }).catch((error) => {
      failures.push(`${route}: "${label}" click failed: ${error.message.split('\n')[0]}`);
    });
    await Promise.all([dialogPromise, requestPromise]);

    const effect = await waitForClickEffect(page, beforeUrl, beforeSignature);
    const realEffect = requestSeen || effect.changed;

    if (dialogSeen && !realEffect) {
      failures.push(`${route}: "${label}" only opened a browser dialog`);
    }
    if (!realEffect) {
      failures.push(`${route}: "${label}" produced no navigation, network request, or DOM change`);
    }
    if (effect.afterUrl !== beforeUrl || effect.changed) {
      await gotoReady(page, route);
      await tagClickTargets(page);
    }
  }

  return { auditedTargets, failures };
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
      const audit = await auditClickEffectsForRoute(page, route);
      console.info(`Audited ${audit.auditedTargets} click targets on ${routeLabel(route)}.`);
      expect(audit.failures).toEqual([]);
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
      const response = await page.goto(process.env.BASE_URL ? `${process.env.BASE_URL}${route}` : `http://127.0.0.1:18789${route}`, { waitUntil: 'domcontentloaded' });
      const status = response?.status() ?? 0;
      if (status >= 400) {
        failures.push(`${routeLabel(route)}: HTTP ${status}`);
        continue;
      }

      const bodyText = await visibleText(page);
      if (/404|not found|application error|failed to load/i.test(bodyText)) {
        failures.push(`${routeLabel(route)}: visible error text found`);
      }
    }

    expect(failures).toEqual([]);
  });

  test('visible internal links resolve to real pages', async ({ page, request }) => {
    test.setTimeout(180000);
    const failures: string[] = [];
    const checked = new Set<string>();
    const appRoutes = discoverAppRoutes();
    console.info(`Discovered ${appRoutes.length} app routes for internal link audit.`);
    expect(appRoutes.length, 'App route discovery must include at least one page.').toBeGreaterThan(0);

    for (const route of appRoutes) {
      await page.goto(process.env.BASE_URL ? `${process.env.BASE_URL}${route}` : `http://127.0.0.1:18789${route}`, { waitUntil: 'domcontentloaded' });
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
        const href = normalizeInternalHref(rawHref);
        if (!href) continue;
        if (href === 'javascript:') {
          failures.push(`${routeLabel(route)}: javascript: link`);
          continue;
        }
        if (checked.has(href)) continue;
        checked.add(href);

        const response = await request.get(href, { failOnStatusCode: false });
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
      await page.goto(process.env.BASE_URL ? `${process.env.BASE_URL}${route}` : `http://127.0.0.1:18789${route}`, { waitUntil: 'domcontentloaded' });
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
        if (link.href === '#' || link.href.startsWith('#')) {
          failures.push(`${target} uses a placeholder hash href`);
          continue;
        }
        if (link.href.startsWith('javascript:')) {
          failures.push(`${target} uses a javascript: href`);
          continue;
        }
        if (isFakeOHCUrl(link.href)) {
          failures.push(`${target} uses fake OHC destination ${link.href}`);
          continue;
        }
        if (link.href.startsWith('mailto:') || link.href.startsWith('tel:')) {
          continue;
        }

        const url = new URL(link.href, 'http://dummy.base');
        if (url.origin === 'http://dummy.base') continue;

        if (!['http:', 'https:'].includes(url.protocol)) {
          failures.push(`${target} uses unexpected protocol ${url.protocol}`);
        }
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

  test('visible enabled click targets have an observable effect', async ({ page }) => {
    test.setTimeout(600000);
    const failures: string[] = [];
    const appRoutes = discoverAppRoutes();
    let auditedTargets = 0;
    console.info(`Discovered ${appRoutes.length} app routes for click target audit.`);
    expect(appRoutes.length, 'App route discovery must include at least one page.').toBeGreaterThan(0);

    for (const route of appRoutes) {
      await gotoReady(page, route);
      const targetCount = await tagClickTargets(page);
      auditedTargets += targetCount;

      for (let index = 0; index < targetCount; index += 1) {
        let target = page.locator(`[data-ui-audit-click-index="${index}"]`);
        if (await target.count() === 0) {
          await gotoReady(page, route);
          await tagClickTargets(page);
          target = page.locator(`[data-ui-audit-click-index="${index}"]`);
        }

        if (await target.count() === 0) {
          break;
        }

        const isCurrentSelection = await target.evaluate((element) =>
          element.getAttribute('aria-pressed') === 'true' ||
          element.getAttribute('aria-current') === 'page' ||
          element.getAttribute('aria-selected') === 'true',
        ).catch(() => false);
        if (isCurrentSelection) continue;

        const label = await describeTaggedTarget(page, index);
        const beforeUrl = page.url();
        const beforeSignature = await pageSignature(page);
        let dialogSeen = false;
        let requestSeen = false;

        const dialogPromise = page.waitForEvent('dialog', { timeout: 75 })
          .then(async (dialog) => {
            dialogSeen = true;
            await dialog.dismiss().catch(() => undefined);
          })
          .catch(() => undefined);
        const requestPromise = page.waitForEvent('request', { timeout: 75 })
          .then(() => { requestSeen = true; })
          .catch(() => undefined);

        await target.evaluate((element) => {
          (element as HTMLElement).click();
        }, undefined, { timeout: 500 }).catch((error) => {
          failures.push(`${route}: "${label}" click failed: ${error.message.split('\n')[0]}`);
        });
        await Promise.all([dialogPromise, requestPromise]);

        const effect = await waitForClickEffect(page, beforeUrl, beforeSignature);
        const realEffect = requestSeen || effect.changed;

        if (dialogSeen && !realEffect) {
          failures.push(`${route}: "${label}" only opened a browser dialog`);
        }
        if (!realEffect) {
          failures.push(`${route}: "${label}" produced no navigation, network request, or DOM change`);
        }
        if (effect.afterUrl !== beforeUrl || effect.changed) {
          await gotoReady(page, route);
          await tagClickTargets(page);
        }
      }
    }

    console.info(`Audited ${auditedTargets} visible enabled click targets.`);
    expect(failures).toEqual([]);
  });

  test('all visible interactive elements are usable and named', async ({ page }) => {
    test.setTimeout(180000);
    const failures: string[] = [];
    const appRoutes = discoverAppRoutes();
    let auditedElements = 0;
    console.info(`Discovered ${appRoutes.length} app routes for interactive element audit.`);
    expect(appRoutes.length, 'App route discovery must include at least one page.').toBeGreaterThan(0);

    for (const route of appRoutes) {
      await page.goto(process.env.BASE_URL ? `${process.env.BASE_URL}${route}` : `http://127.0.0.1:18789${route}`, { waitUntil: 'domcontentloaded' });
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

          return {
            index,
            tag,
            type,
            label: label.trim(),
            width: rect.width,
            height: rect.height,
            pointerEvents: style.pointerEvents,
            disabled: element.hasAttribute('disabled') || element.getAttribute('aria-disabled') === 'true',
          };
        }),
      );
      auditedElements += results.length;

      for (const result of results) {
        const target = `${route}: ${result.tag}${result.type ? `[type=${result.type}]` : ''} #${result.index + 1}`;
        if (result.width < 1 || result.height < 1) failures.push(`${target} has no rendered hit area`);
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
        await page.goto(process.env.BASE_URL ? `${process.env.BASE_URL}${route}` : `http://127.0.0.1:18789${route}`, { waitUntil: 'domcontentloaded' });
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
