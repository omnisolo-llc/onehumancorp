import { test, expect } from './fixtures';

test('Maya operates her custom cake business', async ({ page }) => {
  const id = `operate-business-${Date.now()}-${Math.random()}`;
  const email = `maya+${Date.now()}@example.com`;
  await page.addInitScript((tenantId) => {
    localStorage.setItem('tenant_id', tenantId);
    localStorage.setItem('user_id', tenantId);
    localStorage.removeItem('ohc_wizard_state');
  }, id);

  await page.goto('/onboarding');

  await page.getByRole('button', { name: /Start My Business Next/ }).click();
  await page.getByRole('button', { name: /Online Store/ }).click();
  await page.getByPlaceholder('What is your business called?').fill('Maya Bakery');

  await page.locator('#step-3').getByRole('button', { name: /Next/ }).click();

  await page.getByLabel(/Physical Products/).check();
  await page.locator('#step-4').getByRole('button', { name: /Next/ }).click();
  await page.getByPlaceholder('What is the name of this product?').fill('Custom Cake');
  await page.getByPlaceholder('0.00').fill('75.00');
  await page.locator('#step-5').getByRole('button', { name: /Next/ }).click();

  await page.getByRole('button', { name: 'Online', exact: true }).click();
  await page.getByPlaceholder('e.g. Maya Smith').fill('Maya Baker');
  await page.getByPlaceholder('you@email.com').fill(email);
  await page.getByPlaceholder('Password').fill('password123');
  await page.locator('#step-7').getByRole('button', { name: /Next/ }).click();

  await page.getByRole('button', { name: 'Modern' }).click();
  await page.locator('#step-8').getByRole('button', { name: /Next/ }).click();
  await page.getByRole('button', { name: /Free OHC Domain/ }).click();
  await page.locator('#step-9').getByRole('button', { name: /Next/ }).click();
  await page.getByRole('button', { name: /Publish my business/ }).click();

  await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible();
  await page.getByRole('button', { name: /Launch My Business/ }).click();

  await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
});
