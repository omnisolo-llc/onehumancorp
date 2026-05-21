import { test, expect } from './fixtures';

test.describe('Onboarding Form Keyboard Configuration', () => {
  test('verifies keyboard attributes are present in the DOM', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('#setup-screen')).toBeVisible();

    // Verify Business Name inputs
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Online Store/ }).click();

    const businessNameInput = page.getByPlaceholder('What is your business called?');
    await expect(businessNameInput).toHaveAttribute('autocomplete', 'organization');
    await expect(businessNameInput).toHaveAttribute('enterkeyhint', 'next');

    // Verify Product inputs
    await businessNameInput.fill('Test Company');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Physical Products/).check();
    await page.getByRole('button', { name: /Next/ }).click();

    const productNameInput = page.getByPlaceholder('What is the name of this product?');
    await expect(productNameInput).toHaveAttribute('enterkeyhint', 'next');

    const productPriceInput = page.getByPlaceholder('0.00');
    await expect(productPriceInput).toHaveAttribute('inputmode', 'decimal');
    await expect(productPriceInput).toHaveAttribute('enterkeyhint', 'next');

    // Verify Account inputs
    await productNameInput.fill('Custom Cookies');
    await productPriceInput.fill('24.99');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Online', exact: true }).click();

    const nameInput = page.getByPlaceholder('e.g. Maya Smith');
    await expect(nameInput).toHaveAttribute('autocomplete', 'name');
    await expect(nameInput).toHaveAttribute('enterkeyhint', 'next');

    const emailInput = page.getByPlaceholder('you@email.com');
    await expect(emailInput).toHaveAttribute('autocomplete', 'email');
    await expect(emailInput).toHaveAttribute('enterkeyhint', 'next');

    const passwordInput = page.getByPlaceholder('Password');
    await expect(passwordInput).toHaveAttribute('autocomplete', 'new-password');
    await expect(passwordInput).toHaveAttribute('enterkeyhint', 'done');
  });

  test('verifies keyboard attributes for AI Instant Build', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Instant Build/ }).click();

    const aiInput = page.getByPlaceholder(/I run a local bakery/);
    await expect(aiInput).toHaveAttribute('enterkeyhint', 'done');
  });
});
