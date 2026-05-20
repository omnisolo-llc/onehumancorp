import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // 1. Acquisition & Onboarding start
    await page.goto('/website-builder');

    // Wait for the Smart Builder welcome screen
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' })).toBeVisible();

    // Step 1: Basic info
    await page.locator('#business-name-input').fill("Maya's Cakes");
    await page.locator('#business-type-input').fill("Bakery");
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 2: Description
    await page.locator('#bio-input').fill("I bake custom vegan cakes in Seattle.");

    // Click generate
    await page.getByRole('button', { name: /Build My Storefront/i }).click();

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByText('Preview Mode')).toBeVisible({ timeout: 15000 });

    // We expect some blocks to have been generated
    await expect(page.getByRole('button', { name: /1-Tap Launch/i })).toBeVisible();

    // Publish
    await page.getByRole('button', { name: /1-Tap Launch/i }).click();

    // 3. Activation
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });
});
