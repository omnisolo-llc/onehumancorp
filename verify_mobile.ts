import { test, expect } from '@playwright/test';

test('verify mobile dashboard', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto('http://localhost:3000/dashboard');
  await page.waitForLoadState('networkidle');
  await page.screenshot({ path: 'mobile_dashboard.png', fullPage: true });
});
