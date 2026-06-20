import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the onboarding start page
    await page.goto('/onboarding');
  });

  test('successfully completes the wizard with drafting and instant image url', async ({ page }) => {
    // Step 1: Initial Intro
    await expect(page.locator('h1')).toContainText('Setup');
    await page.getByRole('button', { name: 'Start Setup' }).click();

    // Step 2: Intake Chat
    // Type in a business name/idea
    await page.getByPlaceholder(/Maya's Custom Cakes/i).fill('My E2E Bakery');

    // Type in an image url
    const imageUrlInput = page.getByPlaceholder(/Image URL \(Optional\)/i).first();
    await imageUrlInput.fill('https://example.com/bakery.png');

    // Save draft and verify the message
    await page.getByRole('button', { name: 'Save Draft' }).click();
    await expect(page.getByText('Draft Saved!')).toBeVisible();

    // Proceed to Step 3: What do you sell?
    await page.getByRole('button', { name: 'Next' }).click();

    // In this step, the values should persist
    const newImageUrlInput = page.getByPlaceholder(/Image URL \(Optional\)/i).first();
    await expect(newImageUrlInput).toHaveValue('https://example.com/bakery.png');

    await page.getByRole('button', { name: 'Save Draft' }).click();
    await expect(page.getByText('Draft Saved!')).toBeVisible();
  });
});
