import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the onboarding start page
    await page.goto('/setup.html');
  });

  test('successfully completes the wizard with drafting and instant image url', async ({ page }) => {
    // Step 1: Initial Intro
    await expect(page.locator('h1').first()).toContainText('10-Minute Setup Wizard');
    await page.getByRole('button', { name: 'Instant Build' }).click();

    // Step Instant
    await page.locator('#instant-bio').fill('My E2E Bakery');

    // Type in an image url
    const imageUrlInput = page.getByPlaceholder(/Image URL \(Optional\)/i).first();
    await imageUrlInput.fill('https://example.com/bakery.png');

    // Proceed
    await page.locator('#generate-storefront-btn').click();

    // Loading step
    await expect(page.locator('#loading-title')).toContainText('Building Your Business...');
  });
});
