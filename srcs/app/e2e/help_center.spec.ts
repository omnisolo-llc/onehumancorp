import { test, expect } from '@playwright/test';
import { loginAsAdmin } from './helpers_test';

test('Help Center screen navigation, search and sub-screens work', async ({ page }) => {
  // 1. Login using helper
  await page.goto('/');
  await page.fill('input[name="username"]', 'admin');
  await page.fill('input[name="password"]', 'admin');
  await page.click('button:has-text("Login")');

  // 2. Wait for dashboard load
  await page.waitForSelector('text=Dashboard');

  // Navigate to Help Center via Dashboard appbar button
  await page.click('[aria-label="Open the Help Center for guides and support."]');

  // 3. Verify Help Center title
  await expect(page.locator('text=Help Center')).toBeVisible();

  // 4. Verify initial articles are visible
  await expect(page.locator('text=Getting Started with One Human Corp')).toBeVisible();

  // 5. Test search functionality
  // Type 'stripe' in the search field to filter articles
  await page.fill('input[placeholder="Search for help topics..."]', 'stripe');

  // Wait for the UI to update the search results
  // We expect "Accepting Payments with Stripe" to be visible
  await expect(page.locator('text=Accepting Payments with Stripe')).toBeVisible();
  // And "Getting Started with One Human Corp" should NOT be visible
  await expect(page.locator('text=Getting Started with One Human Corp')).not.toBeVisible();

  // Verify 'Ask AI Agent' button
  await expect(page.locator('text=Ask AI Agent')).toBeVisible();

  // 6. Test Video Tutorials navigation
  await page.click('text=Videos');
  await expect(page.locator('text=Video Tutorials')).toBeVisible();
  await expect(page.locator('text=Set up your store in 5 minutes')).toBeVisible();
  await page.click('button[aria-label="Back"]'); // Or similar back button

  // Wait for Help Center to be visible again
  await expect(page.locator('text=Help Center')).toBeVisible();

  // 7. Test API Docs navigation
  await page.click('text=API Docs');
  await expect(page.locator('text=API Documentation')).toBeVisible();
  await expect(page.locator('text=/api/v1/agents')).toBeVisible();
  await page.click('button[aria-label="Back"]');

  // 8. Test Release Notes navigation
  await page.click("text=What's New");
  await expect(page.locator('text=Release Notes')).toBeVisible();
  await expect(page.locator('text=v1.4.0')).toBeVisible();
});
