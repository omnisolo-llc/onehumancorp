import { test, expect } from './fixtures';

test.describe('Business Setup Wizard - Part 2', () => {
  test('supports the instant build entry point', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Instant Build/ }).click();

    await expect(page.getByRole('heading', { name: 'Describe your business in a sentence' })).toBeVisible();
    await page.getByPlaceholder(/I run a local bakery/).fill('I run a local bakery called Maya Cakes.');
    await expect(page.getByRole('button', { name: /Generate Storefront/ })).toBeVisible();
  });

  test('captures product details in the guided setup', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Restaurant/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Maya Cakes');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByText('Physical Products').click();
    await page.getByRole('button', { name: /Next/ }).click();

    await page.getByPlaceholder('What is the name of this product?').fill('Custom Vegan Cookies');
    await page.getByPlaceholder('0.00').fill('24.99');
    await expect(page.getByPlaceholder('What is the name of this product?')).toHaveValue('Custom Vegan Cookies');
    await expect(page.getByPlaceholder('0.00')).toHaveValue('24.99');
  });

  test('shows domain choices before launch', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Online Store/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Test Store');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByText('Digital Products').click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByPlaceholder('What is the name of this product?').fill('Template Pack');
    await page.getByPlaceholder('0.00').fill('19.00');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Online', exact: true }).click();
    await page.getByPlaceholder('e.g. Maya Smith').fill('Alex Smith');
    await page.getByPlaceholder('you@email.com').fill('alex@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Bold' }).click();
    await page.getByRole('button', { name: /Next/ }).click();

    await expect(page.getByRole('heading', { name: 'Choose your domain' })).toBeVisible();
    await expect(page.getByRole('button', { name: /Free OHC Domain/ })).toBeVisible();
    await expect(page.getByRole('button', { name: /Connect Custom Domain/ })).toBeVisible();
  });
});
