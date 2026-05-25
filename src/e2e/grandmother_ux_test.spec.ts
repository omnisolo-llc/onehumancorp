import { test, expect } from './fixtures';

test.describe('Grandmother UX Fixes E2E tests', () => {
  test.setTimeout(30000); // Enforce 30-second completion metric for core CUJ

  test.setTimeout(30000); // Enforce 30-second completion metric for core CUJ

  test('login screen uses plain language labels', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByText('One Human Corp')).toBeVisible();
    await expect(page.getByText('Sign in to manage your business')).toBeVisible();
    await expect(page.getByRole('button', { name: /Start Business Setup/ })).toBeVisible();
  });

  test('custom software screen uses plain language for external tools', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByText('Connect Tools').click();

    await expect(page.getByRole('heading', { name: 'Connect Custom Software' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Custom Integration' })).toBeVisible();
    await expect(page.getByText('Read Product List')).toBeVisible();
  });
});
