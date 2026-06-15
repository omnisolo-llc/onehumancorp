import { test, expect } from '@playwright/test';

test.describe('Maya Post-Visual Remediation Audit', () => {
  test('Capture Full Dashboard - 375px', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 2000 });

    // Dashboard
    await page.goto('http://localhost:3000/dashboard');
    await page.waitForSelector('h2:has-text("Welcome back")');
    // Scroll a bit to ensure everything loads
    await page.evaluate(() => window.scrollTo(0, 1000));
    await page.waitForTimeout(500);
    await page.evaluate(() => window.scrollTo(0, 0));

    await page.screenshot({ path: 'screenshots/dashboard_full_375px.png', fullPage: true });
  });
});
