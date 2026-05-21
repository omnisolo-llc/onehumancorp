import { test, expect } from '@playwright/test';

test('onboarding flow completes successfully', async ({ page }) => {
  // Mock state logic globally
  let currentStep = 1;

  await page.route('**/api/onboarding/state', async (route) => {
    if (route.request().method() === 'GET') {
      await route.fulfill({ status: 200, json: { step: currentStep } });
    } else {
      const data = JSON.parse(route.request().postData() || '{}');
      if (data.step) currentStep = data.step;
      await route.fulfill({ status: 200, json: {} });
    }
  });

  // Mock intake
  await page.route('**/api/onboarding/intake', async (route) => {
    await route.fulfill({
      status: 200,
      json: {
        business_name: 'Mocked Business',
        business_type: 'Retail',
        initial_products: [{ name: 'Mocked Product', price: '10.00' }]
      }
    });
  });

  // Mock start
  await page.route('**/api/onboarding/start', async (route) => {
    await route.fulfill({
      status: 200,
      json: { message: 'Your business has been successfully launched.' }
    });
  });

  await page.goto('/onboarding');

  // Step 1
  await page.waitForSelector("text=What's the name of your business?");
  await page.locator('input[placeholder="e.g. Maya\'s Cakes"]').fill('My Awesome Bakery');
  await page.locator('button', { hasText: 'Next' }).first().click();

  // Step 2
  await page.waitForSelector("text=What do you sell?", { timeout: 10000 });
  await page.locator('textarea[placeholder="e.g. I bake custom wedding cakes and cupcakes."]').fill('I sell delicious cookies and cakes');
  await page.locator('button', { hasText: 'Next' }).click();

  // Step 3
  await page.waitForSelector("text=Describe your preferred style.", { timeout: 10000 });
  await page.locator('input[placeholder="e.g. Clean and modern with pastel colors"]').fill('Modern minimalist with pink colors');
  await page.locator('button', { hasText: 'Generate Draft' }).click();

  // Step 4
  await page.waitForSelector("text=Looks Great!", { timeout: 10000 });
  await page.locator('button', { hasText: 'Publish Now' }).click();

  // Step 5
  await page.waitForSelector("text=You're Live!", { timeout: 10000 });
});
