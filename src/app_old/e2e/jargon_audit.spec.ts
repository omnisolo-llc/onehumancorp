import { test, expect } from '@playwright/test';

test('UI Jargon replacement E2E flow', async ({ page }) => {
  // Login flow
  await page.goto('/'); // Base URL of the app
  await page.fill('input[name="email"]', 'maya@example.com');
  await page.fill('input[name="password"]', 'password123');
  await page.click('button[type="submit"]');

  // Assert Dashboard Jargon replaced
  await expect(page.locator('text=Business Health & Overview')).toBeVisible();

  // Navigate to Pipelines (now "Business Processes") via UI interaction
  await page.click('text=Business Processes');
  await expect(page.locator('text=Initiated by: Business')).toBeVisible();

  // Navigate to Swarm Memory (now "Team Memory")
  await page.click('text=Team Memory');
  await expect(page.locator('text=Memory Overview')).toBeVisible();
});
