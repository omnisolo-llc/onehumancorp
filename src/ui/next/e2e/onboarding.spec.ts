import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test('completes full onboarding flow', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.locator('text="Let\'s build your business."')).toBeVisible();

    // Click Start
    await page.locator('button:has-text("Start Now")').click();

    // Wait for input step (Step 2)
    await expect(page.locator('text="Tell me what you sell in one sentence."')).toBeVisible({ timeout: 5000 });

    // Fill in the description
    const descriptionInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes in Portland, OR..."]');
    await descriptionInput.fill('I am a freelance handyman in Miami');

    // Intercept API calls
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      json: { initial_products: [{ name: 'Custom Cake', price: '25.00' }] }
    }));

    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 200,
      json: { message: "Your business has been successfully launched." }
    }));

    // Click Generate
    await page.locator('button:has-text("Generate My Business")').click();

    // Loading screen
    await expect(page.locator('text="Building Your Business..."')).toBeVisible({ timeout: 5000 });

    // Live Screen / Dashboard
    await page.waitForURL('**/dashboard', { timeout: 10000 });

    await expect(page.locator('text="Share your store"')).toBeVisible();
    await expect(page.locator('button:has-text("Share Link")')).toBeVisible();
  });
});
