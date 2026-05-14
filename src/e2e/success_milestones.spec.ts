import { test, expect } from '@playwright/test';

test.describe('Success Milestones Notifications', () => {
  test('should verify milestone functionality when order threshold is reached', async ({ page }) => {
    // 1. Authenticate and navigate to dashboard
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/*');

    // 2. Wait for the dashboard to load and show the "Mark Order Ready" button
    // The test mock usually sets new_orders_count = 3
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
});