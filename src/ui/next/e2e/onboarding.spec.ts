import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test('completes full onboarding flow', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.locator('text="Tell us about your business"')).toBeVisible();

    // First step of chat onboarding asks for business name
    const nameInput = page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]');
    await nameInput.fill('My Handyman Business');
    await page.locator('button:has-text("Next")').click();
    await page.waitForTimeout(500);

    // Second step asks for description
    const descriptionInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]');
    await descriptionInput.fill('I am a freelance handyman in Miami');
    await page.locator('button:has-text("Next")').click();
    await page.waitForTimeout(500);

    // Third step asks for location
    const locationInput = page.locator('input[placeholder="e.g. Portland, OR"]');
    await locationInput.fill('Miami, FL');

    // Intercept API calls
    await page.route('**/api/onboarding/intake', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: 'Handyman Services',
          business_name: 'Miami Handyman Pro',
          categories: ['services', 'home_improvement'],
          initial_products: [
            { name: 'Basic Repair Visit', price: '75.00' }
          ]
        })
      });
    });

    await page.route('**/api/onboarding/start', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          message: 'Your business has been successfully launched.',
          tenant_id: 'tenant_123',
          agents_hired: 3,
          tasks_queued: 12
        })
      });
    });

    // Submit Step 1
    await page.locator('button:has-text("Generate My Business")').click();

    // Verify Step 2 (Review Details)
    await expect(page.locator('text="Review Details"')).toBeVisible();
    await expect(page.locator('input[value="Miami Handyman Pro"]')).toBeVisible();
    await expect(page.locator('input[value="Handyman Services"]')).toBeVisible();
    await expect(page.locator('input[value="Basic Repair Visit"]')).toBeVisible();
    await expect(page.locator('input[value="75.00"]')).toBeVisible();

    // Submit Step 2
    await page.locator('button:has-text("Continue")').click();

    // Verify Step 3 (Style & Team)
    await expect(page.locator('text="Style & Team"')).toBeVisible();

    // Select 'Bold' template
    await page.locator('text="Bold"').click();

    // Submit Step 3
    await page.locator('button:has-text("Launch Store")').click();

    // Verify Step 4 (Loading/Building state)
    await expect(page.locator('text="Building Your Business..."')).toBeVisible();

    // Verify Step 5 (Success state)
    await expect(page.locator('text="You\'re Live!"')).toBeVisible();
    await expect(page.locator('text="Your business has been successfully launched."')).toBeVisible();
  });
});
