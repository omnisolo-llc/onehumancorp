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
    await expect(page.locator('text=Your business, live in minutes.')).toBeVisible({ timeout: 5000 });

    // Step 1: Welcome
    await page.locator('button:has-text("Start My Business")').filter({ visible: true }).first().click();

    // Step 2: Business Type
    await expect(page.locator('text=What kind of business are you building?')).toBeVisible();
    await page.locator('text=Online Store').click();
    // await page.locator('button:has-text("Next")').filter({ visible: true }).first().click(); // Now auto-navigates if we click the card but let's assume it doesn't wait

    // Step 3: Company Info
    await expect(page.locator('text=Give your business a name')).toBeVisible();
    await page.locator('input[placeholder="What is your business called?"]').fill('Comprehensive Bakery');
    await page.locator('button:has-text("Auto-suggest Description")').click();
    await page.waitForTimeout(1000); // Wait for auto-generation
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    // Step 4: Selling Categories
    await expect(page.locator('text=What do you sell?')).toBeVisible();
    await page.locator('text=Physical products').click();
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    // Step 5: Add your first product
    await expect(page.locator('text=Add your first product')).toBeVisible();
    await page.locator('input[placeholder="What is the name of this product?"]').fill('Chocolate Chip Cookie');
    await page.locator('input[placeholder="0.00"]').fill('12.99');
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    // Step 6: Payments
    await expect(page.locator('text=How do you want to receive payments?')).toBeVisible();
    await page.locator('text=Online only').click();

    // Step 7: Administrator account
    await expect(page.locator('text=Administrator account')).toBeVisible();
    await page.locator('input[placeholder="e.g. Maya Smith"]').fill('Jane Founder');
    await page.locator('input[placeholder="you@email.com"]').fill('jane@example.com');
    await page.locator('input[placeholder="Password"]').fill('securepassword!');
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    // Step 8: Choose Template
    await expect(page.locator('text=Choose a Template')).toBeVisible();
    await page.locator('text=Modern').click();

    // Step 9: Domain
    await expect(page.locator('text=Choose a Domain')).toBeVisible();
    await page.locator('button:has-text("Free OHC Domain")').filter({ visible: true }).first().click();
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    // Step 10: Review & Launch
    await expect(page.locator('text=Review & Launch')).toBeVisible();
    await page.locator('button:has-text("Launch!")').click();

    // Step 100: Onboarding Complete
    await expect(page.locator('text=Onboarding Complete!')).toBeVisible({ timeout: 10000 });
  });
});
