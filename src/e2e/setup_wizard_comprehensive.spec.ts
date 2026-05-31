import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    const id = `setup-comprehensive-${Date.now()}-${Math.random()}`;
    const email = `alex+${Date.now()}@example.com`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
    }, id);
    await page.goto('/onboarding');

    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Online Store/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Alex Art');

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
    await page.locator('#step-7').getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.locator('#step-8').getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Free OHC Domain/ }).click();
    await page.locator('#step-9').getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible();
  });
});
