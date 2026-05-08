import { test, expect } from '@playwright/test';

test('Test 1: Login Error Visualization', async ({ page }) => {
  // 1. Navigate to the app (defaults to login if unauthenticated usually)
  await page.goto('/');

  // Ensure we are on login screen. We'll simulate a failed login.
  const hasLogin = await page.locator('button:has-text("Sign In")').isVisible().catch(() => false);
  if (hasLogin) {
    await page.fill('input[type="text"]', 'invalid@user.com');
    await page.fill('input[type="password"]', 'wrong');
    await page.click('button:has-text("Sign In")');

    // The exact error message depends on the mock API, but we ensure it is visible
    // and that the old hardcoded "Please check your email..." text is gone
    // (or rather, the actual error message is shown).
    await expect(page.locator('text=We couldn\'t sign you in')).toBeVisible();
  }
});

test('Test 2: Dashboard Store Health Label', async ({ page }) => {
  await page.goto('/');
  // Proceed to dashboard if needed
  await expect(page.locator('text=Store Health')).toBeVisible();
  await expect(page.locator('text=Generative Score')).toBeHidden();
});

test('Test 3: Dashboard Intuitive Navigation', async ({ page }) => {
  await page.goto('/');
  await expect(page.locator('text=Add')).toBeVisible();
  await expect(page.locator('text=Orders')).toBeVisible();
  await expect(page.locator('text=Chat')).toBeVisible();
  await expect(page.locator('text=Stats')).toBeVisible();
  await expect(page.locator('text=Share')).toBeVisible();
});

test('Test 4: Dashboard Menu Plain Language', async ({ page }) => {
  await page.goto('/');
  await page.click('button:has-text("Menu")');
  await expect(page.locator('text=What\'s New')).toBeVisible();
});

test('Test 5: Dashboard Quick Actions Plain Language', async ({ page }) => {
  await page.goto('/');
  await expect(page.locator('text=Quick Actions')).toBeVisible();
  await expect(page.locator('text=Grow')).toBeVisible();
  await expect(page.locator('text=Referrals')).toBeVisible();
  await expect(page.locator('text=Emails')).toBeVisible();
  await expect(page.locator('text=Social')).toBeVisible();
});
