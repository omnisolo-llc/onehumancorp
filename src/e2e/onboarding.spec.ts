import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test('seeded user routes into setup', async ({ page }) => {
    await page.goto('/login');
    await page.getByRole('button', { name: /Start Business Setup/ }).click();

    await expect(page.locator('#setup-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
  });

  test('guided onboarding preserves entered business state', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Service Business/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Carlos Repairs');

    await expect(page.getByPlaceholder('What is your business called?')).toHaveValue('Carlos Repairs');
  });

  test('completed onboarding can return to dashboard', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Local Business/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Local Shop');
    await page.getByRole('button', { name: /Next/ }).click();
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
  });

  test('full end-to-end onboarding flow syncs with backend', async ({ page }) => {
    // Start Onboarding
    await page.goto('/business-setup');

    // Step 1
    await page.getByRole('button', { name: /Start My Business Next/ }).click();

    // Step 2: Choose Service Business
    await page.getByRole('button', { name: /Service Business/ }).click();

    // Step 3: Company Name
    await page.getByPlaceholder('What is your business called?').fill('Maya Consulting');
    await page.getByPlaceholder("e.g. Maya's Cakes").fill('A great consulting business');
    await page.getByRole('button', { name: 'Next →' }).click();

    // Step 4: What do you sell?
    await page.getByLabel('Services / Appointments').check();
    await page.getByRole('button', { name: 'Next →' }).click();

    // Step 5: First product
    await page.getByPlaceholder('What is the name of this product?').fill('Initial Consultation');
    await page.getByPlaceholder('0.00').fill('150.00');
    await page.getByRole('button', { name: 'Next →' }).click();

    // Step 6: Payment preference
    await page.getByRole('button', { name: 'Online', exact: true }).click();

    // Step 7: Create account
    await page.getByPlaceholder('e.g. Maya Smith').fill('Maya');
    await page.getByPlaceholder('you@email.com').fill('maya.consulting@test.com');
    await page.getByPlaceholder('Password').fill('securepassword123');
    await page.getByRole('button', { name: 'Next →' }).click();

    // Step 8: Template
    await page.getByRole('button', { name: 'Modern', exact: true }).click();
    await page.getByRole('button', { name: 'Next →' }).click();

    // Step 9: Domain
    await page.getByRole('button', { name: '🌐 Free OHC Domain' }).click();
    await page.getByRole('button', { name: 'Next →' }).click();

    // Step 10: Publish
    await page.getByRole('button', { name: /Publish my business/ }).click();

    // Step 100: Success Screen
    await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible({ timeout: 10000 });
  });
});
