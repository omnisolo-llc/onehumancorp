import { test, expect } from './fixtures';

test('Maya operates her custom cake business', async ({ page }) => {
  const id = `operate-business-${Date.now()}-${Math.random()}`;
  const email = `maya+${Date.now()}@example.com`;
  await page.addInitScript((tenantId) => {
    localStorage.setItem('tenant_id', tenantId);
    localStorage.setItem('user_id', tenantId);
    localStorage.removeItem('ohc_wizard_state');
  }, id);

  await page.goto('/website-builder');

  await page.getByRole('button', { name: /Start My Business/ }).click();
  await page.getByRole('button', { name: /Online Store/ }).click();
  await page.getByPlaceholder('What is your business called?').fill('Maya Bakery');
  await page.getByPlaceholder("e.g. Maya's Cakes").fill('Custom cakes and pastries');
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
  await page.locator('#step-7').getByRole('button', { name: /Next/ }).first().click();

  await page.getByRole('button', { name: 'Modern' }).click();
  await page.locator('#step-8').getByRole('button', { name: /Next/ }).first().click();
  await page.getByRole('button', { name: /Free OHC Domain/ }).click();
  await page.locator('#step-9').getByRole('button', { name: /Next/ }).first().click();
  await page.getByRole('button', { name: /Publish my business/ }).click();

  await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible();
  await page.getByRole('button', { name: /Launch My Business/ }).click();

  await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
});


test('Maya adds a new Vegan Cake product to her store', async ({ page }) => {
  const id = `operate-business-${Date.now()}-${Math.random()}`;
  const email = `maya+${Date.now()}@example.com`;

  // First create the business using the UI flow to get a valid logged in state
  await page.goto('/website-builder');

  await page.getByRole('button', { name: /Start My Business/ }).click();
  await page.getByRole('button', { name: /Online Store/ }).click();
  await page.getByPlaceholder('What is your business called?').fill('Maya Bakery');
  await page.getByPlaceholder("e.g. Maya's Cakes").fill('Custom cakes and pastries');
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
  await page.locator('#step-7').getByRole('button', { name: /Next/ }).first().click();

  await page.getByRole('button', { name: 'Modern' }).click();
  await page.locator('#step-8').getByRole('button', { name: /Next/ }).first().click();
  await page.getByRole('button', { name: /Free OHC Domain/ }).click();
  await page.locator('#step-9').getByRole('button', { name: /Next/ }).first().click();
  await page.getByRole('button', { name: /Publish my business/ }).click();

  await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible();
  await page.getByRole('button', { name: /Launch My Business/ }).click();

  await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

  // Now navigate to Products
  await page.getByRole('link', { name: 'Products' }).click();

  // Click Add Product
  await page.getByRole('button', { name: 'Add Product' }).click();

  // Fill in the product details
  await page.getByLabel('Product Name').fill('Vegan Chocolate Cake');
  await page.getByLabel('Price').fill('85.00');
  await page.getByLabel('Description').fill('A delicious custom vegan chocolate cake.');

  // Save the product
  await page.getByRole('button', { name: 'Save' }).click();

  // Verify product appears in the list
  await expect(page.getByText('Vegan Chocolate Cake')).toBeVisible();
  await expect(page.getByText('$85.00')).toBeVisible();
});
