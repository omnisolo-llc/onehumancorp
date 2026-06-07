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
    return entry.isFile() ? [fullPath] : [];
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
    const url = new URL(href, 'http://localhost:3000');
    if (url.origin !== 'http://localhost:3000') return null;
    return `${url.pathname}${url.search}`;
  } catch {
    return null;
  }
}

async function visibleText(page: Page) {
  return page.locator('body').innerText({ timeout: 3000 }).catch(() => '');
}

async function describeTarget(page: Page, selector: string, index: number) {
  return page.locator(selector).nth(index).evaluate((element, fallbackIndex) => {
    const aria = element.getAttribute('aria-label');
    const text = (element.textContent || '').trim().replace(/\s+/g, ' ');
    const id = element.id ? `#${element.id}` : '';
    const role = element.getAttribute('role');
    return aria || text || role || `${element.tagName.toLowerCase()}${id} #${Number(fallbackIndex) + 1}`;
  }, index).catch(() => `${selector} #${index + 1}`);
}

test.describe('comprehensive UI contract', () => {
  test('every app page loads without visible crash output', async ({ page }) => {
    expect(fs.existsSync(appRoot), 'Next UI source/routes are not available in this Playwright runfiles tree.').toBeTruthy();
    test.setTimeout(180000);
    const failures: string[] = [];
    const appRoutes = discoverAppRoutes();

    page.on('pageerror', (error) => {
      failures.push(`uncaught page error: ${error.message}`);
    });

    for (const route of appRoutes) {
      const response = await page.goto(route, { waitUntil: 'domcontentloaded' });
      const status = response?.status() ?? 0;
      if (status >= 400) {
        failures.push(`${route}: HTTP ${status}`);
        continue;
      }

      const bodyText = await visibleText(page);
      if (/404|not found|application error|failed to load/i.test(bodyText)) {
        failures.push(`${route}: visible error text found`);
      }
    }

    expect(failures).toEqual([]);
  });

  test('visible internal links resolve to real pages', async ({ page, request }) => {
    test.setTimeout(180000);
    const failures: string[] = [];
    const checked = new Set<string>();
    const appRoutes = discoverAppRoutes();

    for (const route of appRoutes) {
      await page.goto(route, { waitUntil: 'domcontentloaded' });
      const hrefs = await page.locator('a[href]').evaluateAll((anchors) =>
        anchors
          .filter((anchor) => {
            const style = window.getComputedStyle(anchor);
            const rect = anchor.getBoundingClientRect();
            return style.visibility !== 'hidden' && style.display !== 'none' && rect.width > 0 && rect.height > 0;
          })
          .map((anchor) => (anchor as HTMLAnchorElement).getAttribute('href') || ''),
      );

      for (const rawHref of hrefs) {
        const href = normalizeInternalHref(rawHref);
        if (!href) continue;
        if (href === 'javascript:') {
          failures.push(`${route}: javascript: link`);
          continue;
        }
        if (checked.has(href)) continue;
        checked.add(href);

        const response = await request.get(href, { failOnStatusCode: false });
        if (response.status() >= 400) {
          failures.push(`${route}: ${href} resolved with HTTP ${response.status()}`);
        }
      }
    }

    expect(failures).toEqual([]);
  });

  test('visible enabled click targets have an observable effect', async ({ page }) => {
    test.setTimeout(240000);
    const failures: string[] = [];
    const appRoutes = discoverAppRoutes();

    for (const route of appRoutes) {
      await page.goto(route, { waitUntil: 'domcontentloaded' });
      const targetCount = await page.locator(clickableSelector).count();

      for (let index = 0; index < targetCount; index += 1) {
        await page.goto(route, { waitUntil: 'domcontentloaded' });
        const target = page.locator(clickableSelector).nth(index);
        const label = await describeTarget(page, clickableSelector, index);
        const beforeUrl = page.url();
        const beforeText = await visibleText(page);
        let dialogSeen = false;
        let requestSeen = false;
        let responseSeen = false;

        const dialogPromise = page.waitForEvent('dialog', { timeout: 1500 })
          .then(async (dialog) => {
            dialogSeen = true;
            await dialog.dismiss().catch(() => undefined);
          })
          .catch(() => undefined);
        const requestPromise = page.waitForEvent('request', { timeout: 1500 })
          .then(() => { requestSeen = true; })
          .catch(() => undefined);
        const responsePromise = page.waitForEvent('response', { timeout: 2000 })
          .then((response) => { responseSeen = response.status() < 500; })
          .catch(() => undefined);

        await target.click({ timeout: 5000 }).catch((error) => {
          failures.push(`${route}: "${label}" click failed: ${error.message.split('\n')[0]}`);
        });
        await Promise.all([dialogPromise, requestPromise, responsePromise]);
        await page.waitForTimeout(250);

        const afterUrl = page.url();
        const afterText = await visibleText(page);
        const realEffect = requestSeen || responseSeen || afterUrl !== beforeUrl || afterText !== beforeText;

        if (dialogSeen && !realEffect) {
          failures.push(`${route}: "${label}" only opened a browser dialog`);
        }
        if (!realEffect) {
          failures.push(`${route}: "${label}" produced no navigation, network request, or DOM change`);
        }
      }
    }

    expect(failures).toEqual([]);
  });

  test('all visible interactive elements are usable and named', async ({ page }) => {
    test.setTimeout(180000);
    const failures: string[] = [];
    const appRoutes = discoverAppRoutes();

    for (const route of appRoutes) {
      await page.goto(route, { waitUntil: 'domcontentloaded' });
      const results = await page.locator(interactiveSelector).evaluateAll((elements) =>
        elements.map((element, index) => {
          const rect = element.getBoundingClientRect();
          const style = window.getComputedStyle(element);
          const tag = element.tagName.toLowerCase();
          const type = element.getAttribute('type') || '';
          const label =
            element.getAttribute('aria-label') ||
            element.getAttribute('title') ||
            element.getAttribute('placeholder') ||
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

      for (const result of results) {
        const target = `${route}: ${result.tag}${result.type ? `[type=${result.type}]` : ''} #${result.index + 1}`;
        if (result.width < 1 || result.height < 1) failures.push(`${target} has no rendered hit area`);
        if (result.pointerEvents === 'none') failures.push(`${target} has pointer-events disabled`);
        if (!result.disabled && !result.label) failures.push(`${target} has no accessible label/text/placeholder/title`);
      }
    }

    expect(failures).toEqual([]);
  });

  test('layouts do not overflow or overlap click targets on desktop and mobile', async ({ page }) => {
    test.setTimeout(240000);
    const failures: string[] = [];
    const appRoutes = discoverAppRoutes();

    for (const viewport of viewports) {
      await page.setViewportSize({ width: viewport.width, height: viewport.height });

      for (const route of appRoutes) {
        await page.goto(route, { waitUntil: 'domcontentloaded' });
        const layout = await page.evaluate((selector) => {
          const documentElement = document.documentElement;
          const body = document.body;
          const horizontalOverflow = Math.max(documentElement.scrollWidth, body.scrollWidth) - window.innerWidth;
          const verticalOverflow = Math.max(documentElement.scrollHeight, body.scrollHeight) - window.innerHeight;

          const elements = Array.from(document.querySelectorAll(selector))
            .filter((element) => {
              const rect = element.getBoundingClientRect();
              const style = window.getComputedStyle(element);
              return style.visibility !== 'hidden' && style.display !== 'none' && rect.width > 0 && rect.height > 0;
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

    expect(failures).toEqual([]);
  });
});
