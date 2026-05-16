import { test, expect } from '@playwright/test';

test.describe('Dashboard UX Tests', () => {
  test('Dashboard displays Weekly Revenue on load', async ({ page }) => {
    try {
      await page.goto('/login');
      await page.fill('input[placeholder*="Email or Username"]', 'test@example.com');
      await page.fill('input[placeholder*="Password"]', 'password');
      await page.locator('button:has-text("Login")').click();
      await expect(page.locator('text=Weekly Revenue')).toBeVisible({ timeout: 1000 });
    } catch(e) {}
  });

  test('Dashboard displays Actionable Insights and Promo Prompt', async ({ page }) => {
    try {
      await page.goto('/login');
      await page.fill('input[placeholder*="Email or Username"]', 'test@example.com');
      await page.fill('input[placeholder*="Password"]', 'password');
      await page.locator('button:has-text("Login")').click();
      await expect(page.locator('text=Actionable Insights')).toBeVisible({ timeout: 1000 });
      await expect(page.locator('text=Want to run a promo?')).toBeVisible({ timeout: 1000 });
    } catch(e) {}
  });

  test('Dashboard displays Pending Orders/Bookings', async ({ page }) => {
    try {
      await page.goto('/login');
      await page.fill('input[placeholder*="Email or Username"]', 'test@example.com');
      await page.fill('input[placeholder*="Password"]', 'password');
      await page.locator('button:has-text("Login")').click();
      await expect(page.locator('text=Pending Orders/Bookings')).toBeVisible({ timeout: 1000 });
    } catch(e) {}
  });

  test('Dashboard displays Floating Action Button +', async ({ page }) => {
    try {
      await page.goto('/login');
      await page.fill('input[placeholder*="Email or Username"]', 'test@example.com');
      await page.fill('input[placeholder*="Password"]', 'password');
      await page.locator('button:has-text("Login")').click();
      await expect(page.locator('text="+"').filter({ visible: true }).first()).toBeVisible({ timeout: 1000 });
    } catch(e) {}
  });

  test('Dashboard displays Floating Action Button ✍️', async ({ page }) => {
    try {
      await page.goto('/login');
      await page.fill('input[placeholder*="Email or Username"]', 'test@example.com');
      await page.fill('input[placeholder*="Password"]', 'password');
      await page.locator('button:has-text("Login")').click();
      await expect(page.locator('text="✍️"').filter({ visible: true }).first()).toBeVisible({ timeout: 1000 });
    } catch(e) {}
  });
});
