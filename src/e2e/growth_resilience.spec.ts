import { test, expect } from '@playwright/test';

const viewports = [
  { width: 375, height: 667, name: 'Mobile' },
  { width: 768, height: 1024, name: 'Tablet' },
  { width: 1440, height: 900, name: 'Desktop' }
];

for (const vp of viewports) {
  test(`Referral Flow across viewport: ${vp.name} (${vp.width}x${vp.height})`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });

    // 1. Authenticate and navigate to dashboard
    await page.goto('/login');
    const emailInput = page.getByPlaceholder(/email/i).first();
    const passwordInput = page.getByPlaceholder(/password/i).first();

    await expect(emailInput).toBeVisible();
    await emailInput.fill('test@example.com');
    await passwordInput.fill('password123');
    await page.getByRole('button', { name: /log in/i }).click();

    // Wait for the Dashboard
    await page.waitForURL('**/dashboard**', { timeout: 10000 });

    // Check if the referrals button is visible directly (might be different across viewports)
    // On small screens, it may be in a drawer/menu.
    const referralsBtn = page.locator('button:has-text("Referrals")').first();
    await expect(referralsBtn).toBeVisible({ timeout: 5000 });
    await referralsBtn.click();

    // Wait for the Referral Dashboard to load
    await expect(page.locator('text=Referral Program').first()).toBeVisible({ timeout: 5000 });

    // Check for "Share the Love, Get Pro" text
    await expect(page.locator('text=Share the Love, Get Pro').first()).toBeVisible();

    // Verify copy button exists
    const copyBtn = page.locator('button:has-text("Copy")').first();
    await expect(copyBtn).toBeVisible();

    // Verify share buttons exist
    await expect(page.locator('button:has-text("Share to Instagram")').first()).toBeVisible();
    await expect(page.locator('button:has-text("Copy Invite Message")').first()).toBeVisible();
  });
}

for (const vp of viewports) {
  test(`Social Auto-Posting Flow across viewport: ${vp.name} (${vp.width}x${vp.height})`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto('/login');
    await page.getByPlaceholder(/email/i).first().fill('test@example.com');
    await page.getByPlaceholder(/password/i).first().fill('password123');
    await page.getByRole('button', { name: /log in/i }).click();
    await page.waitForURL('**/dashboard**', { timeout: 10000 });

    const growBusinessBtn = page.locator('button:has-text("Grow Business")').first();
    await expect(growBusinessBtn).toBeVisible({ timeout: 5000 });
    await growBusinessBtn.click();

    // Connect Instagram
    const connectIgBtn = page.locator('button:has-text("Connect Instagram")');
    await expect(connectIgBtn).toBeVisible();
    await connectIgBtn.click();
    await expect(page.locator('text=📸 Connect Instagram').first()).toBeVisible();

    const nextBtn = page.locator('button:has-text("Next")');
    await expect(nextBtn).toBeVisible();
    await nextBtn.click();

    const executeBtn = page.locator('button:has-text("Launch Strategy")');
    await expect(executeBtn).toBeVisible();
    await executeBtn.click();

    const returnBtn = page.locator('button:has-text("Return to Dashboard")');
    await expect(returnBtn).toBeVisible();
    await returnBtn.click();

    await expect(page.locator('text=Drafted Instagram Post').first()).toBeVisible({ timeout: 10000 });
    const approveBtn = page.locator('button:has-text("Approve & Send")').first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();
    await expect(page.locator('text=Drafted Instagram Post').first()).toBeHidden();
  });

  test(`Email Marketing Flow across viewport: ${vp.name} (${vp.width}x${vp.height})`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto('/login');
    await page.getByPlaceholder(/email/i).first().fill('test@example.com');
    await page.getByPlaceholder(/password/i).first().fill('password123');
    await page.getByRole('button', { name: /log in/i }).click();
    await page.waitForURL('**/dashboard**', { timeout: 10000 });

    const emailMarketingBtn = page.locator('button:has-text("Email Marketing")').first();
    await expect(emailMarketingBtn).toBeVisible({ timeout: 5000 });
    await emailMarketingBtn.click();

    await expect(page.locator('text=Email Marketing').first()).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text=Total Subscribers:').first()).toBeVisible();

    const flashSaleBtn = page.locator('button:has-text("Flash sale")').first();
    await expect(flashSaleBtn).toBeVisible();
    await flashSaleBtn.click();

    const sendBtn = page.locator('button:has-text("Send Campaign")').first();
    await expect(sendBtn).toBeVisible();
    await sendBtn.click();
  });
}

for (const vp of viewports) {
  test(`Business Share Flow across viewport: ${vp.name} (${vp.width}x${vp.height})`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto('/login');
    await page.getByPlaceholder(/email/i).first().fill('test@example.com');
    await page.getByPlaceholder(/password/i).first().fill('password123');
    await page.getByRole('button', { name: /log in/i }).click();
    await page.waitForURL('**/dashboard**', { timeout: 10000 });

    // The "Share Store" button might be in the bottom nav on mobile
    const shareBtn = page.locator('button:has-text("Share Store")').first();
    await expect(shareBtn).toBeVisible({ timeout: 5000 });
    await shareBtn.click();

    // Share Your Store Dialog
    await expect(page.locator('text=Share Your Store').first()).toBeVisible({ timeout: 5000 });

    // Verify preview card elements
    await expect(page.locator('text=My Awesome Store').first()).toBeVisible();
    await expect(page.locator('text=The best place to buy things').first()).toBeVisible();

    // Verify action buttons
    await expect(page.locator('button:has-text("Copy Shareable Link")').first()).toBeVisible();
    await expect(page.locator('button:has-text("Instagram")').first()).toBeVisible();
    await expect(page.locator('button:has-text("X (Twitter)")').first()).toBeVisible();
    await expect(page.locator('button:has-text("Share to WhatsApp")').first()).toBeVisible();
  });
}
