import { expect, test } from '@playwright/test';

const productRoutes = [
  '/dashboard',
  '/assistant',
  '/orders',
  '/inventory',
  '/inbox',
  '/agents',
  '/settings',
  '/business-analytics',
  '/integrations',
  '/calendar',
  '/diagnostics',
  '/agent-marketplace',
  '/visual-workflow',
  '/website-builder',
  '/booking-widget',
  '/storefront-widget',
  '/onboarding',
  '/login',
] as const;

const viewports = {
  desktop: { width: 1440, height: 1000 },
  mobile: { width: 390, height: 844 },
} as const;

const routesWithSurfacePrimitives = new Set([
  '/dashboard',
  '/orders',
  '/inventory',
  '/inbox',
  '/business-analytics',
  '/integrations',
  '/calendar',
  '/website-builder',
  '/login',
]);

const normalizedSurfaceSelector = [
  '.app-main .app-card',
  '.app-main .app-panel',
  '.app-main .glassmorphism',
  '.app-main .glass-card',
].join(',');

const hydrationFailurePattern = /Text content does not match server-rendered HTML|Text content did not match|Hydration failed|error occurred during hydration|server HTML (?:was )?replaced/i;

test.describe('App shell visual consistency', () => {
  for (const [viewportName, viewport] of Object.entries(viewports)) {
    for (const route of productRoutes) {
      test(`${route} at ${viewportName} uses one product shell without overflow and normalized surfaces`, async ({ page }) => {
        const hydrationFailures: string[] = [];
        const pageErrors: string[] = [];
        if (route === '/inbox') {
          page.on('console', (message) => {
            if (message.type() === 'error' && hydrationFailurePattern.test(message.text())) {
              hydrationFailures.push(`console: ${message.text()}`);
            }
          });
          page.on('pageerror', (error) => {
            pageErrors.push(error.message);
          });
        }

        await page.setViewportSize(viewport);
        await page.goto(route, { waitUntil: 'domcontentloaded' });

        await expect.soft(page.locator('.app-sidebar')).toHaveCount(1);
        await expect.soft(page.locator('.app-topbar')).toHaveCount(1);
        await expect.soft(page.locator('.app-main')).toHaveCount(1);

        const documentDimensions = await page.evaluate(() => ({
          documentWidth: document.documentElement.scrollWidth,
          viewportWidth: window.innerWidth,
        }));
        expect.soft(
          documentDimensions.documentWidth - documentDimensions.viewportWidth,
          `document overflowed horizontally: ${JSON.stringify(documentDimensions)}`,
        ).toBeLessThanOrEqual(1);

        const visibleSurfaces = await page.locator(normalizedSurfaceSelector).evaluateAll((elements) => elements
          .filter((element) => {
            const rect = element.getBoundingClientRect();
            const styles = window.getComputedStyle(element);
            return rect.width > 0
              && rect.height > 0
              && styles.display !== 'none'
              && styles.visibility !== 'hidden';
          })
          .map((element) => {
            const styles = window.getComputedStyle(element);
            return {
              className: element.getAttribute('class') || '',
              radius: parseFloat(styles.borderTopLeftRadius || '0'),
            };
          }));

        if (routesWithSurfacePrimitives.has(route)) {
          expect.soft(
            visibleSurfaces.length,
            `expected ${route} to render at least one visible surface primitive`,
          ).toBeGreaterThan(0);
        }

        expect.soft(visibleSurfaces.filter((item) => item.radius > 8.5)).toEqual([]);

        if (route === '/assistant') {
          const sectionList = page.getByTestId('assistant-section-list');
          await expect(sectionList).toBeVisible();

          const overflowY = await sectionList.evaluate(
            (element) => window.getComputedStyle(element).overflowY,
          );
          expect.soft(overflowY).not.toBe('auto');
          expect.soft(overflowY).not.toBe('scroll');
        }

        if (route === '/agents') {
          const utilitySample = await page.locator('.rounded-2xl').evaluateAll((elements) => elements
            .map((element) => {
              const rect = element.getBoundingClientRect();
              const styles = window.getComputedStyle(element);
              return {
                visible: rect.width > 0
                  && rect.height > 0
                  && styles.display !== 'none'
                  && styles.visibility !== 'hidden',
                radius: parseFloat(styles.borderTopLeftRadius || '0'),
                padding: [
                  styles.paddingTop,
                  styles.paddingRight,
                  styles.paddingBottom,
                  styles.paddingLeft,
                ].map((value) => parseFloat(value || '0')),
              };
            })
            .find((sample) => sample.visible && sample.padding.some((value) => value > 0)));

          expect.soft(utilitySample, 'expected a visible padded .rounded-2xl utility sample').toBeDefined();
          expect.soft(utilitySample?.radius).toBeGreaterThan(0);
          expect.soft(utilitySample?.padding.some((value) => value > 0)).toBe(true);
        }

        if (route === '/inbox') {
          await expect(page.getByTestId('inbox-settled')).toBeVisible({ timeout: 30_000 });
          await page.evaluate(() => new Promise<void>((resolve) => {
            requestAnimationFrame(() => requestAnimationFrame(() => resolve()));
          }));
          expect(hydrationFailures, 'inbox emitted hydration mismatch/replacement errors').toEqual([]);
          expect(pageErrors, 'inbox emitted uncaught page errors').toEqual([]);
        }
      });
    }
  }
});

