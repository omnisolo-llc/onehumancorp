import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test('completes full onboarding flow', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.locator('text="Tell us about your business"')).toBeVisible();

    // Chat step 1: Name
    const nameInput = page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]');
    await nameInput.fill('Miami Handyman');
    await page.locator('button:has-text("Next")').click();

    // Chat step 2: What you sell
    const sellInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]');
    await sellInput.fill('I am a freelance handyman in Miami');
    await page.locator('button:has-text("Next")').click();

    // Chat step 3: Location
    const locInput = page.locator('input[placeholder="e.g. Portland, OR"]');
    await locInput.fill('Miami, FL');

    // Intercept API calls
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      json: {
        business_type: 'Handyman',
        business_name: 'Miami Handyman',
        categories: ['services'],
        initial_products: [{ name: 'Hourly Rate', price: '50.00' }]
      }
    }));

    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 200,
      json: { message: "Your business has been successfully launched." }
    }));

    // Click Generate
    await page.locator('button:has-text("Generate My Business")').click();

    // 2. Wait for Review Details Step and select Template
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 5000 });
    await page.locator('text="Classic"').click();

    // Launch Store
    await page.locator('button:has-text("Launch Store")').click();

    // 3. Loading screen
    await expect(page.locator('text="Building Your Business..."')).toBeVisible({ timeout: 5000 });

    // 4. Live Screen
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
