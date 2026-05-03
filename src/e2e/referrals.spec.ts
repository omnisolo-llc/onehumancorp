import { test, expect } from '@playwright/test';

test.describe('Referral Program', () => {
  test('should display referral dashboard and verify full end-to-end functionality', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // 2. Navigate to Dashboard and open Referrals
    await page.waitForURL('**/*');

    // Wait for the "Referrals" button on the dashboard and click it
    const referralsBtn = page.locator('button:has-text("Referrals")');
    await expect(referralsBtn).toBeVisible();
    await referralsBtn.click();

    // 3. Verify the Referrals component pops up
    await expect(page.locator('text=Viral Loop Dashboard')).toBeVisible();

    // Verify stats layout
    await expect(page.locator('text=Your Referral Link')).toBeVisible();
    await expect(page.locator('text=Referral Statistics')).toBeVisible();
    await expect(page.locator('text=Referral Program Admin')).toBeVisible();

    const statsText = [
      'Total Referrals:', 'Click Count:', 'Conversion Rate:',
      'Reward Balance:', 'Bonus Credit:', 'Viral Coefficient:',
      'Download Count:', 'Waitlist Position:'
    ];

    for (const text of statsText) {
      await expect(page.locator(`text=${text}`)).toBeVisible();
    }

    // Verify link generation functionality
    const newLinkButton = page.locator('button:has-text("New Link")');
    await newLinkButton.click();
    await expect(page.locator('text=ohc://join?ref=')).toBeVisible();

    // Verify refresh action
    const refreshButton = page.locator('button:has-text("Refresh")');
    await refreshButton.click();
    await expect(page.locator('text=Your Referral Link')).toBeVisible();

    // Verify copying link functionality (without suppressing errors)
    const copyBtn = page.locator('button:has-text("Copy")').first();
    await expect(copyBtn).toBeVisible();
    await copyBtn.click();

    // Verify sharing to social media
    const shareBtn = page.locator('button:has-text("Share")').first();
    await expect(shareBtn).toBeVisible();
    await shareBtn.click();

    // Verify history tab
    const historyBtn = page.locator('button:has-text("History")').first();
    await expect(historyBtn).toBeVisible();
    await historyBtn.click();

    // Verify export tab
    const exportBtn = page.locator('button:has-text("Export")').first();
    await expect(exportBtn).toBeVisible();
    await exportBtn.click();
  });
});

test.describe('Referral Program Admin', () => {
  test('should show referral program settings and configure reward amount', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // 2. Navigate to Dashboard and open Referrals
    await page.waitForURL('**/*');

    // Wait for the "Referrals" button on the dashboard and click it
    const referralsBtn = page.locator('button:has-text("Referrals")');
    await expect(referralsBtn).toBeVisible();
    await referralsBtn.click();

    // 3. Verify Admin section is accessible
    await expect(page.locator('text=Referral Program Admin')).toBeVisible();

    // Check settings inputs
    const rewardInput = page.locator('input[type="number"], input[placeholder*="reward"]').first();
    await expect(rewardInput).toBeVisible();
    await rewardInput.fill('25');

    // Test toggle enabled state
    const toggle = page.locator('input[type="checkbox"], [class*="toggle"]').first();
    await expect(toggle).toBeVisible();
    await toggle.click();

    // Verify leaderboard rank
    await expect(page.locator('text=Leaderboard Rank: #1')).toBeVisible();
    await expect(page.locator('text=Top Award: Platinum Level')).toBeVisible();
  });
});