import { test, expect } from '@playwright/test';

test('audit dashboard and onboarding', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 800 });
  await page.goto('/login');
  await page.screenshot({ path: 'audit_login.png' });

  await page.fill('input[type="email"]', 'test@example.com');
  await page.fill('input[type="password"]', 'password123');
  await page.click('button:has-text("Sign In")');

  // Wait for dashboard or wizard
  await page.waitForTimeout(2000);
  await page.screenshot({ path: 'audit_dashboard.png' });

  // Try to find the "?" button
  const helpBtn = page.locator('button:has-text("?")');
  if (await helpBtn.isVisible()) {
      await helpBtn.click();
      await page.screenshot({ path: 'audit_dashboard_hint.png' });
  }

  // Open menu
  const menuBtn = page.locator('button:has-text("Menu")');
  if (await menuBtn.isVisible()) {
      await menuBtn.click();
      await page.screenshot({ path: 'audit_dashboard_menu.png' });
  }
});
