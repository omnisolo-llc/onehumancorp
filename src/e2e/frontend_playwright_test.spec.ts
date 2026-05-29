import { test, expect } from '@playwright/test';

test('Onboarding Wizard', async ({ page }) => {
  await page.goto('http://localhost:3000/onboarding');

  // Step 1: Chat Input 1
  await page.fill('input[placeholder="e.g. Maya\'s Custom Cakes"]', 'Maya Bakery');
  await page.click('button:has-text("Next")');

  // Chat Input 2
  await expect(page.locator('text=What do you sell?')).toBeVisible();
  await page.fill('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]', 'Cakes and stuff');
  await page.click('button:has-text("Next")');

  // Chat Input 3
  await expect(page.locator('text=Where are you located?')).toBeVisible();
  await page.fill('input[placeholder="e.g. Portland, OR"]', 'Seattle, WA');
  await page.click('button:has-text("Generate My Business")');

  // Step 2: Review Details
  await expect(page.locator('text=Review Details')).toBeVisible();
  await page.click('button:has-text("Continue")');

  // Step 3: Style & Team
  await expect(page.locator('text=Style & Team')).toBeVisible();
  await page.click('button:has-text("Launch Store")');

  // Step 5: You're live
  await expect(page.locator("text=You're Live!")).toBeVisible();
});
