import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('should traverse the complete business setup wizard successfully', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('founder@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await page.waitForURL('**/dashboard');

    await page.locator('button:has-text("Start Setup")').click();
    await expect(page.locator('text=Your business, live in minutes.')).toBeVisible({ timeout: 5000 });

    await page.locator('button:has-text("Start My Business")').filter({ visible: true }).first().click();

    await expect(page.locator('text=What kind of business are you building?')).toBeVisible();
    await page.locator('text=🛒 Online Store').click();
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    await expect(page.locator('text=Give your business a name')).toBeVisible();
    await page.locator('#biz-name').fill('Comprehensive Bakery');
    await page.locator('button:has-text("Auto-suggest Description")').click();
    await page.waitForTimeout(1000);
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    await expect(page.locator('text=What do you sell?')).toBeVisible();
    await page.locator('text=📦 Physical products').click();
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    await expect(page.locator('text=How do you want to receive payments?')).toBeVisible();
    await page.locator('text=🌐 Online only').click();

    await expect(page.locator('text=Administrator account')).toBeVisible();
    await page.locator('input[placeholder="e.g. Maya Smith"]').fill('Jane Founder');
    await page.locator('input[placeholder="you@email.com"]').fill('jane@example.com');
    await page.locator('input[placeholder="Password"]').fill('securepassword!');
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    await expect(page.locator('text=Choose a Template')).toBeVisible();
    await page.locator('text=✨ Modern').click();
    await page.locator('text=Sunset').click();
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    await expect(page.locator('text=Add your first product')).toBeVisible();
    await page.locator('input[placeholder="e.g. Custom Birthday Cake"]').fill('Chocolate Chip Cookie');
    await page.locator('input[placeholder="e.g. 50.00"]').fill('12.99');
    await page.locator('button:has-text("Next")').filter({ visible: true }).first().click();

    await expect(page.locator('text=Choose a Domain')).toBeVisible();
    await page.locator('text=🌐 Free OHC Domain').click();

    await expect(page.locator('text=Almost there')).toBeVisible({ timeout: 5000 });
    await page.locator('button:has-text("Review & Launch")').filter({ visible: true }).first().click();

    await expect(page.locator('text=CONFETTI SUCCESS')).toBeVisible({ timeout: 10000 });
  });
});
