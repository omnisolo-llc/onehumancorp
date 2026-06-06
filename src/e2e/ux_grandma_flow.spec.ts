import { test, expect } from './fixtures';

test.describe('Grandmother UX End-to-End Flow Validation', () => {
  test('first-time user sees plain language dashboard headers', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByText('Welcome back, Human.')).toBeVisible();
    await expect(page.getByText('Your agents are working on your behalf.')).toBeVisible();
  });

  test('quick actions expose guidance and custom software', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Start Tour' }).click();
    await expect(page.getByRole('dialog').getByText('Business Analytics')).toBeVisible();
    await page.getByRole('link', { name: 'Integrations' }).click();
    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
  });

  test('login setup action opens the guided setup process', async ({ page }) => {
    await page.goto('/login');
    await page.getByRole('button', { name: /Start Business Setup/ }).click();
    await expect(page.getByRole('heading', { name: 'Setup' })).toBeVisible();
  });
});
