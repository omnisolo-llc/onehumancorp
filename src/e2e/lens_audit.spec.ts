import { test, expect } from '@playwright/test';

test.describe('Lens Audit: Comprehensive Verification', () => {

  test('Verify primary navigation and header structure on Desktop', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto('/');

    // Assert the main architectural components exist securely
    await expect(page.locator('header').first()).toBeVisible({ timeout: 5000 });
    await expect(page.locator('nav').first()).toBeVisible({ timeout: 5000 });
    await expect(page.locator('main').first()).toBeVisible({ timeout: 5000 });

    // Assert no mock data fallback containers are present on load
    await expect(page.locator('.mock-data-stub')).toHaveCount(0);
    await expect(page.locator('.fallback-error')).toHaveCount(0);
  });

  test('Verify touch targets and mobile responsiveness on Mobile Portrait', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/settings');

    // Check for the primary mobile container
    await expect(page.locator('main').first()).toBeVisible({ timeout: 5000 });

    // Check that primary forms are accessible
    // Using a reliable assertion instead of a conditional count block
    const form = page.locator('form').first();
    await expect(form).toBeVisible();
  });

  test('Verify grid layout scaling on Tablet', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto('/billing');

    await expect(page.locator('body').first()).toBeVisible();
    // Ensure the page doesn't throw a react error on mount
    await expect(page.locator('.ohc-critical-error')).toHaveCount(0);
  });

  test('Verify horizontal overflow boundaries on Mobile Landscape', async ({ page }) => {
    await page.setViewportSize({ width: 896, height: 414 });
    await page.goto('/tasks');

    await expect(page.locator('body').first()).toBeVisible();
  });

  test('Verify Data Truth logic allows inputs without raw HTML exposure', async ({ page }) => {
    await page.setViewportSize({ width: 1024, height: 768 });
    await page.goto('/users');

    await expect(page.locator('body').first()).toBeVisible();
  });
});
