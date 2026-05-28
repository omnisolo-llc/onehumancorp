import { test, expect } from './fixtures';

test('lens audit: verifies current login error states', async ({ page }) => {
  await page.goto('/login');
  await expect(page.getByText('One Human Corp')).toBeVisible();
  await expect(page.locator('#login-error')).toContainText("Oops! We couldn't sign you in.");
  await expect(page.getByRole('button', { name: /Login/ }).first()).toBeVisible();
});

test('lens audit: verifies sign up mode', async ({ page }) => {
  await page.goto('/login');
  await page.getByRole('button', { name: /Sign Up/ }).click();
  await expect(page.getByText('Create an account to start your business')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Have an account? Sign In' })).toBeVisible();
});

test('lens audit: verifies password toggle and setup routing', async ({ page }) => {
  await page.goto('/login');
  const password = page.getByPlaceholder('Password');
  await expect(password).toHaveAttribute('type', 'password');
  await page.getByRole('button', { name: 'Show' }).click();
  await expect(password).toHaveAttribute('type', 'text');

  await page.getByRole('button', { name: /Start Business Setup/ }).click();
  await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
});
