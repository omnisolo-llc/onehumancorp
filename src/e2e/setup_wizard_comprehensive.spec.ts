import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    const id = `setup-comprehensive-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('onboarding-storage-v3');
    }, id);
    await page.goto('/onboarding');

    // Step 1: Business Name
    await expect(page.locator('text="What\'s the name of your business?"')).toBeVisible();
    await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill('Alex Art');
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    await expect(page.locator('text="What do you sell?"')).toBeVisible();
    await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('Original art and prints');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    await expect(page.locator('text="Where are you located?"')).toBeVisible();
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');
    await page.locator('button:has-text("Generate My Business")').click();

    // Wait for Review Details Step
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 5000 });
    await page.locator('button:has-text("Continue")').click();

    // Wait for Style & Team Step
    await expect(page.locator('text="Style & Team"')).toBeVisible({ timeout: 5000 });

    // Select Template and Launch
    await page.locator('text="Modern"').click();
    await page.locator('button:has-text("Launch Store")').click();

    // Loading screen
    await expect(page.locator('text="Building Your Business..."')).toBeVisible({ timeout: 5000 });

    // Live Screen
    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text="my-business.ohc.store"')).toBeVisible();
  });
});