test.describe('Mobile global controls', () => {
  const collisionRoutes = [
    '/website-builder',
    '/login',
    '/agent-marketplace',
    '/integrations',
    '/agents',
    '/inbox',
  ] as const;

  for (const width of [320, 390]) {
    for (const route of collisionRoutes) {
      test(`${route} at ${width}px keeps mobile global controls clear of product actions`, async ({ page }) => {
        await page.setViewportSize({ width, height: 844 });
        await page.goto(route, { waitUntil: 'domcontentloaded' });

        const controlSelector = [
          '#ohc-floating-help-btn',
          '#ai-chat-trigger-btn',
          '[aria-label="Voice Assistant"]',
        ].join(',');

        const visibleControlLabels = await page.locator(controlSelector).evaluateAll((controls) => controls
          .filter((control) => {
            const rect = control.getBoundingClientRect();
            const style = window.getComputedStyle(control);
            return rect.width > 0
              && rect.height > 0
              && style.display !== 'none'
              && style.visibility !== 'hidden';
          })
          .map((control) => control.getAttribute('aria-label') || control.id));

        expect(visibleControlLabels).toEqual(['Voice Assistant']);

        const collisions = await page.evaluate(({ controls }) => {
          const isVisible = (element: Element) => {
            const rect = element.getBoundingClientRect();
            const style = window.getComputedStyle(element);
            return rect.width > 0
              && rect.height > 0
              && style.display !== 'none'
              && style.visibility !== 'hidden';
          };
          const intersects = (left: DOMRect, right: DOMRect) => (
            Math.min(left.right, right.right) - Math.max(left.left, right.left) > 1
            && Math.min(left.bottom, right.bottom) - Math.max(left.top, right.top) > 1
          );
          const targetSelector = [
            '.app-brand-mark',
            '.app-nav-link',
            '.app-topbar',
            '.app-main button',
            '.app-main a',
            '.app-main input',
            '.app-main textarea',
            '.app-main select',
            '.app-main [role="button"]',
          ].join(',');
          const targets = [...new Set(document.querySelectorAll(targetSelector))].filter(isVisible);

          return [...document.querySelectorAll(controls)]
            .filter(isVisible)
            .flatMap((control) => {
              const controlRect = control.getBoundingClientRect();
              const outsideViewport = controlRect.left < -1
                || controlRect.top < -1
                || controlRect.right > window.innerWidth + 1
                || controlRect.bottom > window.innerHeight + 1;
              const label = control.getAttribute('aria-label') || control.id;
              const overlaps = targets
                .filter((target) => target !== control
                  && !target.contains(control)
                  && !control.contains(target)
                  && intersects(controlRect, target.getBoundingClientRect()))
                .map((target) => target.getAttribute('aria-label')
                  || target.textContent?.replace(/\s+/g, ' ').trim().slice(0, 80)
                  || `${target.tagName.toLowerCase()}.${target.className}`);

              return outsideViewport || overlaps.length > 0
                ? [{ control: label, outsideViewport, overlaps }]
                : [];
            });
        }, { controls: controlSelector });

        expect(collisions).toEqual([]);
      });
    }
  }
});
