import { test, expect } from '@playwright/test';

test.describe('Onboarding Instant Build - Additional Details', () => {

  test('Instant Build button renders with sparkles icon', async ({ page }) => {
    await page.goto('/onboarding');
    const instantBuildButton = page.locator('button', { hasText: 'Instant Build' }).first();
    await expect(instantBuildButton).toBeVisible();

    // Verify the IconLabel content is inside
    const iconSVG = instantBuildButton.locator('svg');
    await expect(iconSVG).toBeVisible();
  });

  test('Clicking Instant Build navigates to the instant bio view', async ({ page }) => {
    await page.goto('/onboarding');
    const instantBuildButton = page.locator('button', { hasText: 'Instant Build' }).first();
    await expect(instantBuildButton).toBeVisible();
    await instantBuildButton.click();

    // Verify navigating to bio view
    await expect(page.locator('#instant-bio')).toBeVisible();
  });

  test('Submitting an empty bio disables the Generate Storefront button', async ({ page }) => {
    await page.goto('/onboarding');
    const instantBuildButton = page.locator('button', { hasText: 'Instant Build' }).first();
    await instantBuildButton.click();

    const generateButton = page.locator('#generate-storefront-btn');
    await expect(generateButton).toBeVisible();

    const bioInput = page.locator('#instant-bio');
    await bioInput.fill('   ');

    await expect(generateButton).toBeDisabled();
  });

  test('Filling a bio enables the Generate Storefront button', async ({ page }) => {
    await page.goto('/onboarding');
    const instantBuildButton = page.locator('button', { hasText: 'Instant Build' }).first();
    await instantBuildButton.click();

    const generateButton = page.locator('#generate-storefront-btn');
    const bioInput = page.locator('#instant-bio');

    await bioInput.fill('I am a baker in NYC');

    await expect(generateButton).not.toBeDisabled();
  });

  test('Filling a bio and image URL enables the Generate Storefront button', async ({ page }) => {
    await page.goto('/onboarding');
    const instantBuildButton = page.locator('button', { hasText: 'Instant Build' }).first();
    await instantBuildButton.click();

    const generateButton = page.locator('#generate-storefront-btn');
    const bioInput = page.locator('#instant-bio');
    const urlInput = page.locator('#instant-image-url');

    await bioInput.fill('I run a coffee shop');
    await urlInput.fill('https://example.com/shop.jpg');

    await expect(generateButton).not.toBeDisabled();
  });

});
