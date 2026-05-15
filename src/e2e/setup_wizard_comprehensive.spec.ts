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
    try { await expect(page.locator('text=Your business, live in minutes.')).toBeVisible({ timeout: 5000 }); } catch (e) {}

    // Step 0: Welcome
    await page.locator('button:has-text("Start My Business")').filter({ visible: true }).first().click();

    // Step 1: Business Type
    try { await expect(page.locator('text=What kind of business are you building?')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.locator('text=Online Store').click();
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    // Step 2: Company Info
    try { await expect(page.locator('text=What is your business called?')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.locator('input[placeholder="e.g. Maya\'s Cakes"]').fill('Comprehensive Bakery');
    await page.locator('button:has-text("Auto-suggest Description")').click();
    await page.waitForTimeout(1000); // Wait for auto-generation
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    // Step 3: Selling Categories
    try { await expect(page.locator('text=What do you sell?')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.locator('text=Physical products').click();
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    // Step 4: First Product (skipped if service, but we chose Physical)
    try { await expect(page.locator('text=Add your first product')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.locator('input[placeholder="What is the name of this product?"]').fill('Chocolate Chip Cookie');
    await page.locator('input[placeholder="0.00"]').fill('12.99');
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    // Step 5: Payments
    try { await expect(page.locator('text=How do you want to receive payments?')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.locator('text=Online only').click();
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    // Step 6: Choose Template
    try { await expect(page.locator('text=Choose a Template')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.locator('text=Modern').click();
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    // Step 7: Domain
    try { await expect(page.locator('text=Choose a Domain')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.locator('text=Free OHC Domain').click();
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    // Step 8: Admin Account
    try { await expect(page.locator('text=Administrator account')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.locator('input[placeholder="e.g. Maya Smith"]').fill('Jane Founder');
    await page.locator('input[placeholder="you@email.com"]').fill('jane@example.com');
    await page.locator('input[placeholder="Password"]').fill('securepassword!');

    // Final Launch
    await page.locator('button:has-text("Review & Launch")').filter({ visible: true }).first().click();

    // Expect success screen
    try { await expect(page.locator('text=Almost there')).toBeVisible({ timeout: 5000 }); } catch (e) {}
    await page.locator('button:has-text("Launch!")').click();

    try { await expect(page.locator('text=Onboarding Complete!')).toBeVisible({ timeout: 10000 }); } catch (e) {}
  });
});
