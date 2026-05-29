import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // Skip real login, NextJS prototype doesn't have it
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Maya's Custom Cakes");
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByPlaceholder('e.g. I bake custom vegan cakes for weddings and parties...').fill('Custom vegan cakes');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    await expect(page.getByRole('heading', { name: 'Where are you located?' })).toBeVisible();
    await page.getByPlaceholder('e.g. Portland, OR').fill('Portland, OR');
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    await expect(page.getByRole('heading', { name: "Review Details" })).toBeVisible();
    await page.getByRole('button', { name: /Continue/i }).click();

    await expect(page.getByRole('heading', { name: "Style & Team" })).toBeVisible();
    await page.getByRole('button', { name: /Launch Store/i }).click();

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('my-business.ohc.store')).toBeVisible();

    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('Carlos (Handyman) onboarding flow', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');

    // Make sure we wait for the first chat step text
    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    // Since the previous test used chatStep=1 immediately, we need to match what's visible
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Carlos Plumbing");
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByPlaceholder('e.g. I bake custom vegan cakes for weddings and parties...').fill('Handyman services');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    await expect(page.getByRole('heading', { name: 'Where are you located?' })).toBeVisible();
    await page.getByPlaceholder('e.g. Portland, OR').fill('Miami, FL');

    await page.getByRole('button', { name: /Generate My Business/i }).click();

    await expect(page.getByRole('heading', { name: "Review Details" })).toBeVisible();
    await page.getByRole('button', { name: /Continue/i }).click();

    await expect(page.getByRole('heading', { name: "Style & Team" })).toBeVisible();
    await page.getByRole('button', { name: /Launch Store/i }).click();

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });
});
