import { test, expect } from '@playwright/test';

test.describe('UX Improvement Verification', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('dashboard labels and sales metric are correct', async ({ page }) => {
    await page.waitForSelector('#dashboard-screen', { state: 'attached', timeout: 30000 });
    await expect(page.locator('#dashboard-screen h1').first()).toContainText('Overview');
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
  });

  test('mobile bottom nav is visible on small screens', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.waitForSelector('#mobile-bottom-nav', { state: 'attached', timeout: 30000 });
    const nav = page.locator('#mobile-bottom-nav');
    await expect(nav).toBeVisible();
  });

  test('quick actions have the grandmother test hint button', async ({ page }) => {
    await page.waitForSelector('#dashboard-screen', { state: 'attached', timeout: 30000 });
    const hintBtn = page.locator('#dashboard-screen h3:has-text("Quick Actions") button');
    await expect(hintBtn).toBeVisible();
  });

  test('ai assistants screen has plain language title', async ({ page }) => {
    await page.evaluate("showScreen('agents-screen')");
    await page.waitForSelector('#agents-screen', { state: 'visible', timeout: 30000 });
    await expect(page.locator('#agents-screen h1').first()).toContainText('AI Assistants');
  });

  test('launch site screen has plain language title', async ({ page }) => {
    await page.evaluate("showScreen('setup-screen')");
    await page.waitForSelector('#setup-screen', { state: 'visible', timeout: 30000 });
    await expect(page.locator('#setup-screen h1').first()).toContainText('Launch Site');
  });
});
