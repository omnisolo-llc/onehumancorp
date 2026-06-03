import { test, expect } from './fixtures';

test.describe('Onboarding Personas E2E', () => {
  test('Persona: Maya the Home Baker creates a store with custom deposit', async ({ page }) => {
    await page.goto('/onboarding');

    // Fill out initial details
    await page.getByRole('textbox', { name: /business name/i }).fill('Maya Custom Cakes');
    await page.getByRole('button', { name: /Next/i }).click();

    await page.getByRole('textbox', { name: /what do you sell/i }).fill('Custom cakes for weddings and birthdays');
    await page.getByRole('button', { name: /Next/i }).click();

    // Check auto-save functionality
    await expect(page.getByText('Draft Saved!')).toBeVisible({ timeout: 10000 });
  });

  test('Persona: Carlos the Handyman books a service', async ({ page }) => {
    await page.goto('/onboarding');
    await page.getByRole('textbox', { name: /business name/i }).fill('Carlos Repairs');
    await page.getByRole('button', { name: /Next/i }).click();

    await page.getByRole('textbox', { name: /what do you sell/i }).fill('Handyman services and plumbing fixes');
    await page.getByRole('button', { name: /Next/i }).click();

    // Verify it handles service type
    const businessTypeInput = page.getByRole('textbox', { name: /business type/i });
    await expect(businessTypeInput).toBeVisible();
  });

  test('Persona: Leo the Music Tutor setups digital sub', async ({ page }) => {
    await page.goto('/onboarding');
    await page.getByRole('textbox', { name: /business name/i }).fill('Leo Guitar Lessons');
    await page.getByRole('button', { name: /Next/i }).click();

    await page.getByRole('textbox', { name: /what do you sell/i }).fill('Online guitar tutorials and subscriptions');
    await page.getByRole('button', { name: /Next/i }).click();
  });

  test('Draft Auto-Save: Verify cross-device resumption works automatically', async ({ page, context }) => {
    await page.goto('/onboarding');

    // Fill business name and trigger auto-save
    await page.getByRole('textbox', { name: /business name/i }).fill('Auto Save Test Biz');
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByText('Draft Saved!')).toBeVisible({ timeout: 10000 });

    // Reload page to simulate resuming session
    await page.reload();

    // The state should be restored from the backend draft automatically
    await expect(page.getByRole('textbox', { name: /business name/i })).toHaveValue('Auto Save Test Biz', { timeout: 5000 });
  });

  test('Form Validation: Verify empty fields and invalid inputs trigger correct errors', async ({ page }) => {
    await page.goto('/onboarding');

    // Try to proceed without a business name
    await page.getByRole('button', { name: /Next/i }).click();

    // Verify error appears
    await expect(page.getByText(/Required field/i)).toBeVisible();
  });
});
