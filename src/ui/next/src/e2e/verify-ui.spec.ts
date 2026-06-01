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

  // Wait for the modal / overlay text to change if there is any "Building" step.
  await page.waitForTimeout(5000);
  await page.screenshot({ path: 'onboarding-step2.png' });

  // Look for the correct continue button text here.
  const continueButton = page.locator('button', { hasText: 'Continue' }).first();
  await continueButton.waitFor({ state: 'visible', timeout: 5000 }).catch(() => null);

  if (await continueButton.isVisible()) {
      await continueButton.click();
      await page.waitForTimeout(1000);
  }

  const launchStoreButton = page.locator('button:has-text("Launch Store")');
  await launchStoreButton.waitFor({ state: 'visible', timeout: 5000 }).catch(() => null);

  if (await launchStoreButton.isVisible()) {
      await launchStoreButton.click();
      await page.waitForTimeout(5000);
  }

  const goDashboardButton = page.locator('a:has-text("Go to Dashboard")');
  await goDashboardButton.waitFor({ state: 'visible', timeout: 5000 }).catch(() => null);

  if (await goDashboardButton.isVisible()) {
      await goDashboardButton.click();
  }

  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step3.png' });
});
