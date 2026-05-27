import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test('completes full onboarding flow', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.locator('text="What is the name of your business?"')).toBeVisible();

    // Fill in the Business Name
    const nameInput = page.locator('input[placeholder="e.g. Maya\'s Cakes"]');
    await nameInput.fill('Maya Bakery');

    const nextBtn1 = page.locator('button:has-text("Next")');
    await expect(nextBtn1).toBeEnabled();
    await nextBtn1.click();

    // Step 2: Business Category
    await expect(page.locator('text="What kind of business is it?"')).toBeVisible();
    const categoryInput = page.locator('input[placeholder="e.g. Food/Bakery"]');
    await categoryInput.fill('Bakery');

    const nextBtn2 = page.locator('button:has-text("Next")');
    await expect(nextBtn2).toBeEnabled();
    await nextBtn2.click();

    // Step 3: Business Goal
    await expect(page.locator('text="What is your main goal?"')).toBeVisible();
    const goalInput = page.locator('input[placeholder="e.g. Sell my custom cakes online"]');
    await goalInput.fill('Sell cakes');

    // Intercept API calls
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      json: {
        business_type: 'Bakery',
        business_name: 'Maya Bakery',
        categories: ['food'],
        initial_products: [{ name: 'Custom Cake', price: '25.00' }]
      }
    }));

    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 200,
      json: { message: "Your business has been successfully launched." }
    }));

    // Click Generate
    await page.locator('button:has-text("Generate My Business")').click();

    // Step 4. wait for it to generate
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

    await expect(page.locator('text="Daily Brief"')).toBeVisible();
    await expect(page.locator('a:has-text("Upgrade to Starter")')).toBeVisible();
  });
});