import { test, expect } from './fixtures';

test.describe('Grandmother UX End-to-End Flow Validation', () => {
  test('first-time user sees plain language dashboard headers', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/dashboard');
    await expect(page.getByText('Welcome back, Human.')).toBeVisible();
    await expect(page.getByText('Your AI assistants are working on your behalf.')).toBeVisible();
  });

  test('quick actions expose guidance and custom software', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/dashboard');
    await page.getByRole('button', { name: '?' }).click();
    await expect(page.locator('#quick-actions-hint')).toBeVisible();
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
  });

  test('login setup action opens the guided setup process', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/login');
    await page.getByRole('button', { name: /Start Business Setup/ }).click();
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
  });
});
