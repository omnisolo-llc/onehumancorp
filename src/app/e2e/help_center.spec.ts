import { test, expect } from '@playwright/test';

test('In-App Help Center search and article viewing', async ({ page }) => {
  // 1. Login
  await page.goto('/');
  await page.fill('input[name="username"]', 'admin');
  await page.fill('input[name="password"]', 'admin');
  await page.click('button:has-text("Sign In")');

  // 2. Click global help button
  await page.click('button[aria-label="Help Center"]');

  // 3. Verify help center loaded
  await expect(page.locator('text=Help Center').first()).toBeVisible();

  // 4. Verify topics are visible
  await expect(page.locator('text=Getting Started').first()).toBeVisible();
  await expect(page.locator('text=My Store').first()).toBeVisible();

  // 5. Search for a specific topic
  await page.fill('input[placeholder="Search for help..."]', 'Connecting your domain');

  // 6. Verify search result
  await expect(page.locator('text=Connecting your domain').first()).toBeVisible();

  // 7. Click the article
  await page.click('text=Connecting your domain');

  // 8. Verify article content
  await expect(page.locator('text=You can connect a custom web address (like www.mybusiness.com) to your OHC store').first()).toBeVisible();
});
