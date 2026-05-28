import { test, expect } from '@playwright/test';

test('Verify onboarding UI', async ({ page }) => {
  await page.goto('http://localhost:3000/onboarding');
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step1.png' });

  const descriptionInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes in Portland, OR..."]');
  await descriptionInput.fill('I am a freelance handyman in Miami');

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
