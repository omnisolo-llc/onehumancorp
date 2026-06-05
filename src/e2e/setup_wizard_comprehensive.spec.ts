import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the new instant build flow', async ({ page }) => {
<<<<<<< HEAD
=======
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    const id = `setup-comprehensive-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
      localStorage.removeItem('onboarding-storage-v3');
    }, id);

    // We only have the instant build flow now.
    await page.goto('/website-builder');
    await page.waitForLoadState('networkidle');

<<<<<<< HEAD
    await page.getByRole('button', { name: /Instant Build/ }).click();
    await page.getByPlaceholder('e.g. I run a local bakery').fill('I run a modern art shop online');
=======
    await page.getByRole('button', { name: /Start My Business/ }).click();
    await page.getByPlaceholder('e.g. I run a mobile dog grooming service in Portland').fill('I run a modern art shop online');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await page.getByRole('button', { name: /Generate Storefront/ }).click();

    await expect(page.getByText('Agents are building your store...')).toBeVisible({ timeout: 10000 });
    await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible({ timeout: 20000 });
  });

  test('validates empty input in Tell us about your business', async ({ page }) => {
<<<<<<< HEAD
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Instant Build/ }).click();
=======
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business/ }).click();
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))

    // The textarea starts empty
    const generateBtn = page.getByRole('button', { name: /Generate Storefront/ });
    await expect(generateBtn).toBeDisabled();

<<<<<<< HEAD
    await page.getByPlaceholder('e.g. I run a local bakery').fill('A');
=======
    await page.getByPlaceholder('e.g. I run a mobile dog grooming service in Portland').fill('A');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await expect(generateBtn).toBeEnabled();
  });
});
