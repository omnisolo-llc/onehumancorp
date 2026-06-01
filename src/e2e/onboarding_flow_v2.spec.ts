import { test, expect } from './fixtures';

test.describe('Onboarding Flow V2', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the onboarding flow
    await page.goto('/onboarding');
  });

  test('successfully navigates through the wizard and launches store without api mocking', async ({ page }) => {
    // Chat Step 1: Business Name
    const businessNameInput = page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]');
    await expect(businessNameInput).toBeVisible();
    await businessNameInput.fill('Maya Bakery');
    await page.getByRole('button', { name: 'Next' }).click();

    // Chat Step 2: What You Sell
    const whatYouSellTextarea = page.locator('textarea[placeholder*="I bake custom vegan cakes"]');
    await expect(whatYouSellTextarea).toBeVisible();
    await whatYouSellTextarea.fill('Cakes');
    await page.getByRole('button', { name: 'Next' }).click();

    // Chat Step 3: Location
    const locationInput = page.locator('input[placeholder="e.g. Portland, OR"]');
    await expect(locationInput).toBeVisible();
    await locationInput.fill('NY');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Wait for Review Details screen
    await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();

    // Continue from Review Details
    await page.getByRole('button', { name: 'Continue' }).click();

    // Wait for Style & Team screen
    await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();

    // Click Launch Store
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // Assert final state "You're Live!"
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
  });
});
