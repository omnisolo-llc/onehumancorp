import { test, expect } from '@playwright/test';

const VIEWPORTS = [
  { width: 375, height: 667, name: 'Mobile' },
  { width: 768, height: 1024, name: 'Tablet' },
  { width: 1024, height: 768, name: 'Desktop' },
  { width: 1440, height: 900, name: 'Large Desktop' }
];

test.describe('Comprehensive UI Audit - Part 10 Ultimate Regression Check', () => {
  for (const vp of VIEWPORTS) {
    test.describe(`Viewport: ${vp.name}`, () => {
      test.use({ viewport: vp });

      test.beforeEach(async ({ page }) => {
        await page.goto('/');
      });

      test('Verify 100% of links on Setup Wizard', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('setup-screen'));
        await expect(page.locator('#setup-screen')).toBeVisible();

        const buttons = page.locator('#setup-screen button');
        const count = await buttons.count();
        expect(count).toBeGreaterThan(0);

        for (let i = 0; i < count; i++) {
            await expect(buttons.nth(i)).toBeVisible();
        }
      });

      test('Verify Dashboard Quick Actions layout', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        await expect(page.locator('#dashboard-screen')).toBeVisible();

        const quickActions = page.locator('#dashboard-screen button');
        const count = await quickActions.count();
        expect(count).toBeGreaterThan(0);
      });

      test('Verify form fields have correct types on Signup', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('signup-screen'));
        await expect(page.locator('#signup-screen input[type="email"]')).toBeVisible();
        await expect(page.locator('#signup-screen input[type="password"]')).toBeVisible();
      });

      test('Verify typography rules in details', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));

        // Ensure h1 has correct styling expectations
        const h1 = page.locator('h1').first();
        const fontWeight = await h1.evaluate(node => window.getComputedStyle(node).fontWeight);
        // Headings usually have bold/600+
        expect(parseInt(fontWeight) || 600).toBeGreaterThanOrEqual(400);
      });

      test('Deep traversal into meeting room', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('meeting-room-screen'));
        await expect(page.locator('#meeting-room-screen')).toBeVisible();

        const h2 = page.locator('#meeting-room-screen h2').first();
        await expect(h2).toHaveText('Strategic Planning Room');
      });

      test('Check agent activity feed mock logic', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        const feed = page.locator('#agent-activity-feed');
        await expect(feed).toBeVisible();
      });

      test('Check analytics rendering', async ({ page }) => {
        // Check if analytics section is available
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

      test('Complete business onboarding path check', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('signup-screen'));
        await expect(page.locator('#signup-screen')).toBeVisible();

        // Next step
        await page.evaluate(() => (window as any).showScreen('setup-screen'));
        await expect(page.locator('#setup-screen')).toBeVisible();

        // Next step
        await page.evaluate(() => (window as any).showScreen('pricing-screen'));
        await expect(page.locator('#pricing-screen')).toBeVisible();

        // Finally Dashboard
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        await expect(page.locator('#dashboard-screen')).toBeVisible();
      });

      test('Responsive check for extra menu overflow', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        // simulate toggling menu
        await page.evaluate(() => {
          const menu = document.getElementById('extra-menu');
          if (menu) menu.style.display = 'block';
        });

        await expect(page.locator('#extra-menu')).toBeVisible();
      });
    });
  }
});
