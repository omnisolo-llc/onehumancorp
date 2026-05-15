
import { test, expect } from '@playwright/test';

test('Onboarding flow', async ({ page }) => {
  await page.goto('/');
  // Step 1
  await page.fill('input[type="email"]', 'test@test.com');
  await page.fill('input[type="password"]', 'password');
  await page.click('button:has-text("Sign Up")');

  // Step 2
  await page.fill('input[placeholder="Business Type"]', 'Bakery');
  await page.fill('input[placeholder="Company Name"]', 'Test Bakery');
  await page.click('button:has-text("Next")');

  // Step 3
  await page.selectOption('select', 'modern');
  await page.click('button:has-text("Select")');

  // Step 4
  await page.fill('input[placeholder="Product Name"]', 'Cake');
  await page.fill('input[placeholder="Price"]', '20');
  await page.click('button:has-text("Next")');

  // Step 5
  await page.fill('input[placeholder="mybusiness.ohc.app"]', 'testbakery');
  await page.click('button:has-text("Publish")');

  // Step 6
  await expect(page.locator('text=Welcome Checklist')).toBeVisible();
});
