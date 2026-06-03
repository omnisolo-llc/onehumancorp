import { test, expect } from '@playwright/test';

test.describe('OnboardingWizard CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
  });

  test('Maya the Baker can complete the onboarding flow', async ({ page }) => {
    // Mock the draft API call
    await page.route('/api/onboarding/draft', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({})
      });
    });

    // Mock the intake API call (triggered when moving from Step 1 to Step 2)
    await page.route('/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: 'Bakery',
          business_name: 'Maya Bakery',
          categories: ['food', 'baking'],
          initial_products: [{ name: 'Custom Vegan Cake', price: '45.00' }]
        })
      });
    });

    // Mock the start API call (triggered when moving from Step 3 to Step 4/5)
    await page.route('/api/onboarding/start', async route => {
      // Simulate network delay for the loading animation
      await new Promise(resolve => setTimeout(resolve, 500));
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          message: "Your business has been successfully launched."
        })
      });
    });

    // Navigate to the onboarding page
    await page.goto('/onboarding');

    // --- Step 1: Chat/Intake ---
    // Wait for the first prompt
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    // Fill in the business name (Chat Step 1)
    const nameInput = page.getByPlaceholder(/Maya's Custom Cakes/i);
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Maya Bakery');
    await page.getByRole('button', { name: 'Next' }).click();

    // Fill in the description (Chat Step 2)
    const sellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
    await expect(sellInput).toBeVisible();
    await sellInput.fill('I bake custom vegan cakes for weddings and parties.');
    await page.getByRole('button', { name: 'Next' }).click();

    // Fill in the location (Chat Step 3)
    const locInput = page.getByPlaceholder(/Portland, OR/i);
    await expect(locInput).toBeVisible();
    await locInput.fill('Seattle, WA');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // --- Step 2: Review Details ---
    await expect(page.getByText('Review Details')).toBeVisible();

    // Verify AI extracted details correctly
    await expect(page.locator('input[value="Maya Bakery"]')).toBeVisible();
    await expect(page.locator('input[value="Bakery"]')).toBeVisible();
    await expect(page.locator('input[value="Custom Vegan Cake"]')).toBeVisible();
    await expect(page.locator('input[value="45.00"]')).toBeVisible();

    // Maya decides to proceed
    await page.getByRole('button', { name: 'Continue' }).click();

    // --- Step 3: Style & Team ---
    await expect(page.getByText('Style & Team')).toBeVisible();

    // Select website template
    await page.getByText('Modern').click();

    // Check auto respond toggle
    const toggle = page.getByRole('checkbox');
    await expect(toggle).toBeChecked();

    // Account Setup
    await page.getByPlaceholder(/you@example.com/i).fill('maya@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('mypassword123');

    // Launch store
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // --- Step 4 & 5: Loading and Success ---
    // Wait for Step 4 loading text temporarily
    await expect(page.getByText('Building Your Business...')).toBeVisible();

    // Wait for the success screen (Step 5)
    await expect(page.getByText("You're Live!")).toBeVisible();
    await expect(page.getByText('Your business has been successfully launched.')).toBeVisible();

    // Verify links to dashboard and storefront are present
    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Preview Storefront' })).toBeVisible();
  });
});
