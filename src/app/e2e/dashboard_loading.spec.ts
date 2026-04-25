import { test, expect } from '@playwright/test';

test('Dashboard loading shimmer displays correctly', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });

  await page.goto('/');

  // Wait for load
  await page.waitForTimeout(5000);

  // Click reload now if there's a new version banner blocking the app
  try {
      if (await page.locator('text=A new version is available!').isVisible({ timeout: 2000 })) {
          await page.locator('button:has-text("Reload Now")').click();
          await page.waitForTimeout(5000);
      }
  } catch (e) { }

  try {
      if (await page.locator('button:has-text("Enable accessibility")').isVisible({ timeout: 2000 })) {
          await page.locator('button:has-text("Enable accessibility")').click();
          await page.waitForTimeout(5000);
      }
  } catch (e) { }

  // Set fake backend to force loading state longer
  try {
      await page.click('button[aria-label="Remote Connection Settings"]', { timeout: 2000 });
      await page.waitForTimeout(1000);
      await page.fill('input', 'http://127.0.0.1:9090'); // fake backend port
      await page.click('button:has-text("Save")');
      await page.waitForTimeout(2000);
  } catch (e) { }

  // Login
  try {
      await page.fill('input[placeholder="Email or Username"]', 'test@test.com', { timeout: 2000 });
      await page.fill('input[placeholder="Password"]', 'password');
      await page.click('button:has-text("Sign In")');
      await page.waitForTimeout(5000);
  } catch (e) { }

  // Bypass setup
  try {
      await page.click('button:has-text("Get Started")', { timeout: 2000 });
      await page.waitForTimeout(2000);
      await page.click('button:has-text("Continue without Sync")');
      await page.waitForTimeout(2000);
  } catch (e) { }

  await page.goto('/#/dashboard');

  await expect(page.locator('[aria-label="Dashboard Loading Skeleton"]')).toBeVisible({ timeout: 10000 });
});
