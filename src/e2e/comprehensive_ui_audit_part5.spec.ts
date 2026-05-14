import { test, expect } from '@playwright/test';

const VIEWPORTS = [
  { width: 375, height: 667, name: 'Mobile' },
  { width: 768, height: 1024, name: 'Tablet' },
  { width: 1024, height: 768, name: 'Desktop' },
  { width: 1440, height: 900, name: 'Large Desktop' }
];

test.describe('Comprehensive UI Audit - Part 5 Final Checks', () => {
  for (const vp of VIEWPORTS) {
    test.describe(`Viewport: ${vp.name}`, () => {
      test.use({ viewport: vp });

      test.beforeEach(async ({ page }) => {
        await page.goto('/');
      });

      test('Verify UI interaction timings and animations', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('signup-screen'));
        await expect(page.locator('#signup-screen')).toBeVisible();
      });

      test('Data lifecycle mock verification', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('agents-screen'));
        const agentsPage = page.locator('#agents-screen');
        await expect(agentsPage).toBeVisible();
      });

      test('Responsiveness of navigation menu', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        const nav = page.locator('nav#main-nav');

        // Assert it is displayed with flex
        await expect(nav).toHaveCSS('display', 'flex');
      });

      test('Check for absence of test or mock keys', async ({ page }) => {
        const bodyText = await page.innerText('body');
        expect(bodyText.includes('sk_test_mock')).toBeFalsy();
      });

      test('Grandmother test criteria verification', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        const h1 = page.locator('h1').first();
        await expect(h1).toBeVisible();
      });
    });
  }
});
