import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard - Persona: consultant', () => {
  test('Complete flow for Biz Consult', async ({ page }) => {
    await page.goto('/login');
    await page.click('button:has-text("Don\'t have an account? Sign Up")');
    await page.fill('input[placeholder="Email or Username"]', 'consultant@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign Up")');

    await expect(page.locator('text=Your business, live in minutes.')).toBeVisible();
    await page.click('text=🚀 Start My Business');

    await expect(page.locator('text=What kind of business are you building?')).toBeVisible();
    await page.click('text=Service Business');
    await page.click('text=Next →');

    await expect(page.locator('text=What is your business called?')).toBeVisible();
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Biz Consult');
    await page.click('text=Next →');

    await expect(page.locator('text=What do you sell?')).toBeVisible();
    await page.click('text=Services / appointments');
    await page.click('text=Next →');

    await expect(page.locator('text=How do you want to receive payments?')).toBeVisible();
    await page.click('text=Online only');
    await page.click('text=Next →');

    await expect(page.locator('text=Administrator account')).toBeVisible();
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Founder consultant');
    await page.fill('input[placeholder="you@email.com"]', 'founder_consultant@example.com');
    await page.fill('input[placeholder="Password"]', 'securepassword!');
    await page.click('text=Next →');

    await expect(page.locator('text=Choose a Template')).toBeVisible();
    await page.click('text=✨ Modern');
    await page.click('text=Next →');

    await expect(page.locator('text=Add your first product')).toBeVisible();
    await page.fill('input[placeholder="What is the name of this product?"]', 'Starter Package');
    await page.fill('input[placeholder="e.g. 50.00"]', '99.99');
    await page.click('text=Next →');

    await expect(page.locator('text=Choose a Domain')).toBeVisible();
    await page.click('text=🌐 Free OHC Domain');
    await page.click('text=Next →');

    await expect(page.locator('text=Almost there')).toBeVisible();
    await page.click('text=Publish my business →');

    await expect(page.locator('text=CONFETTI SUCCESS')).toBeVisible({ timeout: 10000 });
  });
});
