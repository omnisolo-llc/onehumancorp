import { test, expect } from '@playwright/test';

test('Friction Audit test: Jargon replacement on Swarm Memory Screen', async ({ page }) => {
  // Use a simulated mobile viewport
  await page.setViewportSize({ width: 375, height: 812 });

  await page.goto('http://localhost:8080/');

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

  // 1. Login
  await page.waitForLoadState('networkidle');

  const emailField = page.getByLabel('Email or Username');
  await emailField.fill('oauth@onehumancorp.com');

  const passwordField = page.getByLabel('Password');
  await passwordField.fill('dummy_password');

  const loginBtn = page.getByRole('button', { name: 'Sign In' }).first();
  await loginBtn.click();
  await page.waitForTimeout(5000);

  // 2. Navigate to Swarm Memory and check visualizer
  await page.goto('http://localhost:8080/#/swarm-memory');
  await page.waitForTimeout(5000);
  expect(page.url()).toContain('/swarm-memory');

  // Verify jargon replaced texts
  const automaticTasks = page.locator('text=Automatic Tasks Pipelines').first();
  await expect(automaticTasks).toBeVisible();

  const businessMemory = page.locator('text=Business Memory Overview').first();
  await expect(businessMemory).toBeVisible();

  // Navigate to Walkthrough
  const walkthroughBtn = page.locator('text=View Automatic Tasks Sync Walkthrough').first();
  await walkthroughBtn.click();
  await page.waitForTimeout(5000);
  expect(page.url()).toContain('/autodream-sync');

  const daemonText = page.locator('text=Automatic Tasks Background Sync Task Walkthrough').first();
  await expect(daemonText).toBeVisible();
});
