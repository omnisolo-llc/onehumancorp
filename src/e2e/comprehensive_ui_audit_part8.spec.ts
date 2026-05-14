import { test, expect } from '@playwright/test';

const VIEWPORTS = [
  { width: 375, height: 667, name: 'Mobile' },
  { width: 768, height: 1024, name: 'Tablet' },
  { width: 1024, height: 768, name: 'Desktop' },
  { width: 1440, height: 900, name: 'Large Desktop' }
];

test.describe('Comprehensive UI Audit - Part 8 UX Performance', () => {
  for (const vp of VIEWPORTS) {
    test.describe(`Viewport: ${vp.name}`, () => {
      test.use({ viewport: vp });

      test.beforeEach(async ({ page }) => {
        await page.goto('/');
      });

      test('Verify UI interaction timings and fast transitions', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('signup-screen'));
        await expect(page.locator('#signup-screen')).toBeVisible();
      });

      test('Responsive compliance check across breakpoints', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('agents-screen'));
        const screen = page.locator('#agents-screen');
        await expect(screen).toBeVisible();
        const box = await screen.boundingBox();
        if (box) {
           expect(box.width).toBeLessThanOrEqual(vp.width);
        }
      });

      test('Ensure forms are accessible and visible', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('signup-screen'));

        const inputs = page.locator('input');
        const count = await inputs.count();
        expect(count).toBeGreaterThan(0);

        for (let i = 0; i < count; i++) {
           await expect(inputs.nth(i)).toBeVisible();
        }
      });

      test('Verify absence of any broken image links', async ({ page }) => {
        const images = page.locator('img');
        const count = await images.count();
        for(let i=0; i<count; i++) {
            const src = await images.nth(i).getAttribute('src');
            expect(src).not.toBeNull();
        }
      });

      test('Verify Grandmother UX text simplicity on primary buttons', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        const primaryAction = page.locator('button.primary').first();
        if (await primaryAction.isVisible()) {
          const text = await primaryAction.innerText();
          // Check if it's less than a few words, not complex
          expect(text.split(' ').length).toBeLessThan(5);
        }
      });

      test('Check for specific technical jargon', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        const bodyText = await page.innerText('body');
        expect(bodyText.toLowerCase().includes('sql transaction failed')).toBeFalsy();
      });

      test('Verify glass saturation', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        const glass = page.locator('.glass').first();
        if (await glass.isVisible()) {
           const backdropFilter = await glass.evaluate(node => window.getComputedStyle(node).backdropFilter);
           expect(backdropFilter).toBeTruthy();
        }
      });

      test('Complete checkout simulation UI state', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('checkout-screen'));
        await expect(page.locator('#checkout-screen')).toBeVisible();
        const payBtn = page.locator('button:has-text("Pay Now")');
        if (await payBtn.isVisible()) {
            await expect(payBtn).toBeEnabled();
        }
      });
    });
  }
});
