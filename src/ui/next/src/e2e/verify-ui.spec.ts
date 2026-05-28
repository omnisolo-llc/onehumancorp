import { test, expect } from '@playwright/test';

test('Verify onboarding UI', async ({ page }) => {
  await page.goto('http://localhost:3000/onboarding');
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step1.png' });

  // First step of chat onboarding asks for business name
  const nameInput = page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]');
  await nameInput.fill('My Handyman Business');
  await page.locator('button:has-text("Next")').click();
  await page.waitForTimeout(500);

  // Second step asks for description
  const descriptionInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]');
  await descriptionInput.fill('I am a freelance handyman in Miami');
  await page.locator('button:has-text("Next")').click();
  await page.waitForTimeout(500);

  // Third step asks for location
  const locationInput = page.locator('input[placeholder="e.g. Portland, OR"]');
  await locationInput.fill('Miami, FL');

  await page.route('**/api/onboarding/intake', route => route.fulfill({
    status: 200,
    json: { initial_products: [{ name: 'Custom Cake', price: '25.00' }] }
  }));

  await page.locator('button:has-text("Generate My Business")').click();

  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step2.png' });

  await page.locator('button:has-text("Continue")').click();
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step3.png' });
});
