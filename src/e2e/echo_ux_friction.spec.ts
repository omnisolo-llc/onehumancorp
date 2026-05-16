import { test, expect } from '@playwright/test';

/**
 * 👂 ECHO MISSION: UI/UX Friction Elimination
 * -------------------------------------------
 * Verified against OHC Premium Design Standards.
 */

test.describe('👂 Echo: UX Friction Elimination E2E', () => {

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('TC1: Navigation Clarity', async ({ page }) => {
    // Navigation labels check
    await expect(page.locator('#main-nav')).toContainText('Overview');
    await expect(page.locator('#main-nav')).toContainText('AI Assistants');
  });

  test('TC2: Dashboard Revenue Visibility', async ({ page }) => {
    await page.evaluate("showScreen('dashboard-screen')");
    await expect(page.locator('text=Total Revenue (Today)')).toBeVisible();
    await expect(page.locator('text=$1,284.50')).toBeVisible();
  });

  test('TC3: Mobile Navigation Presence', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await expect(page.locator('#mobile-bottom-nav')).toBeVisible();
    await expect(page.locator('button:has-text("Messages")')).toBeVisible();
  });

  test('TC4: Shimmer Loading States', async ({ page }) => {
    await page.evaluate("showScreen('setup-screen')");
    await page.click('button:has-text("Instant Build with AI →")');
    await expect(page.locator('.shimmer')).toHaveCount(5);
    await expect(page.locator('text=Crafting your storefront...')).toBeVisible();
  });

  test('TC5: Glassmorphism Implementation', async ({ page }) => {
    await page.evaluate("showScreen('dashboard-screen')");
    const glass = page.locator('.glass').first();
    const filter = await glass.evaluate(node => window.getComputedStyle(node).backdropFilter);
    expect(filter).toContain('blur(20px)');
  });

});
