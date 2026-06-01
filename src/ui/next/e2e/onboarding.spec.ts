import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test('completes full onboarding flow', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Business Name
    await expect(page.locator('text="Tell us about your business"')).toBeVisible();
    await expect(page.locator('text="What\\'s the name of your business?"')).toBeVisible();
    await page.locator('input[placeholder="e.g. Maya\\'s Custom Cakes"]').fill('Maya Cakes');
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    await expect(page.locator('text="What do you sell?"')).toBeVisible();
    await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('I bake custom vegan cakes in Portland, OR...');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    await expect(page.locator('text="Where are you located?"')).toBeVisible();
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');

    // Click Generate
    await page.locator('button:has-text("Generate My Business")').click();

    // 4. Loading screen
    await expect(page.locator('text="Building Your Business..."')).toBeVisible({ timeout: 5000 });

    // 5. Live Screen
    await expect(page.locator('text="You\\'re Live!"')).toBeVisible({ timeout: 15000 });
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

  test('fails gracefully when intake API returns error', async ({ page }) => {
    // Mock intake API to return 500 error
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 500,
      body: JSON.stringify({ error: 'Internal Server Error' })
    }));

    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Business Name
    await page.locator('input[placeholder="e.g. Maya\\'s Custom Cakes"]').fill('Maya Cakes');
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('Cakes');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');

    // Click Generate
    await page.locator('button:has-text("Generate My Business")').click();

    // Error should be shown on the same step
    await expect(page.locator('text="Failed to process business details"')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text="Where are you located?"')).toBeVisible();
  });
});
