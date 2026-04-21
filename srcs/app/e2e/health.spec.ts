import { test, expect } from '@playwright/test';

test('Diagnostics screen displays hybrid health info', async ({ page }) => {
  // 1. Login
  await page.goto('/');
  await page.fill('input[name="username"]', 'admin');
  await page.fill('input[name="password"]', 'admin');
  await page.click('button:has-text("Login")');

  // 2. Navigate to Diagnostics
  // Depending on the UI, we might need to click a sidebar link or use the URL
  await page.goto('/#/diagnostics');

  // 3. Verify Hybrid Health section
  await expect(page.locator('text=Hybrid Health Status')).toBeVisible();

  // 4. Verify Cloud Connectivity row (added in this PR)
  await expect(page.locator('text=Cloud Connectivity')).toBeVisible();

  // 5. Verify Stuck Missions count (added logic in this PR)
  await expect(page.locator('text=Stuck Missions')).toBeVisible();
});
