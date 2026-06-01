import { test, expect } from '@playwright/test';

test('Verify onboarding UI', async ({ page }) => {
  await page.goto('http://localhost:3000/onboarding');
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step1.png' });

  // Step 1: Tell us about your business
  await page.locator('textarea').fill('I bake custom vegan cakes in Portland, OR...');

  // Mock intake API
  await page.route('**/api/onboarding/intake', route => route.fulfill({
    status: 200,
    body: JSON.stringify({
      business_type: "Online Store",
      business_name: "Maya Cakes",
      initial_products: [{ name: "Vegan Cake", price: "50.00" }],
      categories: ["physical"]
    })
  }));

  await page.getByRole('button', { name: 'Generate My Business' }).click();

  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step2.png' });
});
