import { test, expect } from '@playwright/test';

test.describe('Social Media Autoposting Flow', () => {
  test('user can connect Instagram and receive an automated post approval task', async ({ page }) => {
    // 1. Authenticate and navigate to dashboard
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // Wait for the Dashboard
    await page.waitForURL('**/dashboard');

    // We will just wait a brief moment for the dashboard to settle
    await page.waitForTimeout(500);

    // Click the grow business action button on dashboard
    const growBusinessBtn = page.locator('button:has-text("Grow Business")').first();
    await expect(growBusinessBtn).toBeVisible();
    await growBusinessBtn.click();

    // Wait for the modal or navigation
    await page.waitForTimeout(500);

    // 3. Connect Instagram
    const connectIgBtn = page.locator('button:has-text("Connect Instagram")');
    await expect(connectIgBtn).toBeVisible();
    await connectIgBtn.click();

    // Verify selected
    await expect(page.locator('text=📸 Connect Instagram').first()).toBeVisible();

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
    await page.waitForTimeout(1000);

    // 5. Check for Drafted Instagram Post in the Agent Activity Feed
    await expect(page.locator('text=Drafted Instagram Post').first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Check out our new products!').first()).toBeVisible();

    // 6. Approve the post
    const approveBtn = page.locator('button:has-text("Approve & Send")').first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify it disappears from the feed
    await expect(page.locator('text=Drafted Instagram Post').first()).toBeHidden();
  });
});