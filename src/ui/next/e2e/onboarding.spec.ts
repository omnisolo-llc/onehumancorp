import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test('completes full onboarding flow', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.locator('text="Tell us about your business"')).toBeVisible();

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

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.locator('text="Building Your Business..."')).toBeVisible({ timeout: 5000 });

    // Step 5: Live Screen
    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text="Your business has been successfully launched."')).toBeVisible();
    await expect(page.locator('text="my-business.ohc.store"')).toBeVisible();

    const dashboardLink = page.locator('a:has-text("Go to Dashboard")');
    await expect(dashboardLink).toBeVisible();
    await expect(dashboardLink).toHaveAttribute('href', '/dashboard');

    await dashboardLink.click();
    await page.waitForURL('**/dashboard');

    await expect(page.locator('text="Morning Briefing"')).toBeVisible();
    await expect(page.locator('a:has-text("Add your first product")')).toBeVisible();
  });
});
