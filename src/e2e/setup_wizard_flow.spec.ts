import { test, expect } from './fixtures';

test.describe('Wizard Refinement E2E - Setup Flow', () => {
  test('Maya the Baker can complete the onboarding wizard and launch her business', async ({ page }) => {
    // Navigate to the onboarding page
    await page.goto('/onboarding');

    // Verify we are on the first step
    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    // Fill out Chat Step 1: Business Name
    const nameInput = page.getByPlaceholder("e.g. Maya's Custom Cakes");
    await nameInput.fill('Maya Bakery');
    await page.getByRole('button', { name: 'Next' }).click();

    // Fill out Chat Step 2: What do you sell?
    const sellInput = page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...");
    await sellInput.fill('Cakes');
    await page.getByRole('button', { name: 'Next' }).click();

    // Fill out Chat Step 3: Location
    const locInput = page.getByPlaceholder("e.g. Portland, OR");
    await locInput.fill('NY');

    // Click Generate
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Wait for Step 2: Review Details
    await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();
    await expect(page.getByDisplayValue('Maya Bakery')).toBeVisible();
    await page.getByRole('button', { name: 'Continue' }).click();

    // Wait for Step 3: Style & Team
    await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();
    await expect(page.getByText('Website Template')).toBeVisible();

    // Select Custom Domain
    await page.getByText('Custom Domain').click();

    // Launch Store
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // Wait for Success Screen (Step 5)
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible();

    // Verify Dashboard and Preview links
    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Preview Storefront' })).toBeVisible();
  });
});
