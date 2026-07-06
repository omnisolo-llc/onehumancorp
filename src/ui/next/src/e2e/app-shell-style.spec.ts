import { expect, test } from '@playwright/test';

const productRoutes = [
  '/dashboard',
  '/assistant',
  '/settings',
  '/ai-usage-paywall',
  '/orders',
  '/inventory',
  '/inbox',
  '/triage',
  '/agents',
  '/business-analytics',
  '/integrations',
  '/calendar',
  '/cost-dashboard',
  '/diagnostics',
  '/agent-marketplace',
];

test.describe('App shell visual consistency', () => {
  test.describe.configure({ mode: 'serial' });

  for (const route of productRoutes) {
    test(`${route} uses the dashboard product shell and normalized surfaces`, async ({ page }) => {
      await page.goto(route, { waitUntil: 'domcontentloaded' });
      await expect(page.locator('.app-sidebar')).toBeVisible();
      await expect(page.locator('.app-topbar')).toBeVisible();
      await expect(page.locator('.app-main')).toBeVisible();

      await expect(page.locator('.app-main')).not.toContainText('Loaded from');
      await expect(page.locator('.app-main')).not.toContainText(/\/api\/ui\//);

      const inconsistentSurfaces = await page.locator([
        '.app-main .app-card',
        '.app-main .app-panel',
        '.app-main .glassmorphism',
        '.app-main [class*="rounded-xl"]',
        '.app-main [class*="rounded-[16px]"]',
        '.app-main [class*="rounded-[24px]"]',
        '.app-main [class*="rounded-xl"]',
        '.app-main [class*="rounded-2xl"]',
      ].join(',')).evaluateAll((elements) => elements
        .filter((element) => {
          const rect = element.getBoundingClientRect();
          return rect.width > 0 && rect.height > 0;
        })
        .map((element) => {
          const styles = window.getComputedStyle(element);
          return {
            className: element.getAttribute('class') || '',
            radius: parseFloat(styles.borderTopLeftRadius || '0'),
          };
        })
        .filter((item) => item.radius > 8.5));

      expect(inconsistentSurfaces).toEqual([]);
    });
  }

  test('assistant Sections menu does not create its own scrollbar', async ({ page }) => {
    await page.goto('/assistant', { waitUntil: 'domcontentloaded' });
    const sectionList = page.getByTestId('assistant-section-list');
    await expect(sectionList).toBeVisible();

    const overflowY = await sectionList.evaluate((element) => window.getComputedStyle(element).overflowY);
    expect(overflowY).not.toBe('auto');
    expect(overflowY).not.toBe('scroll');
  });
});
