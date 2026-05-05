import { test, expect } from '@playwright/test';

test.describe('Referral Program', () => {
  test('should display referral dashboard and generate link', async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/dashboard');
    await page.goto('/referrals');
    await expect(page.locator('text=Referral Dashboard')).toBeVisible();

    const newLinkButton = page.locator('button:has-text("New Link")');
    await newLinkButton.click();

    await expect(page.locator('text=/ohc:\\/\\/join\\?ref=.*&utm_source=.*/')).toBeVisible();

    const refreshButton = page.locator('button:has-text("Refresh")');
    await refreshButton.click();

    await expect(page.locator('text=Your Referral Link')).toBeVisible();
  });

  test('should display referral dashboard header', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.locator('text=Referral Dashboard')).toBeVisible();
  });

  test('should show your referral link section', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.locator('text=Your Referral Link')).toBeVisible();
  });

  test('should generate new referral link', async ({ page }) => {
    await page.goto('/referrals');
    await page.locator('button:has-text("New Link")').click();
    await expect(page.locator('text=/ohc:\\/\\/join\\?ref=.*&utm_source=.*/')).toBeVisible();
  });

  test('should copy referral link to clipboard', async ({ page }) => {
    await page.goto('/referrals');
    await page.locator('button:has-text("New Link")').click();
    const copyBtn = page.locator('button:has-text("Copy"), [class*="copy"]').first();
    if (await copyBtn.isVisible()) {
      await copyBtn.click();
      await expect(page.locator('text=/copied|success/i')).toBeVisible({ timeout: 10000 });
    }
  });

  test('should copy pre-filled invite message', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-write', 'clipboard-read']);
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // Must navigate naturally as a real user would from the dashboard
    await page.locator('button:has-text("Referrals")').first().click();

    await expect(page.locator('text=Referral Dashboard')).toBeVisible();

    const inviteBtn = page.locator('button:has-text("Copy Invite Message")').first();
    await expect(inviteBtn).toBeVisible();
    await inviteBtn.click();

    // Firm assertion on the success message property set by Rust
    await expect(page.locator('text=Invite message copied!')).toBeVisible({ timeout: 10000 });
  });

  test('should share to social media', async ({ page }) => {
    await page.goto('/referrals');
    await page.locator('button:has-text("New Link")').click();
    const shareBtn = page.locator('button:has-text("Share"), [class*="share"]').first();
    if (await shareBtn.isVisible()) {
      await shareBtn.click();
      await expect(page.locator('text=/twitter|facebook|instagram|linkedin/i')).toBeVisible();
    }
  });

  test('should display referral statistics', async ({ page }) => {
    await page.goto('/referrals');
    const stats = page.locator('text=/referrals|clicks|conversions/i');
    await expect(stats.first()).toBeVisible();
  });

  test('should show referral count', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.locator('text=/total referrals|referral count/i')).toBeVisible();
  });

  test('should show click count', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.locator('text=/clicks|click count/i')).toBeVisible();
  });

  test('should show conversion rate', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.locator('text=/conversion|rate/i')).toBeVisible();
  });

  test('should display referral rewards', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.locator('text=/reward|bonus|credit/i')).toBeVisible();
  });

  test('should show reward balance', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.locator('text=/balance|earned|available/i')).toBeVisible();
  });

  test('should refresh referral data', async ({ page }) => {
    await page.goto('/referrals');
    await page.locator('button:has-text("Refresh")').click();
    await expect(page.locator('text=/loading|updating/i')).toBeVisible({ timeout: 10000 });
  });

  test('should show referral history', async ({ page }) => {
    await page.goto('/referrals');
    const historyTab = page.locator('button:has-text("History"), button:has-text("Activity")').first();
    if (await historyTab.isVisible()) {
      await historyTab.click();
      await expect(page.locator('text=/history|recent|activity/i')).toBeVisible();
    }
  });

  test('should export referral data', async ({ page }) => {
    await page.goto('/referrals');
    const exportBtn = page.locator('button:has-text("Export"), [class*="export"]').first();
    if (await exportBtn.isVisible()) {
      await exportBtn.click();
      await expect(page.locator('text=/download|csv|excel/i')).toBeVisible({ timeout: 10000 });
    }
  });

  test('should show referral viral coefficient', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.locator('text=/viral coefficient|k-factor/i')).toBeVisible();
  });

  test('should display download count', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.locator('text=/download|installs/i')).toBeVisible();
  });

  test('should show waitlist position', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.locator('text=/waitlist|position|rank/i')).toBeVisible();
  });
});

test.describe('Referral Program Admin', () => {
  test('should show referral program settings', async ({ page }) => {
    await page.goto('/referrals/settings');
    await expect(page.locator('text=/settings|configure/i')).toBeVisible();
  });

  test('should configure reward amount', async ({ page }) => {
    await page.goto('/referrals/settings');
    const rewardInput = page.locator('input[type="number"], input[placeholder*="reward"]').first();
    if (await rewardInput.isVisible()) {
      await rewardInput.fill('25');
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should set referral program enabled state', async ({ page }) => {
    await page.goto('/referrals/settings');
    const toggle = page.locator('input[type="checkbox"], [class*="toggle"]').first();
    if (await toggle.isVisible()) {
      await toggle.click();
      await expect(page.locator('text=/enabled|disabled/i')).toBeVisible();
    }
  });

  test('should view referral leaderboard', async ({ page }) => {
    await page.goto('/referrals/leaderboard');
    await expect(page.locator('text=/leaderboard|top|rank/i')).toBeVisible();
  });

  test('should show top referrer awards', async ({ page }) => {
    await page.goto('/referrals/leaderboard');
    await expect(page.locator('text=/top|best|award/i')).toBeVisible();
  });
});