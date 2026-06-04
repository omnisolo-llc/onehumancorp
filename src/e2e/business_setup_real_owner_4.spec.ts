import { test, expect } from './fixtures';

test.describe('Real Business Owner - Leo the Music Tutor (Digital/Booking)', () => {
  test.beforeEach(async ({ page }) => {
    const id = `leo-tutor-${Date.now()}-${Math.random()}`;
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

  test('Leo completes the onboarding flow for online music lessons', async ({ page }) => {


    // Step 0: Start
    await page.getByRole('button', { name: /Start My Business/ }).click();

    // Step 1: Business Type (Creator / Influencer)
    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
    await page.getByRole('button', { name: /Creator \/ Influencer/ }).click();

    // Step 2: Name and description
    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    await page.getByPlaceholder('What is your business called?').fill('Leo\'s Guitar Lessons');
    await page.locator('input[placeholder="e.g. Maya\'s Cakes"]').waitFor({ state: 'visible', timeout: 10000 });
    await page.getByPlaceholder("e.g. Maya's Cakes").fill('Online and in-person guitar tutoring');
    await page.locator('#step-3').getByRole('button', { name: /Next/ }).click();

    // Step 3: What do you sell (Digital & Services)
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByLabel(/Digital Products/).check();
    await page.getByLabel(/Services/).check(); // Can select multiple in theory, but let's assume primary
    await page.locator('#step-4').getByRole('button', { name: /Next/ }).click();

    // Step 4: First product/service
    await page.getByPlaceholder('What is the name of this product?').fill('1-Hour Guitar Lesson');
    await page.getByPlaceholder('0.00').fill('40.00');
    await page.locator('#step-5').getByRole('button', { name: /Next/ }).click();

    // Step 5: Payments
    await expect(page.getByRole('heading', { name: 'How do you want to receive payments?' })).toBeVisible();
    await page.getByRole('button', { name: 'Online', exact: true }).click();

    // Step 6: Admin details
    const email = `leo+${Date.now()}@example.com`;
    await page.getByPlaceholder('e.g. Maya Smith').fill('Leo Martinez');
    await page.getByPlaceholder('you@email.com').fill(email);
    await page.getByPlaceholder('Password').fill('Guitar123!');
    await page.locator('#step-7').getByRole('button', { name: /Next/ }).click();

    // Step 7: Template
    await page.getByRole('button', { name: 'Playful' }).click(); // Fun template for creator
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
  });
});
