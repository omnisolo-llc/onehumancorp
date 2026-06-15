import { test, expect } from '@playwright/test';

test.describe('Maya Initial Audit', () => {
  test('Capture initial state screenshots', async ({ page }) => {
    console.log('Navigating to login...');
    await page.goto('http://localhost:3000/login');
    await page.screenshot({ path: 'screenshots/login_initial_1440px.png', fullPage: true });

    console.log('Navigating to dashboard...');
    await page.goto('http://localhost:3000/dashboard');
    // Wait for something specific to be sure it's loaded
    await page.waitForLoadState('networkidle');
    await page.screenshot({ path: 'screenshots/dashboard_initial_1440px.png', fullPage: true });

    console.log('Switching to mobile viewport (375px)...');
    await page.setViewportSize({ width: 375, height: 667 });
    await page.waitForTimeout(2000);
    await page.screenshot({ path: 'screenshots/dashboard_initial_375px.png', fullPage: true });
  });
});
