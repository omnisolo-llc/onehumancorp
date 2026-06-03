import { test, expect } from '@playwright/test';

test.describe('OnboardingWizard Instant Build CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
  });

  test('Maya the Baker can use the Instant AI Build mode to launch her store quickly', async ({ page }) => {
    // NOTE: This test mocks backend API calls for the "Instant Build" functionality
    // because hitting the real backend in the CI pipeline is unreliable due to missing
    // or unconfigured external LLM API keys.

    // Mock the draft API call
    await page.route('/api/onboarding/draft', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({})
      });
    });

    // Mock the intake API call (triggered when Instant Build is submitted)
    await page.route('/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: 'Bakery',
          business_name: 'Maya Instant Bakery',
          categories: ['food', 'baking', 'instant'],
          initial_products: [{ name: 'Instant Vegan Cake', price: '45.00' }]
        })
      });
    });

    // Mock the start API call
    await page.route('/api/onboarding/start', async route => {
      // Simulate network delay for the loading animation
      await new Promise(resolve => setTimeout(resolve, 500));
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          message: "Your instant business has been successfully launched."
        })
      });
    });

    // Navigate to the onboarding page
    await page.goto('/onboarding');

    // Wait for the first prompt
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    // Click the Instant AI Build button
    await page.getByRole('button', { name: 'Instant AI Build' }).click();

    // Verify the instant build textarea is visible
    await expect(page.getByText('Describe your business in a sentence')).toBeVisible();

    // Enter a short description (less than 10 chars will trigger validation, so we make it long)
    const descInput = page.getByPlaceholder(/I run a local bakery that specializes in custom vegan cakes/i);
    await expect(descInput).toBeVisible();
    await descInput.fill('I am Maya and I bake amazing custom vegan cakes for instant delivery.');

    // Click Generate Storefront
    await page.getByRole('button', { name: 'Generate Storefront' }).click();

    // The user should skip "Review Details" and go straight to Step 3: Style & Team
    await expect(page.getByText('Style & Team')).toBeVisible();

    // Launch store directly (using defaults prefilled by the instant flow)
    await page.getByPlaceholder(/you@example.com/i).fill('maya_instant@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('instantpassword123');

    await page.getByRole('button', { name: 'Launch Store' }).click();

    // Wait for Step 4 loading text temporarily
    await expect(page.getByText('Building Your Business...')).toBeVisible();

    // Wait for the success screen (Step 5)
    await expect(page.getByText("You're Live!")).toBeVisible();
    await expect(page.getByText('Your instant business has been successfully launched.')).toBeVisible();

    // Verify links to dashboard and storefront are present
    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Preview Storefront' })).toBeVisible();
  });
});