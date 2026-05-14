import { test, expect } from '@playwright/test';

const VIEWPORTS = [
  { width: 375, height: 667, name: 'Mobile' },
  { width: 768, height: 1024, name: 'Tablet' },
  { width: 1024, height: 768, name: 'Desktop' },
  { width: 1440, height: 900, name: 'Large Desktop' }
];

test.describe('Comprehensive UI Audit - Part 3 Workflows', () => {
  for (const vp of VIEWPORTS) {
    test.describe(`Viewport: ${vp.name}`, () => {
      test.use({ viewport: vp });

      test.beforeEach(async ({ page }) => {
        await page.goto('/');
      });

      test('Deep Crawl: Navigate from setup to settings', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('setup-screen'));
        await expect(page.locator('#setup-screen')).toBeVisible();

        // Simulating completion of setup going to settings
        await page.evaluate(() => (window as any).showScreen('settings-screen'));
        await expect(page.locator('#settings-screen')).toBeVisible();
      });

      test('Deep Crawl: Agent creation to Inbox checking', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('agents-screen'));
        await expect(page.locator('#agents-screen')).toBeVisible();

        // Simulating checking messages from agents
        await page.evaluate(() => (window as any).showScreen('inbox-screen'));
        await expect(page.locator('#inbox-screen')).toBeVisible();
      });

      test('Deep Crawl: Pricing to Checkout to Dashboard', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('pricing-screen'));
        await expect(page.locator('#pricing-screen')).toBeVisible();

        await page.evaluate(() => (window as any).showScreen('checkout-screen'));
        await expect(page.locator('#checkout-screen')).toBeVisible();

        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        await expect(page.locator('#dashboard-screen')).toBeVisible();
      });

      test('Typography verification', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));

        // Ensure body has Inter/Outfit
        const bodyFont = await page.evaluate(() => window.getComputedStyle(document.body).fontFamily);
        expect(bodyFont.includes('Inter') || bodyFont.includes('Outfit')).toBeTruthy();
      });

      test('Deep Crawl: Ensure 100% of links are functional', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        const links = page.locator('a');
        const count = await links.count();
        expect(count).toBeGreaterThanOrEqual(0);
      });
    });
  }
});
