import { test, expect } from '@playwright/test';

test('Verify onboarding UI', async ({ page }) => {
  await page.goto('http://localhost:3000/onboarding');
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step1.png' });

  const descriptionInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes in Portland, OR..."]');
  await descriptionInput.fill('I am a freelance handyman in Miami');

  await page.route('**/api/onboarding/intake', route => route.fulfill({
    status: 200,
    json: {
        business_type: 'Handyman',
        business_name: 'Miami Fixes',
        categories: ['services'],
        initial_products: [{ name: 'Repair Service', price: '25.00' }]
    }
  }));
  await page.locator('button:has-text("Generate My Business")').click();
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step2.png' });

  await page.locator('button:has-text("Continue")').click();
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step3.png' });

  await page.route('**/api/onboarding/start', route => route.fulfill({
    status: 200,
    json: { message: "Success!" }
  }));

  await page.locator('text=Playful').click();
  await page.locator('text=Marketing Agent').click();

  await page.locator('button:has-text("Launch Store")').click();
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'onboarding-step4-loading.png' });

  await page.waitForTimeout(2000); // Wait for the loading screen to transition
  await page.screenshot({ path: 'onboarding-step5-live.png' });
});
