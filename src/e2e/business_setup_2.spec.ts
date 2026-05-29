import { test, expect } from './fixtures';

test.describe('Business Setup Wizard - Part 2', () => {
  test('disables the build button when text is too short', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' })).toBeVisible();

    const buildButton = page.getByRole('button', { name: /Build My Storefront/ });
    await expect(buildButton).toBeDisabled();

    await page.getByPlaceholder('e.g. I run a mobile dog grooming service in Portland').fill('test');
    await expect(buildButton).toBeDisabled();

    await page.getByPlaceholder('e.g. I run a mobile dog grooming service in Portland').fill('longer test bio that should enable');
    await expect(buildButton).toBeEnabled();
  });
});
