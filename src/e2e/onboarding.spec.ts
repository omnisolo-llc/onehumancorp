import { test, expect } from './fixtures';

test.describe('Non-Technical Small Business Owner Onboarding Flow (Day One)', () => {
  test.beforeEach(async ({ page }) => {
    const id = `business-setup-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
    }, id);
    await page.goto('/website-builder');
    await expect(page.locator('#setup-screen')).toBeVisible();
  });

  test('Persona: Maya - Goes from idea to live business', async ({ page }) => {
    // Welcome step
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
    await page.getByRole('button', { name: /Start My Business Next/ }).click();

    // Step: Business Type
    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible({ timeout: 10000 });
    await page.getByRole('button', { name: /Online Store/ }).click();

    // Step: Business Name
    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();

    // Test validation
    await page.getByPlaceholder('What is your business called?').fill('Ma');
    const nextBtn3 = page.locator('#step-3').getByRole('button', { name: /Next/ });
    await expect(nextBtn3).toBeVisible();
    await nextBtn3.click();

    // Ensure it hasn't progressed to the next step
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).not.toBeVisible();

    // Now fill properly
    await page.getByPlaceholder('What is your business called?').fill('Maya Bakery');
    await nextBtn3.click();

    // Step: What do you sell?
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByLabel(/Physical Products/).check();

    const nextBtn4 = page.locator('#step-4').getByRole('button', { name: /Next/ });
    await nextBtn4.click();

    // Step: Product Setup
    await page.getByPlaceholder('What is the name of this product?').fill('Custom Cookies');
    await page.getByPlaceholder('0.00').fill('24.99');

    const nextBtn5 = page.locator('#step-5').getByRole('button', { name: /Next/ });
    await nextBtn5.click();

    // Step: Payments
    await expect(page.getByRole('heading', { name: 'How do you want to receive payments?' })).toBeVisible();
    await page.getByRole('button', { name: 'Online', exact: true }).click();

    // Step: Account Setup
    const email = `maya+${Date.now()}@example.com`;
    await page.getByPlaceholder('e.g. Maya Smith').fill('Maya Smith');
    await page.getByPlaceholder('you@email.com').fill(email);
    await page.getByPlaceholder('Password').fill('password123');

    const nextBtn7 = page.locator('#step-7').getByRole('button', { name: /Next/ });
    await nextBtn7.click();

    // Step: Style & Team
    await page.getByRole('button', { name: 'Modern' }).click();

    const nextBtn8 = page.locator('#step-8').getByRole('button', { name: /Next/ });
    await nextBtn8.click();

    // Step: Domain Selection
    await page.getByRole('button', { name: /Free OHC Domain/ }).click();

    const nextBtn9 = page.locator('#step-9').getByRole('button', { name: /Next/ });
    await nextBtn9.click();

    // Publish
    await expect(page.getByRole('heading', { name: 'Review your choices' })).toBeVisible();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    // Success Step
    await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible({ timeout: 15000 });

    // Verify Next Steps
    const dashboardLink = page.getByRole('button', { name: /View Welcome Checklist/i });
    await expect(dashboardLink).toBeVisible();
    await dashboardLink.click();

    await expect(page.getByText("You're set up! Here's what to do next:")).toBeVisible();
  });
});
