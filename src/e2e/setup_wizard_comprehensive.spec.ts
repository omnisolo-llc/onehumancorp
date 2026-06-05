import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the new instant build flow', async ({ page }) => {
    const id = `setup-comprehensive-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
      localStorage.removeItem('onboarding-storage-v3');
    }, id);

<<<<<<< HEAD
    // We only have the instant build flow now.
    await page.goto('/website-builder');
    await page.waitForLoadState('networkidle');

    await page.getByRole('button', { name: /Instant Build/ }).click();
    await page.getByPlaceholder('e.g. I run a local bakery').fill('I run a modern art shop online');
    await page.getByRole('button', { name: /Generate Storefront/ }).click();
=======
    await page.waitForLoadState('networkidle', { timeout: 30000 }).catch(() => {});

    await page.getByRole('button', { name: /Start My Business/ }).click();
    await page.getByRole('button', { name: /Online Store/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Alex Art');
    await page.locator('input[placeholder*="Maya"]').nth(1).fill('Original art and prints');
    await page.locator('#step-3').getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Physical Products/).check();
    await page.locator('#step-4').getByRole('button', { name: /Next/ }).click();
    await page.getByPlaceholder('What is the name of this product?').fill('Custom Print');
    await page.getByPlaceholder('0.00').fill('49.00');
    await page.locator('#step-5').getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Online', exact: true }).click();
    await page.getByPlaceholder('e.g. Maya Smith').fill('Alex Art');
    await page.getByPlaceholder('you@email.com').fill(email);
    await page.getByPlaceholder('Password').fill('password123');
    await page.locator('#step-7').getByRole('button', { name: /Next/ }).first().click();
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.locator('#step-8').getByRole('button', { name: /Next/ }).first().click();
    await page.getByRole('button', { name: /Free OHC Domain/ }).click();
    await page.locator('#step-9').getByRole('button', { name: /Next/ }).first().click();
    await page.getByRole('button', { name: /Publish my business/ }).click();
>>>>>>> aa3e6e91 (fix(onboarding): ensure cross-device persistence with backend hydration)

    await expect(page.getByText('Agents are building your store...')).toBeVisible({ timeout: 10000 });
    await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible({ timeout: 20000 });
  });

  test('validates empty input in Tell us about your business', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Instant Build/ }).click();

    // The textarea starts empty
    const generateBtn = page.getByRole('button', { name: /Generate Storefront/ });
    await expect(generateBtn).toBeDisabled();

    await page.getByPlaceholder('e.g. I run a local bakery').fill('A');
    await expect(generateBtn).toBeEnabled();
  });
});
