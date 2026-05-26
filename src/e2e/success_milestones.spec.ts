import { test, expect } from './fixtures';

test.describe('Success Milestones Notifications', () => {
  test('should verify milestone functionality when order threshold is reached', async ({ page }) => {
    // 1. Authenticate and navigate to dashboard
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
    await page.waitForURL('**/*');

    // 2. Wait for the dashboard to load and show the "Mark Order Ready" button
    // The seeded order state exercises the milestone threshold.
    const markReadyBtn = page.locator('button:has-text("Mark Order Ready")');
    await expect(markReadyBtn).toBeVisible({ timeout: 10000 });

    // 3. Click the button 3 times to trigger the milestone
    for (let i = 0; i < 3; i++) {
        await markReadyBtn.click();
        await page.waitForTimeout(100);
    }

    // 4. Assert the milestone UI appears
    const milestoneTitle = page.locator('text=🎉 3rd Order!');
    await expect(milestoneTitle).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text=You completed 3 orders!')).toBeVisible();

    // 5. Dismiss the milestone
    const dismissBtn = page.locator('button:has-text("Dismiss")');
    await expect(dismissBtn).toBeVisible();
    await dismissBtn.click();

    // 6. Assert the milestone UI disappears
    await expect(milestoneTitle).toBeHidden();
  });

  test('should verify 1st order milestone', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    const markReadyBtn = page.locator('button:has-text("Mark Order Ready")');
    await expect(markReadyBtn).toBeVisible();
    await markReadyBtn.click();

    await expect(page.locator('text=First Sale!')).toBeVisible();
  });

  test('should verify 10th order milestone', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    const markReadyBtn = page.locator('button:has-text("Mark Order Ready")');
    await expect(markReadyBtn).toBeVisible();

    // Exercise the real button path until the 10th-order milestone appears.
    for (let i = 0; i < 10; i++) {
        await markReadyBtn.click();
        await page.waitForTimeout(50);
    }

    // Might appear later
    await expect(page.locator('text=🎉 10th Order!')).toBeVisible({ timeout: 10000 });
  });

  test('should verify 100th order milestone', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    const markReadyBtn = page.locator('button:has-text("Mark Order Ready")');
    await expect(markReadyBtn).toBeVisible();

    // Exercise the real button path until the 100th-order milestone appears.
    for (let i = 0; i < 100; i++) {
        await markReadyBtn.click();
        await page.waitForTimeout(10);
    }

    await expect(page.locator('text=🎉 100th Order!')).toBeVisible({ timeout: 10000 });
  });

  test('should verify 100 visitors milestone', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    // Our test framework has a single-shot timer for 5s that triggers 100 visitors milestone
    await expect(page.locator('text=🚀 100 Visitors Today!')).toBeVisible({ timeout: 10000 });
  });

  test('should verify milestone dismissal', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    const markReadyBtn = page.locator('button:has-text("Mark Order Ready")');
    await expect(markReadyBtn).toBeVisible();
    await markReadyBtn.click();

    await expect(page.locator('text=First Sale!')).toBeVisible();

    const dismissBtn = page.locator('button:has-text("Dismiss")');
    await expect(dismissBtn).toBeVisible();
    await dismissBtn.click();

    await expect(page.locator('text=First Sale!')).toBeHidden();
  });
});
