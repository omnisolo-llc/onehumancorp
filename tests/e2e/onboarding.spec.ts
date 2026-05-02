import { test, expect } from '@playwright/test';

test.describe('Business Setup Onboarding Flow', () => {
  test('should complete the business setup wizard and persist data', async ({ page }) => {
    // 1. Log in
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('new_business@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Sign In")').click();

    // 2. Start Wizard
    await page.waitForURL('**/*');
    await page.goto('/business-setup');
    await expect(page.locator('text=Welcome')).toBeVisible();
    await page.locator('button:has-text("Next")').click();

    // 3. Step 1: Business type
    await page.locator('input[type="text"]').first().fill('Online Store');
    await page.locator('button:has-text("Next")').click();

    // 4. Step 2: Company name
    await page.locator('input[type="text"]').first().fill('My Verified Store');
    await page.locator('button:has-text("Next")').click();

    // 5. Complete Wizard
    await expect(page.locator('text=What do you sell')).toBeVisible();
    await page.locator('input[type="text"]').first().fill('Digital Art');
    await page.locator('button:has-text("Finish")').click();

    // 6. Verify Full-Stack State (UI -> DB -> UI)
    // The real database should have stored the business name, and the UI should fetch and display it
    await page.waitForURL('**/dashboard');
    await expect(page.locator('text=My Verified Store')).toBeVisible();
  });
});
