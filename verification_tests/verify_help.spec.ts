import { test, expect } from '@playwright/test';

test('Verify Help Center with Search', async ({ page }) => {
  // Desktop Viewport
  await page.setViewportSize({ width: 1280, height: 720 });
  await page.goto('http://localhost:3000/help');
  await page.waitForSelector('h1:has-text("Help Center")');
  await page.screenshot({ path: 'verification_tests/help_center_desktop.png' });

  // Mobile Viewport
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto('http://localhost:3000/help');
  await page.waitForSelector('h1:has-text("Help Center")');
  await page.screenshot({ path: 'verification_tests/help_center_mobile.png' });

  // Test Search functionality
  await page.setViewportSize({ width: 1280, height: 720 });
  await page.goto('http://localhost:3000/help');
  await page.waitForSelector('input[placeholder="Search for help..."]');
  await page.fill('input[placeholder="Search for help..."]', 'Payments');
  await page.waitForTimeout(500); // Give React time to filter
  await page.screenshot({ path: 'verification_tests/help_center_search.png' });
});