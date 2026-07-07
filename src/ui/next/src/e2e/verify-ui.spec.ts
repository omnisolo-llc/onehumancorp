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
  await page.locator('button:has-text("Start My Business")').click();
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step1.png' });

  // Step 1: Business Name
  await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill('Maya Cakes');
  await page.locator('button:has-text("Next")').click();

  // Step 2: What do you sell
  await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes"]').fill('I bake custom vegan cakes in Portland, OR...');
  await page.locator('button:has-text("Next")').click();

  // Step 3: Location
  await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');
  await page.locator('button:has-text("Next")').click();

  // Step 4: Target Audience
  await page.locator('input[placeholder="e.g. Local families, Tech startups"]').fill('Local families');
  await page.locator('button:has-text("Next")').click();

  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step2.png' });

  // Review Details
  await page.locator('button:has-text("Continue")').click();

  // Style & Team
  await page.locator('input[placeholder="e.g. Maya Smith"]').fill('Maya');
  await page.locator('input[placeholder="you@example.com"]').fill('maya@example.com');
  await page.locator('input[placeholder="••••••••"]').fill('password123');
  await page.locator('button:has-text("Approve & Publish")').click();

  await expect(page.locator('text="You\'re Live!"')).toBeVisible();
});

test('Verify Instant Build UI', async ({ page }) => {
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

  await page.goto('/onboarding');
  await page.locator('button:has-text("Instant Build")').click();
  await page.locator('textarea[placeholder="e.g. I run a local bakery that sells custom vegan cakes..."]').fill('I run a test business');
  await page.locator('button:has-text("Generate Storefront")').click();
  await expect(page.locator('text="You\'re Live!"')).toBeVisible();
});
