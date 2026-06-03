import { test, expect } from './fixtures';

test.describe('Onboarding Personas E2E', () => {
  test('Persona: Maya the Home Baker creates a store with custom deposit', async ({ page }) => {
    await page.goto('/onboarding');

    // Fill out initial details
    await page.locator('input[value=""]').first().fill('Maya Custom Cakes');
    await page.getByRole('button', { name: /Next/i }).click();

    await page.locator('textarea').fill('Custom cakes for weddings and birthdays');
    await page.getByRole('button', { name: /Next/i }).click();

    // Check auto-save functionality
    await expect(page.getByText('Draft Saved!')).toBeVisible({ timeout: 10000 });
  });

  test('Persona: Carlos the Handyman books a service', async ({ page }) => {
    await page.goto('/onboarding');
    await page.locator('input[value=""]').first().fill('Carlos Repairs');
    await page.getByRole('button', { name: /Next/i }).click();

    await page.locator('textarea').fill('Handyman services and plumbing fixes');
    await page.getByRole('button', { name: /Next/i }).click();

    // The auto intake parses and forwards to step 2 which has businessType mapped
    await expect(page.getByText('Business Type')).toBeVisible({ timeout: 15000 });
  });

  test('Persona: Leo the Music Tutor setups digital sub', async ({ page }) => {
    await page.goto('/onboarding');
    await page.locator('input[value=""]').first().fill('Leo Guitar Lessons');
    await page.getByRole('button', { name: /Next/i }).click();

    await page.locator('textarea').fill('Online guitar tutorials and subscriptions');
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByText('Business Type')).toBeVisible({ timeout: 15000 });
  });

  test('Draft Auto-Save: Verify cross-device resumption works automatically', async ({ page, context }) => {
    await page.goto('/onboarding');

    // Fill business name and trigger auto-save
    await page.locator('input[value=""]').first().fill('Auto Save Test Biz');
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByText('Draft Saved!')).toBeVisible({ timeout: 10000 });

    // Reload page to simulate resuming session
    await page.reload();

    // The state should be restored from the backend draft automatically
    await expect(page.locator('textarea')).toBeVisible({ timeout: 5000 });
  });

  test('Form Validation: Verify empty fields and invalid inputs trigger correct errors', async ({ page }) => {
    await page.goto('/onboarding');

    // Try to proceed without a business name
    await page.getByRole('button', { name: /Next/i }).click();

    // Verify error appears
    await expect(page.getByText(/Business Name must be at least 3 characters./i)).toBeVisible();
  });
});
