import { test, expect } from '@playwright/test';

test('Competitor Migration Wizard verify', async ({ page }) => {
  await page.goto('http://localhost:3000/onboarding');

  // Wait for the UI to load
  await expect(page.locator('text=Tell us about your business')).toBeVisible();

  // Verify the new question
  await expect(page.locator('text=Do you already have a website?')).toBeVisible();

  // Enter a competitor URL
  await page.fill('input[placeholder="e.g. https://mayas-cakes.myshopify.com (Optional)"]', 'https://my-old-store.wixsite.com/store');

  // Click Next
  await page.click('button:has-text("Next")');

  // It should show the loading screen (step 4)
  await expect(page.locator('text=Building Your Business...')).toBeVisible();
});
