import { test, expect } from '@playwright/test';

test.describe('Instant Setup CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Clean up local storage to ensure fresh start
    await page.addInitScript(() => window.localStorage.clear());
    // Set a known viewport for mobile tests (375px first as per requirements)
    await page.setViewportSize({ width: 375, height: 812 });
  });

  test('Persona: Maya (Home Baker) completes the Zero-Click Instant Onboarding', async ({ page }) => {


    await page.goto('/setup.html');


    // Verify Initial Screen
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();

    // 1. Click "Instant Build"
    await page.getByRole('button', { name: 'Instant Build' }).click();

    // 2. Verify we are in the instant step
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();

    // 3. Fill in the description
    const instantInput = page.locator('#instant-bio');
    await expect(instantInput).toBeVisible();
    await instantInput.fill('I make custom vegan cakes in Austin. I need a website and a way to take bookings.');

    const generateBtn = page.getByTestId('generate-storefront-btn');
    await expect(generateBtn).toBeEnabled();

    // 4. Click generate
    await generateBtn.click();

    // 5. Verify loading texts (animation progress)
    await expect(page).toHaveURL(/.*success.html/, { timeout: 15000 });
  });
});
