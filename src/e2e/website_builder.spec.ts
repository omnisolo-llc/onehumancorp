import { currentAppSmoke } from './current_app_smoke';
import { test, expect } from './fixtures';

currentAppSmoke('website_builder');

test('allows submitting forms by pressing Enter', async ({ page }) => {
  const id = `setup-keyboard-${Date.now()}-${Math.random()}`;
  const email = `alex+${Date.now()}@example.com`;
  await page.addInitScript((tenantId) => {
    localStorage.setItem('tenant_id', tenantId);
    localStorage.setItem('user_id', tenantId);
    localStorage.removeItem('ohc_wizard_state');
  }, id);
  await page.goto('/website-builder');

  await page.waitForLoadState('networkidle');

  // Step 0: Welcome
  await page.getByRole('button', { name: /Start My Business/ }).click();

  // Step 1: Online Store
  await page.getByRole('button', { name: /Online Store/ }).click();

  // Step 2: Name
  await page.getByPlaceholder('What is your business called?').fill('Alex Art');
  // Submit via Enter instead of clicking Next
  await page.getByPlaceholder('What is your business called?').press('Enter');

  // Step 3: Product Types
  await page.getByLabel(/Physical Products/).check();
  await page.locator('#step-4').getByRole('button', { name: /Next/ }).click(); // Checkbox step doesn't have a specific input to press Enter on, click next

  // Step 4: First Product
  await page.getByPlaceholder('What is the name of this product?').fill('Custom Print');
  await page.getByPlaceholder('0.00').fill('49.00');
  // Submit via Enter
  await page.getByPlaceholder('0.00').press('Enter');

  // Step 5: Payments
  await page.getByRole('button', { name: 'Online', exact: true }).click();

  // Step 6: Account
  await page.getByPlaceholder('e.g. Maya Smith').fill('Alex Art');
  await page.getByPlaceholder('you@email.com').fill(email);
  await page.getByPlaceholder('Password').fill('password123');
  // Submit via Enter
  await page.getByPlaceholder('Password').press('Enter');

  // Step 7: Theme
  await page.getByRole('button', { name: 'Modern' }).click();
  await page.locator('#step-8').getByRole('button', { name: /Next/ }).click();

  // Step 8: Domain
  await page.getByRole('button', { name: /Free OHC Domain/ }).click();
  await page.locator('#step-9').getByRole('button', { name: /Next/ }).click();

  // Step 9: Launch
  await page.getByRole('button', { name: /Publish my business/ }).click();

  await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible();
});

test('allows submitting instant build via Enter', async ({ page }) => {
  const id = `setup-keyboard-instant-${Date.now()}-${Math.random()}`;
  await page.addInitScript((tenantId) => {
    localStorage.setItem('tenant_id', tenantId);
    localStorage.setItem('user_id', tenantId);
    localStorage.removeItem('ohc_wizard_state');
  }, id);
  await page.goto('/website-builder');

  await page.waitForLoadState('networkidle');

  // Step 0: Welcome
  await page.getByRole('button', { name: /Instant Build/ }).click();

  // Instant build
  await page.getByPlaceholder('e.g. I run a local bakery').fill('I run a local bakery');
  // Textarea handles Enter without shift
  await page.getByPlaceholder('e.g. I run a local bakery').press('Enter');

  await expect(page.getByText('Agents are building your store...')).toBeVisible();
  await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible({ timeout: 5000 });
});
