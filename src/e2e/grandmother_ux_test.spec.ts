import { test, expect } from './fixtures';

test.describe('Grandmother UX Fixes E2E tests', () => {
  test('login screen uses plain language labels', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'One Human Corp' })).toBeVisible();
    await expect(page.getByText('Sign in to manage your business')).toBeVisible();
    await expect(page.getByRole('button', { name: /Start Business Setup/ })).toBeVisible();
  });

  test('custom software screen uses plain language for external tools', async ({ page }) => {
    await page.goto('/integrations');

    await expect(page.getByRole('heading', { name: 'Connect Custom Software' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Social Media Accounts' })).toBeVisible();
    await expect(page.getByText('Manage all your social media messages and posts in one place.')).toBeVisible();
  });
});
