import { test, expect } from '@playwright/test';

test.describe('Social Media Autoposting Flow', () => {
  test('user can connect Instagram and receive an automated post approval task', async ({ page }) => {
    // 1. Authenticate and navigate to dashboard
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    // Wait for the Dashboard
    await page.waitForURL('**/dashboard');

    // We will just wait a brief moment for the dashboard to settle
    await page.waitForTimeout(500);

    // Click the grow business action button on dashboard
    const growBusinessBtn = page.locator('button:has-text("Grow Business")').filter({ visible: true }).first();
    await expect(growBusinessBtn).toBeVisible();
    await growBusinessBtn.click();

    // Wait for the modal or navigation
    await page.waitForTimeout(500);

    // 3. Connect Instagram
    const connectIgBtn = page.locator('button:has-text("Connect Instagram")');
    await expect(connectIgBtn).toBeVisible();
    await connectIgBtn.click();

    // Verify selected
    await expect(page.locator('text=📸 Connect Instagram').filter({ visible: true }).first()).toBeVisible();

    // Move to next step
    const nextBtn = page.locator('button:has-text("Next")');
    await expect(nextBtn).toBeVisible();
    await nextBtn.click();

    // Confirm step and execute
    const executeBtn = page.locator('button:has-text("Launch Strategy")');
    await expect(executeBtn).toBeVisible();
    await executeBtn.click();

    // 4. Return to Dashboard
    const returnBtn = page.locator('button:has-text("Return to Dashboard")');
    await expect(returnBtn).toBeVisible();
    await returnBtn.click();

    // The modal actually just hides, we don't need to navigate
    // Wait for dashboard view

    // 5. Check for Drafted Instagram Post in the Agent Activity Feed
    await expect(page.locator('text=Drafted Instagram Post').filter({ visible: true }).first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Check out our new products!').filter({ visible: true }).first()).toBeVisible();

    // 6. Approve the post
    const approveBtn = page.locator('button:has-text("Approve & Send")').filter({ visible: true }).first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify it disappears from the feed
    await expect(page.locator('text=Drafted Instagram Post').filter({ visible: true }).first()).toBeHidden();
  });

  test('user can connect Facebook', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    await page.goto('/social-posting');

    const connectFbBtn = page.locator('button:has-text("Connect Facebook")');
    await expect(connectFbBtn).toBeVisible();
    await connectFbBtn.click();

    await expect(page.locator('button:has-text("Facebook Connected")')).toBeVisible();
  });

  test('user can edit an AI draft', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    await page.goto('/social-posting');

    const generateBtn = page.locator('button:has-text("Generate Post with AI")');
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    const textArea = page.locator('textarea').filter({ visible: true }).first();
    await expect(textArea).toBeVisible();
    await textArea.fill('My edited custom post text!');

    await expect(page.locator('text=My edited custom post text!').last()).toBeVisible();
  });

  test('user can schedule a post', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    await page.goto('/social-posting');

    const scheduleBtn = page.locator('button:has-text("Schedule")');
    await expect(scheduleBtn).toBeVisible();
    await scheduleBtn.click();
  });

  test('user can approve a post', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    await page.goto('/social-posting');

    const approveBtn = page.locator('button:has-text("Approve & Post Now")');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();
  });
});
