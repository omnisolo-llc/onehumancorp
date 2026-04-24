import { test, expect } from '@playwright/test';

test('Dashboard UI displays plain-language labels and important metrics', async ({ page }) => {
  // 1. Login
  await page.goto('/');

  // OHC Flutter app loads asynchronously. Wait for it to settle.
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(3000);

  // Force semantics enable so Playwright can query the Flutter DOM (flt-semantics tags).
  await page.evaluate(() => {
    if (window._flutter_semantics_enable) window._flutter_semantics_enable();
  });

  // Use semantics / a11y labels which Flutter Web HTML renderer outputs as standard DOM elements.
  // Add original inputs as fallback for standard HTML renderer in CI.
  await page.fill('input[aria-label="Email or Username"], input[name="username"]', 'admin');
  await page.fill('input[aria-label="Password"], input[name="password"]', 'admin');
  await page.click('flt-semantics[aria-label="Sign In"], button:has-text("Login")');

  // 2. Wait for navigation to Dashboard
  // Assuming after login we are on the dashboard. Let's wait for the overview section.
  await expect(page.locator('flt-semantics[aria-label="Overview"], text=Overview')).toBeVisible({ timeout: 10000 });

  // 3. Verify that the grandmother test plain-language labels exist
  await expect(page.locator('flt-semantics[aria-label*="Today\'s Sales"], text="Today\'s Sales"')).toBeVisible();
  await expect(page.locator('flt-semantics[aria-label*="New Orders"], text="New Orders"')).toBeVisible();
  await expect(page.locator('flt-semantics[aria-label*="Pending Appointments"], text="Pending Appointments"')).toBeVisible();
  await expect(page.locator('flt-semantics[aria-label*="Active AI Helpers"], text="Active AI Helpers"')).toBeVisible();
  await expect(page.locator('flt-semantics[aria-label*="System Status"], text="System Status"')).toBeVisible();
  await expect(page.locator('flt-semantics[aria-label*="Tasks in Progress"], text="Tasks in Progress"')).toBeVisible();
});
