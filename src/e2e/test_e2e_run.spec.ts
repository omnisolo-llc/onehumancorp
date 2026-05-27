import { test, expect } from './fixtures';

test('verify wizard UI state propagation to dashboard', async ({ page }) => {
  await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "State Test Store", business_type: "Online Store", categories: ["physical"], initial_products: [{ name: "Test Product", price: "10.00" }] }) }));
  await page.goto('/website-builder');
  await page.getByPlaceholder('e.g. I bake custom vegan cakes').fill('State Test Store');
  await page.getByRole('button', { name: /Generate Storefront/ }).click();
  await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();
  await expect(page.getByDisplayValue('State Test Store')).toBeVisible();
});

test('verify app settings toggle', async ({ page }) => {
  await page.goto('/dashboard');
  await page.getByRole('button', { name: 'Settings', exact: true }).click();
  await page.getByLabel('Enable Email Notifications').check();
  await expect(page.getByLabel('Enable Email Notifications')).toBeChecked();
});

test('verify checklist and referral routing', async ({ page }) => {
  await page.goto('/dashboard');
  await page.getByRole('button', { name: 'Referrals' }).click();
  await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
  await expect(page.locator('#referral-link')).toContainText('ohc://join?ref=DEFAULT');
});

test('verify website builder publish sheet', async ({ page }) => {
  await page.goto('/storefront-builder');
  await page.getByRole('button', { name: 'Publish Changes' }).click();
  await expect(page.getByRole('heading', { name: 'Publish Site' })).toBeVisible();
  await expect(page.getByRole('button', { name: /Free OHC Subdomain/ })).toBeVisible();
});

test('verify state persistence', async ({ page }) => {
  await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Test Company", business_type: "Online Store", categories: ["physical"], initial_products: [{ name: "Custom Cookies", price: "24.99" }] }) }));

  await page.goto('/website-builder');
  await page.getByPlaceholder('e.g. I bake custom vegan cakes').fill('Test Company sells custom cookies');
  await page.getByRole('button', { name: /Generate Storefront/ }).click();
  await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();

  // Reload the page and verify we're still on the company name step
  await page.reload();
  await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();
});
