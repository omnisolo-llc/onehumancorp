import { test, expect } from './fixtures';

test.describe('Business Setup Wizard', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('#setup-screen')).toBeVisible();
  });

  test('shows the current setup welcome step', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
    await expect(page.getByRole('button', { name: /Generate Storefront/ })).toBeVisible();
  });

  test('moves through intake and review steps', async ({ page }) => {
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Test Company", business_type: "Online Store", categories: ["physical"], initial_products: [{ name: "Custom Cookies", price: "24.99" }] }) }));

    await page.getByPlaceholder('e.g. I bake custom vegan cakes').fill('Test Company sells custom cookies');
    await page.getByRole('button', { name: /Generate Storefront/ }).click();

    await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();
    await expect(page.getByDisplayValue('Test Company')).toBeVisible();
    await expect(page.getByDisplayValue('Online Store')).toBeVisible();
  });

  test('completes the publish path to the checklist', async ({ page }) => {
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Test Company", business_type: "Online Store", categories: ["physical"], initial_products: [{ name: "Custom Cookies", price: "24.99" }] }) }));
    await page.route('**/api/onboarding/start', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched.", organization_id: "org_123" }) }));

    // Step 1
    await page.getByPlaceholder('e.g. I bake custom vegan cakes').fill('Test Company sells custom cookies');
    await page.getByRole('button', { name: /Generate Storefront/ }).click();

    // Step 2
    await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();
    await page.getByRole('button', { name: /Continue/ }).click();

    // Step 3
    await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.getByRole('button', { name: /Launch Store/ }).click();

    // Step 5
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });
});
