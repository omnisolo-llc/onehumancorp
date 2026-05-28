import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test('completes full onboarding flow', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.locator('text="Tell us about your business"')).toBeVisible();

    // Fill in the description (Chat Step 1)
    const nameInput = page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]');
    await nameInput.fill('Miami Handyman');
    await page.locator('button:has-text("Next")').click();

    // Chat Step 2
    const descriptionInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]');
    await descriptionInput.fill('I am a freelance handyman in Miami');
    await page.locator('button:has-text("Next")').click();

    // Chat Step 3
    const locationInput = page.locator('input[placeholder="e.g. Portland, OR"]');
    await locationInput.fill('Miami, FL');

    // Intercept API calls
    await page.route('**/api/onboarding/state', route => {
      // Mock both GET and POST to the state endpoint
      route.fulfill({
        status: 200,
        json: {} // Return empty state to start fresh
      });
    });

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

    // 2. Wait for Review Details Step
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 5000 });

    // Continue to next step
    await page.locator('button:has-text("Continue")').click();

    // 3. Wait for Style & Team Step
    await expect(page.locator('text="Style & Team"')).toBeVisible({ timeout: 5000 });

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
