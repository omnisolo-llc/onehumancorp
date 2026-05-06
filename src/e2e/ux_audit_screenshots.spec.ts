import { test, expect } from '@playwright/test';

const resolutions = [
  { name: 'mobile', width: 375, height: 812 },
  { name: 'tablet', width: 768, height: 1024 },
  { name: 'desktop', width: 1440, height: 900 },
];

test.describe('UX Audit - Before Screenshots', () => {
  for (const res of resolutions) {
    test(`Capture screenshot at ${res.width}x${res.height}`, async ({ page }) => {
      await page.setViewportSize({ width: res.width, height: res.height });

      // Navigate to the app (assuming it's running)
      await page.goto('/');

      // Login if necessary. For audit, we want to see the dashboard.
      // Based on main.rs, the login screen is shown first.
      await page.fill('input[type="email"]', 'test@example.com');
      await page.fill('input[type="password"]', 'password');
      await page.click('button:has-text("Login")');

      // Wait for dashboard to load
      await page.waitForSelector('text=My Business');

      // Take screenshot
      await page.screenshot({ path: `ux_audit_before_${res.width}.png`, fullPage: true });
    });
  }
});
