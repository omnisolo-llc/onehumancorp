import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test('completes full onboarding flow', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');
    await page.evaluate(() => window.localStorage.clear());
    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Business Name
    await expect(page.locator('text="Tell us about your business"')).toBeVisible();
    await expect(page.locator('text="What\'s the name of your business?"')).toBeVisible();
    await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill('Maya Cakes');
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    await expect(page.locator('text="What do you sell?"')).toBeVisible();
    await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('I bake custom vegan cakes in Portland, OR...');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    await expect(page.locator('text="Where are you located?"')).toBeVisible();
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');

    // Click Generate
    await page.route('**/api/onboarding/intake', async route => {
      await route.fulfill({ json: { business_name: 'Maya Cakes', business_type: 'Bakery', categories: ['food'], initial_products: [{ name: 'Cake', price: '20' }] } });
    });
    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({ json: { message: 'Your business has been successfully launched.' } });
    });

    await page.locator('button:has-text("Generate My Business")').click();

    // 2. Wait for Review Details Step
    await expect(page.locator('h2:has-text("Review Details")')).toBeVisible({ timeout: 5000 });

    // Continue to next step
    await page.locator('button:has-text("Continue")').click();

    // 3. Wait for Style & Team Step
    await expect(page.locator('h2:has-text("Style & Team")')).toBeVisible({ timeout: 5000 });

    // Select Template and Launch
    await page.locator('text="Classic"').click();
    await page.locator('button:has-text("Launch Store")').click();

    // 4. Loading screen
    await expect(page.locator('text="Building Your Business..."')).toBeVisible({ timeout: 5000 });

    // 5. Live Screen
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
