import { test, expect } from '@playwright/test';

test('Verify onboarding UI', async ({ page }) => {
  await page.goto('http://localhost:3000/onboarding');
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step1.png' });

  // Step 1: Business Name
  await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill('Maya Cakes');
  await page.locator('button:has-text("Next")').click();

  // Step 2: What do you sell
  await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('I bake custom vegan cakes in Portland, OR...');
  await page.locator('button:has-text("Next")').click();

  // Step 3: Location
  await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');

  await page.locator('button:has-text("Generate My Business")').click();
  await page.waitForTimeout(1000);

  // Fill in Step 2 requirements using explicit DOM locators
  await page.locator("select").selectOption({ label: "Food & Beverage" });
  await page.locator('input[value="Maya Cakes"]').waitFor(); // Ensure Business Name is rendered

  // Find the input fields based on their associated preceding label within their div container
  await page.locator('div:has(> label:has-text("Categories")) > input').fill("cakes, vegan, desserts");
  await page.locator('div:has(> label:has-text("First Product")) > input').fill("Vegan Chocolate Cake");
  await page.locator('div:has(> label:has-text("Price")) > input').fill("45.00");

  await page.locator('button:has-text("Continue")').click();
  await page.waitForTimeout(1000);

  await page.locator('button:has-text("Launch Store")').click();
  await page.waitForTimeout(2000);

  await expect(page.locator('h2:has-text("You\'re Live!")')).toBeVisible();
});
