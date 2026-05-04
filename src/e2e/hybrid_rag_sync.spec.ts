import { test, expect } from '@playwright/test';

test('AutoDream Sync Walkthrough End-to-End', async ({ page }) => {
  // 1. Start from home page after user login via UI
  await page.goto('http://localhost:3000');

  // Fill in login credentials
  await page.fill('input[name="email"]', 'test@onehumancorp.com');
  await page.fill('input[name="password"]', 'password123');
  await page.click('button[type="submit"]');

  // Wait for the dashboard to load
  await expect(page.locator('text=Revenue Summary')).toBeVisible();

  // 2. Trigger the process: Click the AutoDream Sync Tour button
  await page.click('button:has-text("AutoDream Sync Tour")');

  // 3. Progress through every UI step
  // Step 0 -> 1
  await expect(page.locator('text=Welcome to AutoDream Sync')).toBeVisible();
  await page.click('button:has-text("Next")');

  // Step 1 -> 2
  await expect(page.locator('text=Local Data Ingestion')).toBeVisible();
  await page.click('button:has-text("Next")');

  // Step 2 -> 3
  await expect(page.locator('text=Secure mTLS Transfer')).toBeVisible();
  await page.click('button:has-text("Next")');

  // Step 3 -> 4
  await expect(page.locator('text=Cloud Multi-Tenant Vector DB')).toBeVisible();
  await page.click('button:has-text("Next")');

  // Step 4 -> 5
  await expect(page.locator('text=Swarm Intelligence')).toBeVisible();
  await page.click('button:has-text("Next")');

  // Step 5 -> 6 (Finish)
  await expect(page.locator('text=You are all set')).toBeVisible();
  await page.click('button:has-text("Finish")');

  // 4. Assert that the final product matches design/research docs
  // The walkthrough should close and return to the dashboard
  await expect(page.locator('text=Welcome to AutoDream Sync')).not.toBeVisible();
  await expect(page.locator('text=Revenue Summary')).toBeVisible();
});
