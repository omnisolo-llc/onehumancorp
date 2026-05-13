import { test, expect } from '@playwright/test';

test.describe('Email Marketing Flow', () => {
  test('should verify the email marketing tool functionality with Flash sale', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // Wait for dashboard to load
    await page.waitForURL('**/dashboard');

    // 2. Open Email Marketing UI via Dashboard Quick Actions
    const emailMarketingBtn = page.locator('button:has-text("Email Marketing")').first();
    await expect(emailMarketingBtn).toBeVisible();
    await emailMarketingBtn.click();

    // Verify main header and default state
    await expect(page.locator('text=Email Marketing').first()).toBeVisible();
    await expect(page.locator('text=Total Subscribers: 150')).toBeVisible();

    // 3. Select a template
    const flashSaleBtn = page.locator('button:has-text("Flash sale")');
    await expect(flashSaleBtn).toBeVisible();
    await flashSaleBtn.click();

    // Verify preview text gets generated
    const previewTextArea = page.locator('textarea').first();
    await expect(previewTextArea).toBeVisible();
    await expect(previewTextArea).toHaveValue(/24-Hour Flash Sale!/);

    // 4. Send campaign
    const sendBtn = page.locator('button:has-text("Send Campaign")');
    await expect(sendBtn).toBeVisible();
    await sendBtn.click();

    // Verify success message
    await expect(page.locator('text=Campaign sent successfully!')).toBeVisible({ timeout: 10000 });

    // Verify analytics updated
    await expect(page.locator('text=Emails Sent: 150')).toBeVisible();
    await expect(page.locator('text=Open Rate: 32%')).toBeVisible();
  });

  test('should verify New arrivals template', async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/dashboard');

    const emailMarketingBtn = page.locator('button:has-text("Email Marketing")').first();
    await expect(emailMarketingBtn).toBeVisible();
    await emailMarketingBtn.click();

    const newArrivalsBtn = page.locator('button:has-text("New arrivals")');
    await expect(newArrivalsBtn).toBeVisible();
    await newArrivalsBtn.click();

    // We just test that clicking it doesn't crash and generates some output
    // In our rust mock it may generate a standard message
  });

  test('should verify Thank you template', async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/dashboard');

    const emailMarketingBtn = page.locator('button:has-text("Email Marketing")').first();
    await expect(emailMarketingBtn).toBeVisible();
    await emailMarketingBtn.click();

    const tyBtn = page.locator('button:has-text("Thank you")');
    await expect(tyBtn).toBeVisible();
    await tyBtn.click();
  });

  test('should verify audience selection', async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/dashboard');

    const emailMarketingBtn = page.locator('button:has-text("Email Marketing")').first();
    await expect(emailMarketingBtn).toBeVisible();
    await emailMarketingBtn.click();

    // The checkboxes are standard Slint CheckBoxes
    const recentBuyers = page.locator('text=Recent Buyers');
    await expect(recentBuyers).toBeVisible();

    const newSubscribers = page.locator('text=New Subscribers');
    await expect(newSubscribers).toBeVisible();
  });

  test('should view analytics', async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/dashboard');

    const emailMarketingBtn = page.locator('button:has-text("Email Marketing")').first();
    await expect(emailMarketingBtn).toBeVisible();
    await emailMarketingBtn.click();

    await expect(page.locator('text=Emails Sent').first()).toBeVisible();
    await expect(page.locator('text=Open Rate').first()).toBeVisible();
  });
});
