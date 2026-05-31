import { test, expect } from '@playwright/test';

test('Verify onboarding UI and keyboard navigation', async ({ page }) => {
  await page.goto('http://localhost:3000/onboarding');
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step1.png' });

  // Step 1: Business Name
  const businessNameInput = page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]');
  await businessNameInput.fill('Maya Cakes');
  // Submit via Enter key
  await businessNameInput.press('Enter');

  // Step 2: What do you sell
  const whatYouSellInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]');
  await whatYouSellInput.fill('I bake custom vegan cakes in Portland, OR...');
  // Submit via Enter key
  await whatYouSellInput.press('Enter');

  // Step 3: Location
  const locationInput = page.locator('input[placeholder="e.g. Portland, OR"]');
  await locationInput.fill('Portland, OR');
  // Submit via Enter key
  await locationInput.press('Enter');

  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step2.png' });

  // Step 4: Review Details
  const continueButton = page.locator('button:has-text("Continue")');
  await expect(continueButton).toBeVisible();

  // Test submit via form submit logic on Review page
  // The first input is Business Name, so we'll focus it and press Enter
  const nameReviewInput = page.locator('label:has-text("Business Name") + input');
  await nameReviewInput.focus();
  await nameReviewInput.press('Enter');

  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step3.png' });
});
