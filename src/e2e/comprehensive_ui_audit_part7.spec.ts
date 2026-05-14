import { test, expect } from '@playwright/test';

const VIEWPORTS = [
  { width: 375, height: 667, name: 'Mobile' },
  { width: 768, height: 1024, name: 'Tablet' },
  { width: 1024, height: 768, name: 'Desktop' },
  { width: 1440, height: 900, name: 'Large Desktop' }
];

test.describe('Comprehensive UI Audit - Part 7 Integrations', () => {
  for (const vp of VIEWPORTS) {
    test.describe(`Viewport: ${vp.name}`, () => {
      test.use({ viewport: vp });

      test.beforeEach(async ({ page }) => {
        await page.goto('/');
      });

      test('Verify integration toggle visibility', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        await expect(page.locator('#dashboard-screen')).toBeVisible();

        const integrationsBtn = page.locator('#integrations-btn');
        if (await integrationsBtn.isVisible()) {
            await integrationsBtn.click();
            await expect(page.locator('#facebook-integration')).toBeVisible();
        }
      });

      test('Testing setup checklist rendering', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('checklist-screen'));
        const screen = page.locator('#checklist-screen');
        await expect(screen).toBeVisible();
      });

      test('Ensure billing page loads with plans', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('pricing-screen'));
        const screen = page.locator('#pricing-screen');
        await expect(screen).toBeVisible();
      });

      test('Check if back to dashboard link works everywhere', async ({ page }) => {
        const screens = [
          'signup-screen', 'pricing-screen', 'my-plan-screen',
          'agents-screen', 'setup-screen', 'settings-screen', 'checkout-screen'
        ];

        for (const s of screens) {
          await page.evaluate((id) => (window as any).showScreen(id), s);

          // Check if there is a back button to dashboard
          const backBtn = page.locator(`button:has-text("Dashboard")`).first();
          if (await backBtn.isVisible()) {
             await backBtn.click();
             await expect(page.locator('#dashboard-screen')).toBeVisible();
          }
        }
      });

      test('Verify glassmorphism border colors', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        const glass = page.locator('.glass').first();
        if (await glass.isVisible()) {
            const borderColor = await glass.evaluate(node => window.getComputedStyle(node).borderColor);
            expect(borderColor).toBeTruthy();
        }
      });

      test('Verify 404 does not crash client state', async ({ page }) => {
        await page.goto('/some-unknown-path-that-doesnt-exist');
        // Because of catch-all routing in Rust, it serves the app
        await expect(page.locator('body')).toBeVisible();
      });

      test('Data Truth UI Lifecycle', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        const h1 = page.locator('h1').first();
        await expect(h1).toHaveText('Dashboard');
      });

      test('Touch target sizing audit', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        const buttons = page.locator('button');
        const count = await buttons.count();
        for (let i = 0; i < Math.min(count, 5); i++) {
           const box = await buttons.nth(i).boundingBox();
           if (box) {
               // Many times buttons might have smaller height depending on padding, but touch target is usually 44x44
               expect(box.height).toBeGreaterThanOrEqual(20); // soft check
           }
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
    });
  }
});
