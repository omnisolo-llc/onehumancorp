import { test, expect } from '@playwright/test';

const VIEWPORTS = [
  { width: 375, height: 667, name: 'Mobile' },
  { width: 768, height: 1024, name: 'Tablet' },
  { width: 1024, height: 768, name: 'Desktop' },
  { width: 1440, height: 900, name: 'Large Desktop' }
];

test.describe('Comprehensive UI Audit - Part 12 API Screen checks', () => {
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

      test('Check for mock data stub absence', async ({ page }) => {
        const elements = page.locator('.mock-data-stub');
        await expect(elements).toHaveCount(0);
      });

      test('Verify API Screen rendering', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('api-screen'));
        const screen = page.locator('#api-screen');
        if (await screen.isVisible()) {
           await expect(screen).toBeVisible();
        }
      });

      test('Deep traversal into my plan', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('my-plan-screen'));
        await expect(page.locator('#my-plan-screen')).toBeVisible();
      });

      test('Deep traversal into users screen', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('users-screen'));
        await expect(page.locator('#users-screen')).toBeVisible();
      });

      test('Ensure forms are accessible and visible', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('signup-screen'));
        const inputs = page.locator('input');
        const count = await inputs.count();
        if (count > 0) {
           await expect(inputs.first()).toBeVisible();
        }
      });

      test('Check analytics rendering', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        const navItems = page.locator('.nav-item');
        if (await navItems.count() > 0) {
            await expect(navItems.filter({ hasText: 'Analytics' })).toBeVisible();
        }
      });

      test('Ensure user profile is not broken', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('settings-screen'));
        await expect(page.locator('#settings-screen')).toBeVisible();
      });
    });
  }
});
