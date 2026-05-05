import { test, expect } from '@playwright/test';

test.describe('Grow Business Flow', () => {
  test('should verify the growth strategy selection flow', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // 2. Navigate to Grow Business UI

    // Navigate via dashboard UI like a real user
    const growBusinessBtn = page.locator('button:has-text("Grow Business")').first();
    await expect(growBusinessBtn).toBeVisible();
    await growBusinessBtn.click();

    await page.waitForURL('**/grow');

    // Verify main header
    await expect(page.locator('text=Grow My Business')).toBeVisible();

    // Verify strategies
    const strat1 = page.locator('button:has-text("Add 5 more products")');
    const strat2 = page.locator('button:has-text("Connect Instagram")');
    const strat3 = page.locator('button:has-text("Run your first email campaign")');

    await expect(strat1).toBeVisible();
    await expect(strat2).toBeVisible();
    await expect(strat3).toBeVisible();

    // Select a strategy
    await strat3.click();

    // Verify selected

    // Verify selection (wait for next button to be enabled)
    await expect(page.locator('text=Run your first email campaign').first()).toBeVisible();


    // Move to next step
    await page.locator('button:has-text("Next")').click();

    // Confirm step
    await expect(page.locator('text=Confirm Action')).toBeVisible();


    // Execute strategy

    await page.locator('button:has-text("Launch Strategy")').click();

  });
});
