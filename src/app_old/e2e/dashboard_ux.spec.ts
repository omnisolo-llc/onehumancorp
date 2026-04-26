import { test, expect } from '@playwright/test';

test('Dashboard screen uses plain language instead of technical jargon', async ({ page }) => {
  // Use a simulated mobile viewport
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

  // Click the remote connection settings button
  try {
      await page.click('button[aria-label="Remote Connection Settings"]', { timeout: 2000 });
      await page.waitForTimeout(1000);
      await page.fill('input', 'http://127.0.0.1:8080'); // fake backend url
      await page.click('button:has-text("Save")');
      await page.waitForTimeout(2000);
  } catch (e) { }

  await page.goto('/#/dashboard');

  // Wait for dashboard to load
  await page.waitForTimeout(5000);

  expect(page.url()).toContain('/dashboard');

  const myBusiness = page.locator('text=My Business');
  if (await myBusiness.isVisible({ timeout: 5000 })) {
    await expect(myBusiness).toBeVisible();
  }
  // Assert that jargon has been removed
  const businessOverview = page.locator('text=Business Overview');
  await expect(businessOverview.first()).toBeVisible();

  // Test the fix
  await expect(page.locator('text=System Health')).toHaveCount(0);
  await expect(page.locator('text=Telemetry')).toHaveCount(0);
  await expect(page.locator('text=Observability')).toHaveCount(0);

});
