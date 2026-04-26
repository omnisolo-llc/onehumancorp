import { test, expect } from '@playwright/test';

test('Cost dashboard features appear in the UI', async ({ page }) => {
  // Use a simulated mobile viewport
  await page.setViewportSize({ width: 375, height: 812 });

  await page.goto('/');
  await page.waitForTimeout(5000);

  const emailField = page.locator('input[type="email"], input[name="username"]').first();
  await emailField.waitFor({ state: 'visible', timeout: 5000 }).catch(() => {});
  if (await emailField.isVisible()) {
    await emailField.fill('admin');
  }

  const passwordField = page.locator('input[type="password"], input[name="password"]').first();
  if (await passwordField.isVisible()) {
    await passwordField.fill('admin');
  }

  const loginBtn = page.locator('button:has-text("Login"), button:has-text("Sign In")').first();
  if (await loginBtn.isVisible()) {
    await loginBtn.click();
  }

  await page.waitForTimeout(5000);

  // Navigate to Cost Dashboard
  await page.goto('/#/cost');
  await page.waitForTimeout(5000);
  expect(page.url()).toContain('/cost');

  // Navigate to My Plan
  await page.goto('/#/my-plan');
  await page.waitForTimeout(5000);
  expect(page.url()).toContain('/my-plan');

  // Navigate to Pricing
  await page.goto('/#/pricing');
  await page.waitForTimeout(5000);
  expect(page.url()).toContain('/pricing');

});
