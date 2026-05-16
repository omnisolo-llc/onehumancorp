import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  // dummy validation comment
  test('should traverse the complete business setup wizard successfully', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('founder@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await page.waitForURL('**/dashboard');

    // Trigger setup wizard
    await page.locator('button:has-text("Start Setup")').click();
    await expect(page.locator('text=Welcome to OHC!')).toBeVisible({ timeout: 5000 });

    // Step 1: Input details
    await page.locator('input[placeholder="e.g. Maya\'s Cakes"]').fill('Comprehensive Bakery');
    await page.selectOption('select#business-category', 'Restaurant / Food');
    await page.click('text=Next →');

    // Step 2: Generation state
    await expect(page.locator('text=Designing your storefront...')).toBeVisible();

    // Step 3: Launch
    await expect(page.locator('text=CONFETTI SUCCESS')).toBeVisible({ timeout: 10000 });
    await page.click('text="Publish my business →"');

    await expect(page.locator('text=Dashboard')).toBeVisible({ timeout: 10000 });
  });
});
