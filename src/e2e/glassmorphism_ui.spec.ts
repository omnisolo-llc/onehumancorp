import { test, expect } from '@playwright/test';

test.describe('Glassmorphism UI Premium Design Standards', () => {
  test('Dashboard should have glassmorphism elements applied', async ({ page }) => {
    await page.goto('/dashboard');
    const glassElements = await page.$$('.glassmorphism');
    for (const el of glassElements) {
        const style = await el.evaluate((node) => {
            const computed = window.getComputedStyle(node);
            return {
                backdropFilter: computed.backdropFilter,
                webkitBackdropFilter: computed.webkitBackdropFilter,
                backgroundColor: computed.backgroundColor,
                borderRadius: computed.borderRadius
            };
        });
        expect(style.backdropFilter).toContain('blur(30px)');
        expect(style.backdropFilter).toContain('saturate(210%)');
        expect(style.borderRadius).toContain('16px');
    }
  });

  test('Storefront Builder should have glassmorphism panel', async ({ page }) => {
    await page.goto('/storefront-builder');
    const glassPanel = page.locator('.glassmorphism').first();
    await expect(glassPanel).toBeVisible({ timeout: 15000 });
  });

  test('Setup onboarding page glassmorphism check', async ({ page }) => {
     await page.goto('/setup');
     const glassCard = page.locator('.glassmorphism').first();
     await expect(glassCard).toBeVisible({ timeout: 15000 });
  });

  test('POS UI should render glassmorphism', async ({ page }) => {
      await page.goto('/pos');
      const posPanel = page.locator('.glassmorphism').first();
      await expect(posPanel).toBeVisible({ timeout: 15000 });
  });

  test('Viral Goal Tracker UI should render glassmorphism', async ({ page }) => {
      await page.goto('/viral-goal-tracker');
      const vgtPanel = page.locator('.glassmorphism').first();
      await expect(vgtPanel).toBeVisible({ timeout: 15000 });
  });
});
