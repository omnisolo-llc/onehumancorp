import { test, expect } from '@playwright/test';

test('Verify onboarding UI', async ({ page }) => {
  // Mock external API responses to eliminate backend dependencies
  await page.route('**/api/onboarding/intake', route => {
    route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({
        business_type: 'Bakery',
        business_name: 'Maya Bakery',
        categories: ['food'],
        initial_products: [{ name: 'Cake', price: '20' }]
      })
    });
  });

  await page.route('**/api/onboarding/start', route => {
    route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ message: "Success!" })
    });
  });

  await page.route('**/api/onboarding/state', route => {
    route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ wizardState: {} })
    });
  });

  await page.route('**/api/onboarding/draft', route => {
    route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ wizardState: {} })
    });
  });

  await page.goto('/onboarding');
  await page.getByText('Start Onboarding').click();
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
  await page.screenshot({ path: 'onboarding-step2.png' });

  await page.locator('button:has-text("Continue")').click();
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step3.png' });
});
