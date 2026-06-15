import { test, expect } from '@playwright/test';

test.describe('Maya Post-Visual Remediation Audit', () => {
  test('Capture Dashboard and Login - 1440px', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });

    // Login Screen
    await page.goto('http://localhost:3000/login');
    await page.screenshot({ path: 'screenshots/login_remediated_1440px.png' });

    // Dashboard
    await page.goto('http://localhost:3000/dashboard');
    // Wait for the dashboard to load (assuming it has some identifiable element)
    await page.waitForSelector('text=Dashboard');
    await page.screenshot({ path: 'screenshots/dashboard_remediated_1440px.png' });
  });

  test('Capture Dashboard and Login - 375px', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    // Login Screen
    await page.goto('http://localhost:3000/login');
    await page.screenshot({ path: 'screenshots/login_remediated_375px.png' });

    // Dashboard
    await page.goto('http://localhost:3000/dashboard');
    await page.waitForSelector('h2:has-text("Welcome back")');
    await page.screenshot({ path: 'screenshots/dashboard_remediated_375px.png' });
  });
});
