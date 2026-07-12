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

const normalizedSurfaceSelector = [
  '.app-main .app-card',
  '.app-main .app-panel',
  '.app-main .glassmorphism',
  '.app-main [class*="rounded-xl"]',
  '.app-main [class*="rounded-[16px]"]',
  '.app-main [class*="rounded-[24px]"]',
  '.app-main [class*="rounded-2xl"]',
].join(',');

test.describe('App shell visual consistency', () => {
  for (const [viewportName, viewport] of Object.entries(viewports)) {
    for (const route of productRoutes) {
      test(`${route} at ${viewportName} uses one product shell without overflow and normalized surfaces`, async ({ page }) => {
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

        const inconsistentSurfaces = await page.locator(normalizedSurfaceSelector).evaluateAll((elements) => elements
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
          })
          .filter((item) => item.radius > 8.5));

        expect.soft(inconsistentSurfaces).toEqual([]);

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
      });
    }
  }
});
