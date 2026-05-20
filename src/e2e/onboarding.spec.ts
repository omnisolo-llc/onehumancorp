import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 }
  test('Priya (The Boutique Owner) onboarding flow via Next button', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' })).toBeVisible();
    await page.getByRole('button', { name: /Start My Business Next/i }).click();

    await page.getByRole('button', { name: 'Online Store' }).click();
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder('What is your business called?').fill("Priya's Boutique");
    await page.getByRole('button', { name: 'Next' }).click();

    await page.locator('textarea').fill("I sell stylish clothes and accessories.");
    await page.getByRole('button', { name: /Build My Storefront/i }).click();

    await expect(page.getByText('Preview Mode')).toBeVisible({ timeout: 15000 });
  });

  test('Leo (The Music Tutor) onboarding flow via Instant Build', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' })).toBeVisible();
    await page.getByRole('button', { name: /Instant Build/i }).click();

    await page.locator('textarea').fill("I am a music tutor. I give online lessons.");
    await page.getByRole('button', { name: /Generate Storefront/i }).click();

    await expect(page.getByText('Preview Mode')).toBeVisible({ timeout: 15000 });
  });

  test('Fatima (The Food Cart Operator) onboarding flow validation', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' })).toBeVisible();
    await page.getByRole('button', { name: /Start My Business Next/i }).click();

    // Check button states initially disabled
    const nextBtn = page.getByRole('button', { name: 'Next' });
    await expect(nextBtn).toBeDisabled();

    await page.getByRole('button', { name: 'Food & Beverage' }).click();
    await expect(nextBtn).toBeEnabled();
    await nextBtn.click();

    // name must be > 1
    const nextBtn2 = page.getByRole('button', { name: 'Next' });
    await expect(nextBtn2).toBeDisabled();
    await page.getByPlaceholder('What is your business called?').fill("a");
    await expect(nextBtn2).toBeDisabled();
    await page.getByPlaceholder('What is your business called?').fill("Fatima Food");
    await expect(nextBtn2).toBeEnabled();
    await nextBtn2.click();

    // detail must be > 5
    const genBtn = page.getByRole('button', { name: 'Build My Storefront' });
    await expect(genBtn).toBeDisabled();
    await page.locator('textarea').fill("Halal");
    await expect(genBtn).toBeDisabled();
    await page.locator('textarea').fill("Halal food cart serving the best chicken over rice.");
    await expect(genBtn).toBeEnabled();
  });

});

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // 1. Acquisition & Onboarding start
    await page.goto('/website-builder');

    // Wait for the Smart Builder welcome screen
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' })).toBeVisible();

    // Fill in the bio
    await page.locator('#bio-input').fill("I bake custom vegan cakes in Seattle. Maya's Cakes.");

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
