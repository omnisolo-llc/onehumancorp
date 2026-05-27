import { test, expect } from './fixtures';

test.describe('Business Setup Wizard - Part 2', () => {

  test('captures product details in the guided setup', async ({ page }) => {
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Maya Cakes", business_type: "Restaurant", categories: ["physical"], initial_products: [{ name: "Custom Vegan Cookies", price: "24.99" }] }) }));

    await page.goto('/website-builder');
    await page.getByPlaceholder('e.g. I bake custom vegan cakes').fill('Maya Cakes sells custom vegan cookies for 24.99');
    await page.getByRole('button', { name: /Generate Storefront/ }).click();

    await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();
    await expect(page.getByDisplayValue('Custom Vegan Cookies')).toBeVisible();
    await expect(page.getByDisplayValue('24.99')).toBeVisible();
  });

});
