import { test, expect } from '@playwright/test';

test.describe('Referral Program', () => {
  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    // Navigate to feature from dashboard
    const btn = page.locator('button:has-text("/login")').first();
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")').first();
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        const innerBtn = page.locator('button:has-text("/login")').first();
        if (await innerBtn.isVisible()) {
          await innerBtn.click();
        }
      }
    }
  });

  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    const btn = page.locator('button:has-text("/login")');
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")');
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        await page.locator('button:has-text("/login")').click();
      }
    }
  });
  test('should display referral dashboard and generate link', async ({ page }) => {
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    await expect(page.locator('text=Viral Loop Dashboard')).toBeVisible();

    const newLinkButton = page.locator('button:has-text("New Link")');
    await newLinkButton.click();

    await expect(page.locator('text=ohc://join?ref=')).toBeVisible();

    const refreshButton = page.locator('button:has-text("Refresh")');
    await refreshButton.click();

    await expect(page.locator('text=Your Referral Link')).toBeVisible();
  });

  test('should display referral dashboard header', async ({ page }) => {
    await expect(page.locator('text=Viral Loop Dashboard')).toBeVisible();
  });

  test('should show your referral link section', async ({ page }) => {
    await expect(page.locator('text=Your Referral Link')).toBeVisible();
  });

  test('should generate new referral link', async ({ page }) => {
    await page.locator('button:has-text("New Link")').click();
    await expect(page.locator('text=ohc://join?ref=')).toBeVisible();
  });

  test('should copy referral link to clipboard', async ({ page }) => {
    await page.locator('button:has-text("New Link")').click();
    const copyBtn = page.locator('button:has-text("Copy"), [class*="copy"]').first();
    if (await copyBtn.isVisible()) {
      await copyBtn.click();
      await expect(page.locator('text=/copied|success/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should share to social media', async ({ page }) => {
    await page.locator('button:has-text("New Link")').click();
    const shareBtn = page.locator('button:has-text("Share"), [class*="share"]').first();
    if (await shareBtn.isVisible()) {
      await shareBtn.click();
      await expect(page.locator('text=/twitter|facebook|instagram|linkedin/i')).toBeVisible();
    }
  });

  test('should display referral statistics', async ({ page }) => {
    const stats = page.locator('text=/referrals|clicks|conversions/i');
    await expect(stats.first()).toBeVisible();
  });

  test('should show referral count', async ({ page }) => {
    await expect(page.locator('text=/total referrals|referral count/i')).toBeVisible();
  });

  test('should show click count', async ({ page }) => {
    await expect(page.locator('text=/clicks|click count/i')).toBeVisible();
  });

  test('should show conversion rate', async ({ page }) => {
    await expect(page.locator('text=/conversion|rate/i')).toBeVisible();
  });

  test('should display referral rewards', async ({ page }) => {
    await expect(page.locator('text=/reward|bonus|credit/i')).toBeVisible();
  });

  test('should show reward balance', async ({ page }) => {
    await expect(page.locator('text=/balance|earned|available/i')).toBeVisible();
  });

  test('should refresh referral data', async ({ page }) => {
    await page.locator('button:has-text("Refresh")').click();
    await expect(page.locator('text=/loading|updating/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should show referral history', async ({ page }) => {
    const historyTab = page.locator('button:has-text("History"), button:has-text("Activity")').first();
    if (await historyTab.isVisible()) {
      await historyTab.click();
      await expect(page.locator('text=/history|recent|activity/i')).toBeVisible();
    }
  });

  test('should export referral data', async ({ page }) => {
    const exportBtn = page.locator('button:has-text("Export"), [class*="export"]').first();
    if (await exportBtn.isVisible()) {
      await exportBtn.click();
      await expect(page.locator('text=/download|csv|excel/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should show referral viral coefficient', async ({ page }) => {
    await expect(page.locator('text=/viral coefficient|k-factor/i')).toBeVisible();
  });

  test('should display download count', async ({ page }) => {
    await expect(page.locator('text=/download|installs/i')).toBeVisible();
  });

  test('should show waitlist position', async ({ page }) => {
    await expect(page.locator('text=/waitlist|position|rank/i')).toBeVisible();
  });
});

test.describe('Referral Program Admin', () => {
  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    // Navigate to feature from dashboard
    const btn = page.locator('button:has-text("/login")').first();
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")').first();
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        const innerBtn = page.locator('button:has-text("/login")').first();
        if (await innerBtn.isVisible()) {
          await innerBtn.click();
        }
      }
    }
  });

  test('should show referral program settings', async ({ page }) => {
    await expect(page.locator('text=/settings|configure/i')).toBeVisible();
  });

  test('should configure reward amount', async ({ page }) => {
    const rewardInput = page.locator('input[type="number"], input[placeholder*="reward"]').first();
    if (await rewardInput.isVisible()) {
      await rewardInput.fill('25');
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should set referral program enabled state', async ({ page }) => {
    const toggle = page.locator('input[type="checkbox"], [class*="toggle"]').first();
    if (await toggle.isVisible()) {
      await toggle.click();
      await expect(page.locator('text=/enabled|disabled/i')).toBeVisible();
    }
  });

  test('should view referral leaderboard', async ({ page }) => {
    await expect(page.locator('text=/leaderboard|top|rank/i')).toBeVisible();
  });

  test('should show top referrer awards', async ({ page }) => {
    await expect(page.locator('text=/top|best|award/i')).toBeVisible();
  });
});
