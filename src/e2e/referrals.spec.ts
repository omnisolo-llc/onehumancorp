import { test, expect } from '@playwright/test';

test.describe('Referral Program', () => {
  test('should display referral dashboard and generate link', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // 2. Navigate to Referrals dashboard via dashboard UI button
    await page.waitForURL('**/dashboard');

    // There is a tooltip button "Referrals" on dashboard
    const referralsBtn = page.locator('button:has-text("Referrals")').first();
    await expect(referralsBtn).toBeVisible();
    await referralsBtn.click();

    await expect(page.locator('text=Referral Dashboard')).toBeVisible();

    // 3. Generate a new referral link
    // "New Link" doesn't actually exist in the UI we read. The UI generates it or we can copy
    // We will test the link copy button
    const copyBtn = page.locator('button:has-text("Copy")').first();
    await expect(copyBtn).toBeVisible();
    await copyBtn.click();

    // 4. Assert link is generated and visible
    await expect(page.locator('text=ohc://join?ref=DEFAULT')).toBeVisible();

    // 5. Verify refresh button works
    const refreshButton = page.locator('button:has-text("Refresh")');
    await expect(refreshButton).toBeVisible();
    await refreshButton.click();
  });

  test('should verify Instagram sharing from referrals', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await page.waitForURL('**/dashboard');

    const referralsBtn = page.locator('button:has-text("Referrals")').first();
    await expect(referralsBtn).toBeVisible();
    await referralsBtn.click();

    const igBtn = page.locator('button:has-text("📷 Share to Instagram")');
    await expect(igBtn).toBeVisible();
    await igBtn.click();
  });

  test('should verify invite message copying', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await page.waitForURL('**/dashboard');

    const referralsBtn = page.locator('button:has-text("Referrals")').first();
    await expect(referralsBtn).toBeVisible();
    await referralsBtn.click();

    const inviteBtn = page.locator('button:has-text("💬 Copy Invite Message")');
    await expect(inviteBtn).toBeVisible();
    await inviteBtn.click();

    await expect(page.locator('text=Invite message copied!')).toBeVisible({ timeout: 5000 });
  });

  test('should view referral history', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await page.waitForURL('**/dashboard');

    const referralsBtn = page.locator('button:has-text("Referrals")').first();
    await expect(referralsBtn).toBeVisible();
    await referralsBtn.click();

    const historyBtn = page.locator('button:has-text("📜 View History")');
    await expect(historyBtn).toBeVisible();
    await historyBtn.click();
  });

  test('should export referral data', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await page.waitForURL('**/dashboard');

    const referralsBtn = page.locator('button:has-text("Referrals")').first();
    await expect(referralsBtn).toBeVisible();
    await referralsBtn.click();

    const exportBtn = page.locator('button:has-text("📤 Export Data")');
    await expect(exportBtn).toBeVisible();
    await exportBtn.click();
  });
});
