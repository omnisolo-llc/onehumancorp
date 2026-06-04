import { test, expect } from './fixtures';

test.describe('Real Business Owner - Maya the Baker (Physical Products)', () => {
  test.beforeEach(async ({ page }) => {
    const id = `maya-baker-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
      localStorage.removeItem('onboarding-storage-v3');
    }, id);
    await page.goto('/website-builder');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('#setup-screen')).toBeVisible();
  });

  test('Maya completes the onboarding flow for a custom cake shop', async ({ page }) => {


    // Step 0: Start
    await page.getByRole('button', { name: /Start My Business/ }).click();

    // Step 1: Business Type (Online Store)
    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
    await page.getByRole('button', { name: /Online Store/ }).click();

    // Step 2: Name and description
    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    await page.getByPlaceholder('What is your business called?').fill('Maya\'s Custom Cakes');
    await page.locator('input[placeholder="e.g. Maya\'s Cakes"]').waitFor({ state: 'visible', timeout: 10000 });
    await page.getByPlaceholder("e.g. Maya's Cakes").fill('Custom vegan cakes for special occasions');
    await page.locator('#step-3').getByRole('button', { name: /Next/ }).click();

    // Step 3: What do you sell (Physical)
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByLabel(/Physical Products/).check();
    await page.locator('#step-4').getByRole('button', { name: /Next/ }).click();

    // Step 4: First product
    await page.getByPlaceholder('What is the name of this product?').fill('Vegan Chocolate Cake');
    await page.getByPlaceholder('0.00').fill('45.00');
    await page.locator('#step-5').getByRole('button', { name: /Next/ }).click();

    // Step 5: Payments
    await expect(page.getByRole('heading', { name: 'How do you want to receive payments?' })).toBeVisible();
    await page.getByRole('button', { name: 'Online', exact: true }).click();

    // Step 6: Admin details
    const email = `maya+${Date.now()}@example.com`;
    await page.getByPlaceholder('e.g. Maya Smith').fill('Maya Smith');
    await page.getByPlaceholder('you@email.com').fill(email);
    await page.getByPlaceholder('Password').fill('SecurePassword123!');
    await page.locator('#step-7').getByRole('button', { name: /Next/ }).click();

    // Step 7: Template
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.locator('#step-8').getByRole('button', { name: /Next/ }).click();

    // Step 8: Domain
    await page.getByRole('button', { name: /Free OHC Domain/ }).click();
    await page.locator('#step-9').getByRole('button', { name: /Next/ }).click();

    // Step 9: Launch
    await page.getByRole('button', { name: /Publish my business/ }).click();

    // Verify Success and Welcome Checklist
    await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible();
    await page.getByRole('button', { name: /View Welcome Checklist/ }).click();

    // Should navigate to dashboard
    await expect(page).toHaveURL(/\/dashboard/);
    await expect(page.getByText("You're set up! Here's what to do next:")).toBeVisible();
  });
});
